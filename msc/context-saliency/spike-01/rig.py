"""Spike-01 measurement rig.

Prefill the task prompt with attentions OFF (the n-by-n cost lives only in prefill,
and eager prefill at ~1K tokens is trivial anyway); then decode greedily step by
step with output_attentions ON — each step's attention is one row [L, H, 1, n],
aggregated to per-segment mass immediately and discarded. Also records per-step
hidden-state L2 diffs (EpiKV T1 signal) for the generated tokens, and top-K
attended context positions for heavy-hitter turnover.

Everything is (context, query)-conditional by construction: the matrix is
per-decision-step. Outputs a JSON bundle per run.
"""

import json
import sys
import time

import torch
from transformers import AutoModelForCausalLM, AutoTokenizer

from tasks import make_task, occlude

import os
MODEL_ID = os.environ.get("SPIKE_MODEL", "Qwen/Qwen2.5-7B-Instruct")
DEVICE = "mps" if torch.backends.mps.is_available() else "cpu"
TOPK_HH = 16          # heavy-hitter set size
MAX_NEW = 400

_model = None
_tok = None


def load():
    global _model, _tok
    if _model is None:
        _tok = AutoTokenizer.from_pretrained(MODEL_ID)
        _model = AutoModelForCausalLM.from_pretrained(
            MODEL_ID, dtype=torch.bfloat16,   # bf16-native weights overflow in fp16 at 7B scale ("!" storms)
            attn_implementation="eager",       # required for output_attentions in decode
        ).to(DEVICE).eval()
    return _model, _tok


def build_inputs(prompt_text, segments):
    """Chat-format the task, tokenize with offsets, and map segment char spans
    to token index lists. Returns input_ids, seg_token_lists, formatted string."""
    model, tok = load()
    messages = [{"role": "user", "content": prompt_text}]
    kw = {}
    if "Qwen3" in MODEL_ID:
        kw["enable_thinking"] = False  # controlled non-thinking derivations for the spike
    formatted = tok.apply_chat_template(messages, tokenize=False, add_generation_prompt=True, **kw)
    base = formatted.index(prompt_text)  # prompt embedded verbatim
    enc = tok(formatted, return_offsets_mapping=True, return_tensors="pt", add_special_tokens=False)
    offsets = enc.pop("offset_mapping")[0].tolist()
    seg_tokens = []
    for seg in segments:
        s, e = seg.start + base, seg.end + base
        toks = [i for i, (a, b) in enumerate(offsets) if a < e and b > s and b > a]
        seg_tokens.append(toks)
    return enc["input_ids"], seg_tokens, formatted


@torch.no_grad()
def run_instrumented(prompt_text, segments, max_new=MAX_NEW):
    model, tok = load()
    input_ids, seg_tokens, _ = build_inputs(prompt_text, segments)
    input_ids = input_ids.to(DEVICE)
    n_ctx = input_ids.shape[1]
    n_layers = model.config.num_hidden_layers

    # membership matrix: [n_segments, n_ctx] float16 for fast segment aggregation
    memb = torch.zeros(len(seg_tokens), n_ctx, dtype=torch.float32, device=DEVICE)
    for i, toks in enumerate(seg_tokens):
        if toks:
            memb[i, toks] = 1.0

    t0 = time.time()
    out = model(input_ids=input_ids, use_cache=True, output_hidden_states=True)
    past = out.past_key_values
    prev_hidden = torch.stack([h[0, -1].float() for h in out.hidden_states])  # [L+1, d]
    next_id = out.logits[0, -1].argmax()

    steps = []          # per-step dicts
    gen_ids = []
    for step in range(max_new):
        gen_ids.append(next_id.item())
        out = model(input_ids=next_id.view(1, 1), past_key_values=past,
                    use_cache=True, output_attentions=True, output_hidden_states=True)
        past = out.past_key_values

        # --- T2-lite: per-layer, head-mean attention row over the ORIGINAL context ---
        # attn[l]: [1, H, 1, n_total]; restrict to first n_ctx positions (the prompt)
        rows = torch.stack([a[0, :, 0, :n_ctx].float().mean(0) for a in out.attentions])  # [L, n_ctx]
        seg_mass = rows @ memb.T                       # [L, n_segments]
        mean_row = rows.mean(0)                        # [n_ctx] all-layer mean
        hh = torch.topk(mean_row, min(TOPK_HH, n_ctx)).indices.tolist()

        # --- T1: hidden-state L2 diffs for this generated token ---
        hidden = torch.stack([h[0, -1].float() for h in out.hidden_states])   # [L+1, d]
        g = (hidden - prev_hidden).norm(dim=-1)        # [L+1]
        prev_hidden = hidden

        steps.append({
            "tok": tok.decode([next_id.item()]),
            "seg_mass": seg_mass.cpu().tolist(),       # [L][n_segments]
            "ctx_mass_total": float(mean_row.sum()),   # share of attention on prompt vs generated
            "hh": hh,
            "g": g.cpu().tolist(),
        })
        next_id = out.logits[0, -1].argmax()
        if next_id.item() == tok.eos_token_id:
            break
        text_so_far = tok.decode(gen_ids)
        if "MASTER:" in text_so_far and "\n" in text_so_far.split("MASTER:")[-1]:
            break

    gen_text = tok.decode(gen_ids)
    return {
        "n_ctx": n_ctx, "gen_text": gen_text, "n_steps": len(steps),
        "seconds": round(time.time() - t0, 1),
        "seg_roles": [(s.role, s.room, bool(s.reused_detail), len(t))
                      for s, t in zip(segments, seg_tokens)],
        "steps": steps,
    }


@torch.no_grad()
def run_plain(prompt_text, max_new=MAX_NEW):
    """Cheap uninstrumented greedy run (for occlusion sweeps)."""
    model, tok = load()
    messages = [{"role": "user", "content": prompt_text}]
    kw = {"enable_thinking": False} if "Qwen3" in MODEL_ID else {}
    ids = tok.apply_chat_template(messages, add_generation_prompt=True, return_tensors="pt", **kw).to(DEVICE)
    out = model.generate(ids, max_new_tokens=max_new, do_sample=False,
                         pad_token_id=tok.eos_token_id)
    return tok.decode(out[0, ids.shape[1]:], skip_special_tokens=True)


def extract_master(text):
    for line in reversed(text.strip().splitlines()):
        if "MASTER:" in line:
            digits = "".join(c for c in line.split("MASTER:")[-1] if c.isdigit())
            return int(digits) if digits else None
    return None


if __name__ == "__main__":
    which = sys.argv[1] if len(sys.argv) > 1 else "smoke"
    if which == "smoke":
        t = make_task("screened", n_rooms=4, seed=7)
        r = run_instrumented(t.prompt, t.segments, max_new=350)
        print("gen:", r["gen_text"][-200:])
        print("steps:", r["n_steps"], "ctx tokens:", r["n_ctx"], "sec:", r["seconds"])
        print("answer expected:", t.answer, "got:", extract_master(r["gen_text"]))
        json.dump(r, open("out_smoke.json", "w"))
