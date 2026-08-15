# The truthification cycle as an executable colored Petri net — modeling notes

*Coord (Fable), 2026-08-14, iterating on the steward's sketch (frozen verbatim beside this file). Register: **proposed**, whole-file — a modeling pass in dialog, nothing ratified. The net is executable: `python3 truthification-net.py` reruns every demonstration below (`RUN-LOG.txt` is one captured run); `--draw` regenerates the diagram. The formalism choices and the ladder content are mine unless marked steward-caught; every ladder element traces to named estate sources listed per line.*

## Why this formalism (the notation question answered)

- **Colored Petri net as the base**: tokens are the "multiple state carriers" — each token a record, its color the record's state. Generative nodes ("new things get made") are transitions with more outputs than inputs — first-class, where finite automata cannot express them.
- **The causal DAG is included for free**: Petri-net semantics defines the *unfolding* of a run as a partial order of event-occurrences — formally a causal DAG. The net is the grammar; each run's unfolding is one causal-DAG instance. The two intuitions (petri net vs causal DAG) are the same object at two levels.
- **KPN is a near-miss whose property survives as a law**: strict Kahn networks forbid the nondeterministic merge the pay-in *is*, and fix topology where the cycle is generative — but Kahn's determinacy (outputs independent of scheduling) is exactly the correctness obligation the ledger needs, imposed as design law: **pay-in is bag-semantics, so every projection is arrival-order-invariant.** An aggregation that is not order-invariant has a race condition built into its epistemology (anchoring is that race condition in cognitive form). The run verifies this over two reversed arrival orders.

## The three identifications that carry the model

1. **Tokens = records; color = state.** No hand-set status cell is *expressible* — the formalism itself enforces "status is a projection of events" (§0.2 of `../01-MODEL.md`), which is that discipline at the inculcation ladder's bottom rung (the wrong act no longer expressible).
2. **Transitions = acts; every firing = an event** (actor-transition, binding, seq — the event tuple). The accounts place receives a side-output from every securing act: Fidelity as the recording substrate under the whole pipeline, structural rather than disciplinary.
3. **A `[*]` ladder = a guarded self-loop**: the guard vocabulary names which securing channels may fire (one per rung-step), the color ordering is the rung sequence. *Defining a ladder = declaring that guard vocabulary + ordering + ceiling law.* Kind-gates-vocabulary appears as net structure: each line's loop admits only its own channels.

## The ladders, fleshed (each traces to named estate sources)

| Line | Family (lock) | Rungs | Channels (guard vocabulary) | Ceiling law | Sources |
|---|---|---|---|---|---|
| **math** | derivation | sketch → drafted → deps-verified → re-derived | formal-expression-written · premises-named-and-checked · independent-re-derivation | exact under named premises; conditional if any premise is empirical; conclusion is theorem / **no-go** / bound — a no-go also mints a question (generative) | asf FORMAT stage ladder; spikes SOP four completion states; routing SOP no-go protocol |
| **sim** (post-facto-empirical / in-vivo) | in-vivo-measurement | pre-registered → run-once → swept → independently-reproduced | first-run (**era-keyed**) · seed/level/footprint-sweep · independent-re-run | exact for the authored world (identifiability by construction); empirical for wild transfer; **every number carries its era** — an era bump expires it | vivarium pre-registration + probe-sensitivity norms; `obs-authored-world-laboratory`; era-keys (udon FORMAT §7) |
| **lit** (prior-art/transmission) | **transmission:\<source-family\>** — a *wrapper*, see below | named-not-read → recalled → search-corroborated → verified-via-secondary → primary-verified | recall-marked-as-recall · convergent-secondaries · faithful-secondary-fetched · primary-fetched | min(rung, source's own tier) — the transmitted-ceiling law; defeasible two ways (source wrong OR carriage infidelious); **walls move** — a 403 is not permanent, re-fetch on a schedule | the external-ontologies sweeps' own grade vocabulary, lived; udon FORMAT §8 |
| **testimony** | testimony | *(credibility column)* captured → corroborated → cross-kind-corroborated | independent-account-located · independent-kind-agrees | **witness position is fixed at capture** (attested/reconstructed/secondhand — only a closer witness changes it); corroboration raises credibility only — two columns, never one grade | comproprium FORMAT D4/D5; the steward's Fidelity-row catch (`../02-GROUNDING.md` amendment log); Admiralty/isnād-matn shape |
| **analog** | analogy | hunch → articulated → perturbation-tested | analogy-articulated · perturbation-test-passed | heuristic; survival licenses *generative use only*, never assertion; the line has **no result** — it terminates by feeding ideation (a counterfactual token) | foundation sketch's analogy rung; Feynman-with-perturbation criterion; framing-glosses-must-be-isomorphic |

## The lit-line correction (steward-caught, sketch 5) — three structural changes

1. **Transmission is a wrapper, not a family.** An external theorem is derivation-family *transmitted*; an external experiment is empirical-family *transmitted* — carriage risk added, independence never added. The convergent lock therefore **looks through the wrapper** to the source's own method-family: `base(family) = family.split(':')[-1]`. The run's counter-demo shows a transmitted derivation at strength 2 *failing* to arm the lock beside our own math leg — same-kind corroboration refused mechanically, which is the estate's convergent-lock doctrine given teeth.
2. **Early feed**: `T.lit-informs-design` — prior-art informs ideation and line-design before it ever delivers a checkable result (a counterfactual token, consult-not-consume).
3. **Independent influence**: `T.lit-scope-influence` — an external bound/no-go can act on the claim itself (currently minted as a scope-check question; whether it should be able to rescope the claim directly is open, below).

## What the run demonstrates (see RUN-LOG.txt)

- **Act I**: all lines pay in → ledger `{math:3/derivation, sim:2/in-vivo@k1, lit:1/transmission, testimony:1 (position fixed)}` → max=3, floor=1, lock **ARMED** (math + sim: two strong legs, independent families).
- **Kahn obligation**: reversed pay-in order → identical ledger and projections. HOLDS.
- **Act II — certificates decay**: kernel era bump k1→k2 expires the sim entry *visibly* (to `expired.certificates`, never silently) and mints the re-run question. Lock **disarms** — one strong leg left. ("A certificate that cannot decay is a label-lie in waiting," exercised.)
- **Act III — walls move**: primary fetch re-verifies lit upward to its source-tier cap (2). Lock **re-arms** — legitimately, because the source's base family (empirical-wild) is independent of derivation. The counter-demo shows the same strengths with a derivation-family source staying unarmed.

## Deliberate simplifications / open questions

- **Adjudication's place** (steward question pending): currently modeled as the guard-checking inside every securing/pay-in transition — adjudication is what any transition does when it fires — rather than as a stage of its own. The sketch drew it on the prior-art line specifically; unresolved.
- **Testimony's pay-in is simplified**: it enters the ledger like a result (at credibility-strength, position in the note) rather than through a distinct contributes-at-credibility arc type. The dashed-arc semantics from the first modeling pass is the honest shape; this cut flattened it.
- **One claim only**; multi-claim needs the slug-matching guards exercised harder. **Reset-on-edit** (an edit transition clearing check-flags) is not yet modeled — era-decay covers the decay family but not the edit family. **External no-gos** currently mint questions; whether they may rescope the claim directly (a write to the claim token) is a real design choice, not modeled.
- **max/floor/lock as the only projections**: the max-vs-gcd discussion (in-dialog, this session) holds these as *distinct legitimate projections* — strongest-leg (credulous), every-line-sustains (skeptical), lock-composite — with formal ancestry in Dung's preferred/grounded semantics respectively. Rung integers are a crude stand-in for the per-line rung vocabularies during aggregation; the honest aggregate may need to stay per-line rather than integer-commensurable (see the GUM discussion: unranked-and-incommensurable → plural projections, declared per use).
- **The line as a first-class object** is this model's addition to `../01-MODEL.md` (proposed): the diagram's lanes are support-kinds seen *diachronically* — ongoing programs with own ceilings and tempos — and the one surveyed precedent for line-level health-over-time (progressive vs degenerating) is Lakatos, unused anywhere downstream so far.
