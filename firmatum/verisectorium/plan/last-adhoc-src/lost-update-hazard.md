---
slug: lost-update-hazard
type: obs
---

# Lost-update hazard (multi-record file)

**Summary.** When many records share one append-target file, concurrent writers can lose records without the loss appearing as a git conflict; the estate has named this on a live multi-record decision log shape.

## Observation

**Structural failure mode (classical lost update):**

1. Agent A reads multi-record file F into memory, appends record X.
2. Agent B reads F, appends Y, writes F.
3. Agent A writes F from its older base + X → **Y is gone**.
4. Git never saw a conflict between X and Y if both “appended” in memory; the second write clobbers the first’s sibling record. Absences do not show as content disagreements the way dual edits of the same line can.

**Specimen named in estate analysis** (underlying-logical-model / doc-store cluster discussion, 2026-07-29 era; re-checked path existence 2026-08-05):

- Live path: `vivarium/DECISIONS.decision-log.udon` — multi-record decision log at corpus root (field report and logical-model discussions treat this class of file as the concurrent-append hazard surface).
- Prior art analogy cited there: mbox vs Maildir (shared append vs one-file-per-key).

This drafting session **did not** re-simulate a dual-agent race; it re-confirmed the **file class still exists** and the **logical argument** is the same as classical concurrent read-modify-write.

## Method

- Mechanism: standard lost-update on non-atomic multi-record rewrite.
- Estate naming: INFLUX `underlying-logical-model.md` + related acid-for-corpora discussion of multi-record write hazard.
- Path check: vivarium `DECISIONS.decision-log.udon` present 2026-08-05.

## Strength and scope

- Supports: **this failure mode is real for the file shape**; multi-agent claim corpora that pack many records into one growing file inherit it.
- Does not support: measured incidence on that specific file; that all multi-record files are unsafe under single-writer membranes.
- Honest strength: **structural observation + named specimen class**, not an incident log.

## Working Notes

- Feeds [[partition-isolation]] / [[write-safety]]: per-key files or single-writer membranes are the usual repairs.
- Maildir `link()`-fails-on-collision vs `rename()`-clobber detail is useful if we ever implement create-if-absent for event files.
