# Dot-only directory

A directory whose every name starts with `.` says `[dot-only]`. Nothing ordinary lives there: only `.DS_Store`, `.gitkeep`, `.git/`, `.archive/`, `.integrated/`. Before this row such a directory rendered bare or carried only a has-spot, the same shape as a folded directory, the last case [[empty-dir|Empty directory]] left open.

**Decided** (Joseph, 2026-10-03, [[../audit/inbox-2026-10-03|verbatim]]): *"for the '.DS_Store' or, let's say '.keep' etc. ---- let's say that any directory with *only* '.'-prefixed children within it (directory or file or symlink) should get some other kind of status 'empty-ish' or something, that is explained in the help."* On the edge the coordinator raised (a dir whose dot children are hidden furniture already speaking through `[has: …]`): *"empty-ish with .git / .archive being shown is fine with me. No need to hide what we already know might be relevant..."* So the rule is literal, and nothing yields to anything.

## The law

- **Truth condition:** readdir finished, yielded at least one name, and every name starts with `.`. Like `[empty]`, the test runs on the raw names before the furniture map, ignore rules, or show-all filter anything. The child's kind does not matter (dir, file, symlink, special), per Joseph's rule.
- **Spelling — a proposal (Joseph ratifies; "empty-ish" was his placeholder):** `[dot-only]`.
  - It is a word, following the `[denied]` and `[empty]` precedent, and it states the rule rather than a judgment.
  - "empty-ish" asks the reader to guess what the -ish covers. "hidden" collides with furniture's *hide* fate, which is a different mechanism.
  - `[only .*]` (a glob, which agents read fluently) was the runner-up. It was not chosen because a glob inside a mark invites being read as a pattern to apply.
- **Place:** near-right with the marks, right after where `[empty]` would sit. The two are exclusive by construction: `[empty]` needs zero names, `[dot-only]` at least one.
- **Display:** always, not quietable (existence information, the same register as `[empty]`).
- **Coexists with everything the line already says.** `[has: archive ≈1f]  [dot-only]`, `[.gitkeep]  [dot-only]`, `[.integrated/ ≈13f]  [dot-only]`. The has-spot or census says *what* is there; the mark says that nothing *else* is.
- **Every directory line:** an expanded level, a depth cutoff, a symlinked dir (the target's names), and the root's facts line.
- **Not claimed** where the look did not read the directory (`⊘` ignored, `[denied]`, `[unreadable: io]`, `[cycle]`, `[other fs]`), for the same reason as `[empty]`: the mark would be a guess.
- **JSON:** `"dot_only": true`, present-when-true, additive within schema 1. It does not set `truncated`.

## Specimens (dogfood, 2026-10-03)

- `~/src --depth 2`: `audits/ ~3·872. … [.integrated/ ≈13f]  [dot-only]`. The whole of that dir's content sits behind one dot-name, which no earlier look said plainly.
- `~/src --depth 2`: `def/ … [.gitkeep]  [dot-only]`, `findings/ … [.gitkeep]  [dot-only]`.
- `~/src/arch --depth 4`: two `src/ … [.gitkeep]  [dot-only]`.
- Fixture: `dsonly/ [dot-only]` (only `.DS_Store`, which the map omits, so it was bare before) and `arch-only/ [has: archive ≈1f]  [dot-only]`.

## Subfeatures

| # | Sub | Behavior | Test |
|---|---|---|---|
| 1 | The rule | Omitted, listed, hidden-furniture, and symlink dot children each make the dir `[dot-only]`; a dir with any ordinary name does not. | `tests/dot_only.rs::every_name_dotted_says_so` |
| 2 | Coexists | The has-spot stays beside the mark. | same |
| 3 | Distinct from `[empty]` | An empty dir says `[empty]`, never both. | same |
| 4 | Root + JSON | The root's facts line; `dot_only: true`; not truncation. | `root_expanded_and_json` |
| 5 | Help | Taught in the honesty paragraph. | `help_teaches_dot_only` |

## Open

- **Redundant beside a one-name census.** `[.gitkeep]  [dot-only]` says the same thing twice: the census already shows the lone name is dotted. Kept, because the rule is literal and a uniform mark is scannable down a column in a way a census's first character is not. If it grates, the narrow exception would be "omit the mark when a one-name census already shows it". *Why open:* this is aesthetic, and Joseph's.
- **Spelling** — above. *Why open:* vocabulary is Joseph's to ratify.

## Foundations

[[empty-dir|Empty directory]] (the sibling mark; this row closes its `.DS_Store` Open) · [[denied|Denied]] (the word-mark precedent) · [[summarization|Summarization]] (never a silent shape) · [[furniture|Furniture]] (hide/omit fates, has-spot) · [[lattice-2|Lattice 2]] (honesty marks).
