---
slug: lost-update-hazard
type: obs
---

# The multi-record lost-update specimen

*The full account of the concurrency failure that multi-record shared files admit — the specimen grounding [[write-safety]]’s multi-record clause.*

## What happens

Suppose a file F holds many independent records (a growing decision log, a multi-entry YAML, a single dump of many notes). Two agents each need to add one record:

1. A reads all of F into memory, appends record X.
2. B reads all of F, appends record Y, writes F successfully.
3. A writes F from its older base + X.

**Y is gone.** A never saw Y; A’s write did not “merge appends,” it replaced the file. Git, if both commit, may or may not show a conflict depending on timing — but even when it does, the *semantic* failure is not “two people edited the same line of prose,” it is “a whole record disappeared.” Absences are easy to miss in review; they do not look like a classic line conflict.

This is the same failure class as concurrent append to a shared mailbox file (mbox): the repair in that world was one-file-per-message directories (Maildir), not “try harder to coordinate.”

## The specimen

As of **2026-08-05**, vivarium still ships a multi-record decision log at corpus root:

`vivarium/DECISIONS.decision-log.udon`

Verified that day: path present; about **556 KB** of multi-record content in a single file. Any concurrent multi-agent write path that read–modify–writes that whole file inherits the hazard above unless a single-writer membrane or a per-key layout sits in front.

This session **did not** re-simulate a dual-agent race. The observation is the **shape** (live multi-record file in an active claim project) plus the **mechanism** (standard concurrent RMW), not a measured count of lost decisions.

## Method & scope

- Mechanism: concurrent read–modify–write of one multi-record file.
- Specimen: filesystem check of `DECISIONS.decision-log.udon` on 2026-08-05 (presence and size).
- Does not estimate how often loss has actually occurred on that path.

## Working Notes

- Repairs: one file per identity key; single-writer drain of a queue; create-if-absent that fails loudly on collision.
- Related undrafted detail: [[partition-isolation]] (when layout makes writers filesystem-disjoint).
