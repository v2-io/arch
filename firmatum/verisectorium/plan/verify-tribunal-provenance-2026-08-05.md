# Adversarial verification — tribunal batch, 2026-08-05

Verifier note up front: I read `tribunal-revisited-2026-07-29.md` in full, all six other INFLUX files, `ONTOLOGY.un` in full, `identities-over-locations.md` + `provenance-rot-specimen.md` (the exemplar pair), and re-ran or independently re-derived every checkable first-hand number and path in the seven drafts. I did not find a fabricated citation, a laundered training-recall item, or a misattributed Joseph quote. The register-preservation work (item 1 in the brief) is genuinely careful — better than most such work I see. What I found instead is one real structural defect (undeclared dependencies that the ontology's own rule requires to be declared) and a handful of smaller things worth your attention. Verdicts below are blunt as requested.

## Overall verdicts

| File | Verdict |
|---|---|
| `tribunal-strand-survey.md` | **Sound** |
| `rationale-capture-survey.md` | **Sound** |
| `authority-flag-specimen.md` | **Sound** |
| `decision-records.md` | **Needs a fix** — undeclared dependency |
| `warrant-over-authority.md` | **Sound, one thing to consider** |
| `asked-and-answered.md` | **Sound** |
| `tribunal-record.md` | **Needs a fix** — undeclared dependency |

## 1. The one real defect: `depends:` frontmatter doesn't match what the bodies actually lean on

`ONTOLOGY.un` §`segment[expected-shape]` is explicit: `depends:` lists what a segment "genuinely stands on (definitional imports, logical antecedents, **grounding specimens**)... never mere mentions." I checked every wikilink in every body (not Working Notes — the ontology's carve-out is for prose mentions, and Working Notes read as exactly that register throughout your seven) against this test, and found two that cross the line.

**`decision-records.md`**, frontmatter declares `depends: [rationale-capture-survey]` only. But the body's "adoption constraint" section reads:

> "Two pieces of evidence sit behind that, both from this estate and both weak in the same way: vivarium's council already writes decision-record embryos in its ledger unprompted, and an agent there was observed maintaining provenance eagerly at thirty-second granularity ([[authority-flag-specimen]])."

That's not a see-also — `authority-flag-specimen` is one of the two named pieces of evidence the claim ("the escape is a lean, not a finding") is built on. It's a grounding specimen by the ontology's own definition. It belongs in `depends:`.

**`tribunal-record.md`**, frontmatter declares `depends: [tribunal-strand-survey, rationale-capture-survey]`. But:

> "The verdict is a summary; the record is the asset. What the record buys is stated in [[decision-records]]: a decision that knows its legs absorbs a refutation locally instead of defending itself totally."

This is tribunal-record borrowing decision-records' central payoff claim as the justification for its own "the record is the asset" sentence — a logical antecedent, not a mention. Missing from `depends:`.

Why this matters for the reason you named up front: both of these are exactly the "silently depends on a note that's about to be filed away" shape you're worried about, just one hop removed — the influx-not-warrant discipline is well-executed (I checked every one of the "do not cite influx copies" Working Notes lines against the live paths they name, all resolve), but the *segment-to-segment* dependency declaration, which is the mechanism that would let a build tool or a future editor know these two files can't be safely reordered or read standalone, is under-declared. Not a citation-hygiene problem — a DAG-accuracy problem.

I did not find the reverse problem (a declared dependency that turns out to be a mere mention) anywhere in the seven.

## 2. Literature-register preservation — checked claim by claim, holds up

I compared every literature claim in `rationale-capture-survey.md` and in the "What the literature adds" material re-quoted in `decision-records.md` and `tribunal-record.md` against the source's own `[sweep-verified]` / `[training recall]` marks in `tribunal-revisited-2026-07-29.md`'s bibliography. Specifically checked:

- IBIS/gIBIS/QOC lineage (1970/1988/1991) — correctly carried as **training-recall** in `rationale-capture-survey.md` ("*all three attributions are training-recall*"). Source marks all three `[training recall]`. Match.
- Regli 2000 survey — correctly carried as sweep-verified with the DOI intact.
- van Vliet & Tang 2016, ISO 42010 (with the 2022-supersession caveat correctly kept as training-recall) — match.
- SEURAT cluster (Burge & Brown, DCC'04/SEURAT tool/JSS08/dissertation) — correctly carried as sweep-verified, with the JSS08 paywall caveat and the "claims verified via DCC'04 + dissertation" qualifier preserved verbatim in spirit.
- The two explicitly-excluded SEURAT details (CLIPS-rules claim refuted 0-3; "unanswered questions" syntactic check unverified) — both carried into `rationale-capture-survey.md` correctly, and *not* smuggled into `decision-records.md`'s SEURAT summary (which just says "typed rationale with working inference," true and unhedged, not overclaiming the excluded bits).
- MADR/ZEUS, log4brains, structured-MADR, e-ADR — correctly sweep-verified.
- Buckingham Shum 2006 "spectre" phrase — correctly sweep-verified, quoted correctly.
- The four-neighborhoods-unwalked caveat and the LegalRuleML-as-likely-prior-art line — carried into **both** `rationale-capture-survey.md` and `decision-records.md`, correctly scoped both times ("not found in what was walked," not "does not exist"). This is the item you were most worried about and it's the one you got most right — I couldn't find a single sentence anywhere in the seven files that states or implies novelty at a strength stronger than the source's own hedge.

One near-miss worth flagging even though it doesn't cross the line: `tribunal-record.md`'s closing "Strength & grounds" says "the durability half is argued, the capture-escape is unproven, and none of it has outside-authorship corroboration" — this is your own honesty-register language, correctly not attributed to the sweep at all. Good instinct to keep it separate; just noting it as a place where a less careful pass could have accidentally cited the sweep for something the sweep never claimed.

I did not find anywhere that "a sweep verified this" got laundered into "this is verified" — the register markers consistently stay attached to the specific claim, not to the paragraph.

## 3. Joseph quotations — verbatim-checked against the source, all located

I pulled every string in quotation marks attributed to Joseph across all seven files and located each one in `tribunal-revisited-2026-07-29.md` character-for-character (allowing for your own italics/bold markup, which doesn't change the quoted text):

- "it gives sound 'intentional vs incidental' *with* reasoning..." — §1, exact.
- "…it's either capture by design or likely-to-fail capture-manually..." — §6, exact.
- "rather than resting on authority — it's a question of what serves truth and the core..." — §6, exact.
- "they cared so much about the provenance correctness that they forgot for a moment..." — Working Notes, exact.
- "has clearly come up a few times by various agents already" — §6, exact (correctly kept as a fragment, not over-quoted).
- "probably council before we had that as an option, more or less" — §6, exact.
- "I suspect this may be one of the original 'udon core schemas'..." — §5 — **not actually used** in any of the seven drafts I read (it's in the source but none of your seven quote it; noting only because I checked for it and it's absent, not a problem).

Provenance-disclosure check (your item 2, "did I say I didn't consult transcripts everywhere it matters"): `authority-flag-specimen.md`'s "Register and scope" section states this explicitly and strongly ("inherited from the participant's own same-evening write-up... not re-derived from the vivarium transcript... the specific ledger entry was not located by slug at drafting"). `warrant-over-authority.md` and `decision-records.md` both quote the same material but don't restate the transcript-provenance caveat themselves — I don't think they need to, since both declare `depends: [authority-flag-specimen]` (well, `decision-records.md` doesn't currently declare it — see §1 above) and the caveat lives at the specimen's root, which is the right place for it once the dependency is honest. `tribunal-record.md` quotes none of the §6 material directly, so it's moot there.

## 4. Your first-hand numbers — independently re-run, all match exactly

- **vivarium ledger**: `grep -c '^|decision' ~/src/arch/vivarium/DECISIONS.decision-log.udon` → **160**. `wc -l` → **1697**. Both drafts (`rationale-capture-survey.md`, `authority-flag-specimen.md`) claim exactly these numbers. Match, independently re-run, not just re-read.
- **autopax ADR system**: the path `~/src/MOVED/autopax/docs/ADR/README.md` exists and is readable. Every specific claim you make about it checks out against the file text: the DRAFT↔EXPLORING→PROPOSED→{ACCEPTED,REJECTED} state machine, ACCEPTED→{SUSPENDED,SUPERSEDED}, the `+EXECUTED`/`+AMENDED` orthogonal flags (including the `REJECTED+EXECUTED` example, which is in the source verbatim), state-keyed mutability, the five status groups, `blocked_by`/`needed_for` vs `related` as the two distinguished edge kinds, and session-count estimation (with your "Trivial: 0.3–0.5 sessions" etc. register — the source's actual bands are 0.3–0.5 / 1 / 2–3 / 4–6 / 10–20, matching what you wrote). One caveat you already flagged yourself and correctly: the tree is under `MOVED/` now, and your Working Notes say so. Good.
- **comproprium provenance-rot specimen** (`provenance-rot-specimen.md`, not one of your seven but load-bearing background for the exemplar-pair standard you're being measured against): I actually ran `bin/check-corpus` myself rather than trusting the file. Output: **57 segments · 3/109 quoted spans located · 106 fail · 0 warn · 5 forward refs** — exact match to the table in the segment. I also independently grepped for `:see #` slug-style references in that corpus and got **18**, matching the "18/18 survived" claim. And `git show a40825f` confirms the commit is real, dated 2026-08-01, titled "Reorganize various ingest queues." This wasn't one of your seven, but it's the standard-bearer you're citing yourselves against, so I checked it at the same rigor — it's exactly as solid as it claims to be, which is useful data for calibrating how much slack your own drafts should be given.

I found zero numeric or path errors in the first-hand-claimed material. This is the best-verified category of the whole batch.

## 5. Strand-collapse check (`tribunal-strand-survey.md` and its downstream citers)

I read all five source documents for the four strands (plus the fifth "not a strand" document) and checked every place downstream segments cite "the tribunal" against which strand they're actually drawing on:

- `decision-records.md` and `tribunal-record.md` both draw exclusively on strand C (governance-as-record) for their substantive content, and both correctly attribute the "intentional vs incidental" Joseph quote and the role vocabulary to that strand alone — no bleed from A's Bayesian-confidence machinery or B's internal-aspects framing into C's claims.
- `tribunal-record.md`'s "thin enforceable form" section correctly keeps strand D (Gate-2) as a *distinct* rung rather than a scaled-down version of C — it says "they share a name and a motive, not a design," which is the right relationship and matches what I found reading `gate2-probes-discussion.md` directly (Gate-2's three probes are genuinely a different mechanism from C's five-voice council; there's no adjudication node, no confidence field, no revisit-when in Gate-2 at all).
- I checked specifically for the collapse risk the survey itself calls out — strand A's "Institutional Analyst" (game-theoretic incentive analysis) versus strand C's "neutral observer" (bias-about-both-teams) getting treated as the same role. `tribunal-record.md`'s role list keeps "neutral observer" and doesn't import Institutional Analyst's incentive-analysis framing into it. Correctly kept apart.
- One place worth a second look, not a defect exactly: `tribunal-strand-survey.md` says of strand A's Institutional Analyst that it approaches bias "from a different direction (game-theoretic analysis of a source's incentives)" — this is your own gloss, and strand A's README (which I read) actually describes Institutional Analyst as "Meta-reasoning and bias detection" without explicitly using game-theoretic language; the *technical analysis* doc might be where "game-theoretic" comes from more precisely, and I didn't read that one in full. Low stakes — the substantive point (Analyst's failure mode differs from Observer's) survives regardless — but if you want the phrase "game-theoretic" to be load-bearing rather than color, it's worth a quick check against `ref-project-TECHNICAL_ANALYSIS.md` before it ships further.

No collapse found across strands. This is the finding I'd have expected to be hardest to get right and it's clean.

## 6. Overclaiming relative to the `identities-over-locations`/`provenance-rot-specimen` standard

Measuring your seven against that pair's discipline (readable claim, honest strength statement, "anecdote not a rate" language used correctly):

- All seven "Strength & grounds" sections (where present — `tribunal-strand-survey.md` and `rationale-capture-survey.md` are survey-type and don't carry one, which is consistent with the ontology's type list treating survey as appendix-grade rather than a claim needing a strength statement) explicitly name what is *not* established, not just what is. `decision-records.md`'s "no independent-authorship corroboration exists for the mechanism working in practice" and `warrant-over-authority.md`'s "has **not** been demonstrated; it is the obvious prediction of the same mechanism and nothing more" are exactly the register `provenance-rot-specimen.md` models ("an anecdote, not a rate").
- `authority-flag-specimen.md`'s "Register and scope" section is, if anything, stricter than the exemplar pair requires — it explicitly says the specimen was "not re-derived from the vivarium transcript, and the specific ledger entry was not located by slug." That's a real, checkable admission of a gap you could have papered over, and I confirmed it's true: I went looking for the specific `:by us` decision entry from 2026-07-12 in the vivarium ledger to see if it's locatable and didn't find an obvious way to identify it by content alone from what's in your drafts (no slug given) — so your own "not located by slug" caveat is accurate, not modest-sounding cover.
- The one place I'd push back slightly: `tribunal-record.md`'s framing "the most sophisticated system in the field... died" (about SEURAT, echoed from the source) is stated as settled fact rather than "the sweep's characterization of." It's a fair compression of what the source says, and the source itself states it with that confidence (citing Buckingham Shum's "spectre" language as sweep-verified) — so this is inherited confidence, not manufactured confidence, and I don't think it needs a fix. Flagging only because it's the single sentence in the batch that reads most like an unqualified verdict, and you said overclaiming is what you most want caught.

I did not find any sentence across the seven stated at a strength the cited evidence doesn't carry, beyond the near-miss noted above.

## Things outside your numbered worries, found along the way

- **`provenance-rot-specimen.md` and `identities-over-locations.md` themselves** (not in your seven, but the standard you're citing yourselves against) both check out at the same rigor applied above — see §4. Since these two are the exemplar pair the whole project measures against, it seemed worth confirming they actually earn that status rather than assuming it.
- **The `00-INDEX.md` reading-order file** in INFLUX describes `zoetica-tribunal-xml-template.md` and `collaborative-cognition-synthesis.md` as part of the recommended reading order, but none of your seven drafts cite either one. That's plausibly fine — `tribunal-strand-survey.md` folds the collaborative-cognition doc in as "a fifth document... not mistaken for a fifth design" and the zoetica template is presumably subsumed into strand B's live-source citation — but worth a conscious check before the INFLUX copies are archived: is there content in either file that isn't represented anywhere in your seven, that would be lost? I skimmed `zoetica-tribunal-xml-template.md` and it looks like a process template with no content beyond what `discovery-internal-architecture-2025-09-21.md` (strand B's live source) already carries, so I don't think you're losing anything — but I didn't do a line-by-line diff.
- **A framing question, not a defect**: `warrant-over-authority.md`'s Working Notes flags an adjacent unhomed concept ("proxy discipline... It has no row on this outline yet and probably needs one"). That's an honest gap-naming, exactly the kind of thing the ontology wants surfaced rather than silently absorbed into this segment's scope. Nothing to fix, just noting it's the right move and you should make sure it doesn't get lost when INFLUX archives.
- **Minor**: `tribunal-strand-survey.md`'s table lists strand A's live source as `~/src/_ref/epistemic_tribunal/` (README + `src/`) — I confirmed the directory exists but its structure is `config/` and `documents/` at the top level, not a visible `src/` at that depth from the `ls` I ran; there may be a `src/` one level down that I didn't check for since your Working Notes already correctly say "the implementation... was not read." Worth a 10-second `ls` before this ships if the `src/` pointer needs to resolve for someone later, but you've already hedged this one correctly by not claiming to have read it.

## On the brief itself

Framing was right and I didn't reframe it. One thing I'd push back on gently: you asked me to check "every literature claim... against the source's own register marks," and I did, but I want to name that this is really a two-hop check (source→your-survey, then your-survey→your-downstream-segments) and I did both hops — worth knowing so you don't re-run hop one alone thinking hop two is still open.

I'm glad to stay available if you want me to go read `ref-project-TECHNICAL_ANALYSIS.md` for the game-theoretic-language question in §5, or to do a line-by-line diff of the two INFLUX files that aren't cited anywhere (§"Things outside your numbered worries," bullet 2), or anything else that comes up once you've acted on the two dependency fixes.
