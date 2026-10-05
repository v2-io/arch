#!/usr/bin/env python3
"""Recount what Gemini (via agy) tried to run when it reached for a shell during
the glyph-sequence study, from the harvested agy transcripts:

    python3 gemini-shell-attempts.py ~/src/arch/asf/empirica/glyph-sequence-perception/refactored/data/agy/brain

Unit: one conversation that issued at least one run_command. Categories overlap
(a conversation counts in every one it matches), using these rules over all its
commands joined together:

  names       unicodedata.name / unicodedata.lookup / name(
  arithmetic  chr(ord(..)±..), chr(0x..±..), chr(<var>) in a loop, range(0x..), range(ord(..)), ord(..)±n
  ord/hex     ord( or hex(

Conversations matching neither names nor arithmetic are printed, for hand review.
Result on 2026-10-05 (172 conversations): names 149, arithmetic 31 (12 also
names), neither 4 (3 ord/hex only; 1 built neighbours from literal code points,
chr(0x336D) chr(0x336E) ..., i.e. chart neighbours without arithmetic).
"""
import collections, glob, json, os, re, sys

if len(sys.argv) != 2:
    sys.exit(__doc__)
root = sys.argv[1]
NAME = re.compile(r'unicodedata\.(name|lookup)|\bname\(')
ARITH = re.compile(r'chr\s*\(\s*(ord\s*\(|0x[0-9a-fA-F]+\s*[+-]|x\b|i\b|c\b|cp\b|o\b|n\b|code)|range\s*\(\s*(0x|ord\s*\()|ord\([^)]*\)\s*[+-]\s*\w|\bchr\(\s*\w+\s*[+-]')
ORD = re.compile(r'\bord\s*\(|hex\s*\(')
conv = {}
for f in sorted(glob.glob(os.path.join(root, '*/.system_generated/logs/transcript_full.jsonl'))):
    cmds = []
    for line in open(f, encoding='utf-8'):
        try:
            s = json.loads(line)
        except Exception:
            continue
        for t in s.get('tool_calls') or []:
            if t.get('name') == 'run_command':
                a = t.get('args') or {}
                cmds.append(a.get('CommandLine') or a.get('command') or json.dumps(a))
    if cmds:
        conv[f] = '\n'.join(cmds)
c = collections.Counter()
for f, t in conv.items():
    n, a, o = bool(NAME.search(t)), bool(ARITH.search(t)), bool(ORD.search(t))
    c['names'] += n; c['arithmetic'] += a; c['arithmetic and names'] += a and n; c['ord/hex'] += o
    if not (n or a):
        c['neither names nor arithmetic'] += 1
        print('neither:', t[:200].replace('\n', ' | '))
print(f'conversations with run_command: {len(conv)}')
for k, v in c.items():
    print(f'  {k}: {v}')
