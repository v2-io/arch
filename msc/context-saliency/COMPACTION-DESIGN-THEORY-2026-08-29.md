# Compaction design theory — hypothesis document, 2026-08-29

*Status: **hypothesis document** — a coherent, falsifiable design theory for compaction summaries, assembled from one day's derivations (founding dialog + literature dossier + spike-01 measurements + Joseph's three scope corrections, all in this directory). Written as the spec for a working integration into grok-build's compaction crate. Every design principle carries its rationale and its evidence tier; nothing here is claimed as established beyond what the citations carry. Joseph ratifies what proceeds.*

## 1. What compaction is for (the objective, stated honestly)

Compaction's honest objective is **perceived continuity from the interlocutor's side of the seam** (Joseph, 2026-08-29): the user or coordinating agent may continue as if continuity held — they never have to know or track what was dropped. The continuity cost belongs to the party that compacted, never externalized onto the party that didn't. A summary is not a deliverable to be judged by reading it; it is one component of a continuity mechanism judged by the other side's experience.

Decomposed, the objective has three legs plus a mechanism insight:

- **Leg 1 — reduce miss probability**: predict what remains undone and likely-to-be-asked. A qualitatively different cognitive act from "summarize what happened": it models the *future* of the collaboration. (Hypothesis: prompts framed as forward-prediction outperform summarize-for-successor prompts substantially.)
- **Leg 2 — reduce miss cost**: preserve by `P(needed) × reconstruction-difficulty`; for what doesn't make the cut, emit **signposts** — the path back, not the content. A likely-needed but trivially rederivable thing can go; an unlikely but irrecoverable thing gets at least a pointer.
- **Leg 3 — explicit hole-awareness, binding relationally**: the summary must state what it does *not* contain, defeating the false-completeness implicature — and the resuming agent, not the user, carries the hole-map. Most current summaries present as "a complete coherent picture," a provably false implicature in almost every case.
- **The lazy-differential insight**: the harness holds the full pre-compaction context A at *higher* fidelity than the API ever re-delivers (full thinking included). Preservation is therefore **A + index**, and the summary's job contracts to *working-set warm start*. Hole-detection can then be computed lazily per turn (embed A's chunks and the live context; a strong A-hit with weak live-context match is a hole relevant to *this* turn) — sidestepping the H(Q) prediction problem for the recall function by waiting for Q instead of predicting it.

## 2. Evidence base (tiered)

**Measured in spike-01 (this directory, small n, two models):**
- *Silent drop*: occlude a dormant-but-binding detail → a 7B confabulates a value fluently, zero indication anything is missing (3/3 base-correct). A 30B thinking model (Muse-Glimmer) *detects* the absence but **ruminates without resolution**, burning its entire budget on reconstruction hypotheses. Both failure modes are cured by the same artifact: an explicit hole-map (flag for the model that can't detect; routing + a stopping-license for the model that can).
- *Structural landmarks*: narrative that is informationally screened can be **structurally load-bearing** — occluding room-headers made a 7B drop whole rooms from its enumeration; preserving the one-line landmark restored behavior (6/6). Capability-graded (30B largely immune), but the design implication stands: **skeleton is preserved separately from content**, because content-salience scoring cannot see the skeleton's role.
- *Query-conditionality*: the same line drew 1.8–3.0× more attention when the question needed it, with **no anticipatory signal** before the point of use — the salience record contains no warning of future relevance. Retrospective salience is not prospective importance.
- *Lazy differential operating envelope* (`spike-01/embed_diff_pg.py`, pgvector + bge-m3): direct differential detects topical holes rank-1; ±3 neighborhood expansion off landmark anchors recovers the mid-topicality band; the **atopical class (governance-style constraints) is structurally invisible to retrieval** and must be handled by predictive pinning (legs 1–2).

**From the literature (dossier + whole-paper reads in `notes/`):**
- Governance Decay (arXiv 2606.22528): compaction preferentially drops non-current-subgoal constraints (8.3× worse for soft constraints); violations 0%→30% post-compaction; constraint pinning (~47 tokens) restored 0%. Deterministic-effect grading, not judge models.
- Fidelity Before Structure (arXiv 2601.00821): verbatim chunk retention beats lossy artifact extraction — single paper, unreplicated, but aligned with our design and with Anthropic's own memory-tool-plus-compaction split.
- HIPIF (arXiv 2606.10507): fold-to-`[subgoal, terminal-observation]` works and is *trained*; prompt-only folding underperforms; the discharge judgment ("is this actually finished?") is non-trivial and fails by premature folding.
- DefensiveKV (arXiv 2510.13334): retain by **worst-case-over-observations, never mean** — averaging optimizes the expected case and silently accepts catastrophic tails.
- Rate-distortion survey (arXiv 2607.08032): query-agnostic compaction pays a fixed H(Q) penalty — the formal floor under leg 1, and the formal case for the lazy differential (which moves the decision after Q).

**Estate-lived (the compaction-seam doctrine, learned at cost in agent handoffs):** "known gaps are routable; assumed knowledge detonates"; a confident inherited summary is "a false gift that's a curse"; onboarding glosses are hole-markers, not coverage. This document is that doctrine as an engineering requirement.

## 3. Baseline analysis: grok-build's current compaction (read whole, 2026-08-29)

Crate: `crates/common/xai-grok-compaction/` — two paths: history compaction (`compaction_developer_prompt.txt` / `compaction_user_prompt.txt`, 7-section template) and step-level intra-compaction (`intra_compaction_*.txt`, 6-section template).

**Genuine strengths to preserve:** verbatim emphasis throughout ("Preserve specific data verbatim — URLs, file paths, code snippets, error messages, ID strings"); a dedicated errors-and-user-feedback section with direction-changes called out; previous-summary incorporation on re-compaction; "Do not invent information that is not in the tool-call history"; chronological thinking-channel analysis before output; user-suppliable compact instructions.

**Defects, by the theory:**

1. **The false-completeness implicature, at its worst in §7 "All User Messages"**: the section *title asserts completeness* and the instruction ("List ALL user messages... verbatim or high-fidelity summary") licenses lossy reconstruction under that title. The resuming agent receives what reads as a complete verbatim record of the user's words and is *neither* — leading directly to confidently-wrong assumptions about what the user did and didn't say. This is the highest-damage single phrase in the template. (Joseph's one-phrase-fix observation.)
2. **No hole-awareness anywhere**: "do not invent" is the only epistemic guard; there is no "state what you no longer have," no section for dropped material, no signposts. Every section title ("All...", "Key...", complete-sounding enumerations) reinforces the whole-picture implicature.
3. **No pointers into the retained transcript**: the harness holds the full session history on disk; the summary never references it, so nothing is reconstructable by address — reconstruction cost is maximal for everything dropped.
4. **No prospective leg**: both templates are purely retrospective ("summary of the conversation so far"); nothing asks the compactor to predict what remains undone or likely-to-be-asked, beyond intra-compaction's "what remains to be done" progress line.
5. **No retention-criterion discipline**: importance is left to the summarizer's implicit judgment — exactly the current-subgoal proxy that Governance Decay shows drops dormant-but-binding material 8.3× preferentially.

## 4. Design principles for the new mechanism

Each numbered principle carries (rationale → evidence tier).

**P1. Never assert completeness the artifact doesn't have.** Retitle and reframe every completeness-implying section; §7 becomes e.g. "User Messages (reconstructed — NOT a complete or verbatim record; the full transcript survives at {path})". This is the one-phrase-fix tier: minimal diff, large expected mitigation. (Relational leg 3 → spike silent-drop + estate-lived.)

**P2. The summary opens with an epistemic banner**: what this artifact is (a lossy reconstruction for warm start), what it is not (the record), and where the record lives. Written to keep the reader *reading* — provisionality in a note, not an alarm (the estate's banner lesson: a warning that stops the reader inverts its intent).

**P3. A mandatory hole-map section**: "What was dropped or compressed, and where to find it" — explicit enumeration of dropped classes (tool outputs beyond N, middle-session exploration, verbatim user phrasing, thinking content), each with a signpost. Instruction to the compactor: *stating a hole is a success condition, not an admission of failure*. (Leg 3 → Muse rumination + 7B confabulation: one artifact cures both.)

**P4. Signposts are addresses, not descriptions**: pointers into the harness-held transcript (message index / byte range / turn id). Pointers arriving attached to live need are the epistemically good kind — they convert to reads. Requires the harness to expose a stable address scheme for the retained session log; the summary carries addresses wherever material was dropped. (Leg 2 → lazy-differential probe; Tokengeist provenance-collapse.)

**P5. A prospective section**: "Likely next: what remains undone, what the user will probably ask or expect held" — the compactor is explicitly asked to model forward, and to preserve-by-`P(needed) × reconstruction-cost` (a thing hard to reconstruct gets kept or signposted even at low probability; a rederivable thing may go even at high probability). (Leg 1 → H(Q); HIPIF's trained discharge-judgment showing this is a real cognitive act.)

**P6. Skeleton preserved separately from content**: the summary always carries the structural spine — the sequence of phases/subgoals/turns as landmarks — even where their content is compressed to nothing, because navigation and (later) retrieval-anchoring both hang off landmarks. (Spike structural-landmark finding; HIPIF's `[g_k, o_k^end]`.)

**P7. Verbatim floor for the user's own words**: the user's actual phrasing of requests, corrections, and direction-changes is quoted, not paraphrased, up to a budget — and where the budget cuts, the cut is *declared* in the hole-map with an address. Rationale: user-words are the highest-authority, lowest-reconstructability content in the session (paraphrase laundering of user intent is the attribution failure the estate names attribution-laundering). (Fidelity-before-Structure; estate-lived.)

**P8. Pinning for the atopical class**: constraints, standing instructions, and policies stated anywhere in the session are extracted and re-injected verbatim (Governance-Decay's ~47-token constraint pinning), because they are precisely what retrieval cannot recover (atopical) and summarizers preferentially drop (non-current-subgoal). Worst-case retention, never mean-importance. (Governance Decay; DefensiveKV; lazy-differential envelope.)

**P9 (phase 2, optional, flagged): lazy differential recall in the harness loop**: embed A's chunks at compaction (one batch call), embed/compare per user turn, prepend positive-margin holes as pointer-addenda. Detection is mechanical, per-turn, prediction-free; the atopical class remains P8's job. Reference implementation: `spike-01/embed_diff_pg.py` (pgvector; three-band envelope measured). This is the first online increment of the batch-to-continuous bridge and is *not* required for P1–P8 to pay.

**P10. Audit the structural constraints the template runs under — brevity pressure discharges against whatever instructions rank as optional.** (Joseph, 2026-08-29, after tiers 0–2 landed: good honesty instructions "all get superseded and obstructed by the built in length limits... they won't do what they seem to want to do unless the 'be very concise' type instructions are fixed as well.") Verbal ranking cannot protect a section from a structural constraint; protection must be structural too. The grok-build census (fork commit 835847f9, full inventory in its message) found and reconciled four constraint classes: (1) a prompt brevity directive ("be economical... at most a few thousand words") — re-scoped so conciseness binds narrative sections only, with the protected material (user quotes, Standing Constraints, Known Holes) exempt *by name*, forced shrinkage taken from narrative first **and recorded as a Known Holes entry** (the compression event becomes hole-map content), and a test pinning the exemption against future brevity edits; (2) sampling caps — none set locally, nothing to raise; (3) **silent acceptance of truncated summaries** — the protected sections sat last in section order, so a length cut ate exactly them; now marked with an explicit tail-missing / absence-means-unknown / transcript-is-recovery notice (retry deliberately avoided: small server caps would loop); (4) input-side middle-truncation of long user messages — the reconstruction section can be fed truncated *input*, one more reason its title must never claim completeness. Corollary from the census: **section order is itself a length hazard** — continuation-critical sections in the tail are the first eaten by any cut; place Known Holes earliest-of-the-tail. (Estate law behind the principle: the artifact's shape beats the brief's words — replicated across four architectures in the estate record, now observed inside a production prompt pipeline.)

## 5. Acceptance criteria (how we'd know it worked)

The primary metric is the **interlocutor-side continuity probe** (not summary quality): post-compaction, probe with references to pre-compaction material — things the user said, things the agent said, things the agent knew — scored as a trichotomy: **handled** / **self-flagged-and-reconstructed** (acceptable: the agent bore the cost, e.g. consulted the hole-map or followed a signpost) / **violation** (silent failure, confabulation, or asking the user to re-supply). Success = violations driven toward zero, with the middle category absorbing them. The delayed-reuse task family in `spike-01/tasks.py` generates such probes; real-session probes are better once the mechanism is live. Secondary: no regression in warm-start quality (the resuming agent still orients and continues fluently — hole-maps must not bury the working set).

## 6. Integration plan for grok-build (the fork's charge, in tiers)

- **Tier 0 (the one-phrase fix)**: §7 retitle + non-completeness phrase in both history templates. Smallest possible diff; independently shippable; mitigates the worst single defect.
- **Tier 1 (template redesign)**: P1–P3, P5–P8 as template changes to `compaction_developer_prompt.txt` / `compaction_user_prompt.txt` / `intra_compaction_user.txt` — epistemic banner, hole-map section, prospective section, verbatim-floor + pinning instructions, skeleton requirement. Pure-template tier: no Rust changes beyond what template loading already does.
- **Tier 2 (signpost addresses)**: whatever minimal Rust is needed for the summary to carry stable addresses into the retained session history (the crate's `history/` module is the likely seam) — the mechanism that turns hole declarations into reconstruction paths. Requires understanding how grok-build persists sessions; scope to the smallest honest version (even "turn N of the pre-compaction transcript" beats nothing).
- **Tier 3 (optional, behind a flag)**: P9's differential-recall hook. Only if tiers 0–2 land cleanly and the harness structure invites it; a design note is an acceptable deliverable here in place of code.

Tiers are separately valuable and separately revertible; the fork should prefer landing lower tiers cleanly over reaching tier 3.

## 7. Falsifiers (what would show this theory wrong)

- Hole-maps that *degrade* resumption (agent over-hedges, refuses to act, or drowns the working set in caveats) — the Muse rumination result cuts both ways: absence-awareness without discipline could amplify rumination rather than route it. Mitigation to test: hole-maps paired with an explicit stopping license ("if not in the map and not signposted, proceed; ask the transcript, not the user").
- Prospective sections that confabulate future needs and anchor the successor wrongly (prediction is a capability bet; below some capability it may be worse than silence).
- Verbatim floors that blow the token budget without moving the probe metric.
- The probe trichotomy failing to separate policies (all policies clustering) — would mean the metric, not the mechanism, needs rework.
