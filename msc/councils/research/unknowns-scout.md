# Unknowns for crossing ASF into aat-v3: what reading and computing settle

*2026-11-04. Claude Opus 5.5, a read-only scout for councils-46, for Joseph, the two pilots, and councils-46. ASF read at `2133cf2e` (its trail has had no event since 2026-11-02: 2,259 events, the same count the flow review used). The aat-v3 pilot read at `5262fb9`. vsect source read at `firmatum/utils/vsect/src/`. Nothing in ASF, its `.vsect/`, or aat-v3 was changed. The experiments in §1 ran on a scratch copy outside every repo.*

*Marks: **measured** (with the method beside it), **estimate** (a computed figure resting on a stated proxy), **guess**.*

*How the brief moved while I worked, so you can see what this file answers. The brief opened with five unknowns for deciding between in-place and a new store. Then councils-46 dropped unknown 2 (§7 says why), and then put three items first: what a new store can do that in-place can't, whether verbatim crossing keeps vsect's evidence, and the inventory of what accreted. Then Joseph decided: ASF crosses into a new store in `~/src/aat-v3/`, and anything already right can be copied as-is. So this file answers **how the crossing should go**. The in-place-versus-new comparison I'd started is folded into §3 as what to leave behind. The flow numbers (§6) don't bear on the decision, but they do bear on the new store's routing rules.*

## 0. The short version

1. **Verbatim copying keeps a check's binding only under narrow conditions, and the pilot's own way of crossing breaks them all.** A version is SHA-256 over the kind name, the frontmatter `form:` value, the normalizer name, and the normalized body minus `## Working Notes`. The store name, the path, the slug, and every other frontmatter field are outside the hash. A check holds only if its entire `depends:` closure is present with the same ids and every version in it unchanged. I tested this on a scratch copy (**measured**, §1.2):
   - verbatim copy into a renamed store with a flat `src/`: all 35 holding checks still hold;
   - stripping Working Notes, dropping `status:`/`stage:`, and adding `predecessor:`: all 35 still hold;
   - declaring `form:` on one root, or adding one carving tag `*[Definition (coupling)]*` to one root: strict holds fall from 23 to 0;
   - crossing only the Coupled Loop chapter: 16 of 35 still hold.

   The pilot carves bodies with tags, so the versions of its two crossed segments already differ from ASF's (`54ed06cb` vs `da0b3e97`, `d63308e3` vs `62f8ce05`, **measured**).
2. **There is little evidence to keep, so the crossing shouldn't bend to keep it.** Of 488 recorded checks, 35 bind to current text (7%). Only 10 of those are independent, and 8 of the 10 sit on `scope-adaptive-system`, which carries open narrowed findings. Keeping all 35 would require 106 segments to cross verbatim with their edges unchanged and no carving (**measured**). What is worth carrying is the trail as a *source*, not as standing: 488 criteria, each with its recorded recheck method. That is a ready-made re-check suite (§2.3).
3. **What to leave behind, with sizes, is §3.** The heaviest items are:
   - the effort tree: 153 files, about 19.8k lines;
   - 24 `AUDIT-WORKING-*` directories: 1,410 files, 47k lines;
   - live spikes: 264 files, 37.8k lines;
   - Working Notes: 5,759 lines, 18% of all segment lines;
   - the 336 lapsed checks.

   Two couplings are easy to miss. `kinds.yaml`'s `owners_from:` points into the effort tree. And 179 items cite the effort tree as their source.
4. **No cluster is mature on evidence rungs, so maturity can't set the crossing order.** Dependencies do: a crossed segment's checks can only evaluate once its closure has crossed. Counted by computed evidence, three segments sit above "drafted". 132 segments have no open finding or question, but 114 of those were never independently checked. Their quiet means unchecked, not settled. My read is in §4: cross roots first, copy the 132 verbatim, and treat the first independent check in the new store as the real maturity test. On the record, that check finds something on more than half of definitions and hypotheses.
5. **The open ledger is mostly real, but mixed.** In a seeded sample of 30 open items (**estimate**):
   - 13 are concrete owed fixes to canon text or reader clarity;
   - 11 are research seeds;
   - 2 are unwritten planned segments;
   - 3 are build or tooling;
   - 1 is a source to retire;
   - about 1 is residue.

   So the ledger should cross, split by kind. It shouldn't be dropped (§5).

## 1. Does verbatim crossing keep the checks valid?

### 1.1 From the source

`norm.rs`: `own_version = sha256("kind: {kind}\nform: {form}\nnormalizer: vsect-norm/0\n---\n" + normalize(text))`. `record.rs`: `text` is `statement_text(body)`, meaning the body minus the final `## Working Notes` section and minus vsect's generated marks (block quotes opening `> [!vsect `). `form` is the frontmatter `form:` value, which is empty for every ASF segment. The normalizer joins soft-wrapped lines, collapses whitespace, keeps fenced code and display math byte for byte, and drops md-press markers.

So these are **in** the hash: the literal kind name (`segment`), `form:`, the normalizer name, and the body's words and math.

These are **outside** it: the store name, the file path or directory, the slug, every frontmatter field except `form:`, Working Notes, and line wrapping.

`state.rs` `verdict()`: a check holds only if three things are true.
- **The same ids.** The set of ids in its recorded basis must equal the basis computed now: the statement plus its transitive `depends:` closure, through edges declared `in_basis`/`meaning`. If the ids differ, the verdict is "basis changed", or "cannot evaluate" when a member is missing.
- **The same versions.** Every version in the basis must be unchanged, or covered by a recorded `confirm-rewording` equivalence.
- **Read sets, for upstream changes.** A check that declared a read set (`actor.read`) is demoted only to the lighter `✓~` by an upstream change outside that set.

Events are read from `.vsect/events/**`, and their ids are content hashes. Nothing compares an event's `store:` field to the reading store's name (I read `event.rs` and `state.rs`; there's no such check). core.md §3.2 says outright that "declared form or statement-kind changed" is classified like a text edit.

What a carried check therefore needs:
- the same kind name, `segment`, with no `form:`;
- the same slugs;
- the same `depends:` edges and the same edge declarations;
- the whole closure present;
- the body text unchanged up to whitespace;
- the event files copied, including any `confirm-rewording` events that re-bound it.

### 1.2 Tested

Here's the setup. I copied all 247 live segments into one flat `src/` in a scratch git repo, with `.vsect/events`, `.vsect/objects`, `reviews/`, `questions/` and `items/`. I gave it a `kinds.yaml` with the store renamed `xtest`, a single `find: src/*.md`, and `planned:` and `owners_from:` removed. Then I counted the marks over the 17 segments that hold checks in ASF:

| Variant | Strict holds (`✓`) | Holds incl. `✓~` |
|---|---|---|
| ASF itself | 23 | 35 |
| verbatim copy, renamed store, flat directory | 23 | 35 |
| + all Working Notes stripped, `status:`/`stage:` deleted, `predecessor:` added | 23 | 35 |
| + `form: definition` on `def-agent-environment` only | 0 | 28 |
| + one carving tag `*[Definition (coupling)]* ` before the first sentence of `def-agent-environment` only | 0 | 28 |
| only the seven Coupled Loop segments present | 16 | 16 |

The scratch dashboard otherwise matched ASF's line for line: 182 findings, 52 questions, 632 items, 336 lapsed, 6 records holding. The one difference was "7 dangling targets", from removing `planned:`.

One edit to one root takes every strict hold to zero, because all 23 strict holds have `def-agent-environment` (`depends: []`) in their basis, as the experiment itself shows. The 28 that survive as `✓~` are checks that declared read sets without that root.

### 1.3 What this means for the crossing

The pilot carves each segment into statements with in-body tags and moves paragraphs under them. For `def-agent-environment`, the diff shows 18 changed lines (**measured**, `diff` of `vsect read --bare` in both stores). So under the pilot's method, no check binding survives the crossing of any segment it touches, and any carving of a root lapses everything downstream. The same is true in place, by the same arithmetic: carving or declaring forms lapses the same checks in ASF itself. This cost belongs to carving and typing, not to the choice of store.

If keeping the bindings mattered, the recipe would be: copy segments verbatim and uncarved, under kind `segment` with no `form:`, roots first, with events copied. Then carve later, as an ordinary revise in the new store. §2 is why I don't think it's worth that constraint.

## 2. What the trail is worth now (unknown 1)

### 2.1 Where the 488 checks stand

**Measured** by parsing `vsect read` for all 247 segments, and cross-checked by recomputing every check's basis against current versions from the raw events.

| State | Count | Self-recorded | Independent |
|---|---|---|---|
| holds (`✓`) | 23 | 14 | 9 |
| holds for what it read; an unread upstream changed (`✓~`) | 12 | 11 | 1 |
| confirmed, lapsed (`↻`) on own text | 257 | 146 | 111 |
| confirmed, lapsed upstream or by re-declaration | 79 | 48 | 31 |
| broken or narrowed, on text since revised | 112 | 3 | 109 |
| broken or narrowed, still binding its text | 3 | 0 | 3 |
| target gone | 2 | | |

Of the 35 that hold:
- 25 are self-recorded. On the record, an author's re-derivation of their own landing has found nothing in 116 tries (**measured**, the flow review's figure reproduced).
- The 10 independent ones are on `scope-adaptive-system` (8), `def-action-transition` (1) and `der-causal-hierarchy-requirement` (1).

From the raw events with no re-bindings, 20 hold. `confirm-rewording` events (74 equivalent, 3 not) re-bound the other 15.

### 2.2 How far the lapsed checks are from the current text

For the 270 confirmed checks whose own text changed, I walked each segment's `revise` chain from the checked version to now (**measured**):
- 57 are separated only by edits their reviser classed `reworded`, which is the cheap confirm route;
- 211 have a `redefined` edit in between;
- 2 have no chain.

As a word diff between the checked text and the current text (**measured**, `difflib` over whitespace tokens of `vsect read --at … --bare` against current):
- 12 changes are Claim Record insertions only;
- 34 change under 2% of words;
- 50 change 2–10%;
- 126 change 10–50%;
- 48 change 50% or more;
- the median distance is 19% of words.

Word distance overstates how much a check's subject moved, since one confirm pass found 3 real changes in about 120 lapses. But it shows that most lapsed checks are on text that has been substantially rewritten, not reworded.

Almost none of the trail is about the canon as it stood before the sprint. Of the 218 distinct versions any check read, 7 equal the tag text (**measured**: each checked version's bare text compared with the tag file's body minus Working Notes; the method reproduces exactly on all 75 segments unchanged since the tag). That is 8 of 488 checks. The trail's yields therefore measure text under active revision, and the inherited canon's own defect rate is essentially unmeasured by vsect. 4 of those 8 checks were independent, and all 4 found something. That's too few to estimate a rate.

### 2.3 What it is worth carrying

As standing, very little (§1, §2.1).

As a source, a lot. Every check event records a `criterion` (what was claimed) and usually a `recheck` (how it was verified: the hand derivation, the simulation with its parameters, the citation). Look at the `deriv-sector-condition` trail: about 30 criteria with methods, covering the killed-moment bound, the exit-time benchmark, the Kalman cap and the numeric tables. That is a regression suite for the new store's first independent pass. It is better than a bare brief for confirming that mathematics survived crossing. It is worse for finding new defects, because criteria prime. Crossing them as owed re-checks keyed by slug would carry the knowledge without carrying lapsed state. The findings (313, 182 open) carry the same way, and the pilot is already re-targeting them.

## 3. What accreted: leave behind, copy, or carry as items

Sizes are **measured** with `find`, `wc -l`, `git diff --quiet` against `pre-integration-2026-10-30`, and vsect's own counts.

### 3.1 Leave behind: history that ASF keeps and the new store cites by path

| Thing | Size | Notes |
|---|---|---|
| `msc/inside-out-2026-10/` (the effort tree) | 153 files, 19,785 lines (md+py) | The tag-vs-now assessment said ~54k lines in 127 files on 10-31. It has changed or was counted differently; I did not reconcile. 179 open items cite it as source, and `kinds.yaml` `owners_from:` reads owners from its `INTENT.md`. A new store needs its own owner declaration, or it inherits KR ids, which Joseph has said can't own work. |
| `audits/AUDIT-WORKING-*` | 24 dirs, 1,410 files, 47,009 lines | Plus `audits/*.md`, 10,685 lines (14 `audit-findings-*` files and trackers), and `audits/.integrated` (86 files). |
| live spikes (not in `.integrated/`/`.archived/`) | 20 dirs + 37 top-level files; 264 files, 37,842 lines | `.integrated/` holds another 165 files, 43.5k lines. Several live spikes are named by open items as "held at root with payload not in canon" (sample §5). |
| lapsed checks | 336 confirmed + 112 broken/narrowed on revised text | Not crossable as standing (§1). Carry the criteria as re-checks (§2.3). |
| sprint-dated integration blocks in segments | 63 mentions of 2026-10-30/31 or 11-0x in segment files | Mostly in Working Notes. History, not canon. |
| `msc/` (other) | 45 entries, e.g. `summary-attempt/` 170 files 5.9k lines, `quiz-responses/` 2.1k, `meta-process-review-2026-07-07/` 2.1k, `reflections/` 1.6k | Provenance. |
| root trackers still present | `TODO.md` (504 lines; 71 of its lines imported as items, the rest not), `TST-IDEAS.md` (450), `PRACTICA.md` (122; 33 imported), `LOG.md` (frozen archaeology), `HISTORICAL-CONTEXT.md`, `README-auditor.md` | The tag-vs-now note lists thirteen trackers folded into the ledger. `TODO.md` and `PRACTICA.md` are still at the root, partly imported. |
| 189 items closed in the record but open in their files; 17 `close --by-commit` citations to unreachable commits | flow review, **measured** there | Commit-seam artifacts. Don't copy the files; cross from the record. |

### 3.2 Copy, or regenerate in the new store

| Thing | Size | Notes |
|---|---|---|
| segment bodies | 247 segments, 32,459 lines incl. frontmatter and Working Notes | 172 changed since the tag; 75 are byte-identical to it. |
| Working Notes inside segments | 5,759 lines in 238 segments, 18% of segment lines | Outside the hash (§1.2), so stripping them costs no evidence. Many open items were drained *from* Working Notes ("Source: … Working Notes"), so the notes now duplicate the ledger. If an item exists, its source note is residue. Notes without an item still need a disposition before they're dropped. That's the same delete-test as sources. |
| Claim Records in bodies | 24 segments | Being measured by the ASF lead's Line A. In the hash, so moving them lapses checks either way. |
| `depends:` graph | 247 nodes; 5 targets with no file (`def-cognitive-fusion`, `der-active-salience-management`, `der-self-referential-closure`, `form-structured-rich-context`, `obs-backward-inference-empathy`) | Planned rows in OUTLINE, per Fable's read. |
| generated front surfaces | `CURRENT-VOL1.md` (11.6k lines) and `-APPENDICES.md` (9.4k), FINDINGS, LEXICON, README, READMEs, from `bin/refresh-all` | Regenerate; don't copy. |
| `terminology/` | 343 files, 6k lines (177 entries, 152 decision dirs) | Content is canon-adjacent; worth a crossing decision of its own. |
| `ref/`, `02-tst-core/lit-review/`, `empirica/` | 297 files 32k lines; 49 files 15.5k; 6,736 files 203k lines (mostly the glyph-perception archive) | Reference and empirical material. The new store can cite them by path. |

### 3.3 Inbound references the crossing must keep resolving

Other repos cite ASF by slug far more than by path (**measured**: `grep` for `#(def|der|deriv|result|scope|form|hyp|disc|emp|obs|post)-…` and for `asf/0[1-4]-` paths):

| Repo | Slug references | Files with paths |
|---|---|---|
| vivarium | 2,881 | 3 |
| ops | 231 | 49 |
| proprium | 141 | 6 |
| logos | 72 | 14 |
| charter | 55 | 0 |
| neurips | 7 | 3 |

Keeping slugs stable matters more than keeping paths. ASF's published identity also lives in `.zenodo.json` and `CITATION.cff` (`v2-io/agentic-systems`, 2,079 commits since 2026-03-09). Whether aat-v3 is the same published work or a successor is a question for Joseph that the crossing will have to answer.

## 4. Which clusters can cross first, as-is (unknown 4)

**The rungs don't discriminate.** `kinds.yaml` declares `derivation: drafted → premises-checked → independently-rederived`. Computed from holding independent checks, three segments sit above "drafted":
- `scope-adaptive-system`, which is narrowed by check 8, with open findings;
- `def-action-transition`;
- `der-causal-hierarchy-requirement`.

167 of 247 segments have never had an independent check of any kind (**measured**).

**Dependency order is the real constraint.** A crossed segment's checks evaluate only once its whole closure has crossed (§1.2, last row). The Coupled Loop chapter (the five roots, `scope-agency`, `post-causal-structure`) is in every closure, so it crosses first whatever its maturity, and the pilot started there correctly. The closures of the 17 holding segments reach 106 segments: 95 in Volume 1, 7 in Volume 3, 4 in Volume 2.

**Check history, by OUTLINE chapter** (**measured**; open rows from `vsect open --format json`, independent checks from the trail, lapsed rows included):

| Cluster | Segs | Unchanged since tag | Open findings | Open questions | Lapsed | Items | Indep. checks / found |
|---|---|---|---|---|---|---|---|
| I · Coupled Loop | 7 | 1 | 9 | 6 | 53 | 52 | 53 / 12 |
| I · Reality Model | 5 | 2 | 2 | 2 | 2 | 31 | 2 / 1 |
| I · Cycle in Motion | 10 | 2 | 0 | 2 | 6 | 63 | 6 / 1 |
| I · Persistence and Limits | 13 | 5 | 3 | 9 | 18 | 92 | 11 / 4 |
| II · Meta-Architecture I | 8 | 2 | 9 | 7 | 11 | 58 | 3 / 0 |
| II · Lift to Purposeful State | 7 | 0 | 33 | 1 | 14 | 60 | 20 / 15 |
| II · Causal Access | 11 | 3 | 11 | 2 | 13 | 44 | 10 / 4 |
| II · Strategy Structure | 7 | 1 | 3 | 4 | 5 | 51 | 6 / 4 |
| II · Strategy Dynamics | 12 | 3 | 2 | 4 | 8 | 91 | 8 / 4 |
| II · Orient Cascade | 3 | 1 | 5 | 3 | 3 | 20 | 2 / 2 |
| III · Meta-Architecture II | 4 | 3 | 1 | 1 | 0 | 29 | 0 / 0 |
| III · Scope and Composition Formation | 3 | 1 | 1 | 1 | 0 | 18 | 0 / 0 |
| III · Composition Machinery | 7 | 3 | 8 | 1 | 2 | 61 | 2 / 1 |
| III · Unity, Communication | 7 | 5 | 3 | 7 | 5 | 39 | 5 / 4 |
| III · Cooperative/Adversarial | 6 | 2 | 2 | 3 | 7 | 30 | 5 / 2 |
| III · Strategic Composition | 6 | 0 | 5 | 2 | 8 | 32 | 3 / 0 |
| Vol 1 Appendices (Details) | 52 | 21 | 35 | 7 | 76 | 230 | 44 / 13 |
| Vol 1 Appendices (Domains) | 4 | 0 | 4 | 0 | 9 | 6 | 5 / 0 |
| TST · Foundations | 7 | 1 | 5 | 1 | 18 | 10 | 12 / 5 |
| TST · Developer Agent | 7 | 1 | 1 | 3 | 3 | 16 | 5 / 2 |
| TST · Code Structure | 7 | 2 | 3 | 3 | 29 | 5 | 22 / 10 |
| TST · System Measures | 8 | 5 | 0 | 0 | 2 | 14 | 2 / 1 |
| Vol 3 · Common Roots | 7 | 1 | 9 | 1 | 6 | 22 | 4 / 3 |
| Vol 3 · §I Primitive | 3 | 0 | 1 | 0 | 3 | 12 | 3 / 1 |
| Vol 3 · §II Scaffolded | 7 | 0 | 5 | 1 | 3 | 22 | 3 / 2 |
| Vol 3 · §III Closed-Loop | 6 | 4 | 1 | 0 | 0 | 17 | 0 / 0 |
| Vol 4 · Common Roots | 6 | 0 | 5 | 2 | 9 | 24 | 8 / 6 |
| Vol 4 · §1 Identity | 6 | 0 | 9 | 1 | 15 | 21 | 13 / 9 |
| Vol 4 · §2 Emergence | 3 | 3 | 1 | 0 | 0 | 5 | 0 / 0 |
| Vol 4 · §3 Deaths | 4 | 2 | 2 | 0 | 2 | 29 | 1 / 1 |
| Vol 4 · §4 Sovereignty | 3 | 1 | 0 | 1 | 6 | 9 | 6 / 5 |

(One Volume 1 segment has no OUTLINE row.)

Independent yield by declared type shows that the record doesn't follow the intuitive maturity order (**measured**, broken-or-narrowed over independent checks):

| Declared type | Yield |
|---|---|
| definition | 29/43 = 0.67 |
| hypothesis | 19/31 = 0.61 |
| derived | 21/40 = 0.53 |
| scope | 19/60 = 0.32 |
| derivation | 11/45 = 0.24 |
| result | 3/14 = 0.21 |

By declared status it is 0.30 for `exact`/`axiomatic` and 0.53–0.58 for `conditional`, `discussion-grade` and `robust-qualitative`. By volume it is 0.36 for Volume 1, 0.44 for Volume 2, 0.60 for Volume 3 (n=10) and 0.75 for Volume 4.

**My read, as judgment rather than measurement:**
- **The mathematics is the most settled.** Part I's derivation and result segments (persistence, sector condition, cycle in motion) have the most independent checking per segment and the lowest yields. Fable re-derived the central changes on 10-31. The record supports these as the clusters whose text can cross verbatim with the least expectation of rework. That doesn't make them first: they depend on the Coupled Loop roots, which go first anyway.
- **The definitions are the least settled.** The Coupled Loop and the Lift to Purposeful State carry the most open findings (9, and 33), and definitions draw findings at 0.67. In the record, the roots move. The pilot is crossing exactly these first, and carving them, so the pilot will meet the most findings per segment that exist anywhere. That is a good stress test of the new store's rules. It's a poor sample for estimating crossing cost elsewhere.
- **Quiet means unchecked, not settled.** 132 segments have no open finding or question; 114 of them have no independent check; 60 are byte-identical to the tag. Their statuses: conditional 42, discussion-grade 39, axiomatic 15, robust-qualitative 14, exact 10, sketch 8. These can be copied as-is because nothing open asks for a change. Expect a first independent check in the new store to find something on roughly half of them (**estimate**, from the 0.57 first-review yield, which was measured on sprint text, so treat it as a prior, not a prediction).

## 5. The open ledger: real work or residue? (unknown 5)

**Method.** I drew a seeded random sample (`random.seed(20261104)`) of 30 from the 632 open items, where open means not closed by `close` or a non-`holds` `recheck` event, and read each one.

**Result** (**estimate**, n=30):

| Category | Count | Examples |
|---|---|---|
| concrete owed fix to canon text | 7 | a contradiction landed on 10-30, stale chapter intros, a false "global" claim in an experiment line, non-forkability asserted at three tiers and derived nowhere |
| reader clarity or editorial | 6 | |
| research seeds or strengthenings | 11 | unification questions, hysteresis, step-shaped cognitive load, open formalizations lifted from Working Notes |
| unwritten planned segment | 2 | |
| build or typesetting | 3 | one of them partly stale: it names 23 instances "mostly in old-tst-* files", which have since been deleted |
| source to retire | 1 | a spike held at root with its payload not in canon |
| residue | about 1 | the stale part of the tooling item |

The rough 95% interval on residue is 0–17% (Wilson, 1 of 30).

The ledger has already been filtered: the item recheck removed 53% of imported tracker lines on 10-30 (flow review, **measured**). What remains is real and of mixed kind. The 4,339-line display is inflated by repetition (1,927 rows for 1,214 distinct things, **measured** from `vsect open --format json`) and by mixing kinds (1,321 item rows, 336 lapsed checks, 182 findings, 76 questions, 12 upstream-unread).

**For the crossing:** carry items and findings keyed by slug, typed at crossing (fix / clarity / seed / planned / build / retire), and leave the lapsed-check rows behind (§2.3). The research seeds are about 40% of the ledger, and they're the part a new store could reasonably route to a separate seeds record rather than owed work.

## 6. Flow numbers for the new store's routing rules (unknown 3)

These don't bear on the decision Joseph has made. They are priors for the routing rules aat-v3 will declare.

**Process-invariant entries.** All **measured** from the trail, by my own tally unless marked (flow review figures reproduced):

| Entry | Value |
|---|---|
| self-recorded re-derivation yield | 0/116 (all self checks: 3/222, all `run`/`read-check`) |
| independent yield, all | 114/266 = 0.43 |
| independent yield by the order of the review on that segment | 1st 58/102 = 0.57 · 2nd 38/75 = 0.51 · 3rd 10/44 = 0.23 · 4th+ 8/45 = 0.18 |
| independent reviews that found anything | 22/28 |
| confirm precision (not-equivalent among confirmed pairs) | 3/77 = 0.04 |
| revises per answered finding | ≤ 246/131 = 1.9 (an upper bound; not every revise answers a finding) |
| own-text lapses separated only by rewordings | 57/270 = 0.21 |
| checks on text unchanged since the tag | 8/488 |

The review-order row is new, and it qualifies the flow review's "flat yield" reading. Pooled by segment, independent yield falls by about two-thirds from the first or second review to the fourth or later. The flat lanes in `03-loops.md` are per lane and per round. The two readings are compatible if the later rounds were spread over segments that had already been reviewed. The text changed between reviews, so this curve mixes the defects being found out with the defects being made; the L1 experiment that `asf-flows.md` §10 names is still the thing that separates them.

**Recommendation for the rules.** Price two independent checks per new or redefined version (expected yield 0.57 then 0.51 on revised text). After that, only check again when the text changes by more than a rewording. For self-checks, record runs as reproducibility, and don't count them as derivation evidence. The flow review and the maturation plan already say this; the record supports it.

## 7. Unknown 2 is dropped: its premise doesn't hold

The corrections on main since the pre-sprint tag concerned a reset to the tag, which would have stranded them. A new store takes current main as its influx, so the corrections are in what crosses. The remaining risk is crossing fidelity, which the pilot measures directly. I wrote nothing on it before the correction arrived.

## 8. Found along the way

- **Strict holds hang on one root.** `def-agent-environment` is in the basis of every strict hold, so any edit to it, including the first carving tag, takes all 23 strict holds to zero (§1.2). In the new store, a root's carving is the textbook changeset case from `asf-flows.md` §6, and it arrives on day one.
- **`vsect read … | head` panics** with "failed printing to stdout: Broken pipe" (Rust, `stdio.rs:1165`). It's cosmetic, but agents pipe vsect into `head` constantly, and a panic message reads like a store fault. Reproduce with `vsect read segment:def-agent-environment | head -1` in aat-v3.
- **The new store's `owners_from:` question.** ASF declares owners via the effort tree's `INTENT.md`. Leaving that tree behind means aat-v3 needs its own owner declaration before items cross, or items arrive owned by KR ids.
- **`TODO.md` and `PRACTICA.md` are still live at ASF's root,** partly imported. If they cross as files, they'll duplicate items.
- **Fan-out wasn't recomputed by me.** Fable reproduced it on 11-03. My closure union for the 17 holding segments (106) doesn't contradict the roots reaching 230–245; I didn't compare them directly.

## 9. What I didn't get to, and how far the figures reach

- I read no segment body for correctness, and classified no finding by type.
- The ledger sample is 30 items, classified by me alone.
- The word-diff distances are a proxy for how far a check's subject moved, and they overstate it.
- I didn't recompute the L1 gain. Fable's figures (≈1.3 for the sprint, 0.6–0.8 at one check per landing) stand as they were.
- I didn't test importing ASF's event files into aat-v3 itself, only into a scratch store. Reading the source, nothing rejects a foreign `store:` field, and the scratch store showed the same verdicts under another name.
- I didn't measure the Claim Records; that's Line A's.
- The scripts are in `unknowns-scout-scripts/`, with a README saying what each computes. On 2026-11-04 the whole chain was rerun from an empty work directory and reproduced every figure here. That rerun also filled two §1.2 cells that the first pass hadn't counted.
