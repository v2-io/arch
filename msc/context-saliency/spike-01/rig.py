"""Spike-01 measurement rig.

Prefill the task prompt with attentions OFF (the n-by-n cost lives only in prefill,
and eager prefill at ~1K tokens is trivial anyway); then decode greedily step by
step with output_attentions ON — each step's attention is one row [L, H, 1, n],
aggregated to per-segment mass immediately and discarded.

Signals:
  T1-temporal — per-step generated-token hidden-diff `g` plus entropy/logprob
  T1-spatial  — prefill hidden-diff and KV variance, rolled to segments (once)
  T2-lite     — all-head all-layer decode-row mass (pre-filter; AGMR heads unwired)
  heavy-hitter sets for P2 turnover

Everything is (context, query)-conditional by construction for T2/T1-temporal.
T1-spatial is ingestion-time (query-unknown). Outputs a JSON bundle per run.
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
    to token index lists. Returns input_ids, seg_token_lists, formatted string.

    Token membership is exclusive (first-wins). The overlapping rule `a<e and b>s`
    double-counted boundary tokens into adjacent segments; at 8–16 tokens/span
    that distorts per-token normalization. Prefix tokens (chat template) stay
    unclaimed.
    """
    model, tok = load()
    messages = [{"role": "user", "content": prompt_text}]
    kw = {}
    if "Qwen3" in MODEL_ID:
        kw["enable_thinking"] = False  # controlled non-thinking derivations for the spike
    formatted = tok.apply_chat_template(messages, tokenize=False, add_generation_prompt=True, **kw)
    base = formatted.index(prompt_text)  # prompt embedded verbatim
    enc = tok(formatted, return_offsets_mapping=True, return_tensors="pt", add_special_tokens=False)
    offsets = enc.pop("offset_mapping")[0].tolist()
    claimed = set()
    seg_tokens = []
    for seg in segments:
        s, e = seg.start + base, seg.end + base
        toks = [i for i, (a, b) in enumerate(offsets)
                if i not in claimed and a < e and b > s and b > a]
        claimed.update(toks)
        seg_tokens.append(toks)
    return enc["input_ids"], seg_tokens, formatted


def _entropy_logprob(logits, token_id):
    logp = torch.log_softmax(logits.float(), dim=-1)
    p = logp.exp()
    return float(-(p * logp).sum()), float(logp[token_id])


def _kv_layers(past, n_ctx):
    """Yield (key, value) tensors per layer, each [H, n_ctx, D]."""
    if hasattr(past, "key_cache"):
        pairs = zip(past.key_cache, past.value_cache)
    else:
        pairs = past
    for k, v in pairs:
        if k.dim() != 4:
            raise RuntimeError(f"unexpected KV rank {tuple(k.shape)}")
        k, v = k[0], v[0]  # drop batch → 3D
        # HF Qwen2 DynamicCache: [H, S, D]. Seq-first caches: [S, H, D].
        if k.shape[1] >= n_ctx:
            yield k[:, :n_ctx].float(), v[:, :n_ctx].float()
        elif k.shape[0] >= n_ctx:
            yield k[:n_ctx].transpose(0, 1).float(), v[:n_ctx].transpose(0, 1).float()
        else:
            raise RuntimeError(f"cannot locate seq dim in KV {tuple(k.shape)} n_ctx={n_ctx}")


def _seg_mean_max(pos_vals, memb):
    """pos_vals [C, n_ctx] × memb [n_seg, n_ctx] → (mean, max) each [C, n_seg]."""
    ntok = memb.sum(1).clamp(min=1.0)
    mean = (pos_vals @ memb.T) / ntok
    n_seg = memb.shape[0]
    cols = []
    for j in range(n_seg):
        m = memb[j] > 0
        if m.any():
            cols.append(pos_vals[:, m].max(dim=1).values)
        else:
            cols.append(pos_vals.new_zeros(pos_vals.shape[0]))
    return mean, torch.stack(cols, dim=1)


def t1_spatial_from_prefill(out, n_ctx, memb):
    """Ingestion-time T1-spatial: prefill hidden-diff and KV variance, rolled to segments.

    Prefill hidden-diff is ||h_l(pos) − h_l(pos−1)|| (EpiKV's signal along the
    prompt axis). Position 0 has no predecessor and is stored as 0 — analysis
    must detrend; the positional trend is expected and load-bearing to remove.
    """
    hs = torch.stack([h[0, :n_ctx].float() for h in out.hidden_states])  # [L+1, n_ctx, d]
    dhs = (hs[:, 1:] - hs[:, :-1]).norm(dim=-1)                          # [L+1, n_ctx-1]
    g_pre = torch.cat([dhs.new_zeros(dhs.shape[0], 1), dhs], dim=1)      # [L+1, n_ctx]
    g_mean, g_max = _seg_mean_max(g_pre, memb)

    k_pos, v_pos = [], []
    for k, v in _kv_layers(out.past_key_values, n_ctx):
        # variance over heads × dim, per position
        k_pos.append(k.var(dim=(0, 2)))  # [n_ctx]
        v_pos.append(v.var(dim=(0, 2)))
    k_pos = torch.stack(k_pos)  # [L, n_ctx]
    v_pos = torch.stack(v_pos)
    k_mean, k_max = _seg_mean_max(k_pos, memb)
    v_mean, v_max = _seg_mean_max(v_pos, memb)
    return {
        "g_prefill_seg_mean": g_mean.cpu().tolist(),
        "g_prefill_seg_max": g_max.cpu().tolist(),
        "k_var_seg_mean": k_mean.cpu().tolist(),
        "k_var_seg_max": k_max.cpu().tolist(),
        "v_var_seg_mean": v_mean.cpu().tolist(),
        "v_var_seg_max": v_max.cpu().tolist(),
    }


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
    spatial = t1_spatial_from_prefill(out, n_ctx, memb)
    prev_hidden = torch.stack([h[0, -1].float() for h in out.hidden_states])  # [L+1, d]
    logits = out.logits[0, -1]
    next_id = logits.argmax()
    ent, lp = _entropy_logprob(logits, next_id)

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

        # --- T1-temporal: hidden-state L2 diffs for this generated token ---
        hidden = torch.stack([h[0, -1].float() for h in out.hidden_states])   # [L+1, d]
        g = (hidden - prev_hidden).norm(dim=-1)        # [L+1]
        prev_hidden = hidden

        steps.append({
            "tok": tok.decode([next_id.item()]),
            "seg_mass": seg_mass.cpu().tolist(),       # [L][n_segments]
            "ctx_mass_total": float(mean_row.sum()),   # share of attention on prompt vs generated
            "hh": hh,
            "g": g.cpu().tolist(),
            "entropy": ent,
            "logprob": lp,
        })
        logits = out.logits[0, -1]
        next_id = logits.argmax()
        ent, lp = _entropy_logprob(logits, next_id)
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
        "seg_meta": [{"role": s.role, "room": s.room, "reused": bool(s.reused_detail),
                      "n_tokens": len(t), "text": s.text.strip()}
                     for s, t in zip(segments, seg_tokens)],
        "t1_spatial": spatial,
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
    """v3: MASTER is a dash-separated code sequence (string). Returns e.g. '72-266-175-36'."""
    import re as _re
    for line in reversed(text.strip().splitlines()):
        if "MASTER:" in line:
            m = _re.search(r"[\d]+(?:\s*-\s*[\d]+)*", line.split("MASTER:")[-1])
            return _re.sub(r"\s", "", m.group(0)) if m else None
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
