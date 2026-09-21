#!/usr/bin/env python3
"""Deterministic precompaction updates.jsonl → simplified temporal transcript.

Emits one JSON object per line, in the source event order:

  user       typed message (images as uri/mime only, no pixels)
  assistant  displayed reply text (thoughts / recaps / tool payloads omitted)
  tool       file/command summary (name: read/edit/write/shell/commit/…)

Usage:
  python3 transcript.py fixtures/01a0533d-precompact-updates.jsonl -o out/01a0533d-precompact-transcript.jsonl
"""

from __future__ import annotations

import argparse
import difflib
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

HOME = str(Path.home())

COMMIT_HEREDOC = re.compile(
    r"""git\s+commit\s+-m\s+["']?\$\(cat\s+<<['\"]?EOF['\"]?\n(.*)\nEOF""",
    re.S,
)
COMMIT_MINUS_M = re.compile(r"""git\s+commit\s+-m\s+['\"](.+?)['\"]""", re.S)
SHA_BRACKET = re.compile(r"\[(?:\w+)\s+([0-9a-f]{7,40})\]")
STAT_LINE = re.compile(r"^\s*.+\s+\|\s+\d+\s+[+-]+", re.M)
FILES_CHANGED = re.compile(r"\d+ files? changed")


def iso(ts: int | float) -> str:
    """Accept unix seconds (< 1e12) or milliseconds."""
    x = float(ts)
    ms = int(x if x >= 1e12 else x * 1000)
    dt = datetime.fromtimestamp(ms / 1000.0, tz=timezone.utc)
    return dt.strftime("%Y-%m-%dT%H:%M:%S") + f".{ms % 1000:03d}Z"


def common_dir_prefix(paths: list[str]) -> str | None:
    abs_paths = [p.rstrip("/") for p in paths if isinstance(p, str) and p.startswith("/")]
    if not abs_paths:
        return None
    parts = [p.split("/") for p in abs_paths]
    prefix: list[str] = []
    for column in zip(*parts):
        if len(set(column)) == 1:
            prefix.append(column[0])
        else:
            break
    if len(prefix) <= 1:
        return None
    return "/".join(prefix) or None


def _dir_of(p: str) -> str:
    p = p.rstrip("/")
    parent, _, name = p.rpartition("/")
    return parent or p


def discover_cwd(events: list[tuple[int, dict]]) -> str | None:
    explicit = None
    dirs: list[str] = []
    for _ts, u in events:
        if u.get("sessionUpdate") == "task_backgrounded" and u.get("cwd"):
            explicit = explicit or u["cwd"].rstrip("/")
        if isinstance(u.get("cwd"), str) and u["cwd"].startswith("/"):
            dirs.append(u["cwd"].rstrip("/"))
        for loc in u.get("locations") or []:
            if isinstance(loc, dict) and isinstance(loc.get("path"), str):
                dirs.append(_dir_of(loc["path"]))
        ri = u.get("rawInput") if isinstance(u.get("rawInput"), dict) else {}
        for k in ("target_file", "file_path"):
            v = ri.get(k)
            if isinstance(v, str) and v.startswith("/"):
                dirs.append(_dir_of(v))
        for k in ("path", "cwd", "working_directory"):
            v = ri.get(k)
            if isinstance(v, str) and v.startswith("/"):
                dirs.append(v.rstrip("/"))
    if explicit:
        return explicit
    return common_dir_prefix(dirs)


def strip_cwd(text: str | None, cwd: str | None) -> str | None:
    if text is None:
        return None
    if not text or not cwd:
        return text
    cwd = cwd.rstrip("/")
    home = HOME.rstrip("/")
    aliases = [cwd]
    if cwd == home:
        aliases.append("~")
    elif cwd.startswith(home + "/"):
        aliases.append("~/" + cwd[len(home) + 1 :])
    aliases = sorted({a for a in aliases if a}, key=len, reverse=True)
    out = text
    for a in aliases:
        out = out.replace(a + "/", "")
        out = re.sub(re.escape(a) + r'(?=\s|$|["\'])', ".", out)
        if out == a:
            out = "."
    return out


def relpath(p: str | None, cwd: str | None = None) -> str | None:
    if not p:
        return None
    p = p.rstrip("/")
    stripped = strip_cwd(p, cwd)
    if stripped is not None and stripped != p:
        return stripped
    home = HOME.rstrip("/")
    if p == home:
        return "~" if cwd != home else "."
    if p.startswith(home + "/"):
        return "~/" + p[len(home) + 1 :]
    return p


def extract_text(content: Any) -> str:
    if content is None:
        return ""
    if isinstance(content, str):
        return content
    if isinstance(content, dict):
        if content.get("type") == "text":
            return content.get("text") or ""
        inner = content.get("content")
        if inner is not None and inner is not content:
            return extract_text(inner)
        return content.get("text") or ""
    if isinstance(content, list):
        return "".join(extract_text(p) for p in content)
    return ""


def line_diff_stat(path: str, old: str, new: str) -> tuple[int, int, str]:
    old_lines = old.splitlines()
    new_lines = new.splitlines()
    added = removed = 0
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(
        a=old_lines, b=new_lines
    ).get_opcodes():
        if tag == "replace":
            removed += i2 - i1
            added += j2 - j1
        elif tag == "delete":
            removed += i2 - i1
        elif tag == "insert":
            added += j2 - j1
    return added, removed, format_stat(path, added, removed)


def format_stat(path: str, added: int, removed: int) -> str:
    total = added + removed
    plus = "+" * min(added, 40)
    minus = "-" * min(removed, 40)
    bar = f"{plus}{minus}".strip()
    head = f" {path} | {total} {bar}".rstrip()
    parts = ["1 file changed"]
    if added:
        parts.append(f"{added} insertion{'s' if added != 1 else ''}(+)")
    if removed:
        parts.append(f"{removed} deletion{'s' if removed != 1 else ''}(-)")
    if not added and not removed:
        parts = ["1 file changed, 0 insertions(+), 0 deletions(-)"]
    summary = parts[0] if len(parts) == 1 else parts[0] + ", " + ", ".join(parts[1:])
    return f"{head}\n {summary}"


def parse_commit_message(command: str) -> str | None:
    m = COMMIT_HEREDOC.search(command)
    if m:
        return m.group(1).strip("\n")
    # single-line -m, but not a heredoc
    if "<<'EOF'" in command or "<<EOF" in command or '<<"EOF"' in command:
        return None
    m = COMMIT_MINUS_M.search(command)
    if m:
        return m.group(1)
    return None


def parse_sha(stdout: str) -> str | None:
    m = SHA_BRACKET.search(stdout)
    return m.group(1) if m else None


def parse_stat_from_stdout(stdout: str) -> str | None:
    lines = []
    seen_changed = False
    for line in stdout.splitlines():
        s = line.rstrip()
        if STAT_LINE.search(s):
            lines.append(s)
        elif FILES_CHANGED.search(s):
            if not seen_changed:
                lines.append(s)
                seen_changed = True
    return "\n".join(lines) if lines else None


def is_full_read(raw: dict) -> bool:
    # No window → the Read tool returned the whole file (or failed).
    return raw.get("offset") is None and raw.get("limit") is None


STRIP_KEYS = (
    "path",
    "summary",
    "command",
    "stat",
    "text",
    "message",
    "description",
    "error",
    "response",
)


class Transcript:
    def __init__(self, cwd: str | None = None) -> None:
        self.records: list[dict] = []
        self.cwd = cwd.rstrip("/") if cwd else None
        self.known_well: set[str] = set()
        self._assistant: list[str] = []
        self._assistant_ts: int | None = None
        self._assistant_ts_end: int | None = None
        self._user_open: dict | None = None

    def emit_cwd(self, ts: int, path: str) -> None:
        self.cwd = path.rstrip("/")
        self.records.append(
            {"type": "meta", "kind": "cwd", "ts": iso(ts), "value": self.cwd}
        )

    def _strip_rec(self, rec: dict) -> dict:
        if rec.get("type") in ("cwd", "meta") or not self.cwd:
            return rec
        out = dict(rec)
        for k in STRIP_KEYS:
            if k in out and isinstance(out[k], str):
                out[k] = strip_cwd(out[k], self.cwd)
        if "known_well" in out:
            out["known_well"] = [strip_cwd(p, self.cwd) or p for p in out["known_well"]]
        return out

    def emit(self, rec: dict, *, known_changed: bool = False) -> None:
        if known_changed:
            rec["known_well"] = sorted(self.known_well)
        self.records.append(self._strip_rec(rec))

    def flush_assistant(self) -> None:
        if not self._assistant:
            return
        text = "".join(self._assistant)
        self._assistant = []
        ts = self._assistant_ts
        ts_end = self._assistant_ts_end
        self._assistant_ts = self._assistant_ts_end = None
        if not text.strip():
            return
        rec = {"type": "assistant", "ts": iso(ts or 0), "text": text}
        if ts_end is not None and ts_end != ts:
            rec["ts_end"] = iso(ts_end)
        self.emit(rec)

    def flush_user(self) -> None:
        if self._user_open is None:
            return
        rec = self._user_open
        self._user_open = None
        if not rec.get("text") and not rec.get("images"):
            return
        self.emit(rec)

    def add_user_text(self, ts: int, prompt_index: Any, text: str) -> None:
        self.flush_assistant()
        if (
            self._user_open is not None
            and self._user_open.get("prompt_index") == prompt_index
        ):
            if text:
                prev = self._user_open.get("text") or ""
                self._user_open["text"] = (prev + "\n" + text) if prev else text
            return
        self.flush_user()
        rec: dict[str, Any] = {"type": "user", "ts": iso(ts), "text": text}
        if prompt_index is not None:
            rec["prompt_index"] = prompt_index
        self._user_open = rec

    def add_user_image(self, ts: int, prompt_index: Any, image: dict) -> None:
        self.flush_assistant()
        if (
            self._user_open is None
            or self._user_open.get("prompt_index") != prompt_index
        ):
            self.flush_user()
            rec: dict[str, Any] = {"type": "user", "ts": iso(ts), "text": ""}
            if prompt_index is not None:
                rec["prompt_index"] = prompt_index
            self._user_open = rec
        imgs = self._user_open.setdefault("images", [])
        imgs.append(image)

    def add_assistant_chunk(self, ts: int, text: str) -> None:
        self.flush_user()
        if not self._assistant:
            self._assistant_ts = ts
        self._assistant_ts_end = ts
        self._assistant.append(text)

    def mark_known(self, path: str | None) -> bool:
        if not path:
            return False
        rp = relpath(path, self.cwd)
        if not rp or rp in self.known_well:
            return False
        self.known_well.add(rp)
        return True

    def add_tool(self, ts: int, rec: dict, paths_known: list[str]) -> None:
        self.flush_user()
        self.flush_assistant()
        changed = False
        for p in paths_known:
            changed = self.mark_known(p) or changed
        self.emit(rec, known_changed=changed)


RESPONSE_KEEP = 500
EVAL_UNMATCHED = re.compile(r"\(eval\):[^:\n]*: unmatched '")

TOOL_NAME = {
    "search_replace": "edit",
    "read_file": "read",
    "run_terminal_command": "shell",
    "kill_command_or_subagent": "kill",
    "get_command_or_subagent_output": "get_output",
}


def strip_eval_unmatched(s: str) -> str:
    return EVAL_UNMATCHED.sub("", s or "")


def tool_response_snippet(stdout: str) -> str:
    s = " ".join(strip_eval_unmatched(stdout).split())
    return s[:RESPONSE_KEEP]


def hunk_arrow(old: str, new: str) -> str:
    o = " ".join((old or "").split())
    n = " ".join((new or "").split())
    if not o and n:
        return "+ " + n
    if o and not n:
        return "- " + o
    return f"{o} → {n}"


def _stat_result_line(stat: str) -> str:
    """Drop the path|bar line so the view doesn't repeat the path in the result column."""
    for line in reversed((stat or "").splitlines()):
        s = line.strip()
        if "changed" in s:
            return s
    return tool_response_snippet(stat)


def build_tool_record(
    title: str, raw: dict, status: str | None, stdout: str, cwd: str | None = None
) -> dict:
    raw = raw or {}
    rec: dict[str, Any] = {
        "type": "tool",
        "name": TOOL_NAME.get(title, title),
        "status": status or "completed",
    }
    snippet = tool_response_snippet(stdout)
    if snippet:
        rec["response"] = snippet
    if title == "read_file":
        path = relpath(raw.get("target_file"), cwd)
        rec["path"] = path
        rec["kind"] = "read"
        if is_full_read(raw):
            rec["span"] = "full"
        else:
            off = raw.get("offset")
            lim = raw.get("limit")
            rec["span"] = f"lim={lim}" if lim is not None else f"offset={off}"
        return rec
    if title == "search_replace":
        path = relpath(raw.get("file_path"), cwd)
        rec["path"] = path
        rec["kind"] = "edit"
        old, new = raw.get("old_string") or "", raw.get("new_string") or ""
        added, removed, stat = line_diff_stat(path or "?", old, new)
        rec["stat"] = stat
        rec["hunk"] = hunk_arrow(old, new)
        if status == "failed":
            rec["error"] = stdout.strip().split("\n", 1)[0] if stdout else "failed"
            rec["response"] = rec["error"]
        else:
            # The call args are what sat in context; the success sentence is not load-bearing.
            rec["response"] = rec["hunk"]
        return rec
    if title == "write":
        path = relpath(raw.get("file_path"), cwd)
        rec["path"] = path
        rec["kind"] = "write"
        content = raw.get("content") or ""
        n = len(content.splitlines())
        rec["stat"] = format_stat(path or "?", n, 0)
        rec["summary"] = f"write ({n} lines)"
        if status == "failed":
            rec["error"] = stdout.strip().split("\n", 1)[0] if stdout else "failed"
            rec["response"] = rec["error"]
        else:
            rec["response"] = tool_response_snippet(content)
        return rec
    if title == "grep":
        rec["kind"] = "search"
        rec["path"] = relpath(raw.get("path"), cwd)
        rec["pattern"] = raw.get("pattern")
        rec["summary"] = f"grep {raw.get('pattern')!r}"
        return rec
    if title == "run_terminal_command":
        rec["kind"] = "shell"
        rec["description"] = raw.get("description")
        cmd = raw.get("command") or ""
        rec["command"] = cmd
        rec["summary"] = raw.get("description") or first_line(cmd)
        return rec
    if title == "kill_command_or_subagent":
        rec["kind"] = "kill"
        tid = raw.get("task_id") or raw.get("task_ids")
        if isinstance(tid, list):
            rec["summary"] = ",".join(str(x) for x in tid)
        elif tid:
            rec["summary"] = str(tid)
        return rec
    if title == "get_command_or_subagent_output":
        rec["kind"] = "get_output"
        tid = raw.get("task_id") or raw.get("task_ids")
        if isinstance(tid, list):
            rec["summary"] = ",".join(str(x) for x in tid)
        elif tid:
            rec["summary"] = str(tid)
        return rec
    rec["kind"] = "other"
    rec["summary"] = title
    rec["raw_keys"] = sorted(raw.keys())
    return rec


def first_line(cmd: str) -> str:
    for line in cmd.splitlines():
        s = line.strip()
        if s:
            return s[:200]
    return "(empty command)"


def known_paths_for(title: str, raw: dict) -> list[str]:
    raw = raw or {}
    if title == "read_file" and is_full_read(raw):
        p = raw.get("target_file")
        return [p] if p else []
    if title == "write":
        p = raw.get("file_path")
        return [p] if p else []
    # search_replace: hunk-only; not known-well unless already
    return []


FILE_TOOLS = {"edit", "read", "write", "grep"}
FILE_TOOL_LABEL = {
    "edit": "edit",
    "read": "read",
    "write": "write",
    "grep": "grep",
}


def commit_stat_paths(stat: str) -> list[str]:
    paths = []
    for line in (stat or "").splitlines():
        if "|" not in line or "changed" in line:
            continue
        path = line.split("|", 1)[0].strip()
        if path:
            paths.append(path.replace("\\", "/"))
    return paths


def path_enveloped(tool_path: str, commit_paths: list[str]) -> bool:
    tp = (tool_path or "").replace("\\", "/")
    if not tp or not commit_paths:
        return False
    for cp in commit_paths:
        if tp == cp or tp.endswith("/" + cp) or tp.endswith(cp):
            return True
        if cp.endswith("/" + tp) or cp.endswith(tp.split("/")[-1]) and tp.split("/")[-1] == cp.split("/")[-1] and len(tp.split("/")) > 1:
            # basename-only match is too weak unless directories also match
            if cp.split("/")[-1] == tp.split("/")[-1] and (
                "/" + "/".join(tp.split("/")[-2:]) == "/" + "/".join(cp.split("/")[-2:])
                or tp.endswith(cp)
            ):
                return True
    return False


def _run_summary(counts: dict[str, int], n_fail: int) -> str:
    bits = []
    for name, label in FILE_TOOL_LABEL.items():
        n = counts.get(name, 0)
        if n:
            bits.append(f"{n} {label}{'s' if n != 1 else ''}")
    if n_fail:
        bits.append(f"{n_fail} fail")
    return ", ".join(bits) if bits else "work"


def merge_file_run(run: list[dict]) -> dict:
    first, last = run[0], run[-1]
    counts: dict[str, int] = {}
    n_fail = 0
    last_fail = ""
    for r in run:
        counts[r.get("name") or ""] = counts.get(r.get("name") or "", 0) + 1
        if r.get("status") == "failed":
            n_fail += 1
            last_fail = r.get("error") or r.get("response") or last_fail
    rec = {
        "type": "tool",
        "name": "work",
        "path": first.get("path"),
        "ts": first["ts"],
        "n": len(run),
        "counts": {k: v for k, v in counts.items() if k},
        "n_fail": n_fail,
        "summary": _run_summary(counts, n_fail),
        "status": "failed" if n_fail == len(run) else "completed",
    }
    if last.get("ts_end") or last["ts"] != first["ts"]:
        rec["ts_end"] = last.get("ts_end") or last["ts"]
    if n_fail:
        rec["error"] = last_fail
        rec["response"] = last_fail
    elif last.get("response"):
        rec["response"] = last["response"]
    if last.get("stat"):
        rec["stat"] = last["stat"]
    if last.get("hunk") and not n_fail:
        rec["hunk"] = last["hunk"]
        rec["response"] = last["hunk"]
    for r in run:
        if r.get("known_well"):
            rec["known_well"] = r["known_well"]
    return rec


def collapse_file_runs(records: list[dict]) -> list[dict]:
    out: list[dict] = []
    i = 0
    while i < len(records):
        r = records[i]
        if (
            r.get("type") != "tool"
            or r.get("name") not in FILE_TOOLS
            or not r.get("path")
        ):
            out.append(r)
            i += 1
            continue
        run = [r]
        j = i + 1
        while (
            j < len(records)
            and records[j].get("type") == "tool"
            and records[j].get("name") in FILE_TOOLS
            and records[j].get("path") == r.get("path")
        ):
            run.append(records[j])
            j += 1
        out.append(merge_file_run(run) if len(run) > 1 else r)
        i = j
    return out


def mark_enveloped(records: list[dict]) -> list[dict]:
    n = len(records)
    forthcoming: list[list[str] | None] = [None] * n
    current: list[str] | None = None
    for i in range(n - 1, -1, -1):
        if records[i].get("name") == "commit" or records[i].get("type") == "commit":
            current = commit_stat_paths(records[i].get("stat") or "")
        forthcoming[i] = current
    out = []
    for i, r in enumerate(records):
        files = forthcoming[i]
        if (
            r.get("type") == "tool"
            and r.get("path")
            and files
            and path_enveloped(r["path"], files)
        ):
            r = dict(r)
            r["enveloped"] = True
            r["summary"] = ((r.get("summary") or "work") + " → commit").strip()
            if not r.get("n_fail"):
                r.pop("response", None)
        out.append(r)
    return out


def convert(events: list[tuple[int, dict]]) -> list[dict]:
    cwd = discover_cwd(events)
    t = Transcript(cwd)
    if cwd and events:
        t.emit_cwd(events[0][0], cwd)
    # Stash tool_call rawInput until completion
    open_tools: dict[str, dict] = {}

    for ts, u in events:
        kind = u.get("sessionUpdate")
        if kind == "user_message_chunk":
            meta = u.get("_meta") or {}
            pi = meta.get("promptIndex")
            content = u.get("content") or {}
            if content.get("type") == "image":
                t.add_user_image(
                    ts,
                    pi,
                    {
                        "uri": content.get("uri"),
                        "mime": content.get("mimeType"),
                        "n": (content.get("_meta") or {}).get(
                            "xai.dev/imageDisplayNumber"
                        ),
                    },
                )
            else:
                t.add_user_text(ts, pi, extract_text(content))
        elif kind == "agent_message_chunk":
            t.add_assistant_chunk(ts, extract_text(u.get("content")))
        elif kind == "tool_call":
            cid = u.get("toolCallId")
            open_tools[cid] = {
                "title": u.get("title"),
                "raw": u.get("rawInput") or {},
                "ts": ts,
            }
        elif kind == "tool_call_update":
            cid = u.get("toolCallId")
            info = open_tools.get(cid) or {
                "title": u.get("title"),
                "raw": {},
                "ts": ts,
            }
            if u.get("rawInput"):
                info["raw"] = u["rawInput"]
            if u.get("title") and not info.get("title"):
                info["title"] = u.get("title")
            open_tools[cid] = info
            status = u.get("status")
            if status not in ("completed", "failed"):
                continue
            title = info.get("title") or "unknown"
            raw = info.get("raw") or {}
            stdout = extract_text(u.get("content"))
            tool_cwd = raw.get("working_directory") or raw.get("cwd")
            if isinstance(tool_cwd, str) and t.cwd and tool_cwd.rstrip("/") != t.cwd:
                t.emit_cwd(info.get("ts") or ts, tool_cwd)
            rec = build_tool_record(title, raw, status, stdout, t.cwd)
            rec["ts"] = iso(info.get("ts") or ts)
            if ts != info.get("ts"):
                rec["ts_end"] = iso(ts)
            rec["id"] = cid

            commit_msg = (
                parse_commit_message(raw.get("command") or "")
                if title == "run_terminal_command"
                else None
            )
            if commit_msg:
                crec = {
                    "type": "tool",
                    "name": "commit",
                    "kind": "commit",
                    "ts": rec["ts"],
                    "message": commit_msg,
                    "sha": parse_sha(stdout),
                    "stat": parse_stat_from_stdout(stdout),
                    "id": cid,
                    "response": rec.get("response"),
                }
                t.add_tool(info.get("ts") or ts, crec, [])
            else:
                t.add_tool(
                    info.get("ts") or ts,
                    rec,
                    known_paths_for(title, raw),
                )
            open_tools.pop(cid, None)
        elif kind == "turn_completed":
            t.flush_user()
            t.flush_assistant()
        elif kind in (
            "agent_thought_chunk",
            "session_recap",
            "retry_state",
            "image_compressed",
            "task_backgrounded",
            "task_completed",
        ):
            continue
        else:
            continue

    t.flush_user()
    t.flush_assistant()
    # Post-commit envelope collapse (and consecutive same-path file-tool
    # folding) is parked while we sort which extinct DAG legs to compress
    # first, and how. Re-enable with:
    #   return mark_enveloped(collapse_file_runs(t.records))
    recs = t.records
    if recs:
        recs = [
            {
                "type": "meta",
                "kind": "ts",
                "ts": recs[0]["ts"],
                "value": recs[0]["ts"],
            }
        ] + recs
    return attach_file_aliases(attach_dir_aliases(recs, t.cwd))


def _parent_dir(path: str) -> str | None:
    p = path.replace("\\", "/").strip()
    if p.startswith("<"):
        return None
    p = p.rstrip("/")
    if "/" not in p:
        return None
    return p.rsplit("/", 1)[0] + "/"


def _dir_forms(parent: str, cwd: str | None) -> list[str]:
    """Spellings of a directory prefix we might see in columns."""
    parent = parent if parent.endswith("/") else parent + "/"
    home = HOME.rstrip("/")
    forms = [parent]
    if parent.startswith("~/"):
        rest = parent[2:]
        forms.append(home + "/" + rest)
        if cwd == home:
            forms.append(rest)
    elif not parent.startswith("/"):
        forms.append("~/" + parent)
        if cwd:
            forms.append(cwd.rstrip("/") + "/" + parent)
        forms.append(home + "/" + parent)
    else:
        if parent.startswith(home + "/"):
            forms.append("~/" + parent[len(home) + 1 :])
            if cwd == home:
                forms.append(parent[len(home) + 1 :])
    out = []
    for f in forms:
        if f and f not in out:
            out.append(f)
    return out


def _dir_value(parent: str, cwd: str | None) -> str:
    """Canonical display for a meta dir row (prefer ~/ when under home)."""
    parent = parent if parent.endswith("/") else parent + "/"
    home = HOME.rstrip("/")
    if parent.startswith("~/"):
        return parent
    if parent.startswith(home + "/"):
        return "~/" + parent[len(home) + 1 :]
    if cwd == home and not parent.startswith("/"):
        return "~/" + parent
    if cwd and parent.startswith(cwd.rstrip("/") + "/"):
        rel = parent[len(cwd.rstrip("/")) + 1 :]
        if cwd == home or cwd.startswith(home + "/"):
            tilde = "~" if cwd == home else "~/" + cwd[len(home) + 1 :]
            return tilde + "/" + rel
        return rel
    if not parent.startswith("/") and cwd == home:
        return "~/" + parent
    if not parent.startswith("/"):
        return "~/" + parent if not parent.startswith("~") else parent
    return parent


def _apply_dir_alias(text: str, dirs: list[dict]) -> str:
    if not text or not dirs:
        return text
    for d in sorted(dirs, key=lambda x: max(len(f) for f in x["forms"]), reverse=True):
        for form in sorted(d["forms"], key=len, reverse=True):
            if form and form in text:
                text = text.replace(form, d["alias"])
    return text


def attach_dir_aliases(records: list[dict], cwd: str | None) -> list[dict]:
    """Insert meta dir rows and rewrite later path prefixes to <d1>, <d2>, …"""
    dirs: list[dict] = []
    out: list[dict] = []
    str_keys = (
        "path",
        "summary",
        "command",
        "stat",
        "text",
        "message",
        "description",
        "error",
        "response",
        "hunk",
    )

    def intern(parent: str, ts: str) -> dict | None:
        parent = parent if parent.endswith("/") else parent + "/"
        for d in dirs:
            if d["parent"] == parent:
                return None
        alias = f"<d{len(dirs) + 1}>"
        value = _dir_value(parent, cwd)
        rec = {
            "type": "meta",
            "kind": "dir",
            "ts": ts,
            "alias": alias,
            "value": value,
            "parent": parent,
        }
        dirs.append(
            {"alias": alias, "parent": parent, "value": value, "forms": _dir_forms(parent, cwd)}
        )
        return rec

    for rec in records:
        rec = dict(rec)
        if rec.get("type") == "meta":
            out.append(rec)
            continue
        parent = _parent_dir(rec.get("path") or "")
        if parent:
            intro = intern(parent, rec["ts"])
            if intro:
                out.append(intro)
        for k in str_keys:
            if k in rec and isinstance(rec[k], str):
                rec[k] = _apply_dir_alias(rec[k], dirs)
        if "known_well" in rec:
            rec["known_well"] = [
                _apply_dir_alias(p, dirs) for p in rec["known_well"]
            ]
        out.append(rec)
    return out


def attach_file_aliases(records: list[dict], min_uses: int = 2) -> list[dict]:
    """Second pass: alias files that show up more than once (`<f1>`, `<f2>`, …).

    Runs after dir aliases so values look like `<d5>projects`. A path used
    once is left as-is — the meta row would cost more than it saves.
    """
    counts: dict[str, int] = {}
    for rec in records:
        if rec.get("type") != "tool":
            continue
        p = rec.get("path")
        if isinstance(p, str) and p:
            counts[p] = counts.get(p, 0) + 1
    reuse = {p for p, n in counts.items() if n >= min_uses}
    if not reuse:
        return records

    interned: dict[str, str] = {}
    out: list[dict] = []
    for rec in records:
        rec = dict(rec)
        p = rec.get("path")
        if rec.get("type") == "tool" and isinstance(p, str) and p in reuse:
            if p not in interned:
                alias = f"<f{len(interned) + 1}>"
                interned[p] = alias
                out.append(
                    {
                        "type": "meta",
                        "kind": "file",
                        "ts": rec["ts"],
                        "alias": alias,
                        "value": p,
                    }
                )
            rec["path"] = interned[p]
            if "known_well" in rec:
                rec["known_well"] = [interned.get(x, x) for x in rec["known_well"]]
        out.append(rec)
    return out


def load_transcript(path: Path) -> list[dict]:
    recs = []
    with path.open() as f:
        for line in f:
            if line.strip():
                recs.append(json.loads(line))
    return recs


def _parse_ts(ts: str) -> datetime:
    raw = ts[:-1] + "+0000" if ts.endswith("Z") else ts
    if "." in raw:
        return datetime.strptime(raw, "%Y-%m-%dT%H:%M:%S.%f%z")
    return datetime.strptime(raw, "%Y-%m-%dT%H:%M:%S%z")


ELIDE = 150
ELLIPSIS = "…"


def _one_line(text: str, n: int | None = ELIDE) -> str:
    s = " ".join((text or "").split())
    if n is None or len(s) <= n:
        return s
    return s[:n] + ELLIPSIS


def format_print(records: list[dict], verbose: int = 0) -> str:
    if not records:
        return "(empty transcript)\n"
    lines = []
    prev_ts = _parse_ts(records[0]["ts"])
    for i, rec in enumerate(records):
        ts = _parse_ts(rec["ts"])
        delta = 0.0 if i == 0 else max(0.0, (ts - prev_ts).total_seconds())
        prev_ts = ts
        typ = rec.get("type")
        name = rec.get("name") or ""
        dialog_n = None if verbose >= 1 else ELIDE
        if typ == "user":
            role, body = "user", f'"{_one_line(rec.get("text") or "", dialog_n)}"'
        elif typ == "assistant":
            role, body = "agnt", f'"{_one_line(rec.get("text") or "", dialog_n)}"'
        elif typ == "meta":
            role = "meta"
            kind = rec.get("kind") or "meta"
            if kind in ("dir", "file"):
                body = f'{kind} | {rec.get("alias")} = "{rec.get("value")}"'
            elif kind == "ts":
                body = f"ts | {rec.get('value') or rec.get('ts')}"
            elif kind == "cwd":
                body = f"cwd | {rec.get('value') or rec.get('path')}"
            else:
                body = f"{kind} | {rec.get('value') or ''}"
        elif name == "commit" or typ == "commit":
            role = "tool"
            bits = ["commit"]
            if rec.get("sha"):
                bits.append(rec["sha"])
            msg = rec.get("message") or ""
            if msg:
                bits.append(f'"{_one_line(msg, dialog_n)}"')
            extra = _stat_result_line(rec.get("stat") or "")
            if extra:
                bits.append(extra)
            joined = " | ".join(bits)
            body = joined if verbose >= 1 else _one_line(joined)
        else:
            role = "tool"
            parts = [name]
            path = rec.get("path") or ""
            if path:
                parts.append(path)
            extra = rec.get("summary") or ""
            if extra == name:
                extra = ""
            if not extra and rec.get("span"):
                extra = rec["span"]
            if extra:
                parts.append(extra)
            response = strip_eval_unmatched(
                rec.get("response") or rec.get("error") or ""
            )
            if not response and rec.get("stat") and not rec.get("enveloped"):
                response = _stat_result_line(rec["stat"])
            response = " ".join(response.split()) if response else ""
            if response:
                parts.append(response)
            body = _one_line(" | ".join(parts))
        row = f"{delta:7.2f} | {role:<4} | {body}"
        if typ == "user":
            lines.append("")
        lines.append(row)
    return "\n".join(lines) + "\n"


def event_ts_ms(obj: dict) -> int:
    meta = (obj.get("params") or {}).get("_meta") or {}
    if "agentTimestampMs" in meta:
        return int(meta["agentTimestampMs"])
    return int(obj["timestamp"]) * 1000


def load_updates(path: Path) -> list[tuple[int, dict]]:
    events = []
    with path.open() as f:
        for line in f:
            if not line.strip():
                continue
            o = json.loads(line)
            events.append((event_ts_ms(o), o["params"]["update"]))
    return events


def main(argv: list[str] | None = None) -> int:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("jsonl", type=Path, help="precompaction updates.jsonl, or transcript jsonl with --print")
    p.add_argument("-o", "--out", type=Path, default=None)
    p.add_argument(
        "--print",
        action="store_true",
        help="pretty-print an already-built transcript jsonl (same view as convert)",
    )
    p.add_argument(
        "-v",
        action="count",
        default=0,
        help="do not elide user, agent, or commit-message bodies (tools still 150; -vv later)",
    )
    args = p.parse_args(argv)
    if args.print:
        sys.stdout.write(format_print(load_transcript(args.jsonl), verbose=args.v))
        return 0
    records = convert(load_updates(args.jsonl))
    view = format_print(records, verbose=args.v)
    out = args.out
    if out is None:
        sys.stdout.write(view)
    else:
        out.parent.mkdir(parents=True, exist_ok=True)
        with out.open("w") as f:
            for rec in records:
                f.write(json.dumps(rec, ensure_ascii=False) + "\n")
        view_path = out.with_suffix(".txt")
        view_path.write_text(view)
        sys.stdout.write(view)
    counts: dict[str, int] = {}
    for rec in records:
        counts[rec["type"]] = counts.get(rec["type"], 0) + 1
    dest = f"{out} + {out.with_suffix('.txt')}" if out else "stdout (view)"
    print(f"{len(records)} records  {counts}  -> {dest}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
