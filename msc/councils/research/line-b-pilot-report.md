# Line B pilot: report (kept here so it survives)

*The pilot founded a store in `~/src/aat-v3/` on 2026-11-04 and crossed five ASF root segments into it (commits `2f12d7d`…`3e17ed6` there). Its harness refused to write `report.md`, so the report reached councils-46 only as a message, and is preserved here verbatim. Its `debrief.md` is in aat-v3 at `msc/pilot-founding-2026-11-04/`.*

*How to read it. Joseph, the same day: the pilot was not implementing the new system. It applied the current regime (vsect 0.1.0, `kinds.yaml`, crossing by hand) in a new location. That was the coordinator's framing error, not the pilot's. Its value now is as influx and data for designing the new system:*
- *the statement-grain findings in §4;*
- *the crossing costs in §3;*
- *the tool facts in §6.*

---

### 1. What was done

Founded `~/src/aat-v3/`: `.vsect/kinds.yaml` (kinds `segment`, `review`, `question`, `item`; edge `depends`; six evidence lines; declared fields `source`, `statements`; `stores: {asf: ../arch/asf}`), `README.md`, `main.outline.md`, `influx/00-INDEX.md` (dispositions log), `CHANGELOG.md`.

Crossed Part I Chapter 1's five roots: `def-agent-environment`, `def-action-transition`, `def-observation-function`, `def-chronica`, `scope-adaptive-system`. 548 ASF lines → 5 records, **32 statements**, text verbatim at ASF `2133cf2e` (sentence-level check: 364 source sentences outside Working Notes, all present; only structural change is bullets→paragraphs where an item became a statement). `scope-agency` and `post-causal-structure` are gap rows.

Carried: 5 findings (two review records, re-targeted to statements), 5 questions, 4 owed items, 11 owed re-checks (check 8's criteria as a regression suite). `vsect check` clean, 27 records, 7 events. Left in ASF and logged per record: the audit-gold harvests (≈55% of each root's lines), lapsed checks and rewordings, spike trails, items targeting segments outside the cluster.

### 2. Founding: cost and what it bought

≈15 minutes and ~90 lines, after ≈90 minutes of reading and a 10-minute scratch store that told me what vsect 0.1.0 does. Bought: no typed status anywhere; `source:` provenance on every record; one permanent intake path; findings/questions/items as records from the first hour. Declared no outline above the chapter and no statement-kind as settled.

What it did not buy: **vsect 0.1.0 is record-grain.** The 32 statements and 38 statement-grain edges are in the files per `msc/vsect/core.md` §1 and the tool does not read them. Standing, basis, findings bind to the whole record. Until the tool grows, the store's grain is ASF's grain with better files.

### 3. Crossing cost per segment

| Record | ASF lines | Statements | Felt time | What took it |
|---|---|---|---|---|
| def-agent-environment | 102 | 2 | 25 min | boundary's kind (choice, not definition); hidden premise on the scope; umbrella/tier paragraph |
| def-action-transition | 91 | 3 | 15 min | one Epistemic Status paragraph split three ways; opacity's kind |
| def-observation-function | 93 | 5 | 25 min | an untagged, independently-checked claim (`kernel-is-learnable-content`) pulled out of a clause |
| def-chronica | 110 | 2 | 15 min | non-forkability as its own claim; four follow-ups routed elsewhere |
| scope-adaptive-system | 152 | 20 | 70 min | one 650-word definition paragraph = six separately-failable commitments; ten membership items; two untagged exact claims; 38 edges |
| total | 548 | 32 | ≈150 min | + ≈40 min carrying and logging |

Typing was never the cost; carve judgment was. About a quarter of the judgments are flagged needs-review in the log. Roots are the worst case (scout: definitions draw findings at 0.67/check; this chapter has the most open findings per segment).

### 4. What the crossing found in the source

1. **One tag, many fates.** ASF's `*[Word (id)]*` marks where a formal statement starts, not what can fail alone. The scope's single `*[Definition …]*` paragraph held six things with separate fates and separate open questions; its one `*[Derived …], exact*` covered nine items check 8 verified in five separate checks. Every edit to any lapsed every check on all (63 lapsed on this segment). This is the mechanism under "rounds fed themselves".
2. **Exact claims in non-claim sections.** `non-vacuity` (Epistemic Status), `warrant-stock-and-renewal` (Discussion), `kernel-is-learnable-content` (inside a definitional clause) — all independently checked, none addressable in ASF's format.
3. **Record-grain `depends:` hides cross-directional premises.** `def-agent-environment` has `depends: []`, yet its boundary rests on the scope's predicate; opacity in `def-action-transition` is defined through the scope's $\pi_0$ rule. Record grain would be a cycle; statement grain has none. Three of five roots have this shape.
4. **Audit gold filed where the reader stood, not where the defect is.** Three of `def-chronica`'s four live follow-ups target other segments.
5. **A finding can name two statements in two records** (F4). Record grain placed it on one; the statement that takes the step it calls "left to the reader" (`generated-by-kernels`) exists only after the carve.
6. **The scope record is 13 exact claims + 7 declarations/teachings in one file**; the retrospective already said its proofs belong in an appendix. Now a count, not a decision.

Nothing crossed was found false. Four ASF findings stay open here on the statements they belong to.

### 5. Lost vs gained against ASF

**Lost:** ASF's trail on these segments (190 events on the scope alone): 11 check-8 checks, 63 lapsed checks, 7 rewordings, revise history. The scout measured that any carving tag changes the version, so none could bind. Carried instead: criteria and methods as owed re-checks. Also lost, deliberately: gold texture, integration-record Working Notes, the `stage:` column.

**Gained:** 32 addressable statements vs 5; 38 edges vs 10; every finding on the sentence it's about; questions/items keyed to statements; an outline whose rows are gaps or records; records a cold reader can `cat` and see kind and premises before any claim.

**A cold reader meets:** in ASF, a 152-line scope whose first screen is one 650-word paragraph, then proofs, then 36 lines of audit residue, under a dashboard saying "known broken" over two wording items. Here, the same words under 20 tags, proofs each opening with what they are, no residue, `vsect open` listing 2 findings, 2 questions, 14 items — and a front door saying the tool cannot yet read what the tags say.

### 6. Tool facts met on the way

`vsect take` cannot bind a carried check to another store's commit (`at: 3d17de13` → "Not a valid object name"); `vsect revise` mints nothing at record grain; `ask`/`note` auto-slugs are ugly (`owed-re-check-regression-from-asf-check-8-c1-lap`) and want a `--slug`.

### 7. Plan for the rest, from these rates (judgment; queue ownership not mine)

1. **vsect reads tags before many more carved records land.** These five files are the grammar's first fixtures (display words `Definition`, `Choice`, `Scope`, `Claim`, `Teaching`, `of`; `form:` in frontmatter, not `, exact` in the tag).
2. **Carve only what is worked; copy the rest as-is.** Scout: 132 segments with nothing open and no independent check; ≈5 min each to copy (≈11 h) vs ≈30 min to carve. Carve on first touch, by the mind that needs the result.
3. **Order by dependency, roots first**: Chapter 1's two gap rows, then Part I Ch. 2–4 (28 segments, the most settled mathematics).
4. **Cross the ledger split by kind** (scout §5: fixes/clarity/seeds/planned/build/retire; ≈1 in 30 residue); seeds may want their own kind.
5. **Declare the owner model before assigning work**; every item here says `unassigned`.
6. **Leave behind**, named in the log: effort tree, audit working dirs, live spikes, lapsed checks, gold harvests. Per Joseph's correction: **do not keep old slugs for their own sake** — name by what's true and publish an old-slug → fate mapping (renamed/split/merged/retired/copied) in `influx/`, generable from `source:`; vivarium's 2,881 references resolve through it.
7. **The first independent check here is the real maturity test.** Everything starts at drafted; the eleven owed re-checks are one bare brief from running and would be the tool's first real binding exercise.

Rough total: ≈115 segments with something open carved at ≈30 min ≈ 60 h; ≈132 copied ≈ 11 h; ≈630 ledger items typed at ≈3 min ≈ 30 h; plus the tool work in step 1, unestimated. Three to four agent-weeks single-threaded; parallel by chapter now the roots are in.

### 8. Scope honesty

I carved alone; nothing was checked by a second mind. I re-derived no mathematics. I read nothing of ASF beyond the five segments, check 8, the Part I outline rows, INTRODUCTION, and the brief's named documents. Felt times are mine; the extrapolations use the scout's counts unrecomputed and my worst-case five-segment rates.
