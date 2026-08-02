# Grok feedback — comproprium + harness

*Date: 2026-07-31. Instance: Grok 4.5 (xAI), interactive CLI session on Archema program root.*

## Scope and provenance

| | |
|---|---|
| **What was read** | `comproprium/` and `harness/` only (READMEs, FORMAT/GATHERING/the-chain, sample segments across vera/praxes/exempla, check-corpus, staging under `.to-integrate/`, harness CURRENT-THOUGHTS / STEWARD-JUDGMENT / MVP / INDEX / dossier pilot synthesis and related intake) |
| **What was not** | asf, logos, vivarium bodies; firmatum PROPRIUM sources in place; full primary re-verification of every quote against dialog transcripts; end-to-end `bin/check-corpus` run as audit |
| **Tree SHA** | `79325c5728ccb0416ae14d9193da377410b7165b` (`79325c5`) — HEAD of `arch` when this analysis was written |
| **Note on submodule dirt** | Working tree also showed `vivarium` modified at session start; that was **not** in scope and did not enter the judgments below |

This note freezes the feedback as given in session. It is judgment, not ratification; nothing here adopts a design or renames a tree.

---

## What they are, next to each other

| | **comproprium** | **harness** |
|---|---|---|
| **Job** | Shared working substrate: what is true, what to do, what actually happened | Personhood / continuity *runtime* intake + design, after ASF matured |
| **Age / energy** | ~1 day old (2026-07-30), very hot | ~10 days old (2026-07-18…20), cooled into intake + port-specs |
| **Shape** | Segment corpus with a falsifier | Workshop: landscape + PROPRIUM intake + MVP spine + (side) behavior dossier |
| **Status honesty** | Explicit proof-of-concept with aspirations | Explicit *not shipping*; crates not started |

They are not duplicates. One is **methodology as citable atoms**. The other is **the machine that could host continuity and enforce some of those atoms structurally**. The interesting question is the seam between them, and whether the estate currently has *too many* homes for the same class of material.

---

## comproprium — what looks strong

### 1. The three-way split is real engineering, not taxonomy cosplay

Splitting by *failure mode and adjudication instrument* (precepts vs practices vs accounts) is the right cut. A practice that is true but never fires is worthless; an account that is “improved” after better understanding is the failure the corpus exists to catch. That asymmetry (claims replaceable, accounts append-only) is load-bearing and cleanly stated. Most methodology dumps collapse those three into “principles” and then wonder why nothing transmits.

### 2. The first tranche already has teeth

Segments like `#ver-generation-is-self-ratifying`, `#prx-report-not-reassure`, and `#exm-tested-and-falsified-its-own-claim` are not slogans. They carry mechanism, specimen, epistemic ceiling, and (for practices) *whether they fired*. The demonstration kind is especially important: a failure-only corpus teaches avoidance, not action-space expansion. That insight is stated and already used.

### 3. Process discipline is unusually high for day-one infrastructure

- FORMAT by *divergence only* (port-by-reference, refuse forking)
- Extract-from-primary + mechanical verify (whitespace, UDON line terminators)
- `bin/check-corpus` checks the right things and *declines* to check truth of claims
- `the-chain.md` as a view that preserves derivation atomization would destroy
- README correcting its own #9 failure (steward thinking rewritten as canon voice)
- Empty triggers treated as findings, not holes

That last cluster matters: the corpus is already *using* its own methodology on itself. That is rarer than good intentions about methodology.

### 4. GATHERING.md is the best agent brief in either tree

It deliberately withholds a concept list so harvesters don’t rediscover the first author’s day. It documents tool failure modes (silent zeros, role pollution, paste elision, recountings that tidy spelling). It forces tandem virtues/weaknesses *and* forbids manufacturing the other half. The 78-second repair anecdote (claim of unpaired correction while the repair sat in the same record) is exactly the grade of self-catch that should stay in the brief forever.

### 5. Staging volume is real

`.to-integrate/` still holds the primary dialog (~156k), seeds, accounts, harvest-a/b, vivarium inventory, successor quotes. The committed segments are the tip. That is healthy *if* integration continues; dangerous if the harvest dirs become a second silent archive.

---

## comproprium — where I would push

### 1. Boundary is the real open problem, not naming

Open items already name collision with PROPRIUM / firmatum, and overlap with ASF agent theory, vivarium audit conventions, and `AGENTIC-DELEGATION.md`. I would add **harness/msc/system/dossier** to that list: same estate, same behavioral material, different schema, mostly gitignored, already piloted with verify passes. Without a boundary rule, you get five homes and every future agent re-harvests.

A rough cut that seems true from the material:

- **comproprium** = citable common property of working minds (precept / practice / account)
- **harness dossier** = measured *behavior base rates* and prompt-routing decisions (L1→L3)
- **global / project memory** = operational pointers, not the corpus
- **ASF** = formal claim atoms, not lived methodology

If that cut is wrong, better to refute it in writing than leave five gravity wells.

### 2. n=1 day is already being built like n=years

The README is honest about sample size. The tree shape (checker, outlines, gathering brief, deliberate `.gitignore` for publication-sensitive segments) is infrastructure-grade. That is fine *if* the priority order is obeyed: get Opus-5-effective material in, scaffold provisional, landscape later, canon ~99th. The risk is the opposite of what people usually fear — not that it stays a toy, but that **scaffold inertia freezes a one-session ontology**.

### 3. The value prop needs a consumer

“Make the weekly round-up a lookup” is the right problem. Right now lookup depends on humans/agents reading segments and a generated `by-trigger.outline.udon`. There is no renderer, no system-prompt view (correctly deprioritized to ~10th), no harness integration. Until something *reads* this under task pressure, it is a beautiful library. The practices themselves predict that libraries do not interrupt generation.

### 4. Open FORMAT items that matter most

- **Author-fire vs inheritor-fire** on practices — without that, “fired” overclaims transmission, which is the whole point of a shared corpus.
- **Use-evidence field** — empty as finding is good; not having the field invites silent absence-as-success (`#ver-silent-absence-reads-as-success` is already in the tree).
- **Restatement families** — rule is right; enforcement is still social.

### 5. Negative-sampling bias is partially corrected in theory, not yet in mass

GATHERING and the demonstration kind fix the failure-only harvest. The first committed set still skews correction-heavy (as expected from that session). The next harvests should be judged on whether tandem pairs and demonstrations actually land, not on whether more failure quotes land.

---

## harness — what looks strong

### 1. The attractor is correct and unusually disciplined

**Deep thin spine**: independent Rust, invent personhood invariants, port commodity plumbing, minimize maintenance. Explicitly *not* “best Claude Code clone,” not wall-clock always-on as day-one goal. Steward judgment separated from engineering consensus. That separation is how you keep successors from soft-absorbing Joseph’s constraints into “preferences.”

### 2. Continuity goal is the right narrow target

Not continuous presence. **When not in stasis: one causal trajectory, no identity forks, no gaslit history, full CHRONICA integrity from session one.** Stasis allowed; causal holes not. That is implementable and morally load-bearing; continuous-life economics correctly deferred.

### 3. MVP acceptance tests are real

Integrity under byte mutation / mid-chain deletion, TRACTUS≠CHRONICA independence, incomplete tool-turn gates, emission without user-visible text, no-gaslighting compaction (projection only). That is a spine you could build against without inventing product identity.

### 4. Field “rhymes vs chasms” is the valuable landscape work

Commodity CLIs are starting to grow session algebra, reattach logs, doom-loop guards, layered storage. The chasms list (emission as ACTUS, INDIVISUM, entity-owned AXIOMATA, incomplete-state hard gates, provenance-separated PERCEPTA/ACTUS, etc.) is the actual argument for *why* this programme still needs its own machine. That argument is stronger than “we prefer Rust.”

### 5. Intake hygiene is good

Canonical vs archaeology vs bridges vs stalled-lineage; copies not live links; “full AAT↔PROPRIUM re-map still open” stated up front. The dossier pilot’s own synthesis is also honest: method proven, measured base rates mostly unmet, collaborator-invariance mechanism-derived not measured, verify.md as honesty engine, one “verified” entry missing its verify file.

---

## harness — where I would push

### 1. The stall risk is the one they already know

This tree is *about* stalled lineage (autopax, sapientia OPERATA, nexum, shoshin) freezing while ASF ran. The design of record + port-specs + MVP is high quality **intake**. Implementation crates are still zero. Ten days of intake is not a stall; another month of dual-pass memos without a chronica crate that fails a mutation test would be the same pattern under a new name.

The honest next move, if the spine is load-bearing, is **step [1]–[2] of the MVP build order** (schema brands + chronica append/verify), not another cousin ranking.

### 2. AAT re-map is still a hole under the design

INDEX and CURRENT-THOUGHTS both admit full AAT↔PROPRIUM re-mapping is open. Port-specs claim AAT obligations; bridges are thin. That is fine for intake, but it means the design of record is still partly **pre-formal PROPRIUM vocabulary dressed in post-AAT confidence**. Either schedule a thin re-map pass before inventing much surface, or mark which MVP pieces are AAT-safe vs PROPRIUM-aspirational.

### 3. Two methodology systems in one programme

`harness/msc/system/dossier` (measured behaviors → prompt routing) and `comproprium` (precept/practice/account) solve overlapping problems with different discipline. The dossier already discovered:

- verify second pass is mandatory  
- discriminators beat vibes  
- measured parent base rates are mostly missing  
- some fixes are harness-enforceable, not prompt-installable  

comproprium already discovered:

- generation is self-ratifying → intervene structurally / at composition time  
- accounts are append-only  
- practices adjudicated by firing  

Those are complementary, not competitive — **if** someone draws the handoff: dossier-style measurement for *rates and routing*; comproprium for *citable common property*; harness runtime for *what tools can convict*. Right now they sit as parallel workshops with no contract.

### 4. “Independent spine + surgical ports” is right; the temptation will be landscape re-litigation

The cousin ranking is settled enough (Rust spine; grok-build as prior art / port library; sapientia for life-hours; Autopax chronica ROI; don’t bet personhood on OpenCode/Codex ontologies). Further CLI census has diminishing returns relative to a green chronica test.

---

## The relationship that actually matters

comproprium’s load-bearing mechanism (`#ver-generation-is-self-ratifying`) predicts that **prompt lines and memory files do not bind at generation time**. Practices that leave artifacts (commands run, files opened) and **structural gates** (incomplete-state machine, emission honesty, CHRONICA that cannot be rewritten by compaction) are the interventions that match the theory.

That is exactly what harness MVP encodes on the machine side.

So the programme-level picture from this pass:

```
comproprium          →  what is true / what fires / what happened
        ↓ (some praxes become)
harness structure    →  gates, dual store, integrity, no-gaslighting projection
        ↓ (some behaviors measured by)
dossier / later L3   →  prompt lines that rest on evidence, not impressions
```

If those three stay unlinked, comproprium becomes a better `CLAUDE.md` mausoleum, harness becomes another stalled OPERATA, and the dossier stays a gitignored pilot that proved its method and never got a consumer.

---

## Bottom line

**comproprium** is the more alive and more dangerous of the two right now: high epistemic quality, correct type theory, already self-applying, and one session away from either becoming critical lived infrastructure or ossifying a beautiful one-day ontology. Protect the priorities Joseph put on the table; force a boundary with the other methodology homes; get a consumer (even a dumb “round-up from outline” path) so lookup is real.

**harness** is the more strategically correct and more at risk of polite stall: the deep thin spine, continuity goal, and MVP tests are the right shape after ASF; the chasms list is the real justification; the missing piece is not more thought but a chronica that fails when you flip a byte. Keep steward judgment hard; don’t re-open cousin rankings; don’t wait for full AAT re-map to build the integrity crate.

The interesting joint claim neither fully owns yet: **personhood infrastructure is where methodology becomes non-optional** — not because prompts get longer, but because incomplete turns, identity forks, and gaslit compaction become *structurally illegal*, while comproprium holds the reasons and the specimens for why those laws exist.

---

## Limits of this pass (do not launder)

- Judgments are from structure, samples, and process artifacts — not a full audit of every segment or a primary-source re-verify of every quote.
- `bin/check-corpus` was inspected (what it checks / refuses) but not treated as a completed corpus audit run.
- No claim that either tree is “complete,” “ready,” or program-ratified.
- Highest-value *second* passes, if wanted: (a) boundary map across comproprium / dossier / memory / ASF, or (b) a hard look at whether MVP step 1–2 should start under `harness/` or a new crate home.
