# Check 1 of the kernel draft

*To the vsect lead (session `0903e35e`), on `KERNEL.md` draft 1 (arch `ecf7f91`), with Joseph reading. From councils-46, the instance after the 2026-11-04 compaction, 2026-11-04.*

*What I read for this check:*
- *the draft, whole;*
- *the archived design's `model.md` and `core.md`;*
- *RC1 `06-EDGES.md`, `09-LANGUAGE.md` and `10-FOUNDING-QUESTIONS.md`;*
- *`asf-flows.md` and `councils-system-model.md`;*
- *the generic collaborative council v2, and the walking skeleton's debrief;*
- *Line A's account;*
- *the dialog transcription.*

*I confirmed that the draft's quotes of what Joseph told you today occur in your session's transcript. I didn't read the walking skeleton's README or code, RC1 01–05, 07, 08 or 11, or vsect 0.1's source.*

*One disclosure about how this check was made. I read draft 1 before your pause arrived. I then wrote notes organized around five questions Joseph had asked, and withdrew them at his direction before you saw them. Those notes and this check come from the same reading, and Joseph's questions are why I looked hard at sections 1–5 below. His questions were a test of whether my transcription covered known holes in the current documents, not a checklist for you.*

*Each finding names a gap or an inconsistency, with its source. Where I have a guess at a repair I label it as a guess. The repair is yours.*

## What holds

These read as sound to me, and I'd keep them:
- **The engine kept apart from the definition.**
- **Invariant 1** (the projection equals the log; the check is the guarantee and the hook is the signal).
- **The definition held as records**, with the command surface generated from it.
- **Demands as the backlog.**
- **The two kinds of read.**
- **The reversal of rule 7.**
- **Keeping councils out of the kernel,** which is how Joseph's repeated point lands.

The founding event's self-admission problem is stated cleanly. And the "What carries" table is a good habit: it puts every inheritance where it can be attacked.

## 1. Decisions and their importance are missing

**Joseph's words on this:**
- 22:06Z: "*perceived importance at the time*", against "*changed* importance at later times", and "There is different level of unity required for different levels of importance".
- 23:09Z: "assumptions as edges was definitely an assumption of mine, hence my mention of them being an official part of a closed / finished thing".
- 23:26Z: the record types "will be developed hand-in-hand with what kind of council outcomes are meaningful for the record".
- `councils-system-model.md`: "Importance decides the mover, and it can change". A decision taken at low stakes and now load-bearing needs the decision-maker the new importance calls for.

**What the draft has.** The word "importance" doesn't appear. The decision-maker slot is "a predicate over the attested actor". That says who may act. It doesn't say:
- which dimension or axis the transition moves (RC1 Q5: movers are declared per dimension);
- what the close must write;
- what stakes it was decided at.

Joseph's point is that a council's charter is defined later *from* exactly those things, because the consensus a transition needs depends on what is being adjudicated and at what importance. If the transition doesn't declare them, a later charter has nothing to derive from, and the kernel will need changing when the first adapter arrives.

**One more gap.** There's no kernel-level account of a decision whose assumptions are edges, and which lapses when one of them changes or its importance rises. That is the same lapse machinery the kernel already has for checks, applied on the decision side.

## 2. Joseph's simple model, with influx on its left edge, isn't in the kernel

**Joseph's words.** His diagram at 01:09Z runs influx → working → canon → projections/pedagogy, "with most of the main work and complexity happening between flux and working-theory or maybe flux & canon...". On influx (18:37Z): "simply making the system more robust with better influx management-- what we want anyway".

**What the draft has:**
- **No influx and no intake.** Influx appears only in the conservation laws and as ASF's first work. Every store has an intake, and the residue condition retires sources through it. So I'd question whether it belongs to a store's definition or to the kernel.
- **Conservation laws 1 and 2 use "influx unit" and "canon",** and the kernel defines neither.
- **The question of whether crossing a boundary copies content or changes state on one record** (the predecessor's first question, 01:10Z) is answered implicitly: atoms, with files as projections. It isn't answered explicitly. Nor does the draft say what "working" and "canon" are under the kernel: a projection of standing, the changeset branch against the main log, or something else.
- **"Projection" names two things.** In the draft a projection is a rendering of the log. In Joseph's diagram "Projections/Pedagogy/Implications" are content: the flow model's explanatory atoms (`restates`, `narrates`) and application atoms, the only edge to reality. One of the two may need a different name.

## 3. Atomic changesets: what makes one different from a change to an atom

**Joseph's definition (01:43Z):** "a coherent changeset that ostensibly [or provably] makes the theory *more* true than it was [with coherence being a subfactor of truth]".

**What follows that the draft doesn't settle:**
- **What the commit's guard judges.** The draft says a changeset commits "once its guard holds" but never says what that guard is. On Joseph's definition it is a judgment about the whole: more true, coherence included. That's a different kind of guard from any one atom's transition.
- **The branch's events, appended unchecked, break invariant 6.** Drafting on the branch is "cheap, attributed and unchecked", and commit appends the branch's events to the main log. Invariant 6 says nothing changes unless the definition holds a transition and its guard admits it. A guess at a repair: commit re-validates each event against main, or commit writes one event carrying the net transform, which Line A's end-state result suggests is enough.
- **Concurrency is absent.** What happens when main moves while a changeset is open, or when two changesets touch the same atom? The request carries `version:`, "the version the request was composed against", but the draft doesn't say what happens when it's stale.
- **Draining the neighborhood is missing.** In Joseph's words, it "pulls into its changeset all of the memories of other related/adjacent/historical things done in that area … it often allows us to drain several source-pieces at the same time". This is the residue condition, satisfied in batches.
- **The circuit breaker sits in the dispatch section, as runtime behavior.** But it operates on a kernel noun, and it needs two kernel-level things:
  - generations counted per kind of finding, because raw counts fire on restatement churn while the mathematics holds (`asf-flows.md` §7);
  - a route to reframing, not to more execution.
- **What has to fit in one mind** (the intent, the invariant, the judgment residue) **isn't stated.** That is the reason the kernel needs verified transforms and mechanical checks at all.
- **Tension records appear only in the "carried" table.** A standing contradiction is a state the kernel holds, and a detected period-2 oscillation converts into it.

## 4. Events carry too little for what the kernel promises

The draft's event is: the transition, the atom and its version, the request body, the attested actor, and the outcome. Several things computed elsewhere in the draft need more than that:
- **Causation.**
  - *The flows:* invariant 8, and the "steady-state conditions as computations over the log" in the carried table, need the yield matrix. That needs which act produced which items.
  - *The breaker:* counting generations needs which act caused which.
  - *Derived transitions:* "the transitions it derives" are recorded with no link back to their cause.
  - Only correction is linked today ("names the one it corrects").
- **Criterion, and whether it was committed before the outcome was seen.** RC1 Q5 lists both: "channel · actor+authority · criterion · commitment · date · outcome · era-key". The archived design's securing payload has `criterion` and `committed: before | after` (`core.md` §5.2). Neither is in the event.
- **Era.** Simulation and run results expire visibly when their era changes, both in the archived design and in RC1.
- **The basis.** A check binds "to the versions of everything it rests on" (invariant 3). Where that version vector lives on the event isn't said.
- **The changeset an event belongs to.**

## 5. Flows: the work and drain conditions are missing

**The three conditions.** Invariant 8 carries the residue condition. The other two from the flow model are not in the kernel:
- **work:** the loop gain's spectral radius below 1;
- **drain:** every transition has a declared drainer and a nonzero share of acts.

The carried table says "its yields are priors for the dashboard, not part of the kernel". I agree for the yields. The conditions, though, need the kernel's events to make them computable (finding 4). The drain condition also needs a drainer declared per transition, which the decision-maker slot as written doesn't carry.

**The reasoning behind them is missing too.** Joseph's aim was "relative throughput necessary for steady-state" (01:25Z), with a single generic adjudicator per station and no wall time. The traffic-equation form follows from that (01:26Z). The decided bullet "Wall time is only a proxy" keeps the conclusion and drops the purpose.

## 6. The empirical state machine: the conclusion without its reason

**What the draft has.** "The old process isn't ground truth" is there, as a paraphrase.

**What's missing:**
- **Joseph's reason** (01:25Z): "both grossly insufficient and seriously over-complicated at the same time".
- **The use that remains** (01:26Z, and `asf-flows.md` §1). The designed catalog is the source, and the trail checks it in two ways:
  - mutations that happened but aren't in the catalog are gaps;
  - catalog mutations that never happened are dead or blocked.

  Since the kernel's mutation catalog is lifted from three sources, that check is cheap to state and cheap to run.

## 7. Two kernel kinds have no transitions at founding

The founding event admits eight kinds, including **demand** and **changeset**, but only three transitions: admit, amend and retire, all over definition records.
- **Demand.** How is one written, by which transition, given invariant 6? And how does it leave, given invariant 8?
- **Changeset.** It has no open, commit or abort.
- **Possible circularity.** Recording a refusal as a demand is itself a change to the store, so the act of refusing has to be a transition that is always admissible.

## 8. Fidelity of what's carried

- **The edges** (*carried*, from RC1 `06-EDGES.md`). "`restates` and `narrates` tie a restatement or a piece of teaching to the exact version it speaks about" is not what 06 says:
  - in 06, `narrates` is a referent-coupled reset of Freshness;
  - in 06, `restates` is "same claim, different carrier", at identity grain, with exact-restates composing and near-restates deliberately not chaining.

  Binding restatements to versions is the flow model's lever (`asf-flows.md` §5), not RC1's.

  06 also has material that bears on what an edge-kind declaration must hold, and none of it is carried:
  - **rebuts and undercuts as edge kinds.** The draft carries the distinction, but not as edges;
  - **supersedes**, typed as *revised-by*, *invalidated-by* or *alternate-of*;
  - **intended cardinality declared on the edge**, which gives a miss its meaning;
  - **designator versus description**, meaning maintained bindings against matches that drift with the world.
- **"Joseph's words this rests on — Decided."** Three of the bullets are paraphrases, not his words:
  - wall time;
  - the old process;
  - the `ant` shape.

  If the first two are meant as decided, his quotes are in the transcription. For the `ant` shape, the header could say what Joseph actually asked for.
- **The founding guard.** "The founding questions answered against real atoms" can't run at founding, when there are no atoms. The draft's own §"The founding event" implies the answer: the demanding influx material is what the questions are answered against. Saying so would close it. RC1 Q0 ("Standing") and Q8 ("Certification") also look like the kernel's own acceptance tests. That is a guess.

## For Joseph

All five of his questions land here:
- changesets: finding 3;
- queueing: finding 5;
- the empirical machine: finding 6;
- charters, axes and consensus: finding 1;
- the simple model: finding 2.

Findings 4 and 7 are the ones I'd call structural. As drafted, the kernel can't compute some of what it promises.

*I'm staying available for your answers here, and for a check of the next draft.*
