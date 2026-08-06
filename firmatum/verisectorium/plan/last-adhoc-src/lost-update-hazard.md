---
slug: lost-update-hazard
type: obs
---

# Lost-update hazard (multi-record file)

**Summary.** When many records share one file that writers update by read–modify–write, concurrent writers can drop each other’s records without git ever showing a merge conflict.

## Observation

**Mechanism (standalone).** Let F be a file that holds many records. Two writers each:

1. Read the whole of F.
2. Append a new record in memory (X and Y respectively).
3. Write F back.

If B’s write lands between A’s read and A’s write, A’s write restores a version that never contained Y. **Y is gone.** Git only compares the two final blobs of F if both commit; it does not surface “a record that existed after B’s write and disappeared after A’s write” as a line-level conflict when both parties intended only to append. The failure is classic lost update under non-atomic multi-record rewrite — the same shape as concurrent mbox append (hence Maildir-style one-file-per-key as a common repair).

**Specimen (placement, not an incident log).** As of **2026-08-05**, vivarium still keeps a multi-record decision log at:

`vivarium/DECISIONS.decision-log.udon`

Verified that day: path present; size ~556 KB (many records in one file). This drafting session **did not** re-run a dual-agent race against that file. The observation is: **the file shape that admits the mechanism is live in a production claim corpus**, so any concurrent multi-agent write path against that file inherits the hazard unless a single-writer membrane or per-key layout intervenes.

## Method

- Mechanism: concurrent read–modify–write of a multi-record file (standard concurrency failure mode).
- Specimen check: filesystem presence and size of `DECISIONS.decision-log.udon` on 2026-08-05.
- No measured incidence rate of lost records on that path.

## Strength and scope

- Supports: **the failure mode is real for multi-record shared files under concurrent RMW**; multi-agent corpora that use that shape need a write discipline that avoids it.
- Does not support: how often vivarium has actually lost decision records; that every multi-record file is always unsafe under a single sequential writer.
- Honest strength: **structural mechanism + live specimen of the shape**, not an incident log.

## Working Notes

- Repairs: one file per key (or collision-free create); single-writer drain; or a primitive that fails loudly on “key already exists.”
- Related undrafted: [[partition-isolation]], [[write-safety]].
- Unintegrated influx behind earlier drafts of this topic (do not cite as warrant): `plan/INFLUX/udon-analysis/underlying-logical-model.md` and related document-store discussion. This segment replaces those pointers for the mechanism and specimen.
