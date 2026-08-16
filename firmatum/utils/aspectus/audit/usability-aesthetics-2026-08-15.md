# Usability & aesthetics pass — 2026-08-15 (coordinator, primed)

*Binary `aspectus 0.1.8+0b76d27.dirty (built 2026-08-14T22:09:14Z)` — the last code commit; HEAD `549c5b5` differs from it only in docs/inbox. Suite: 232 tests green (`cargo test --release`). Method: read the outline, origin, PRACTICA, every `design/` and `impl/` file, the defs, all four `audit/` files, and `form-agentic-eyes` whole; then used the tool as an end-user on the crate, `~/src` (depth 1/2/3 × lines 200/300 — the vertical-info gauntlet), `asf`, `/tmp`. Grounds per item as in the prior audits: **verified** = ran it / read the lines; **struck me** = end-user impression. Nothing fixed, nothing committed to code. Numbered so the discussion with Joseph can point at items.*

*The prior fable instance's spike (`2c7ded7`, reset away, reflog-only) was **not** consulted — it was built without the pipeline; if any of it turns out useful it can be cherry-picked deliberately later.*

---

## A. The three inbox asks (Joseph, 2026-08-14) — routed, all live

All three are real on the current binary and all three are one design cluster: **the fact-columns should hold every fact of their kind, and a line's non-column material should be allowed to spill to owned sub-lines rather than shove the columns around.** Routed into `design/vertical-info.md` (§Steward asks) and `design/overview-invariants.md` (config drift). Details in those files; the outline rows carry the pointers.

1. **Mass lines belong in the `lines` column** — verified: `synthesis/  0.00 · 8.8d ago  [md×4]  ≈681 lines` puts the one number the column exists for at the far end of the line, unaligned. (→ vertical-info; interacts with `design/mass.md` §rendering, whose "lines follow the census" call this supersedes.)
2. **Header names every effective setting that differs from built-in default** — verified need: my `aspectus config` shows `depth = 3 (user-home)`; every look I ran "at defaults" was depth 3, and nothing on the look said so. (→ overview-invariants, new subfeature, unbuilt.)
3. **Wrap a line's description material to owned sub-lines so columns stay put; `--lines` counts logical lines** — verified: `claim-naming-criteria.md -> ../../../theory/src/claim-naming-criteria.md     52   0.00 · 5.2d ago` — the symlink decoration is name-fused, so the whole far-right block slides. (→ vertical-info; the design already listed "a line owning sub-lines" as material — now a steward ask with the budget rule decided: *logical* count.)

## B. Verified defects / rough edges on the current look

4. **Empty directory prints bare** — verified (`~/src/glimmer/`, `~/src/semachrome/`, `arch/INGEST/`): a bare `glimmer/` with no census, no cluster, nothing. Every non-empty dir carries *something* (census or children), so bare *means* empty — but only to a reader who already knows the census law; a cold reader sees the same shape as "no facts obtained." `design/dir-census.md` says "Empty directories print no census" — deliberate then; I'd argue empty is a fact worth one glyph (`[empty]` / `∅`, spelling Joseph's), same honesty family as `[denied]`: the nothing-case should be legible as nothing, not as silence. **Ask for ratification.**
5. **Dangling `·` when a git-known line has age but no score** — verified: `principia-ars-technica/                          ·  4.2d ago` (whole repo has no scored files at the visible depth) and `.dedupe-work/ · 4.2d ago`. The close-tranche decision ("a git-known unscored line carries its age; score absent, never faked") is right; the *separator* surviving with nothing to separate reads as a render glitch. Options: drop the `·` when the score is absent (age still right-aligned under `age`), or keep it as the deliberate "unscored" mark and teach it. Aesthetic call — Joseph's.
6. **`has:` claims `≈0f`** — verified, still present: `memorata/ … [has: agents ≈0f, …]`, `causal-language/ … agents ≈0f`, `vivarium/ … trash ≈0f`. Close-audit §7 flagged it; unfixed. A presence claim with a zero-file count reads as a contradiction; either suppress the count at 0 or render `(empty)`.
7. **Numeric-suffix junk in censuses** — verified, still present: `_ref/ [… · 1×1 · 2×1 · 3×1 · 4×1 · 5×1 · 6×1 · …]` (man-page / numbered files). Close-audit §7 flagged it. Cheap fix: all-digit suffixes fold to `other` (or to a `numbered×N` bucket).
8. **Root line's age is the root inode's mtime** — verified on this crate: root says `4.3h ago` while children say `4m ago`. Close-audit §7 raised "newest beneath" for the root; still as it was. Small but it is the first number a reader sees.
9. **Help page is a ~230-line unstructured essay after the Options block** — verified. Everything after `Examples:` is fourteen paragraphs with no headings: line counts, mass, heat, symlinks, gitignore, globify, readme-title, furniture, honesty marks, quiet, important files, JSON, headings, footer, config. Two hallway testers already called it "a moving essay" / "a lot of ontology before I have a tree." The content is good and is the law channel; the *shape* fights scanning. Cheap, high-leverage: section headings (`Honesty marks`, `Aliveness (heat · age)`, `Furniture`, `Quiet facts`, `Asking for more`, …) — `form-help-shape` fixes usage → what-it-is → commands → options → examples and says nothing against headed sections after that. Also verified: the `--inspect git` example wraps mid-sentence into three ragged lines in the Examples block.
10. **`perms` heading appears at `~/src` depth 2 for three `700` directories** — verified (`Paperpile/`, `arXiv-*/`, `linkedin-archive-*/`); it is quiet working as designed (700 among 755s is odd), so not a defect — but note the far-right block at `~/src` depth 2 is now four columns (`lines perms heat · age`) and at `/tmp` six (`lines perms owner size mtime heat · age`). Vertical-info should decide the far-right block's ceiling deliberately.
11. **The `~/src` gauntlet, measured** — verified: `--lines 300 --depth 3 ~/src` renders in 2.1 s and 44 of 302 lines exceed 200 characters; the widest (`_self/` at depth 1) is ~330. This is the vertical-info row's problem statement, confirmed unchanged.

## C. Things that read well (so the pass isn't only defects)

12. **verified, good:** the `~/src --depth 1` look remains the killer artifact — every repo with branch/sha/dirty/mass/aliveness in one screen; the has-spot answered "what is this place" for 30 dirs before any README.
13. **verified, good:** refusals — `--sort nonsense`, `--inspect nope`, missing path — all class-named with a next action; SIGPIPE clean; `--format json | jq` valid.
14. **verified, good:** `aspectus config` fact inventory is exactly the discoverability surface `design/columns.md` asked for; I learned every ask from it without opening source.
15. **struck me:** the heading line (`lines  heat · age`) fixed the unglossed-number problem completely for me; `≈`/`~`/`≥` read as three different honesties once the help paragraph was read once.

## D. Recommendation for the discussion (mine, held lightly)

The next code wave is the **vertical-info** row and it now has enough decided material to design and split for delegation: (1) mass into the `lines` column; (2) an owned-sub-line mechanism (facets/has/link targets spill below the name, indented, no columns; logical line count); (3) far-right block ceiling per look; (4) the config-drift header line. Items 4–8 above are small independent fixes that can ride the same wave as separate agent tasks with fixtures. Item 9 (help sections) is a one-agent job with no design dependency.

Not recommending: touching heat's cross-repo scale, the root-vs-dirs budget tension, or presets — those are steward-grade decisions with open design.
