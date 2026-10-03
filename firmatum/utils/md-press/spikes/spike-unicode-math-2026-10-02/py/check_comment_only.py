"""Show that two versions of a Python file differ only in comments.

usage: python3 py/check_comment_only.py <git-rev> <path> [<path> ...]
Compares the file at <git-rev> with the working copy: the token streams
with comments removed, and the parsed ASTs, must both be identical.
Used for the 2026-10-03 comment-only scrub of the frozen converters
(notes/LOG.md §29)."""
import ast, io, subprocess, sys, tokenize


def tokens(src):
    return [(t.type, t.string) for t in tokenize.generate_tokens(io.StringIO(src).readline)
            if t.type not in (tokenize.COMMENT, tokenize.NL, tokenize.NEWLINE)]


rev, paths = sys.argv[1], sys.argv[2:]
ok = True
for p in paths:
    old = subprocess.run(['git', 'show', f'{rev}:./{p}'], capture_output=True, text=True, check=True).stdout
    new = open(p, encoding='utf-8').read()
    t = tokens(old) == tokens(new)
    a = ast.dump(ast.parse(old)) == ast.dump(ast.parse(new))
    diff_lines = sum(1 for x, y in zip(old.splitlines(), new.splitlines()) if x != y)
    print(f'{p}: non-comment tokens identical={t}  AST identical={a}  lines changed={diff_lines}')
    ok &= t and a
sys.exit(0 if ok else 1)
