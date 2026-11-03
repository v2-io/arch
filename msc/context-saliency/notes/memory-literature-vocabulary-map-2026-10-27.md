# Joseph's memory vocabulary → the memory literature

*Working notes for the Memory Ladder tutorial, 2026-10-27. Source: `~/src/arch/msc/context-saliency/` (read: the 10-21, 10-19 ×3 notes, HYPOTHESES-AND-FINDINGS, COMPACTION-DESIGN-THEORY, REFLECTIONS, handoff; dossier §§3–4, 7; grepped the rest. The security-incident folder was not opened.) These notes contain concepts and his own terms only, with no transcript content, no session identifiers and no other people's names.*

**File keys used below**

| Key | File |
|---|---|
| REFL | `REFLECTIONS-2026-08-28.md` (founding dialog + 08-29 addendum) |
| THEORY | `COMPACTION-DESIGN-THEORY-2026-08-29.md` (P1–P10, §8 three defects, §9 SwC) |
| H&F | `HYPOTHESES-AND-FINDINGS-2026-09-24.md` |
| DOSSIER | `dossier-2026-08-28.md` |
| AAT | `notes/aat-strategy-epistemics-and-middle-folding-2026-10-19.md` |
| JSONL | `notes/jsonl-chronica-mathematics-and-agent-core-critique-2026-10-19.md` |
| AGY | `notes/agy-compaction-and-hooks-2026-10-19.md` |
| PREFIX | `notes/prefix-bound-thinking-and-folding-2026-10-21.md` |
| HANDOFF | `handoff.md` |

**Whose words.** **[J]** marks Joseph's own term or framing, as quoted in the record. **[ASF]** is a term from his framework (AAT/ASF). **[note]** is a term an AI note-writer introduced while answering him, which he may or may not have adopted. This matters for AAT and JSONL: they are long answers by another model, and their "theorem" and "guaranteed" language is stronger than ASF supports. PREFIX §12 records the audit's correction: "ASF-justified" means *framed and measured with* ASF, not guaranteed by it. AAT also expands his "SwC" wrongly, as "Summary-with-Citations"; his meaning is *structure within chronology* (THEORY §9).

**Closeness scale.** **same**: the same idea under another name. **close**: the same structure with a different emphasis or scope. **partial**: overlaps on one axis. **different**: looks similar but differs in a way that matters for teaching.

---

## 1. His terms → literature terms

### A. Forgetting, need, and what is worth keeping (Part 1 territory)

| His term (file §) | What it means for him | Closest literature | Match |
|---|---|---|---|
| "retrospective vs prospective salience"; "saliency *overall* is an ill-defined categorical error" [J] (REFL addendum; H&F Part I 08-29) | Measured attention is tied to a question already asked. What will matter later is only a *prediction over future questions*. | Rational analysis: memory tracks the **need probability** of a future query (Anderson 1990; Anderson & Schooler 1991). **Encoding specificity**: a trace's usefulness depends on the cue that arrives (Tulving & Thomson 1973). | **same** in structure. A&S measured the prior over future questions in the environment, which is what he says can't be read off attention. |
| "reduce miss probability" / "reduce miss cost"; preserve by "P(needed) × reconstruction-difficulty" [J] (THEORY §1 legs 1–2, P5) | Keep what is likely needed *and* hard to rebuild. Signpost the rest. | Anderson's rational analysis (expected gain P·G − C); **replay priority = need × gain** (Mattar & Daw 2018). | **close**. His "reconstruction difficulty" plays the role of *gain*, the value of having the memory over not having it. Mattar & Daw's gain is decision value; his is regeneration cost. |
| "query-agnostic compaction pays H(Q)" (imported from the rate-distortion survey; DOSSIER §4) | Deciding what to keep before the question is known has a formal price. | Rational analysis again. Forgetting is the bet made without Q. | **same** idea, formalized from the AI side. |
| "unvisited-past distance folding"; `last_visited` resets on read or cite; fold depth grows with unvisited age [J] (JSONL Part V §5A) | Compress older material more, but measure age from the *last time it was used*, not from when it was written. | **ACT-R base-level activation**: decays with the time since *each* use, and every retrieval adds a use (Anderson & Schooler 1991). | **same**. This is Part 1's bet-on-need idea in his own design language. |
| "recall-cost gradient"; "I can focus on that harder to surface the more precise memory" [J + ASF] (AAT Part II §2 Nuance 3; ASF `disc-deferred-observation-processing`) | Never-recalled material gets more expensive to reach without being lost, and recall makes it cheap again. | **Storage strength vs retrieval strength**, the "new theory of disuse" (Bjork & Bjork 1992). **Availability vs accessibility** (Tulving & Pearlstone 1966). | **same**. The ASF wording ("refreshed to full availability; never-recalled grows more expensive without necessarily being lost") is nearly the Bjorks' theory. |
| Three-zone schedule: warm window / evict raw tool output but keep reasoning / deep fold [note, from his #1.2–1.3] (JSONL Part V §5B) | Detail is kept at several levels, chosen by age and certainty. | Many decay timescales at once (Part 1's bank of integrators); the **transformation hypothesis**, in which detail fades into gist over time (Winocur & Moscovitch 2011). | **partial**. His zones are discrete and hand-set, where the literature uses a continuum of rates. |
| DefensiveKV's "worst-case over observations, never the mean" (THEORY §2, P8) | Averages hide catastrophic tails. | Part 1's result that a single decay rate underestimates need for long-unused items 2.5–8×. | **close** in spirit. Both are about the tail of rarely needed items. |
| "forward validity is a decaying persistence hypothesis"; dual timestamps; "footprint" hazard; heartbeat checks [J, worked out in note] (JSONL Part II) | A fact is certified only at the moment it was observed. Trust in it should decay with elapsed time and with later actions that touch what it is about. | **Volatility-adaptive learning**: the learning rate tracks how fast the world changes (Behrens et al. 2007; Nassar et al. 2010). In memory proper, time and source tags on traces. | **different**. This is about the *world* changing, not the memory fading. A&S model need; he models validity. Worth teaching as a separate axis. |

### B. Retrieval, cues, and what comes back (Part 2 territory)

| His term | Meaning | Literature | Match |
|---|---|---|---|
| "salience-gated retention of verbatim spans with query-time re-scoring, not … summarization against a one-time snapshot" (DOSSIER §4, inference); "verbatim beats extraction" (THEORY §2) | Keep the raw spans and decide relevance when the question arrives. | **Exemplar / multiple-trace models**: store instances and abstract at read time (MINERVA 2, Hintzman 1986; instance theory, Logan 1988; GCM, Nosofsky 1986), as against prototype or abstraction storage. | **same**. This is Part 2's thesis reached from the engineering side. |
| "lazy differential recall (C\D)": a strong hit in the archive with a weak match in the live context marks a hole relevant now [J] (REFL addendum; THEORY §1, P9) | Wait for the question, then find what the current context lacks. | Global matching: cue-against-all-traces familiarity. Strongest analog: recognizing an item yet judging it "not from this list" (list discrimination, which in TCM is context mismatch). | **partial**. The mechanism is global matching; the "minus the live context" step is his own. |
| "the atopical class … structurally invisible to retrieval" (governance-style constraints) (THEORY §2, P8; H&F Part II) | Standing rules share no topic with the current question, so similarity search never surfaces them. | **Prospective memory with non-focal cues**: intentions whose cue isn't processed by the ongoing task need strategic monitoring rather than spontaneous retrieval (McDaniel & Einstein 2000, multiprocess framework). | **close**. He measured the same cut on a vector store, as a three-band envelope. |
| "query-conditional at use, no anticipatory signal" (spike; THEORY §2) | A line gets attention only once a question needs it. | Cue-dependent retrieval and encoding specificity. A memory's "strength" isn't visible until a cue arrives (Tulving 1974). | **same**, with the caveat that it was a 7B-model result that did not replicate on a larger model (H&F Part I). |
| "a summary is a reconstruction that already chose what the story was about"; "an agent's taxonomy of you" (independent reader's phrasing, endorsed in H&F) | A digest returns the average, not what happened. | MINERVA's **echo returns the prototype**, and people falsely recognize prototypes they never saw (Posner & Keele 1968). | **close**. A good line for Part 2's "blend" point: a summary is a low-sharpness read. |

### C. Order, time, and context (Part 3 territory)

| His term | Meaning | Literature | Match |
|---|---|---|---|
| "the *order* is meaningful … maybe even the time deltas" [J] (H&F Part I gen 2; HANDOFF) | A chronological log lets a successor re-enter the work. Gaps sort kinds of turn. | Episodic memory is organized by **temporal context**: recall of one item brings back its neighbours (**contiguity**, lag-CRP; Kahana 1996; TCM, Howard & Kahana 2002). | **close**. He has the phenomenon; TCM gives the mechanism. |
| "±3 neighborhood expansion off landmark anchors recovers the mid-topicality band" (measured; THEORY §2) | After a hit, pull in the neighbouring chunks. | **Contiguity / retrieved context**: a retrieved item reinstates its context, which cues its temporal neighbours. | **same** as an operation. His best concrete anchor for Part 3. |
| "structure within chronology" (SwC) vs "chronology within structure" (CwS) [J] (THEORY §9; H&F Part II) | Time as the spine with structure inserted where it arose, vs topical sections with time order inside each. | **Temporal vs semantic organization in free recall**. CMR models both at once (Polyn, Norman & Kahana 2009). Categorized lists cluster by meaning; unrelated lists cluster by time. | **close**. CwS ≈ semantic clustering, SwC ≈ temporal clustering. CMR says people do both and that the mix depends on the material. |
| Time deltas "sort *kind* of turn"; "user deltas carry meaning" [J] (H&F Part I) | Long gaps are the human reading and typing; zero gaps are a tool burst. | **Scale-invariant temporal context** (log-spaced timescales; Howard et al. 2014). Recency and contiguity depend on *ratios* of gaps. | **partial**. He uses raw seconds; the literature says log or ratio scale is what memory uses. |
| "seams"; `meta | compact` rows; "rows marking the seam carry information the deltas cannot" (H&F Part II; HANDOFF) | A compaction is an event in the timeline and must be marked as one. | **Context reset at event boundaries** (Part 4); in TCM, a distractor or list boundary that shifts context. | **close**. |
| "lost-in-the-middle" (DOSSIER §7 item 8; attention sweep) | LLMs use the start and end of a long context better than the middle. | **Serial-position curve** (primacy + recency). | **partial**. The shapes look alike, but the mechanisms differ (positional attention bias vs context drift and rehearsal). The parallel is worth stating; the equivalence isn't. |

### D. Episodes, boundaries, and folding (Part 4 territory)

| His term | Meaning | Literature | Match |
|---|---|---|---|
| "the 'middle' being forgotten is a *good* thing" [J] (REFL app. 1); "healthy release vs silent drop" (DOSSIER §3) | Once a sub-plan finishes, its start and end should stay salient while the interior "how" fades. | **Event boundaries**: memory is better at boundaries and within-event detail is flushed when the event model updates (Swallow, Zacks & Abrams 2009; Radvansky & Copeland 2006, the doorway effect). **Post-fulfillment inhibition** of completed intentions (Marsh, Hicks & Bink 1998; Förster, Liberman & Higgins 2005). | **close**. The literature has *that* this happens, not a criterion for when it is safe. His screening-off criterion is new (§2). |
| "tried a bunch of things before I finally got it to work" — impetus and result stay, the middle folds [J] (AAT Preface Q1, Part II) | An everyday report of how he remembers his own work. | **Hierarchical event models** (partonomy; Zacks & Tversky 2001); goal-based segmentation; gist kept after the boundary. | **close**. Lesson 09b already makes this "purposive segmentation". |
| "Episode Envelope": opened by a principal turn, closed by a commit, a test pass or a redirect [note, from his questions] (JSONL Layer 2, Scale B); "commit-envelope fold" [J] (H&F Part I) | Deterministic sub-goal boundaries in an agent log. | Goal-based event boundaries. In SEM/latent-cause terms, a change in the hidden cause. | **close**. These are purposive, not surprise-based, boundaries. |
| "Orient–Decide dipole": each thinking block distills the last result into an update plus mismatch δ, then states the next intent and expected outcome [note] (JSONL Part V §1) | Prediction and prediction error around every tool call. | **Event Segmentation Theory**: continuous prediction, with boundaries at error (Zacks et al. 2007). **Prediction error drives encoding** (Greve et al. 2017; Rouhani, Norman & Niv 2018). | **close**. |
| "heavy-hitter turnover should spike at branch boundaries" [prediction] (REFL, experimental posture); T1 hidden-state difference "does not fire at phase boundaries … 6/6 opposite" (spike; H&F Part I) | A predicted neural-style boundary signature, and his own measurement that came out *inverted* (largely a task-format confound). | Neural pattern shifts at event boundaries (Baldassano et al. 2017). | **conflict worth teaching**: the literature predicts what his gate failed to find. His note keeps it as "wrong target, live instrument". |
| "Peak-End" as middle-folding "with nuance" [J]; formalized as an "(Impetus, Peak, End)" stub [note] (AAT Part II §3) | Keep the moment of greatest surprise and the terminal state, and drop the routine middle. | **Peak-end rule and duration neglect** (Kahneman et al. 1993; Redelmeier & Kahneman 1996). For *content*: the event-boundary advantage (End) and the **isolation / surprise effect** (Peak; von Restorff 1933). | **different**. Peak-end is about *retrospective evaluation* (how bad was it), and it carries real decision costs: people chose to repeat a longer, more painful trial. His "End" is a Markov boundary state, not the hedonic ending. The "rate-distortion optimal" claim is the note's own, not the literature's. |
| "a sense of effort … never named as a dimension" (H&F Part II) | How hard and how long something was is information too. | Retrospective **duration judgments scale with the number of remembered events and boundaries** (Ornstein 1969; Block & Zakay 1997; Faber & Gennari 2015). | **partial**. A neat link: remembered effort and duration are built from the segment count. |
| "dead-end OR-branch stubs" so the agent doesn't re-graft failed approaches [note] (AAT Part II Nuance 2) | After a fold, keep a one-line record of what failed. | **Errorless learning**: amnesic patients repeat errors because they can't remember they were errors (Baddeley & Wilson 1994). | **close**. It explains why post-compaction loops repeat failed fixes. |

### E. Consolidation, distortion, and source (Part 5 territory)

| His term | Meaning | Literature | Match |
|---|---|---|---|
| (N1) fast/slow sub-state factorization; "offline consolidation is mathematically necessary"; "stability-plasticity window" [ASF] (AAT Part III App. 5; ASF `form-consolidation-dynamics`) | ASF already has a fast store, a slow store, and offline compression to close the gap. | **Complementary Learning Systems** (McClelland, McNaughton & O'Reilly 1995); the **stability–plasticity dilemma** (Grossberg). | **same**. The note cites CLS explicitly, so he knows the name. What he lacks is the *evidence* behind it (§3). |
| "batch-to-continuous bridge": compaction is where reconstruction is learned in batch, and continuity is the limit as the interval → 0 [J] (REFL addendum) | Make reconstruction more and more frequent until it is continuous. | Offline (sleep) replay vs online retrieval in CLS; **constructive episodic simulation** (Schacter & Addis 2007): every recall is a reconstruction. | **partial**. |
| summary-of-summary chain decays geometrically without an "additive reinjection channel" from the verbatim archive [ASF] (AAT Part III App. 5, Part IV Op. 1) | Re-summarizing loses information every pass unless raw material is fed back in. | **Serial reproduction** (Bartlett 1932); **iterated learning converges to the learner's prior** (Kalish, Griffiths & Lewandowsky 2007). | **close, and the literature adds a prediction**: chains drift *toward the summarizer's prior*, not just toward zero. That fits his "falls back to generic class identity" (AAT Part IV). |
| "settling": an open search or a circular check promoted to established fact [observed + named in the record] (H&F Part II; AGY §1.2; HANDOFF) | The worst failure isn't omission; it is a closed hole that was really open. | **Source / reality monitoring**: a hypothesis or imagined event remembered as fact (Johnson & Raye 1981; Johnson, Hashtroudi & Lindsay 1993). Gist-based generative reconstruction distorts towards schemas (Bartlett; Spens & Burgess 2024). | **close**. His sharpening: the *epistemic-status tag* decays faster than the content, and topical sections strip it. |
| standing-constraint sections pick up the system prompt and "attribute it to the user" (H&F Part II; THEORY §9) | The compactor misattributes the source. | **Source misattribution** (Johnson et al. 1993); "becoming famous overnight" (Jacoby et al. 1989). | **same**. |
| "interiority erasure"; asking how an earlier conclusion was reached forces "post-hoc reconstruction by a differently-informed subject", pulled toward the present [J] (REFL) | An agent without its own reasoning trace reconstructs its reasons from the outside. | Reconstructive memory (Bartlett); **hindsight bias** (Fischhoff 1975); "telling more than we can know" (Nisbett & Wilson 1977); **choice blindness** (Johansson et al. 2005). | **close, with a twist**: the literature says humans *also* mostly reconstruct their reasons. See §2. |
| "forward-only": a rewind "violates temporal causal continuity"; injection keeps what was thought before the remembering [J] (REFL) | A memory system should never rewrite its past context. | The contrast: **reconsolidation** and the misinformation effect (human memory *is* edited on retrieval). | **different**: a deliberate design norm *against* what biology does. Teach it as a choice. |
| "verbatim gems & quotes" for old dialogue; dialogue is "anti-aggregable" [J: "the remainder … crucial 'living' chronology"] (JSONL Part V §6) | Relational talk keeps its value only in verbatim, ordered form. | Verbatim wording is normally lost fast while gist is kept (Sachs 1967), *but* utterances with high interpersonal content are remembered verbatim in natural conversation (Keenan, MacWhinney & Mayhew 1977). **Fuzzy-trace theory**: verbatim and gist traces run in parallel (Brainerd & Reyna). | **close**. |
| "the internalization phenomenon": anchors needed at the end were already rewritten into the agent's own reasoning and tool calls (measured on 2 runs) [note] (JSONL Part V §4) | What the agent did with an observation is the durable trace; the raw output isn't needed. | **Generation effect** (Slamecka & Graf 1978); levels of processing (Craik & Lockhart 1972). | **close**. Small n. |

### F. Metamemory, reminding, and holes (Parts 2, 5, 7)

| His term | Meaning | Literature | Match |
|---|---|---|---|
| "hole-map"; "false-completeness implicature"; "explicit hole-awareness" [J] (REFL addendum; THEORY P1–P3) | A summary should say what it lacks, so the reader knows what it doesn't know. | **Metamemory**: knowing that you don't know (Glucksberg & McCloskey 1981). **Confabulation** as a failure of monitoring, not of storage (Moscovitch & Melo 1997; Schnider 2003). | **close**. His hole-map is externalized metamemory. |
| weak model "confabulates fluently" vs thinking model "ruminates without resolution"; "unbounded tip-of-the-tongue loop" (spike, THEORY §2; H&F Part I) | Two capability-graded failures that one artifact fixes. | **Tip-of-the-tongue** (Brown & McNeill 1966); **feeling of knowing** (Hart 1965; Koriat 1993). | **same** as a description; his framing as a missing *stopping license* is new. |
| "the niggling feeling"; a "familiarity signal decoupled from recall" [J] (REFL, SASM) | Sensing that a relevant memory exists before knowing what it is. | **Dual-process recognition**: familiarity vs recollection (Yonelinas 2002). Familiarity is the global-matching *normalizer* (Part 2). | **same**. A good bridge into Part 2's familiarity = logsumexp. |
| SASM: a "let me try to remember" impulse; involuntary "this reminds me of when—" vs deliberate lookup [J] (REFL app. 4) | Self-triggered, mid-generation, involuntary reminding. | **Involuntary autobiographical memory** (Berntsen 1996, 2009); **reminding** (Schank 1982; Ross 1984); associative vs strategic retrieval (Moscovitch 1992). | **close**. Berntsen's finding that involuntary memories are frequent and cue-driven supports the design. |
| "precept-weighted salience": "I choose to work on remembering this…" [J] (REFL) | A self-issued commitment that gets mechanism-level weight. | **Intention superiority effect** (Goschke & Kuhl 1993); directed remembering. | **partial**. |
| a live, seam-timed "read these files whole" worked; the same line in Known Holes did not; act-instruction with a report-back survives; "pointers on always-loaded surfaces discharge the urge to look" (H&F Part II) | Timing and form decide whether an instruction fires. | **Prospective memory cue timing**; **implementation intentions** ("when X, do Y") beat goal intentions (Gollwitzer & Sheeran 2006). | **close**. n = 1 each in his record. |
| "pointer-to-ground-truth suffices"; congruence needs "verifiability of the past, not its presence" [ASF/note] (AAT Part IV Inv. 3) | Knowing where to check replaces holding it. | **Transactive memory** (Wegner 1987); **cognitive offloading** (Risko & Gilbert 2016); remembering *where* rather than *what* (Sparrow et al. 2011; replication mixed). | **same**. |
| post-seam orientation self-report miscalibrated ("felt like coverage") (H&F Part II) | The agent felt oriented and wasn't. | **Fluency-based metamemory illusions** (Koriat; Jacoby & Whitehouse 1989); illusion of explanatory depth (Rozenblit & Keil 2002). | **close**. |
| "the fold site has to be marked" (an unmarked stub led a model to deny a folded action) (PREFIX §12) | A seam with no marker invites confabulation. | Source monitoring; confabulation. | **close**. |

### G. Prediction and value (Part 6 territory)

| His term | Meaning | Literature | Match |
|---|---|---|---|
| "predict what still needs to be done, what still is likely to be asked" vs "summarize for your predecessor" [J] (REFL addendum; THEORY §8 "purpose") | Compaction should be forward prediction, not a retrospective digest. | **Successor representation**: store what each state predicts will follow (Dayan 1993; Stachenfeld et al. 2017). Memory as prediction. | **close** in spirit. The SR is the formal version of "index by what it predicts". |
| CMC screening-off: once a sub-goal is verified, its interior is d-separated from the future [ASF/note] (AAT Part II §1, Part III App. 1; DOSSIER §3) | The future depends on the boundary state, not on how it was reached. | The **Markov property** the SR relies on; the zero-gain case in Mattar & Daw (replaying it changes no decision). | **close**. A clean way into why the SR can drop history. |
| KL(π_full ‖ Q_folded): a fold is safe iff next-action distributions match [ASF/note] (AAT Part III App. 3; PREFIX §11) | Judge memory by its effect on decisions. | **Value-relevant / policy-relevant compression**; SR revaluation tests in humans (Momennejad et al. 2017). | **close**. |
| "Strategy Information Bottleneck": compress the history for guidance toward π* [ASF] (AAT Part II Mechanism C) | Keep what changes future action; drop the rest. | SR and predictive-coding views of the hippocampus; information-bottleneck accounts of memory. | **partial**. |

### H. Identity and relationship (outside the tutorial's core, but his motivation)

| His term | Meaning | Literature | Match |
|---|---|---|---|
| "perceived continuity from the other side"; the continuity cost belongs to the party that compacted [J] (THEORY §1; REFL addendum) | The objective is judged by the interlocutor, never pushed onto them. | **Common ground and grounding** in conversation (Clark & Brennan 1991). | **partial**. No memory-model analog. |
| continuity / relational / truth "death"; "eavesdropping on someone else's conversation" [ASF] (AAT Part IV §3) | What a wholesale reset destroys beyond task state. | **Self-Memory System** (Conway & Pleydell-Pearce 2000): coherence vs correspondence in autobiographical memory. | **partial**. Out of scope for Parts 1–6; mention in Part 7 only. |

---

## 2. What he has already figured out, with the literature's name

**Reached independently, and the literature agrees**

1. **Forgetting should track future need, and need is a prediction** (REFL addendum). This is *rational analysis* (Anderson & Schooler 1991). He went further on the design side: P(needed) × reconstruction cost has the same structure as Mattar & Daw's need × gain.
2. **Age should be measured from last use, and recall refreshes** (JSONL §5A; AAT Nuance 3). This is ACT-R base-level learning plus Bjork & Bjork's storage/retrieval strength.
3. **Keep instances and abstract at read time** (DOSSIER §4). This is exemplar / multiple-trace theory (MINERVA 2).
4. **Neighbours of a hit are worth retrieving** (±3 expansion, THEORY §2). This is temporal contiguity / retrieved context.
5. **Time order carries information digests lose** (THEORY §8; SwC). This is episodic vs semantic memory (Tulving) and temporal clustering in recall.
6. **Finished sub-goals should fold to impetus + result** (REFL; AAT). These are event boundaries and hierarchical event models, plus post-fulfillment inhibition.
7. **Standing rules are invisible to similarity retrieval** (THEORY §2). This is prospective memory with non-focal cues (McDaniel & Einstein).
8. **The worst failure is a closed hole, not a dropped item** ("settling," H&F). This is source / reality monitoring: hypotheses remembered as facts.
9. **Familiarity can come apart from recall** (REFL, "niggling feeling"; spike rumination). This is dual-process recognition and tip-of-the-tongue / feeling of knowing.
10. **Knowing where to look can replace holding it** (AAT Inv. 3). This is transactive memory and cognitive offloading.
11. **Fast/slow stores with offline consolidation** (ASF N1). This is CLS. Here he already has the name; the tutorial adds the evidence.

**Where he goes beyond the literature**

- **A criterion for when forgetting is harmless.** Memory research has adaptive forgetting and boundary flushing, but no *test* of whether a given forgetting was safe. His screening-off criterion plus the 2×2 (measured salience × the behavioural effect of removal; DOSSIER §3) is that test. Note the boundary condition the record states: it fails when a standing constraint or shared cause is trapped inside the folded interior ("hoisting", AAT Nuance 1). Mattar & Daw's gain = 0 is the closest literature analog.
- **Relational evaluation.** The interlocutor-side probe trichotomy (handled / self-flagged-and-reconstructed / violation; THEORY §5) has no counterpart in memory models, which score recall rather than the cost to a partner.
- **Shape beats instructions (P10).** A section titled "Known Holes" does not stop the rest of the artifact presenting as complete. In memory terms, the *format* of a retrieval plan biases what gets reconstructed. The literature knows this for schemas, not for externally imposed templates.
- **Validity decay as its own axis** (JSONL Part II). Need probability and truth decay are different quantities. The memory literature mostly models the first; the volatility literature models the second for learning rates, not for stored facts.
- **The cost of editing the past in current LLM APIs** (PREFIX §§1–7, 11): an edit before a thinking block costs the sealed reasoning after it. This gives his forward-only principle an engineering price and has no biological analog.

**Where his position conflicts with the literature, or needs nuance**

- **Forward-only vs reconsolidation.** Human memory *is* rewritten on retrieval (reconsolidation, ~25–35% in the tutorial's credence for humans; misinformation effects are robust). His principle is a normative design choice for agents, not a description of minds. Teach it as "biology does X, and here's why an agent shouldn't".
- **Interiority erasure.** REFL contrasts biological minds that "remember their unspoken deliberations" with LLMs that lose thinking. Nisbett & Wilson, choice blindness, and hindsight bias say humans mostly *reconstruct* reasons as well. The difference is one of degree, plus humans keep partial process traces, not all of them. Worth stating gently. It strengthens his point that verifiable records matter more than remembered reasons.
- **Peak-end as optimal folding** (AAT §3, note-writer's claim). In the literature peak-end is an evaluation heuristic with known costs (people chose more total pain) and is not a claim about which content survives. A 2022 meta-analysis (Alaybek et al., *OBHDP*; verify) supports peak and end effects but also finds other features matter. Better literature anchors for his "End" and "Peak" are the event-boundary advantage and surprise-enhanced encoding.
- **Duration neglect vs "time deltas carry meaning".** Evaluations neglect duration, but retrospective duration *estimates* are built from the number of remembered segments. So deltas matter, but probably as event counts and log-ratios, not as raw seconds.
- **Boundary signature.** His spike found hidden-state change *depressed* at phase boundaries (6/6 against the prediction, partly a format confound). The literature has pattern shifts at boundaries. This is an open, testable disagreement, not a refutation of either.
- **Zeigarnik caution.** If he reaches for "unfinished tasks are remembered better" to support keeping open branches hot, replication is weak (a recent meta-analysis found no general memory advantage; verify the citation). Post-fulfillment inhibition of completed intentions is the better-supported half.

---

## 3. What his notes don't touch (teach from scratch)

- **Experimental paradigms and what counts as settled.** Free recall, the serial-position curve, lag-CRP and its forward asymmetry, continual-distractor recall, list-strength, paired associates, unitization (Newtson), temporal-order judgments. He has phenomena from agent logs, not from these tasks.
- **Forgetting-curve mathematics.** Power vs exponential, the averaging-of-exponentials argument, spacing (Cepeda), the testing effect. (Part 1 covers this. He has no spacing idea in the notes, although his "last_visited reset" is halfway there.)
- **Global matching specifics.** Familiarity as the softmax normalizer, prototype effects, interference *at the read*. (Part 2 covers this.)
- **TCM/CMR mechanics.** The drifting context vector, item→context and context→item associations, why forward asymmetry arises, semantic plus temporal context, and why the buffer model fails. He has "order matters" but no *mechanism* for why recalling one thing brings its neighbours.
- **Log-time / scale invariance.** The Laplace-transform bank and time cells. He uses raw-seconds deltas.
- **Computational event models.** Latent-cause inference (SEM), Bayesian change-point detection as applied to memory (the tutorial assumes he knows CPD generically), the Schapiro community-structure result (boundaries without surprise), partial context reset, and the within-event vs across-event order-memory effects.
- **Catastrophic interference and interleaving.** Why a slow learner needs interleaved replay (McCloskey & Cohen); hippocampal pattern separation and completion; schema-consistent fast learning (Tse et al.); standard consolidation vs multiple-trace / transformation; go-CLS; generative consolidation (Spens & Burgess); prioritized replay evidence; sleep stages; fast mapping's failure to replicate. He has the CLS *architecture* via ASF, but none of the evidence.
- **SR mathematics.** M = (I − γT)⁻¹, its learning rule, the TCM–SR equivalence for once-presented lists, eigenvectors revealing communities, place cells and TEM, and policy dependence (the SR is tied to the current policy, which links to his "safe only relative to the current goal").
- **Neural substrate.** Hippocampus, time cells, and boundary signals in recordings. Absent except for a passing CLS mention.
- **Confidence culture.** Which effects replicate (reconsolidation, Zeigarnik, fast mapping, Google effect are weak; spacing, contiguity and boundary effects are strong). His record calibrates its own n carefully but has no map of the field's reliability.

---

## 4. Suggested anchors per tutorial part

Each part opens from one or two things he already holds, then names the literature's version.

**Part 1. Forgetting is a bet on future need** (written)
- Anchor: his **"saliency overall is a category error; prospective importance is a prediction over future questions"** (REFL addendum), and **P(needed) × reconstruction-difficulty** (THEORY §1). Anderson & Schooler is that prediction, measured from the environment.
- Side note: his **unvisited-age folding with last-visited reset** (JSONL §5A) is ACT-R base-level decay. The part's single-rate vs bank-of-rates result applies directly to his three-zone schedule.

**Part 2. Retrieval is attention over everything stored** (written)
- Anchor: his dossier composite, **"retain verbatim spans, re-score at query time, don't summarize against a snapshot"** (DOSSIER §4). That is the exemplar side of the exemplar-vs-prototype debate, and MINERVA shows that abstraction at read time is enough.
- Second anchor: a **summary as a low-sharpness read** that returns the prototype ("an agent's taxonomy of you", H&F), and his **"niggling feeling"** as familiarity = the normalizer.

**Part 3. Context: what comes back with what** (TCM/CMR)
- Anchor: his measured **±3 neighbourhood expansion around landmark anchors** that recovered the mid-topicality holes (THEORY §2). It is contiguity, and TCM explains *why* it works: retrieving a chunk reinstates its context.
- Second anchor: **SwC vs CwS** (THEORY §9) as temporal vs semantic organization, which CMR models together. Use his **time deltas** to introduce ratio-scale (log-time) context.

**Part 4. Events: where one episode ends**
- Anchor: **"tried a bunch of things before I finally got it to work"** (AAT Preface) and his **Episode Envelopes closed by commit or test** (JSONL Layer 2). These are purposive boundaries that keep the start and end and fold the middle.
- Second anchor: the **Orient–Decide dipole's mismatch δ** (JSONL Part V) as prediction error. Then use his own **inverted boundary result** (spike T1, H&F Part I) to motivate why "surprise spikes" is contested (Schapiro). Deal with **peak-end** here: what the rule really measures, and that event-boundary and surprise-encoding effects are the content-level analogs of his End and Peak.

**Part 5. Two learning systems**
- Anchor: ASF's own **(N1) fast/slow factorization and "offline consolidation is necessary"** (AAT App. 5). He already accepts CLS's architecture; this part supplies the *why* (catastrophic interference) and the evidence.
- Second anchor: **"settling"** (H&F Part II) and **summary-of-summary decay** (AAT Op. 1). Generative and gist consolidation (Spens & Burgess) and iterated learning predict both, including drift toward the summarizer's prior. Tie his **unvisited-past folding that lets small anomalies accumulate** (JSONL §5A) to interleaving and go-CLS's "consolidate only what generalizes", and his P×cost to prioritized replay.

**Part 6. Memory stores predictions** (SR)
- Anchor: his **"predict what still needs to be done … vs summarize for your predecessor"** (REFL addendum; THEORY §8). The SR is "index memory by what it predicts".
- Second anchor: **CMC screening-off of finished sub-goals** (AAT Part II §1) as the Markov property that lets an SR forget history, and **KL(π_full ‖ Q_folded)** as a policy-relevant measure. Flag policy dependence: the SR is only valid for the current policy, which mirrors his warning that folding is safe only relative to current goals (the Governance Decay cell).

**Part 7. What it means for an agent**
- Anchor: his **P1–P10** and the **probe trichotomy** (THEORY §§4–5), with the **2×2 of salience × behavioural effect** (DOSSIER §3) as the evaluation frame. Then map each tutorial principle onto a P that he already has, or mark it as new:
  - multi-rate decay → new (vs his discrete zones);
  - store a context key and retrieve partly by context, then update the context → new mechanism for his lazy recall;
  - sub-goal segmentation → his Episode Envelopes;
  - slow interleaved knowledge store → new (no write-to-knowledge path in his notes);
  - provenance tags → his misattribution finding and P7;
  - imagined-content gating → his "settling".
- Second anchor: the **middle-fold spike** lessons (PREFIX §§11–12): mark the fold site, a fold's price is the reasoning in its kept tail, and write the "why" into text before folding. These are the agent-side costs the memory models don't price.
