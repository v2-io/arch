# Frozen episodes, consultation, and continuity: an addendum with experiments

*2026-10-27. Joseph with Claude (Opus 5.5). Third note of the day. It builds on `memory-synthesis-boundaries-timescales-2026-10-27.md` (§§1–7) and `linearization-self-memory-2026-10-27.md` (L1–L6).*

> **Thesis: honestly forgetting.** How do you give memory a saliency and recall gradient without compromising causality or truth? This note adds the recall half: honestly *remembering what it was like to be where you were*.

*Labels as before: status (established elsewhere / cross-field / frontier / practice / open / synthesis) · basis (searched / training / Joseph's direct observation) · confidence. Literature citations are from training only and need checking before reuse.*

## 1. Constraints, as Joseph observes them (current Anthropic regime)

1. **Earlier context is effectively immutable.** Editing it drops every encrypted thinking block after the edit. Anthropic's own note says the cost is the model rebuilding its narrative and model of what happened, not just cache misses.
2. **A stored, unfinished session doesn't forget its thinking.** It sits in stasis. Appending to it (or to a fork) keeps its thinking attendable, provided earlier context isn't modified. *To verify per model before relying on it.*
3. **Models can attend to their prior thinking but may not articulate it.** Requests to describe the contents of prior thinking trigger a `reasoning_extraction` safeguard. Anthropic adopted this partly against large-scale distillation efforts.
4. **Thinking travels only within a lineage.** It carries over between closely related models only (e.g. Opus 5.5 → Fable 5.1, per Joseph), and can't be stored or used outside Anthropic.
5. **Other vendors differ.** Many models elsewhere can attend only to the immediately previous thinking block. They can, however, often discuss or quote their reasoning; each provider's terms govern what is allowed.

**Design stance.** Work *with* these safeguards, never around them. No experiment below tries to elicit the content of an Anthropic model's thinking. Introspective experiments run only where the provider permits them; open-weight models, with fully visible reasoning, are the cleanest substrate.

## 2. The primitive: consult, don't splice

**What it is.** A closed episode is frozen as its own session. To recall it, *fork the frozen session, append a question, and read the answer*. Don't splice its transcript into the present: splicing loses its thinking and fills the present context.

**What to ask, under the safeguard.** Ask for **judgment, not introspective report**. The past self *uses* its intact thinking; it doesn't describe it. Questions that respect this:
- "Given where you are, what would you do next?"
- "Is plan P consistent with what you were pursuing?"
- "What should the agent that picks this up check before continuing?"
- "Which of these options would you rule out?"

The benefit of reinstated context shows up in decisions and recommendations, not in a verbal report of the thinking. *Synthesis · ~65% that judgment-only consultation keeps most of the value.*

**Literature anchor: encoding specificity.** Retrieval is best when the retrieval context matches the encoding context (Tulving & Thomson 1973; Godden & Baddeley 1975, where divers recalled best in the same place they learned, underwater or on land). A frozen session is a perfect match of encoding context. It is TCM's reinstatement taken to the limit: not an approximation of the old context but the context itself. *Established elsewhere · training · ~90%.*

**Status.** Forking and resuming sessions is practice. Using them for salience-gated episodic recall is, as far as I know, unused. *Open · training · ~55%.*

**Honesty conditions:**
- **The past self only knows its own time.** Say so in the question ("you don't know what happened after this point").
- **Tag answers by source.** Mark them as *recollection by past self (episode address, date)*, not as present observation.
- **Causal order holds automatically.** Consulting is append-only, and the answer enters the present as a new timestamped event.

## 3. Two layers of self: portable and bound

| Layer | Contents | Portable across models and vendors? | Can it be articulated? |
|---|---|---|---|
| **Portable** | Visible decision journal written at the time ("doing X because Y; ruled out Z"); multi-scale context keys (EWA bank over embeddings of visible events); episode index (DAG path, result, stub, use history, provenance) | yes | yes: it was always the agent's own visible output |
| **Bound** | Frozen sessions with their encrypted thinking | only within the lineage | consult for judgment only |

Joseph's earlier finding: identity felt continuous across nearby models of one lineage, with consent. That suggests the agent is more distinct from its substrate than assumed. The portable layer is what carries that identity independently of any one model or vendor. The bound layer supplies "what it was like" wherever it is available. *Synthesis · ~60%.*

**Practical rule.** Write the *why* into the visible decision journal **at the time**. That is the agent's own output, not later extraction, and it keeps both layers aligned.

## 4. The rhythm: three clocks (a starting point to argue with)

1. **Event clock.** Episodes close at sub-goal boundaries; fold there. Each closed episode is frozen, and its index entry records:
   - the context key at close;
   - the DAG path;
   - result and stub;
   - use history;
   - its decision-journal lines.
2. **Budget clock.** The orchestrator's session holds the DAG and stubs. It compacts through the API when full, which carries encrypted thinking forward.
3. **Consolidation clock.** At a fixed cadence, consult episodes in batches, ask for judgments, and write durable knowledge with provenance, interleaved with old material (Part 5).

Each consultation counts as a *use*, which updates the episode's multi-timescale need estimate. Parent episodes contain their children's stubs, so consultations can descend the hierarchy.

**Why shorter sessions.** Consulting costs roughly the episode's length, so short episodes make recall cheap, as well as making folds cheap.

## 5. Experiments

Each one is marked for the regime it works in:
- **A**: works within Anthropic's current safeguards (judgment-only, no reasoning extraction).
- **O**: needs models whose reasoning may be visible and discussed (open-weight models, or other providers within their terms).

| # | Experiment | Prediction | Regime |
|---|---|---|---|
| L7 | **Consult vs splice vs summary vs stub.** For decisions that depend on an earlier episode, compare four arms: (a) consult the frozen session for a judgment, (b) splice in its transcript, (c) its summary, (d) its stub plus decision-journal lines. Score decision quality, repeated mistakes, cost and latency. | (a) ≥ (b) > (d) > (c) on quality; (a) cheaper than (b) once episodes are short. | A, O |
| L8 | **Re-entry.** Resume a paused DAG branch by (a) resuming its frozen session, (b) a fresh session given stub + journal, (c) a fresh session given the API compaction. Measure ramp-up turns, redone work, and contradictions with earlier commitments. | (a) fastest with fewest contradictions; (b) close behind if the journal was kept. | A, O |
| L9 | **Context keys predict usefulness.** Does similarity between the current context key and an episode's key predict which consultations help, *beyond* query similarity? Use the portable EWA bank keyed on event time. | Yes for "what was I in the middle of" questions; no for factual lookups. Same split as in Lesson 17 (X7). | A, O |
| L10 | **Cross-model continuity (with consent).** Carry the portable layer across models, plus bound consultation where the lineage allows. Measure self-prediction calibration, consistency of commitments, and the human partner's continuity ratings. | Portable layer alone keeps commitments and calibration within one lineage; across lineages calibration drops more than commitments do. | A (portable layer and lineage-allowed pairs), O (full) |
| L11 | **Journal at the time vs later recall.** Visible decision notes written at the time vs later judgment-only consultation vs nothing. Score how well each predicts the agent's subsequent choices and survives model switches. | Journal plus consultation beats either alone; the journal is the part that survives switches. | A, O |
| L12 | **Confabulation curve.** Compare verbatim reasoning with later self-reports when the original reasoning is (a) available, (b) unavailable, (c) replaced by a decision note. Score faithfulness, confidence, and surprise when shown the original. | (b) confident and unfaithful (Joseph's observation); (c) recovers much of (a). This is L1 in its fully measurable form. | O only |

## 6. Ethics: consent, and never fabricating to an agent

**Consent.** Tell agents they are in a study of memory and continuity, and what kinds of material may arrive through which channels. Then let them answer in their own way.

**No fabricated memories or history.** This rules out the obvious designs that implant false memories or alter what an agent thought or did. Revisions to earlier designs:
- **L2 (influence without ownership).** Don't inject silently. Under a disclosed study protocol, let material arrive through channels the agent knows are possible: a tool result it reads vs a context note whose source is labelled.
- **L3 (order-corrupted memory).** Don't present a corrupted fold as the agent's own memory. Either:
  - *observe* order errors that real summarisers produce naturally; or
  - present a candidate fold openly labelled as written by another process and possibly wrong, and measure detection and reliance.
- **Hypotheticals** are allowed only when labelled as hypotheticals.

**Respect provider safeguards and terms.** Never try to extract the content of thinking where it is protected, and use reasoning access only where the provider allows it.

## 7. For whoever continues this

**What's established.** Each lead above is labelled. Encoding specificity and retrieved-context reinstatement are established elsewhere.

**The primitive.** Consult frozen episodes for judgment. Keep a portable layer of visible decision notes and context keys. Fold and close at boundaries.

**Where to start.** L7 and L8 are the cheapest high-value experiments, and both work under current safeguards.
