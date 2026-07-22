# fmt-md — synthesis and plan

*2026-07-22. Synthesis over `PROBLEM.md` + `research/landscape.md` (empirical tool survey) + `research/requirements.md` (40 requirements, R-numbers cited below) + `research/unwrap-failure-modes.md`. Decision status: recommended, awaiting Joseph's ratification on the marked questions.*

## The decision picture

**Adopt-only is honestly ruled out, on evidence.** Every real formatter normalizes *all* soft breaks (destroying licensed chunk-per-line prose, format.sop §Line Wrapping); every linter only wraps, never unwraps; nothing knows the discovered math rules; the only tool with a genuine line-break policy (mdslw) mangles display math. The policy layer — "join column-wrap debris, preserve deliberate chunks, flag when unsure" — does not exist anywhere in the ecosystem. That, plus the math-rule layer (implemented nowhere but our own lint-md), is the build case. It is narrower than "build a markdown formatter": the parser substrate exists (comrak, with math-dollars / wikilinks / frontmatter natively opaque).

**No interim pipeline on live corpora (Joseph, 2026-07-22 — hard ruling).** A dprint-based interim pass was run on udon-needs `02-tooling-needs/` and **reverted at Joseph's direction**: running half-solutions on directories with active work was outside the mandate. The episode still yielded evidence: dprint (`textWrap: never`) matched the March human reflow on 2/3 history pairs and passed idempotence — but its printer **altered code content in 2 of 45 real files** (dedented a ` ```text ` fence's ASCII art; trimmed a semantic trailing space inside an inline code span), caught only by the per-file render-equality gate. Minimal repros are in `fixtures/sharp-edges/`. Standing conclusion: existing formatters are research references and test oracles only; **nothing runs against a live tree until the real tool exists and that run is itself the commissioned work.**

**Recommended shape for the real tool: a fresh Rust crate on comrak** (working name `fmt-md`, home decided at package time — `utils/` vs `common/utl/` with the other udon-core-adjacent Rust tools).

Why comrak over forking dprint-plugin-markdown, the other serious shape:

- comrak already treats this corpus's foreign regions as first-class (`math-dollars`, `wikilinks-title-after-pipe`, `front-matter`) — R2/R3/R4 solved at the parser, which the landscape identified as the real root cause alongside the missing policy layer.
- The join policy needs source positions + statistics over raw lines (fill-column inference, paren-depth, R15f signals); comrak's AST carries source spans, and we emit via our own minimal-diff writer over original bytes (R11 byte-stability; dprint's printer normalizes by design — its table churn and cell deletion live exactly there).
- The rule registry (R32 — link rules and future house conventions) wants our architecture, not a patch series against someone else's printer.
- Udon-ecosystem affinity: same language, and the policy/rule engine is the part that later generalizes to udon documents; the markdown parser stays swappable behind it.
- Cost acknowledged: we re-implement the join engine dprint already has. The engine is not the hard part (the policy is), and the `de1082d` pairs plus dprint's outputs give us its behavior as a scored reference.

## Architecture sketch (for the implementing agent — a starting frame, not a work order)

Pipeline: **parse (comrak, all extensions) → foreign-region map (R2–R4, byte-opaque) → block classifier (R15/R15f/R16: per-block wrap vs chunk vs ambiguous, file-level statistics as prior) → join engine (R17 container-aware, R15a–e guards) → rule registry pass (math R22–R31, blank lines R19–R21, house rules R5, future link rules) → minimal-diff emitter (original bytes except where a rule fired) → verify (re-parse render-equality R12, idempotence R11)**. Check mode = same pipeline, diagnostics only, machine-readable (R14; agents are a primary diagnostics consumer).

## Phases

- **Phase 0 — interim relief + fixtures (no crate yet).** (a) Harvest the `de1082d` 14-pair ground-truth set + later reflow commits into `fixtures/`; (b) build the fixture-regeneration harness (R35: prompt battery × model roster, chat-vs-file asymmetry gives labeled pairs); (c) optionally patch lint-md's `\w$` join test (one line, the single highest-leverage known bug) for immediate incremental relief; (d) after Q1/Q2 ratification, run the guarded interim pipeline on udon-needs — the commissioning need — with render-diff gate and diff review before commit.
- **Phase 1 — crate skeleton:** comrak parse + foreign-region map + minimal-diff emitter + the two invariant harnesses (idempotence, render-equality) running over all three corpora as CI property tests.
- **Phase 2 — the policy layer:** block classifier + join engine against the scored fixture set (target: ≥ the interim pipeline's score on history pairs, *without* joining the chunk-per-line segments it would wrongly join — the differentiator no existing tool has).
- **Phase 3 — the rule registry:** port lint-md's math rules (R22–R31, now over real spans, multi-line-safe per R31), blank-lines (R19–R21), Unicode-math promotion (flag-first, per the reframe's disease form), config discovery + frozen-region exclusions (R10/R33 — ship asf and udon defaults).
- **Phase 4 — rollout:** replace lint-md's overlapping checks in asf (keeping its asf-specific outline/slug tooling), adopt in vivarium + udon + ops; retire the interim pipeline.

## Policy — RESOLVED (Joseph, 2026-07-22)

Joseph rejected the block-classifier + leave-and-flag framing: *"the point of this project was to see if we could avoid yet another heuristic approach that does it somewhat and forces intervention the rest of the time"* — and: the tool must **remove** a job from his plate, not mint flag-triage jobs. The principled replacement, which the evidence supported all along:

- **Canonical form: one logical line per prose paragraph, joined deterministically.** A soft break is render-invisible, so structure-aware join-all (through the parser, with the §3b construct guards and render-equality verification) provably never changes the rendered document — it is safe by construction, not aggressive. The recovered March human policy already was join-all (sentence-per-line prose was approvingly joined). format.sop *permits* chunk-per-line authoring; the formatter canonicalizes it away, exactly as those passes did. No block classifier, no ambiguity disposition, no flags. Total function, idempotent. (R15/R16's statistics demote to optional diagnostics; R15a–f guards and R17 container-awareness remain load-bearing.)
- **Lists/blockquotes/footnotes are structural, never heuristic:** a continuation joins because the parse says same-paragraph-same-container — the parser's judgment, not a line-shape guess. (This was the direct answer to "are you including lists in this ad-hoc heuristic?" — no; the heuristic layer is deleted entirely.)
- **Residual semantic-risk cases (misparses R15e, malformed tables) route to an agent, never to Joseph**; mechanically, the render-equality gate catches them.
- Chunk-per-line preservation, if ever wanted for a specific file, is an explicit opt-out (config exclusion or in-file directive), not an inference problem.

Build shape ratified: **fresh Rust crate on comrak.** Ownership: this project is delegated — decisions default to the implementing agent(s), verified mechanically, not gated on Joseph.
