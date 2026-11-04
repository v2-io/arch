# Scripts behind `../unknowns-scout.md`

These scripts compute every **measured** figure in the scout report. They were all run against ASF at `2133cf2e`. The trail had no events after 2026-11-02 and held 2,259 events.

On 2026-11-04 the whole chain was rerun from an empty work directory. Every figure in the report came out the same.

All scripts are read-only on ASF. They read `ASF_ROOT`, which defaults to `~/src/arch/asf`. The commands assume `vsect` (0.1.x) and Python 3 with PyYAML.

If you run this at a later commit, the numbers will move. That's expected: the scripts compute the state of the store at the commit you run them against.

## Running it

Make an empty work directory, change into it, and run:

```
D=~/src/arch/msc/councils/research/unknowns-scout-scripts
$D/00-fetch.sh
python3 -I $D/parse_reads.py reads
python3 -I $D/checks.py
python3 -I $D/diffs.py
python3 -I $D/chain.py
python3 -I $D/yields.py
python3 -I $D/tagtext.py
python3 -I $D/ready.py
python3 -I $D/clusters.py
python3 -I $D/sample_items.py
$D/xstore.sh /some/new/scratch/dir
```

Run these in the order shown. Later steps read the JSON that earlier steps write into the work directory. `sample_items.py` and `xstore.sh` are the exceptions: they don't depend on the others.

## What each one computes

| Script | Computes | Report § |
|---|---|---|
| `00-fetch.sh` | Fetches the inputs into the work directory (see below). | — |
| `parse_reads.py` | Tallies every check's mark (`✓`, `✓~`, `↻`, `⚠`) as vsect shows it, split by self or independent and by why it lapsed. This gives the 23 / 12 / 336 / 115 split. | 2.1 |
| `checks.py` | Recomputes each check's binding from the raw events against the current versions, without the re-bindings that confirms make (20 hold). It also applies the read-set split and writes `checkrows.json`. | 2.1 |
| `diffs.py` | Word-diffs each checked version against the current text. It sorts the distance into: Claim Record only, under 2%, 2–10%, 10–50%, and 50% or more. It also finds the median (19%). | 2.2 |
| `chain.py` | Walks each segment's `revise` chain from the checked version to now. It finds which lapses crossed only `reworded` edits (57) and which crossed a `redefined` one (211). | 2.2 |
| `yields.py` | Computes independent and self yields, and the independent yield by the order of the review on a segment (0.57 / 0.51 / 0.23 / 0.18). It also breaks yields down by line, declared type, declared status and volume. It writes `yrows.json`. | 4, 6 |
| `tagtext.py` | Finds which checked versions equal the pre-sprint tag text (7 of 218). This replaces the `tagtext` field in `yields.py`, which depended on `at-commit`, and most events don't record it. | 2.2 |
| `ready.py` | Counts each segment's open rows by kind from `open.json`, whether it changed since the tag, its independent-check history, and its holding marks. It writes `segrows.json`. | 4, 5 |
| `clusters.py` | Rolls `segrows.json` up by OUTLINE chapter to make the cluster table. | 4 |
| `sample_items.py` | Draws the seeded sample of 30 open items (seed `20261104`) and prints each one. The sorting into categories was done by hand and isn't in any script. | 5 |
| `xstore.sh`, `holdcount.sh` | The hash-identity experiment. `xstore.sh` builds a scratch store from a verbatim copy, then runs each variant with `holdcount.sh`. The variants are: renamed store and flat directory; Working Notes and `status:`/`stage:` removed; `form:` on one root; one carving tag on one root; only the Coupled Loop chapter. `holdcount.sh strict\|all` counts `✓`, or `✓` and `✓~`, over the 17 segments that hold checks. | 1.2 |

`00-fetch.sh` writes these inputs into the work directory:
- `reads/`: `vsect read` of all 247 live segments;
- `open.json`: `vsect open --format json`;
- `tag/`: each segment file at `pre-integration-2026-10-30`;
- `vers/`: the bare text of every version a check read (`pairs.txt` lists them), plus each checked segment as it is now (`@CUR`).

## Known limits

- **Independence** is read from `actor.role`. A role containing "integrator", "author" or "adopter" counts as self-recorded. This is the rule the flow review's `fsm.py` uses.
- **Word distance** is a proxy for how far a check's subject moved, and it overstates it.
- **Cluster mapping** comes from the `[#slug]` links in each `OUTLINE.md` table row. One Volume 1 segment has no row.
- **The ledger categories in §5** are one reader's judgment.
