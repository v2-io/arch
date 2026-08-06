---
slug: atom-grain-parallelism
type: form
depends:
  - atom
  - write-safety
---

# Grain is a parallelism decision, not only a readability one

*How finely a corpus is cut decides how many minds can work it at once, how much of it a session must load to change one thing, and how precisely truth can be attached to a part — so the grain is chosen against those pressures, and choosing it for readability alone quietly caps the work.*

## The claim

Atomization is usually argued as a comprehension move: one topic per record, so a reader can find and follow it. That argument is real but partial. Under total agentic turnover ([[turnover-solution]]), grain is also the unit of three other things, and each of them pushes toward *finer* than readability alone would ask for.

**(A) Grain is the unit of concurrency.** Isolation between simultaneous workers comes from layout, not from locks: when one record is one placement, two agents editing different records never meet, and the write can be an atomic replace ([[write-safety]], [[partition-isolation]]). Coarse records put more claims behind one write, so more work serializes behind one file, and the contention that results is not a merge conflict a tool will surface — it is a silent loss or a stale overwrite. Halve the grain and the number of independently assignable work items roughly doubles; the practical ceiling on parallel work in a corpus is closer to its record count than to anything about the agents.

**(B) Grain is the unit of context.** A session must load what it is going to change, plus what that stands on. Coarse records force an agent to read — and hold — material irrelevant to its task, spending the scarcest thing it has. This is the same asymmetry that makes structure-now cheaper than structure-later: comprehension cost is paid per reader, and with full turnover the reader count is the session count.

**(C) Grain is the unit of truth-attachment.** Confidence, verification status, dependencies and provenance attach to a record. A record carrying several claims can only carry the *weakest* honest label for all of them, or it must carry qualifications inline where no instrument can read them. Fact-grain records with their own bands let a corpus hold a well-established claim beside a speculative one without either contaminating the other, and let a change to one re-price only what actually stood on it.

**(D) The pressures conflict, and the balance is a real decision.** Finer is not free. Each new atom adds an identity to name and maintain, a row in a view, and edges to declare; connective argument that spans several atoms has to live somewhere ([[appendix-placement]], and the bridge-kind question in ch. 9); and past some point a reader is following references rather than reading. So the claim is not *finer is better* — it is that the parallelism, context and truth-attachment consequences are part of the decision and are the ones most often left out of it, because they are invisible to the single reader whose experience usually drives the choice.

**(E) The corollary for delegation.** Where the grain is fine and one record is one placement, work can be divided by *claim* rather than by file region, and the division needs no coordination protocol beyond the assignment itself. Where it is coarse, delegation needs a membrane — a single writer, a spool, a lock — and the coordination cost is paid every cycle rather than once at design time.

## Strength & grounds

**Heuristic; (A) and (E) carry lived evidence from this corpus, (B) and (C) are argued rather than measured.**

The estate's multi-agent practice is built on (A) and (E) without having stated them. Two distinct patterns in ASF's multi-agent SOP show it (`~/src/arch/asf/doc/sop/multi-agent.sop.md`, read first-hand 2026-08-06): the *parallel sweep*, which runs several agents over shared canon and has the parent own the commits — with the stated reasons being attribution races between simultaneous commits and the review window for catching a defect in the brief, not anything about record grain; and *cluster reconciliation / consolidation-audit*, which partitions a portfolio too large to hold in one head into balanced clusters with one agent each. Different problems, but both take for granted that the material divides into independently assignable pieces. The practice assumes the grain and argues only the protocol.

This corpus is its own live specimen, in both directions. Its 2026-08-05 integration batch ran five parallel drafting agents over one segment directory; work divided by chapter and claim, and no two agents collided over a segment body. What they *did* collide over, twice, was the creation of new slugs — the one shared namespace the grain does not partition — which is why this instance's own law now names slug creation as the unprotected concurrency surface (`plan/ONTOLOGY.un`, `|parallel-batch-law`). That is a genuine boundary on (A): fine grain isolates edits to existing records, and does nothing for operations on the namespace itself.

(B) is an argument from a mechanism the estate has repeatedly observed but not measured here — no one has compared the context cost of one coarse record against the several fine ones it would become. (C) is likewise structural: the estate's format documents attach status per record, and the failure it predicts (a mixed-strength record carrying one label) has not been systematically looked for. Single-estate throughout; the agreement between instances is coherence, not corroboration.

## Working Notes

- The cheapest available test of (A): take one instance's history and count records ever edited concurrently by two agents against records that had to serialize. Nobody has run it, and git supplies the data.
- (E) meets the delegation discipline the estate already practises but keeps in agent-facing SOPs rather than in the corpus's own theory; where that lands is ch. 5 ([[work-coordination]]), not here.
- Unstated anywhere and worth stating when the ch. 1 gap on families is worked: the grain question and the duplicate question are the same question seen from two sides — "is this one topic or two" and "are these two records the same claim" both need the same test, and the estate only has a mechanical answer for the second (N2 in `plan/TODO.md`).
