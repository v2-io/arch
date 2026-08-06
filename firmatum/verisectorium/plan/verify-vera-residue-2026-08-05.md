# Independent verification: `residue-vera-2026-08-05.md` and companions

*Adversarial check, requested and performed 2026-08-05. Scope: the main register (`plan/residue-vera-2026-08-05.md`) plus its two companion inventories. Everything below was checked against live files, not against the author's own account of them — I re-ran commands, re-diffed files with my own stripping logic, and grepped live primaries independently.*

**Bottom line up front:** the register's *content* claims — byte-identity, quotations, numeric counts, the `:max-attainable` and dead-path facts — all hold up, several to the character. I found **one real defect** (a dispatch-record claim about file moves that did not happen) and **one stale citation path**. Neither touches the substance of what's claimed about the source material; both are about the register's own bookkeeping. I did not find any ✔ mark that should have been (agent) or (scout) — where I checked ✔ items against primaries myself, they were correct.

---

## 1. The one real defect — the dispatch table describes moves that never happened

The "Dispatch record" table (lines 28–38) states, as settled fact with no hedge:

> `.archive/vera/comproprium/`, `.archive/vera/proprium/`, `.archive/vera/vox-vera/`, `.integrated/vera/` — as destinations for 21 files, all "✔ ... byte-identical," implying the files now live there.

I checked the filesystem directly:

```
$ find plan/.archive -iname "*vera*"
(no output)
$ find plan/.integrated -iname "*vera*"
(no output)
```

**None of these directories exist.** All 21 files are still sitting untouched in `plan/INFLUX/vera/` — `comproprium-segments/`, `vox-vera/`, the four proprium-mapping files, `00-INDEX.md`, `synthesis-scatter-2026-08-05.md`, and the two ennaos files that were *deliberately* left in place. The dispatch table for the ennaos pair is honest ("stays in INFLUX"); the other five rows assert a destination as if the move already happened.

This matters for exactly the reason your own `ONTOLOGY.un` names: `influx-not-warrant` exists so that "when INFLUX is fully `.integrated/` or `.archive/`, every segment body still stands" — and right now INFLUX is *not* metabolized the way the register's own dispatch table claims. A reader who trusts this table and later looks for `.archive/vera/comproprium/ver-durability-wager.udon` will not find it. It's not a fabrication about content — every byte-identity claim in that table is true of the *source* files wherever they currently sit — but the table asserts a filing action that is still only intended, not done. This is the shape of thing your ONTOLOGY calls out by name: a present-tense claim about an act that hasn't fired.

**Verdict: overstated / false-as-written.** The fix is mechanical (either run the moves, or change "Destination" language to "intended destination, not yet executed" / "queued"), but as it stands the table is wrong about the current state of the filesystem, and it's the kind of thing Joseph would act on directly (e.g. by looking for the archived files) without re-checking.

I did not find any corresponding false claim in `INFLUX-DISPATCH-2026-08-05.md` — its dispatch table's destinations *do* exist (`.archive/udon-theory/`, `.archive/primary/`, `.integrated/synthesis/` all present on disk), so this looks specific to the vera register, not a pattern across both files.

## 2. One stale citation path (V17)

V17 cites its source as `INFLUX/synthesis/relata-methods-for-verisectorium-2026-08-05.md`. That file is **not in INFLUX** — it was already dispatched by an earlier pass and now lives at `plan/.integrated/synthesis/relata-methods-for-verisectorium-2026-08-05.md`. The content claim itself checks out exactly (§6.2 of that file does say vox-vera is "a *presentation* calibration for confidence language" and relata "a *decision* calibration for evidence combination... both belong under VERA, at different layers" — near word-for-word what V17 claims), so this is a citation-path staleness, not a content error. Given item 1 above, I'd guess both are symptoms of the same thing: dispatch-table bookkeeping lagging the prose that assumes it's done.

**Verdict: minor / cosmetic**, but worth fixing alongside item 1 since it's the same class of defect — a path asserted as current that isn't.

## 3. Byte-identity claims (item 1 of the brief) — all 21 files verified independently

I wrote my own banner-stripper (Python, strip from `<!--` through the matching `-->` plus one following blank line) rather than reusing any logic in the register, and diffed every file named in the dispatch table against its live original:

| File | Live original | Result |
|---|---|---|
| `comproprium-FORMAT.md` | `proprium/comproprium/FORMAT.md` | identical |
| `comproprium-README.md` | `proprium/comproprium/README.md` | identical |
| `comproprium-segments/*.udon` (12) | `proprium/comproprium/vera/*.udon` | identical, all 12 |
| `PROPRIUM-ONTOLOGY-v2.md` | `proprium/INGEST/msc-from-harness/canonical/...` | identical |
| `PROPRIUM-ARCHITECTURE-v2.md` | same dir | identical |
| `asf-def-proprium-mapping.md` | `asf/04-eli-core/src/def-proprium-mapping.md` | identical |
| `asf-terminology-proprium-mapping.md` | `asf/terminology/entries/proprium-mapping.md` | identical |
| `vox-vera/*` (4) | `_core/zoetica/docs/refs/vox-vera/*` | identical, all 4 |
| `ennaos-vera-architecture-final-specification.md` | `_core/ennaos/docs/research/vera/vera-architecture-final-specification.md` | identical |
| `ennaos-gemini-chat.md` | `_core/ennaos/docs/research/vera/gemini-chat.md` | identical |

That's 21/21. The "all copies byte-identical" claim is correct in substance — the actual defect is only that the *archival filing* the table describes hasn't happened (item 1), not that the identity claim is wrong.

## 4. Quotations, V1–V15 — checked against live comproprium files

I grepped each quoted phrase against the live `FORMAT.md`, `README.md`, and the twelve `vera/*.udon` files directly (not via the INFLUX copies). Every substantive quotation located, verbatim, including multi-clause ones (V1's "a claim's job is to be presently true, an account's job is to be what happened"; V7's "An absent answer makes no claim, so it can collide with nothing"; V8's "not one failure recorded here was a knowledge gap"; V9's full "manufacture originals" mechanism paragraph; V12's "source §9's own test" language reused twice; V14's "the corpus's own #9" self-correction).

One near-miss worth flagging as a genuine (if trivial) misquote, since the brief specifically asked for meaning-preserving misquotes too: **V6** quotes README.md as *"a segment's type is intrinsic, its position belongs to a view"* — the live text is *"a segment's **type** is intrinsic, its **position** belongs to a view"* with markdown emphasis (`*type*`, `*position*`) on those two words. The register silently drops the emphasis markers. Meaning is unaffected, but per your own stated bar this counts as a defect and I'm naming it. Nothing else in V1–V15 showed this pattern — the rest preserve emphasis and punctuation exactly where I checked.

## 5. Specific facts (item 3 of the brief)

- **Comproprium working tree clean, last commit `a40825f` (2026-08-01):** confirmed. `git status --porcelain -- comproprium` returned nothing; `git log -1 -- comproprium` returned exactly `a40825f... 2026-08-01 Reorganize various ingest queues`.
- **`bin/check-corpus` output `57 segments · 3/109 · 106 fail · 0 warn · 5 forward refs`:** confirmed by re-running the checker myself, including the exact five named forward references (`#ver-assume-wrong`, `#ver-write-the-prior-down`, `#prx-tally-recurrence`, `#prx-hold-two-things-at-once`, `#exm-priced-low-three-of-three`).
- **`~/src/firmatum/` does not exist, and the two paths in `asf/terminology/entries/proprium-mapping.md` are dead:** confirmed — `ls ~/src/firmatum` fails, and lines 39–40 of that file do point at `~/src/firmatum/PROPRIUM-ONTOLOGY-v2.md` / `...ARCHITECTURE-v2.md`. I also independently confirmed the file's *other* internal reference (`#def-proprium-mapping` → `04-eli-core/src/def-proprium-mapping.md`) does resolve, matching V19's "one file, two reference designs, opposite outcomes" framing exactly.
- **V2's `:max-attainable` split (11 empirical / 1 discussion-grade):** confirmed by grepping all twelve `vera/*.udon` files directly — eleven declare `:max-attainable empirical`, and exactly one, `ver-durability-wager.udon`, declares `:status discussion-grade :max-attainable discussion-grade`, with the reasoning quoted ("no amount of further work turns this into a result...") present verbatim in the file's `|epistemic-status` section.
- **V20's duplicate-copy claim:** confirmed — `PROPRIUM-ONTOLOGY-v2.md` and `PROPRIUM-ARCHITECTURE-v2.md` exist under both `INGEST/msc-from-harness/canonical/` and `INGEST/old-firmatum/`, and `INGEST/msc-from-harness/bridges/` does contain the draft files named.

## 6. Strand separation

I read through the register specifically watching for cross-contamination between the four "VERA" referents and didn't find any collapse. The dispatch table, the four-strand table, and the residue rows consistently tag each item with its strand number and keep PROPRIUM-VERA (strand 1), comproprium `vera/` (strand 2), ennaos VERA (strand 3), and vox-vera (strand 4) separate — including in places where it would have been easy to blur them, like V16/V17 (vox-vera vs. relata's *different* calibration machinery) and V21 (an ennaos mechanism compared to V2's comproprium mechanism, kept as "three independent takes... side by side," not merged into one). No instance found where a claim about one strand leaked into another strand's row.

## 7. Register honesty — V8 and the routing verdicts

**V8** is explicitly labeled `discussion-grade` / `:max-attainable empirical` inside the *source* segment itself, and the register's own framing around it ("the most decision-changing item in this chunk," "a candidate answer," "the author's own caution is kept and is load-bearing") reads as opinion clearly marked as opinion, not as a measured result dressed up. The register does not claim V8 is established — it explicitly relays the source segment's own hedge (an artifact of a day of *orientation* failures, may not generalize to *reasoning* failures) rather than dropping it. I'd call this honest: it's Joseph's own steward-priority pattern (V14) applied to the register's own author — a judgment, named as a judgment, with the counter-evidence attached.

The **routing verdicts generally** (residue → OUTLINE row) read the same way throughout: none of the "Owed to" column entries are phrased as settled fact, and several rows explicitly flag ambiguity in the routing itself (V12's "strengthens R14," V17's "wherever ch. 8 is finally organized"). I didn't find a routing claim dressed up as more certain than "this looks like where it belongs."

## 8. The two companion inventories — checked against the main register's one-line summaries

I read both companion inventories in full and compared them line-by-line against every place the main register cites or summarizes them (V18, V20, V21, V22, and the strand-4 table row).

- **V18** ("PROPRIUM leg is a named slot with a one-paragraph mechanism sketch... no representation for uncertainty, no retraction or propagation protocol, 'epistemic council' named but never defined") matches the proprium inventory's §1 and §3 conclusions closely, including the "epistemic council... appears exactly once... neither primary document defines it" finding. One soft spot: the register's "placement in three generic tables" compresses what the inventory actually describes as several distinct tables/mentions (the §5.1–5.4 memory-forms tables, the PULSUS cadence table, the emergent-regime tables, plus non-table mentions in CONTEXTUALIZE and CONSPECTUS assembly). "Three" isn't clearly wrong, but it's a compression a careful reader could contest — I'd call it defensible, not an upgrade of confidence.
- **V20** matches the inventory's §5 duplicate-files finding exactly, including "not diffed."
- **V21** and **V22** match the ennaos inventory's §3 and §5 findings closely, including the verbatim phrase "dropped, status unclear" for V22 and "plausibility vs. validated truth... first-class field" for V21.
- I did not find any place where the main register upgrades an (agent) finding's confidence, hedges, or caveats in the retelling. If anything the register under-claims relative to the inventories in places (e.g., it doesn't repeat the proprium inventory's speculative "why VERA is thin" inference in §2, correctly leaving that out as the inventory's own flagged guess).

## Holistic read

This is a well-disciplined document. The verification marks (✔/agent/scout) are honest everywhere I checked them against primaries — I did not find a single ✔ that should have been an inference, and the (agent) marks correctly track material the author didn't personally re-read. The quotation discipline is close to exact; the one dropped-emphasis instance (V6) is trivial. The numeric and factual claims (commit hash, checker output, `:max-attainable` split, dead paths) are all exactly right, to the character — this is not "roughly correct," it's checked-and-correct.

The one finding worth acting on before Joseph reads this in a working session is **item 1**: the dispatch table's "Destination" column reads as done when it isn't. That's exactly the "inference wearing verification's confidence" shape this whole exercise is meant to catch, just displaced from a content-claim onto a bookkeeping-claim — the content is fine, the filing-system state asserted around it is not. I'd fix that (and the item-2 stale path, same cause) before this goes into the session, since both are things Joseph could act on directly (e.g., "go look in `.archive/vera/`") and hit a dead end.
