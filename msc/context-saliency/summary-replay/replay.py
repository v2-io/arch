#!/usr/bin/env python3
"""Replay a grok-build compaction_requests JSON with a swapped summarization prompt.

The on-disk artifact is the exact ConversationItem list that was sent, plus the
prompt already appended as the last user message. This tool keeps that history
and lets you replace only the instruction.

  python3 replay.py inspect
  python3 replay.py extract-prompt -o prompts/00-as-shipped.txt
  python3 replay.py convert --prompt prompts/00-as-shipped.txt
  python3 replay.py run --prompt prompts/00-as-shipped.txt --name 00-as-shipped

API key: $XAI_API_KEY, or the contents of ~/xai-api-key.
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import time
import urllib.error
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

from convert import (
    build_create_body,
    input_stats,
    last_user_prompt_text,
)

HERE = Path(__file__).resolve().parent
DEFAULT_REQUEST = Path(
    "/Users/josephwecker-v2/.grok/sessions/"
    "%2FUsers%2Fjosephwecker-v2/"
    "01a0533d-9ff3-7e11-adf4-00fffc4cd0f2/"
    "compaction_requests/9385047a-a0ef-4806-8a67-3f963b3efa06.json"
)
API_URL = "https://api.x.ai/v1/responses"
KEY_FILE = Path.home() / "xai-api-key"


def load_key() -> str:
    env = os.environ.get("XAI_API_KEY", "").strip()
    if env:
        return env
    if KEY_FILE.is_file():
        return KEY_FILE.read_text().strip()
    raise SystemExit(
        f"No API key. Set XAI_API_KEY or put the key in {KEY_FILE}"
    )


def load_request(path: Path) -> dict:
    return json.loads(path.read_text())


def cmd_inspect(args: argparse.Namespace) -> int:
    req = load_request(args.request)
    history = req["chat_history"]
    by_type: dict[str, int] = {}
    for it in history:
        t = it.get("type") or "?"
        by_type[t] = by_type.get(t, 0) + 1
    prompt = last_user_prompt_text(history)
    print(f"request:     {args.request}")
    print(f"request_id:  {req.get('request_id')}")
    print(f"created_at:  {req.get('created_at')}")
    print(f"trigger:     {req.get('trigger')}")
    print(f"variant:     {req.get('prompt_variant')}")
    print(f"model:       {req.get('model')}")
    print(f"file_bytes:  {args.request.stat().st_size}")
    print(f"history:     {len(history)} items  {by_type}")
    print(f"tools:       {len(req.get('tools') or [])}")
    print(f"orig_summary_chars: {len(req.get('summary') or '')}")
    print(f"last_prompt_chars:  {len(prompt)}")
    body = build_create_body(
        req,
        include_tools=not args.no_tools,
        drop_images=args.drop_images,
    )
    stats = input_stats(body["input"])
    print(f"converted:   {stats['n_items']} input items  {stats['by_type']}")
    print(f"text_chars:  {stats['text_chars']}")
    print(f"images:      {stats['images']}")
    print(f"rough_tokens:{stats['rough_tokens']}  (chars/4; not a tokenizer)")
    if stats["rough_tokens"] >= 200_000:
        print(
            "note: grok-4.6 long-context pricing starts at 200k prompt tokens "
            "($4 / $12 per 1M in/out vs $2 / $6 below)."
        )
    return 0


def cmd_extract_prompt(args: argparse.Namespace) -> int:
    req = load_request(args.request)
    text = last_user_prompt_text(req["chat_history"])
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(text)
    print(f"wrote {args.out} ({len(text)} chars)")
    return 0


def cmd_convert(args: argparse.Namespace) -> int:
    req = load_request(args.request)
    prompt = args.prompt.read_text() if args.prompt else None
    body = build_create_body(
        req,
        prompt=prompt,
        model=args.model,
        include_tools=not args.no_tools,
        drop_images=args.drop_images,
    )
    stats = input_stats(body["input"])
    print(json.dumps(stats, indent=2))
    if args.dump:
        args.dump.parent.mkdir(parents=True, exist_ok=True)
        args.dump.write_text(json.dumps(body))
        print(f"wrote {args.dump} ({args.dump.stat().st_size} bytes)")
    return 0


def cmd_run(args: argparse.Namespace) -> int:
    req = load_request(args.request)
    prompt_path: Path | None = args.prompt
    prompt = prompt_path.read_text() if prompt_path else None
    body = build_create_body(
        req,
        prompt=prompt,
        model=args.model,
        include_tools=not args.no_tools,
        drop_images=args.drop_images,
        reasoning_effort=args.reasoning_effort,
    )
    stats = input_stats(body["input"])
    name = args.name or datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    out_dir: Path = args.out_dir / name
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / "stats.json").write_text(json.dumps(stats, indent=2) + "\n")
    if prompt is not None:
        (out_dir / "prompt.txt").write_text(prompt)
    else:
        (out_dir / "prompt.txt").write_text(last_user_prompt_text(req["chat_history"]))
    meta = {
        "name": name,
        "request": str(args.request),
        "request_id": req.get("request_id"),
        "model": body["model"],
        "include_tools": not args.no_tools,
        "drop_images": args.drop_images,
        "reasoning_effort": args.reasoning_effort,
        "started_at": datetime.now(timezone.utc).isoformat(),
    }
    print(f"submitting {name}: {stats['n_items']} items, ~{stats['rough_tokens']} tokens")
    t0 = time.time()
    try:
        response = _post_responses(body, timeout=args.timeout)
    except urllib.error.HTTPError as e:
        err_body = e.read().decode("utf-8", errors="replace")
        (out_dir / "error.json").write_text(
            json.dumps({"status": e.code, "body": err_body}, indent=2) + "\n"
        )
        meta["error"] = {"status": e.code, "elapsed_s": round(time.time() - t0, 2)}
        (out_dir / "meta.json").write_text(json.dumps(meta, indent=2) + "\n")
        print(f"HTTP {e.code}\n{err_body[:4000]}", file=sys.stderr)
        return 1
    elapsed = time.time() - t0
    summary = _response_text(response)
    (out_dir / "response.json").write_text(json.dumps(response, indent=2) + "\n")
    (out_dir / "summary.txt").write_text(summary)
    usage = response.get("usage") or {}
    meta.update(
        {
            "elapsed_s": round(elapsed, 2),
            "summary_chars": len(summary),
            "usage": usage,
            "response_id": response.get("id"),
            "status": response.get("status"),
        }
    )
    (out_dir / "meta.json").write_text(json.dumps(meta, indent=2) + "\n")
    print(f"wrote {out_dir / 'summary.txt'} ({len(summary)} chars, {elapsed:.1f}s)")
    if usage:
        print(f"usage: {json.dumps(usage)}")
    return 0


def _post_responses(body: dict, timeout: int) -> dict:
    key = load_key()
    data = json.dumps(body).encode("utf-8")
    req = urllib.request.Request(
        API_URL,
        data=data,
        method="POST",
        headers={
            "Authorization": f"Bearer {key}",
            "Content-Type": "application/json",
        },
    )
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        return json.load(resp)


def _response_text(response: dict) -> str:
    """Pull concatenated output_text from a Responses API body."""
    if isinstance(response.get("output_text"), str) and response["output_text"]:
        return response["output_text"]
    chunks: list[str] = []
    for item in response.get("output") or []:
        if item.get("type") == "message":
            for part in item.get("content") or []:
                if isinstance(part, dict) and part.get("type") in (
                    "output_text",
                    "text",
                ):
                    chunks.append(part.get("text") or "")
        elif item.get("type") == "output_text":
            chunks.append(item.get("text") or "")
    return "".join(chunks)


def _add_common(p: argparse.ArgumentParser) -> None:
    p.add_argument(
        "--request",
        type=Path,
        default=Path(os.environ["COMPACTION_REQUEST"])
        if os.environ.get("COMPACTION_REQUEST")
        else DEFAULT_REQUEST,
        help="compaction_requests/*.json (default: first natural auto-compact of 01a0533d)",
    )
    p.add_argument("--no-tools", action="store_true", help="omit tool defs (less faithful)")
    p.add_argument(
        "--drop-images",
        action="store_true",
        help="strip user image parts (cheaper, less faithful)",
    )
    p.add_argument("--model", default=None, help="override model id (default: the request's)")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="cmd", required=True)

    p_ins = sub.add_parser("inspect", help="describe a saved request and converted size")
    _add_common(p_ins)
    p_ins.set_defaults(func=cmd_inspect)

    p_ex = sub.add_parser("extract-prompt", help="write the shipped summarization prompt")
    _add_common(p_ex)
    p_ex.add_argument("-o", "--out", type=Path, default=HERE / "prompts" / "00-as-shipped.txt")
    p_ex.set_defaults(func=cmd_extract_prompt)

    p_cv = sub.add_parser("convert", help="build the Responses body; optionally dump it")
    _add_common(p_cv)
    p_cv.add_argument("--prompt", type=Path, default=None)
    p_cv.add_argument("--dump", type=Path, default=None)
    p_cv.set_defaults(func=cmd_convert)

    p_run = sub.add_parser("run", help="submit to api.x.ai/v1/responses")
    _add_common(p_run)
    p_run.add_argument("--prompt", type=Path, default=None, help="replacement summarization prompt")
    p_run.add_argument("--name", default=None, help="output directory name under --out-dir")
    p_run.add_argument("--out-dir", type=Path, default=HERE / "out")
    p_run.add_argument("--reasoning-effort", default="high")
    p_run.add_argument("--timeout", type=int, default=3600)
    p_run.set_defaults(func=cmd_run)

    args = parser.parse_args(argv)
    if not args.request.is_file():
        raise SystemExit(f"request not found: {args.request}")
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
