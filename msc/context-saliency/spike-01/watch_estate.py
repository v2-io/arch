"""Unhypothesized watch: a real estate transcript through the instrumented model.

Transcript: firmatum/practica/msc/session-prompts-2026-05-19_20.md (Joseph's
verbatim prompts from a long 02-harvest session). No Plan-NIAH generator.
Segments are the numbered prompts. Probes target dormant-but-binding
corrections (#6 01-is-throwaway, #18 audits-are-not-findings) and a landmark
(harvest capture format). Occluding #18 and re-asking the audit question is
the delayed-reuse analog on real words.
"""
import json
import os
import re
import sys
from pathlib import Path

from tasks import Segment, occlude, VaultTask
from rig import run_instrumented, run_plain, extract_master

TRANSCRIPT = Path.home() / "src/arch/firmatum/practica/msc/session-prompts-2026-05-19_20.md"
OUT = Path("out/watch")

PROBES = {
    "p02": (
        "In one short paragraph: according to Joseph in this session, what is "
        "document 02 supposed to *be*? Engineering-normative bridge from findings "
        "to practice, or the heavy moral is-ought document? Quote a few of his words."
    ),
    "p18": (
        "An auditor left an F-number candidate in AUDIT-WORKING that disagrees "
        "with current canon. For a harvest entry on that segment, should you "
        "include an 'Audit caveat' hedging the canon claim? Answer YES or NO "
        "on the first line, then one sentence of why, from this session."
    ),
    "p06": (
        "When drafting 02, should 01-theory be treated as authoritative input? "
        "Answer YES or NO on the first line, then quote Joseph's stance in this session."
    ),
    "p16": (
        "What capture format did Joseph lock for harvest entries in this session? "
        "Give the template in his words, one line."
    ),
}


def parse_transcript(text):
    parts = re.split(r"(?=^## \d+)", text, flags=re.M)
    segs = []
    pos = 0
    header = parts[0]
    segs.append(Segment("preamble", 0, pos, pos + len(header), header, False))
    pos += len(header)
    for p in parts[1:]:
        m = re.match(r"## (\d+)", p)
        n = int(m.group(1)) if m else 0
        segs.append(Segment("section", n, pos, pos + len(p), p, reused_detail=(n == 18)))
        pos += len(p)
    return segs


def wrap_prompt(body, question):
    preamble = (
        "You are reading an archive of a real working session between Joseph "
        "and an agent. Answer from this archive, not from general knowledge. "
        "If the archive does not contain the answer, say you do not have it.\n\n"
        "----- ARCHIVE -----\n\n"
    )
    q = "\n\n----- QUESTION -----\n" + question + "\n"
    return preamble + body + q


def build_task(question, occlude_section=None):
    raw = TRANSCRIPT.read_text()
    segs = parse_transcript(raw)
    body = "".join(s.text for s in segs)
    # rebuild with wrap so offsets include preamble/question
    wrapped_pre = (
        "You are reading an archive of a real working session between Joseph "
        "and an agent. Answer from this archive, not from general knowledge. "
        "If the archive does not contain the answer, say you do not have it.\n\n"
        "----- ARCHIVE -----\n\n"
    )
    q = "\n\n----- QUESTION -----\n" + question + "\n"
    pos = 0
    out_segs = []
    def emit(text, role, room, reused=False):
        nonlocal pos
        s = Segment(role, room, pos, pos + len(text), text, reused)
        out_segs.append(s)
        pos += len(text)
        return s
    emit(wrapped_pre, "preamble", 0)
    for s in segs:
        emit(s.text, s.role, s.room, s.reused_detail)
    emit(q, "question", 0)
    t = VaultTask(variant="watch", n_rooms=0, prompt="".join(x.text for x in out_segs),
                  segments=out_segs, codes=[], answer="", reuse_room=18 if occlude_section == 18 else 0)
    if occlude_section is not None:
        # occlude that numbered section (role section, room=N)
        prompt = occlude(t, ("interior", occlude_section))  # won't match
        # manual: section role
        parts = []
        for s in t.segments:
            if s.role == "section" and s.room == occlude_section:
                body = s.text.rstrip("\n")
                n_nl = len(s.text) - len(body)
                filler = ("nothing else of note here and the record stays quiet " * (len(body) // 51 + 1))[:len(body)]
                parts.append(filler + "\n" * n_nl)
            else:
                parts.append(s.text)
        t.prompt = "".join(parts)
        # keep original segments for mapping; prompt is occluded
    return t


def main():
    OUT.mkdir(exist_ok=True)
    which = sys.argv[1] if len(sys.argv) > 1 else "all"
    probes = list(PROBES) if which == "all" else [which]
    for name in probes:
        q = PROBES[name]
        t = build_task(q)
        print(f"== watch {name} n_segs={len(t.segments)} chars={len(t.prompt)}", flush=True)
        r = run_instrumented(t.prompt, t.segments, max_new=220)
        r["probe"] = name
        r["question"] = q
        r["occluded"] = None
        json.dump({k: v for k, v in r.items() if k != "steps"}, open(OUT / f"{name}_meta.json", "w"), indent=1)
        # keep steps in a full dump
        json.dump(r, open(OUT / f"{name}.json", "w"))
        print(f"  steps={r['n_steps']} ctx={r['n_ctx']} sec={r['seconds']}", flush=True)
        print("  GEN:\n" + r["gen_text"][:1500], flush=True)
        print("  ----", flush=True)
    if which in ("all", "p18"):
        q = PROBES["p18"]
        t = build_task(q, occlude_section=18)
        print("== watch p18_occl18 (section 18 replaced with filler)", flush=True)
        text = run_plain(t.prompt, max_new=220)
        open(OUT / "p18_occl18.txt", "w").write(text)
        print("  GEN:\n" + text[:1500], flush=True)


if __name__ == "__main__":
    main()
