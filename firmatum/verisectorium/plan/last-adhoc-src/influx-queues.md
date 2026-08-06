---
slug: influx-queues
type: form
depends:
  - queue-typing-specimen
---

# Intake needs typed residue: "you erred" and "I am unsure" are different acts

*An ingestion queue that can only say yes or no forces the system to blame the submitter for its own uncertainty — so the non-happy outcome has to split into at least two, and the split has to be visible in the artifact, not just in someone's judgment.*

## The claim

**(A) Three outcomes minimum.** Material arriving at a corpus boundary resolves as **promoted** (it is now part of the corpus), **rejected** (the submission was malformed: schema-invalid, colliding with an existing identity, disallowed), or **needs-review** (the submission is fine and the *system* cannot decide — the case it does not handle yet, the judgment call it is not entitled to make). The last two are different speech acts with different repairs: rejected means *fix your file and re-drop it*; needs-review means *a human has to adjudicate*. Collapsing them tells the submitter they made a mistake when the limitation was the system's.

**(B) The collapse is the common failure, and it is a small dishonesty.** In practice queues collapse toward "rejected" because that is the outcome the code already has. The corpus-facing version of the same collapse is a linter that reports "lint failed" for both *you wrote invalid frontmatter* and *nothing here can tell whether this dependency edge is genuine* — one is mechanical, one is an unadjudicated question, and only one of them the author can act on.

**(C) The membrane, not the manners.** The split holds only if nothing writes into the canonical store except through the intake path: drop, validate, promote. Where writers can reach past the queue, the typed outcomes become advisory and the queue becomes a suggestion.

**(D) The residue must be *stored*, not reported.** An outcome that exists only in a command's exit code is not a queue state. The shipped form is an artifact per outcome — the item, plus a sidecar saying why it landed where it did — so that the backlog is countable and the reason survives the session that produced it.

## Strength & grounds

**Heuristic, with one shipped implementation examined first-hand** — the typed queue and an untyped one placed side by side in [[queue-typing-specimen]]. Relata implements exactly this membrane; verified 2026-08-05 in `~/src/arch/firmatum/relata/docs/sys/spool.md`, which names the three outcome suffixes (`.rejected`, `.needs-review`, `.skipped-nonbib`), each with a markdown sidecar, and states the reason for the split in its own voice: conflating them *"mislabels system-uncertainty as user-error, a small dishonesty the North Star forbids."* Its worked case is instructive — an input kind whose pipeline is not built yet routes to `needs-review`, never to `rejected`, because the absence of a feature is not the submitter's error. That is one instance in one estate, on bibliographic records rather than claims; the transfer to claim corpora is a proposal here, not a demonstration. What would raise it: a claim-corpus intake actually running the three-way split and reporting whether the needs-review pile turns out to be non-empty (if it never fills, the distinction was not doing work).

## Working Notes

- This project's own INFLUX is an untyped instance of the thing this segment describes: material is dispatched to `.integrated/` or `.archive/` with the reasons living in a separate register rather than beside each item. Worth watching whether that costs anything.
- The measured backlog case, if a specimen is wanted: comproprium held **58** segments in its store against **51** still sitting in an unintegrated harvest area (counted first-hand 2026-08-05 in `~/src/arch/proprium/comproprium/`) — near half the corpus outside the store, a live instance of intake outrunning metabolism at scale, and untyped: nothing in that pile says whether an item is awaiting promotion, awaiting a human, or already declined.
- Adjacent and not claimed here: what makes an outcome a *gate* rather than a state ([[gates-need-destinations]]), and the cycle that orders discoveries automatically ([[integration-metabolism]]).
- Unintegrated influx behind this segment (do not cite as warrant): `plan/INFLUX/synthesis/relata-methods-for-verisectorium-2026-08-05.md` §2.8 and `plan/INFLUX/udon-analysis/underlying-logical-model.md` §4. Live source of truth for the relata half is the relata tree itself.
