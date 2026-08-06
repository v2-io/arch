---
slug: write-safety
type: form
---

# Write safety for multi-agent corpora

*A write is safe when concurrent agents cannot silently destroy each other’s records — which means atomic single-placement updates for one record, and no multi-record file under concurrent read–modify–write without a membrane.*

## The claim

**(A) Single-record placement.** When one filesystem object is one record (one segment file, one terminology entry, one event file named for collision-free identity), a writer can use atomic replace of that object (temp + fsync + rename, or equivalent). Concurrent writers to *different* keys do not meet. Concurrent writers to the *same* key last-write-wins on content, but the conflict is local and reviewable in git as a single-file disagreement.

**(B) Multi-record shared files.** When many records share one file and writers update by reading the whole file, appending, and writing back, concurrent writers can drop records that never appear as a merge conflict. That is not a special “agent” bug — it is classical lost update. The estate’s live specimen of the dangerous shape is [[lost-update-hazard]] (vivarium’s multi-record decision log).

**(C) Membranes.** Where multi-record files are still wanted (human-readable single log, streaming friendliness), concurrent writers must not RMW the file freely: a single-writer drain, an append-only protocol the filesystem can enforce, or a spool that promotes into the log under one hand. Absent that, prefer per-key files.

**(D) What this is not.** This formulation does not solve same-key contention policy (who wins when two agents rewrite the same segment), directory-fsync durability details, or cross-file transactions beyond “git commit as multi-file snapshot.” Those are adjacent concerns ([[partition-isolation]], durability footnotes elsewhere).

## Strength & grounds

**(A)–(C)** are **heuristic / engineering**, grounded in standard concurrency failure modes plus the live multi-record specimen in [[lost-update-hazard]]. Atomic single-file replace is already estate practice (terminology, refs, relata `safe_write` lineage). Not a formal proof; not a claim that vivarium has lost N decisions — only that the shape admits silent loss under concurrency.

## Working Notes

- Dogfood: this plan instance uses one file per segment slug — the safe default for claim atoms.
- Open: event-child directories vs embedded frontmatter event logs (cost vs concurrent append) — see also [[atom-as-cluster]].
