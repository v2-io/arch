#!/usr/bin/env python3
"""Per-call size rows for Fable 5.1, Opus 5.5, and Grok 4.7.

Rows are lengths and token counts only. No summary text, no ciphertext.
Summary tokens are o200k_base (one ruler for all three). Thinking tokens
are the vendor's reported count.

Outputs:
  claude_messages.jsonl   one row per assistant message id
  claude_blocks.jsonl     one row per thinking block (signature)
  grok_sessions.jsonl     one row per session, from usage.json
  grok_records.jsonl      one row per reasoning record id
  SUMMARY.json            the pooled ratios
"""

import hashlib
import json
import os
from collections import defaultdict

import tiktoken

ENC = tiktoken.get_encoding("o200k_base")
OUT = os.path.dirname(os.path.abspath(__file__))
CLAUDE_ROOT = "/Users/josephwecker-v2/.claude/projects"
GROK_ROOT = "/Users/josephwecker-v2/.grok/sessions"
MODELS = ("claude-opus-5-5", "claude-fable-5-1")


def ntok(text: str) -> int:
    if not text:
        return 0
    return len(ENC.encode(text))


def median(xs):
    xs = sorted(xs)
    if not xs:
        return None
    n = len(xs)
    mid = n // 2
    if n % 2:
        return xs[mid]
    return (xs[mid - 1] + xs[mid]) / 2


def mean(xs):
    if not xs:
        return None
    return sum(xs) / len(xs)


def new_msg():
    return {
        "sigs": {},
        "red": {},
        "seen": set(),
        "asst": 0,
        "tool": 0,
        "tt": None,
        "ot": None,
        "effort": None,
        "stop": None,
        "model": None,
    }


def consider_claude_file(path):
    try:
        sz = os.path.getsize(path)
    except OSError:
        return False
    if sz < 200:
        return False
    with open(path, "rb") as f:
        head = f.read(min(sz, 250_000))
    if b"claude-opus-5-5" in head or b"claude-fable-5-1" in head:
        return True
    if sz <= 250_000:
        return False
    with open(path, "rb") as f:
        f.seek(sz // 2)
        mid = f.read(200_000)
    return b"claude-opus-5-5" in mid or b"claude-fable-5-1" in mid


def collect_claude():
    msg_path = os.path.join(OUT, "claude_messages.jsonl")
    block_path = os.path.join(OUT, "claude_blocks.jsonl")
    pooled = {m: defaultdict(int) for m in MODELS}
    ratios = {m: {"msg_token": [], "file_token": [], "block_byte": []} for m in MODELS}

    with open(msg_path, "w") as mf, open(block_path, "w") as bf:
        for dirpath, _dirs, files in os.walk(CLAUDE_ROOT):
            for fn in files:
                if not fn.endswith(".jsonl"):
                    continue
                path = os.path.join(dirpath, fn)
                if not consider_claude_file(path):
                    continue
                session = path[len(CLAUDE_ROOT) + 1 :]
                msgs = defaultdict(new_msg)
                with open(path) as f:
                    for line in f:
                        if "claude-opus-5-5" not in line and "claude-fable-5-1" not in line:
                            continue
                        try:
                            o = json.loads(line)
                        except json.JSONDecodeError:
                            continue
                        msg = o.get("message") or {}
                        if msg.get("role") != "assistant":
                            continue
                        model = msg.get("model")
                        if model not in MODELS:
                            continue
                        mid = msg.get("id")
                        if not mid:
                            continue
                        rec = msgs[mid]
                        rec["model"] = model
                        if o.get("effort"):
                            rec["effort"] = o["effort"]
                        if msg.get("stop_reason"):
                            rec["stop"] = msg["stop_reason"]
                        usage = msg.get("usage") or {}
                        det = usage.get("output_tokens_details") or {}
                        if "thinking_tokens" in det:
                            rec["tt"] = det["thinking_tokens"]
                        if "output_tokens" in usage:
                            rec["ot"] = usage["output_tokens"]
                        content = msg.get("content")
                        if isinstance(content, str):
                            h = hashlib.sha1(content.encode()).hexdigest()
                            if h not in rec["seen"]:
                                rec["seen"].add(h)
                                rec["asst"] += len(content.encode())
                            continue
                        if not isinstance(content, list):
                            continue
                        for b in content:
                            if not isinstance(b, dict):
                                continue
                            t = b.get("type")
                            if t == "thinking":
                                sig = b.get("signature") or ""
                                if sig and sig not in rec["sigs"]:
                                    rec["sigs"][sig] = b.get("thinking") or ""
                            elif t == "redacted_thinking":
                                data = b.get("data") or ""
                                if data and data not in rec["red"]:
                                    rec["red"][data] = len(data.encode())
                            elif t == "text":
                                text = b.get("text") or ""
                                h = hashlib.sha1(text.encode()).hexdigest()
                                if h not in rec["seen"]:
                                    rec["seen"].add(h)
                                    rec["asst"] += len(text.encode())
                            elif t == "tool_use":
                                raw = json.dumps(b.get("input"), ensure_ascii=False, sort_keys=True)
                                h = hashlib.sha1(raw.encode()).hexdigest()
                                if h not in rec["seen"]:
                                    rec["seen"].add(h)
                                    rec["tool"] += len(raw.encode())
                file_sum_stok = defaultdict(int)
                file_sum_tt = defaultdict(int)
                models_here = set()
                for mid, rec in msgs.items():
                    model = rec["model"]
                    if model not in MODELS or rec["tt"] is None or rec["ot"] is None:
                        continue
                    models_here.add(model)
                    blocks = []
                    summary_b = 0
                    summary_tok = 0
                    cipher = 0
                    n_empty = 0
                    for i, (sig, text) in enumerate(rec["sigs"].items()):
                        cb = len(sig.encode())
                        sb = len(text.encode())
                        st = ntok(text) if text else 0
                        cipher += cb
                        summary_b += sb
                        summary_tok += st
                        if text:
                            n_empty_flag = 0
                            ratios[model]["block_byte"].append(sb / cb if cb else None)
                        else:
                            n_empty += 1
                            n_empty_flag = 1
                        blocks.append(
                            {
                                "model": model,
                                "session": session,
                                "message_id": mid,
                                "block_index": i,
                                "effort": rec["effort"],
                                "summary_bytes": sb,
                                "summary_tokens": st,
                                "cipher_bytes": cb,
                                "empty_summary": n_empty_flag,
                                "message_thinking_tokens": rec["tt"],
                                "message_output_tokens": rec["ot"],
                            }
                        )
                    red_b = sum(rec["red"].values())
                    cipher_all = cipher + red_b
                    row = {
                        "model": model,
                        "session": session,
                        "message_id": mid,
                        "effort": rec["effort"],
                        "stop_reason": rec["stop"],
                        "thinking_tokens": rec["tt"],
                        "output_tokens": rec["ot"],
                        "n_blocks": len(rec["sigs"]),
                        "n_empty_summaries": n_empty,
                        "n_redacted": len(rec["red"]),
                        "summary_bytes": summary_b,
                        "summary_tokens": summary_tok,
                        "cipher_bytes": cipher_all,
                        "assistant_text_bytes": rec["asst"],
                        "tool_arg_bytes": rec["tool"],
                    }
                    mf.write(json.dumps(row) + "\n")
                    for b in blocks:
                        bf.write(json.dumps(b) + "\n")
                    p = pooled[model]
                    p["messages"] += 1
                    p["thinking_tokens"] += rec["tt"]
                    p["output_tokens"] += rec["ot"]
                    p["cipher_bytes"] += cipher_all
                    p["summary_bytes"] += summary_b
                    p["summary_tokens"] += summary_tok
                    p["assistant_text_bytes"] += rec["asst"]
                    p["tool_arg_bytes"] += rec["tool"]
                    p["empty_blocks"] += n_empty
                    p["blocks"] += len(rec["sigs"])
                    if rec["tt"] > 0 and summary_tok > 0 and n_empty == 0 and rec["sigs"]:
                        ratios[model]["msg_token"].append(summary_tok / rec["tt"])
                        file_sum_stok[model] += summary_tok
                        file_sum_tt[model] += rec["tt"]
                for model in models_here:
                    pooled[model]["files"] += 1
                    if file_sum_tt[model] > 0 and file_sum_stok[model] > 0:
                        ratios[model]["file_token"].append(file_sum_stok[model] / file_sum_tt[model])
    return pooled, ratios


def collect_grok():
    sess_path = os.path.join(OUT, "grok_sessions.jsonl")
    rec_path = os.path.join(OUT, "grok_records.jsonl")
    pooled = defaultdict(int)
    session_ratios = []
    block_byte = []
    with open(sess_path, "w") as sf, open(rec_path, "w") as rf:
        for dirpath, _dirs, files in os.walk(GROK_ROOT):
            if "summary.json" not in files or "chat_history.jsonl" not in files or "usage.json" not in files:
                continue
            summary = json.load(open(os.path.join(dirpath, "summary.json")))
            if summary.get("current_model_id") != "grok-4.7":
                continue
            usage = json.load(open(os.path.join(dirpath, "usage.json")))["session"]
            session = os.path.basename(dirpath)
            rt = usage.get("reasoningTokens") or 0
            ot = usage.get("outputTokens") or 0
            by_id = {}
            asst = 0
            tool = 0
            seen = set()
            effort = summary.get("reasoning_effort")
            with open(os.path.join(dirpath, "chat_history.jsonl")) as f:
                for line in f:
                    o = json.loads(line)
                    t = o.get("type")
                    if t == "reasoning":
                        eid = o.get("id") or ""
                        blob = o.get("encrypted_content") or ""
                        cb = len(blob.encode())
                        texts = []
                        for it in o.get("summary") or []:
                            if isinstance(it, dict) and it.get("text"):
                                texts.append(it["text"])
                        text = "\n".join(texts)
                        prev = by_id.get(eid)
                        if prev is None or cb > prev[0]:
                            by_id[eid] = (cb, text, eid.split("_")[0])
                    elif t == "assistant":
                        if o.get("reasoning_effort"):
                            effort = o["reasoning_effort"]
                        c = o.get("content") or ""
                        if isinstance(c, str) and c:
                            asst += len(c.encode())
                        for tc in o.get("tool_calls") or []:
                            raw = tc.get("arguments") or ""
                            if not isinstance(raw, str):
                                raw = json.dumps(raw, ensure_ascii=False, sort_keys=True)
                            h = hashlib.sha1(raw.encode()).hexdigest()
                            if h not in seen:
                                seen.add(h)
                                tool += len(raw.encode())
            cipher = 0
            summary_b = 0
            summary_tok = 0
            n_empty = 0
            pending = []
            for eid, (cb, text, kind) in by_id.items():
                cipher += cb
                sb = len(text.encode())
                st = ntok(text) if text else 0
                summary_b += sb
                summary_tok += st
                if not text:
                    n_empty += 1
                else:
                    block_byte.append(sb / cb if cb else 0)
                pending.append(
                    {
                        "model": "grok-4.7",
                        "session": session,
                        "record_id": eid,
                        "kind": kind,
                        "effort": effort,
                        "summary_bytes": sb,
                        "summary_tokens": st,
                        "cipher_bytes": cb,
                        "empty_summary": 0 if text else 1,
                        "session_reasoning_tokens": rt,
                        "session_output_tokens": ot,
                    }
                )
            for rec in pending:
                rec["session_cipher_bytes"] = cipher
                rf.write(json.dumps(rec) + "\n")
            # fill session cipher now that it is known; records already written
            # with null. A second value lives on the session row. Regressions
            # that need it join on session.
            row = {
                "model": "grok-4.7",
                "session": session,
                "effort": effort,
                "title": summary.get("generated_title"),
                "created": (summary.get("created_at") or "")[:10],
                "reasoning_tokens": rt,
                "output_tokens": ot,
                "model_calls": usage.get("modelCalls"),
                "n_records": len(by_id),
                "n_empty_summaries": n_empty,
                "summary_bytes": summary_b,
                "summary_tokens": summary_tok,
                "cipher_bytes": cipher,
                "assistant_text_bytes": asst,
                "tool_arg_bytes": tool,
            }
            sf.write(json.dumps(row) + "\n")
            pooled["sessions"] += 1
            pooled["thinking_tokens"] += rt
            pooled["output_tokens"] += ot
            pooled["cipher_bytes"] += cipher
            pooled["summary_bytes"] += summary_b
            pooled["summary_tokens"] += summary_tok
            pooled["assistant_text_bytes"] += asst
            pooled["tool_arg_bytes"] += tool
            pooled["empty_records"] += n_empty
            pooled["records"] += len(by_id)
            if rt > 0 and summary_tok > 0:
                session_ratios.append(summary_tok / rt)
    return pooled, session_ratios, block_byte


def pack(model, pooled, token_ratios, byte_ratios):
    cipher = pooled["cipher_bytes"]
    emitted = cipher + pooled["summary_bytes"] + pooled["assistant_text_bytes"] + pooled["tool_arg_bytes"]
    tt = pooled["thinking_tokens"]
    ot = pooled["output_tokens"]
    return {
        "model": model,
        "files_or_sessions": pooled.get("files", pooled.get("sessions")),
        "messages_or_records": pooled.get("messages", pooled.get("records")),
        "thinking_over_output": (tt / ot) if ot else None,
        "thinking_tokens": tt,
        "output_tokens": ot,
        "cipher_over_emitted": (cipher / emitted) if emitted else None,
        "cipher_bytes": cipher,
        "emitted_bytes": emitted,
        "summary_bytes": pooled["summary_bytes"],
        "summary_tokens": pooled["summary_tokens"],
        "empty_summaries": pooled.get("empty_blocks", pooled.get("empty_records")),
        "blocks": pooled.get("blocks", pooled.get("records")),
        "median_summary_tokens_over_thinking_tokens": median(token_ratios),
        "mean_summary_tokens_over_thinking_tokens": mean(token_ratios),
        "n_token_ratios": len(token_ratios),
        "median_summary_bytes_over_cipher_bytes": median([r for r in byte_ratios if r is not None]),
        "mean_summary_bytes_over_cipher_bytes": mean([r for r in byte_ratios if r is not None]),
        "n_byte_ratios": len([r for r in byte_ratios if r is not None]),
        "pooled_summary_tokens_over_thinking_tokens": (pooled["summary_tokens"] / tt) if tt else None,
    }


def main():
    claude_pooled, claude_ratios = collect_claude()
    grok_pooled, grok_session_ratios, grok_byte = collect_grok()
    summary = {
        "tokenizer": "o200k_base",
        "claude_token_ratio_grain": "assistant message with a non-empty summary and thinking_tokens > 0",
        "grok_token_ratio_grain": "session, summary tokens / usage.json reasoningTokens",
        "byte_ratio_grain": "one thinking block or reasoning record with a non-empty summary",
        "emitted": "cipher + summary text + assistant text + tool-call arguments",
        "models": [],
    }
    for model in MODELS:
        summary["models"].append(
            pack(
                model,
                claude_pooled[model],
                claude_ratios[model]["msg_token"],
                claude_ratios[model]["block_byte"],
            )
        )
        summary["models"][-1]["median_file_summary_tokens_over_thinking"] = median(claude_ratios[model]["file_token"])
        summary["models"][-1]["mean_file_summary_tokens_over_thinking"] = mean(claude_ratios[model]["file_token"])
        summary["models"][-1]["n_files_with_token_ratio"] = len(claude_ratios[model]["file_token"])
    summary["models"].append(pack("grok-4.7", grok_pooled, grok_session_ratios, grok_byte))
    with open(os.path.join(OUT, "SUMMARY.json"), "w") as f:
        json.dump(summary, f, indent=2)
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
