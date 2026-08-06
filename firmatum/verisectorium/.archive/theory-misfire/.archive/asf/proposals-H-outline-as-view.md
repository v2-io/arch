<!--
  Verisectorium notes gather — extract, not full-file authority.
  Provenance: arch/asf/PROPOSALS.md §H (approx. lines 403–438), especially H.4–H.5
  Copied: 2026-08-05
  Full source also has portfolio content unrelated to verisectorium.
  Do not edit here expecting to update the live original.
-->

# Extract: ASF PROPOSALS §H — outline-as-view / segments evergreen

*From `arch/asf/PROPOSALS.md` (portfolio audit conventions). The load-bearing
verisectorium material is items **4** and **5**; items 1–3 are general audit
hygiene retained for context.*

---

## §H. Conventions for future audits

Four operating principles worth preserving from this audit cycle:

1. **Retire aggressively, distribute pointers.** Proposals that have landed, split, or been superseded by other work should move to §A or §F, not remain in the active portfolio. Each retirement carries a "where it lives now" pointer so the navigability isn't lost.

2. **Bundle before ranking.** The two cross-cutting bundles (framework-face reframe; Section III completion) are the highest-leverage organizing moves. Treating their members as individual proposals understates their coupling; treating them as bundles surfaces what the real work-items are.

3. **Freshness is structurally inevitable.** Proposals written during one cycle become stale as subsequent cycles land partial absorption. The audit structure should explicitly surface "proposal is stronger than the entry reads" (O-BP10 post-DA2'-inc) and "portfolio has expanded underneath the proposal" (O-BP11) as first-class conditions, not edge cases.

4. **Outlines are cheap; segments are expensive.** `bin/build` accepts any outline file and reassembles segments in the specified order. The segment substrate is presentation-neutral — an outline is a *view* (a particular selection + ordering + framing prose) over that substrate. Multiple outlines can coexist, each buildable, each telling a coherent story, sharing the same segment atoms. **Before scoping a proposal as a segment rewrite, ask: can this land as a new outline?** Several proposals in this portfolio are cheaper and cleaner as outline-views than as segment-edits:

   - **O-BP15 (comprehensive worked example)** — most naturally a new `OUTLINE-WORKED-EXAMPLE.md` weaving existing segments through one motivating domain, not a new monolithic segment.
   - **Bundle 1 (framework-face reframe)** — an `OUTLINE-EPISTEMIC-ARCHITECTURE.md` opening with the three meta-segments then organizing Section I/II/III around them is far cheaper than multi-segment preamble rewrites. The README update is still segment-external (public surface), but the structural commitment lives in one new outline file. Independence becomes **high** in the outline-view form.
   - **SP-13 (emergence conditions)** — may be the right first form: a new outline selecting emergence-relevant segments across AAT + `03-` + `04-` with framing prose, testing the shape as a view before forcing the AAT-core-vs-logozoetic-core placement decision.
   - **Three-way presentation split (2026-03-13 review)** — was retired as superseded, but under the outline-as-view affordance the three views can coexist with the convergent epistemic-architecture reframe. `OUTLINE-CORE-RESULTS.md` / `OUTLINE-CONDITIONAL-ARCHITECTURE.md` / `OUTLINE-EMPIRICAL-PROGRAMS.md` would each be a new outline; none requires segment edits. Worth reopening as a low-cost set of reading-paths rather than a primary organizing axis.
   - **Paper drafts** (future work) — are outlines. When paper-writing-time arrives, it is a new outline file + framing prose, not a rewrite. Defuses "paper vs. framework" tension in the portfolio.

   Outline-cost ≈ one new file + framing prose. Segment-cost = rewriting load-bearing content that downstream segments depend on. The default should be outline-first unless the proposal genuinely changes what a segment claims or requires a new segment of its own.

5. **Keep segments evergreen; filter audience layers at the outline level.** A modest extension to the build script's existing row-level filtering (which already handles `--GAP--` rows specially) would let outlines filter *within* segments — by header name (include `## Formal Expression`, exclude `## Working Notes`), by status marker (show only `claims-verified` or above), or by content tags. This makes the segment substrate *genuinely evergreen*: author each segment with every layer that any view might need, and let outlines select which layers their audience gets.

   **Segment-authoring discipline under this affordance:**
   - **Use consistent header names** across segments so filters work reliably. The FORMAT.md cadence (Summary / Formal Expression / Epistemic Status / Discussion / Working Notes) is already near-consistent; tighten where drift exists.
   - **Keep audience-layers self-contained.** A Reader's Path sentence shouldn't require reading the Formal Expression to parse; a Narrative Framing paragraph shouldn't assume the Epistemic Status is visible.
   - **Prefer adding a new layer to squeezing content into an existing one.** If an ELI10 paragraph would clarify a segment, add a `## Narrative Framing` section, not a parenthetical in Discussion.
   - **Don't delete Working Notes to promote.** FORMAT.md currently says Working Notes are "removed at candidate stage" — under outline-filtering, Working Notes can stay in the segment and be filtered out of promoted views. Keeps development archaeology without polluting mature reading paths. **FORMAT.md should be updated accordingly** as a small editorial item.

   **Proposals that change under this affordance:**
   - **SP-5 (Reader's Path) — reopen, reclassify.** Currently deferred behind Bundle 1 at §F. Under segment-layer framing, SP-5 is one more filterable layer (a 1–2 sentence load-bearing preamble per segment, tagged for its audience). Per-segment cost is low; independence becomes **high** (per-segment additions don't conflict); downstream value is high (enables ELI10 / narrative / pedagogical outlines without separate documents). Worth reopening as §B.4 or §C.5 on the next audit.
   - **C-BP4 (claim-level statuses) — composes cleanly.** Claim-level tags are themselves a filter target: an outline could include only `exact` claims, or exclude `discussion-grade` content. The layer discipline (which sections) and the status discipline (which claim-strengths) interlock.
   - **Free presentations.** ELI10 outline. Control-theorist entry path. Causal-inference entry path. Paper-section outline. Historical archaeology outline showing TFT-origin lineage where present. Each is a new outline file + filter flags + framing prose, not a rewrite. None requires segment edits.

   **Build-script extension:** adding a filter flag (e.g., `filter: include-headers=[...]`, `filter: exclude-headers=[...]`, `status-min: claims-verified`) is a small technical enhancement that Joseph has flagged as trivial. Scope of implementation: modest; scope of enabled presentation-space: large.

**Next portfolio audit:** recommended after either (a) Bundle 1 lands, or (b) any three items from §B/§C complete — whichever comes first. At next audit, verify Bundle 1 and Bundle 2 membership hasn't drifted; reassess §D items against what's landed; check §E gates. Also: re-examine each active proposal through the outline-view lens before scoping execution as segment edits.
