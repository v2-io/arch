# Sweep: Document/Standard Lifecycle Systems

Domain: formal standards-track and record-lifecycle ontologies — systems whose job is to say "how mature/current/trustworthy is this document" and to explain, structurally, what happens when a document stops being true, current, or in force. Gathered for verisectorium's atom-kind taxonomy work (TAXONOMY-FILL-v1.md, epistemic-map.md). Written for truth, not for convenience — includes what's inconvenient or unresolved.

---

## 1. IETF Standards Track (RFC 2026, amended by RFC 6410)

**(a) Structure, verified against RFC 6410 primary text (fetched):**

Pre-2011 (RFC 2026, 1996) had *three* standards-track maturity levels: Proposed Standard → Draft Standard → (Internet) Standard.

RFC 6410 (2011) collapsed this to **two**: **Proposed Standard → Internet Standard**. Verbatim from the RFC: "This maturity level is a merger of Draft Standard and Standard as specified in RFC 2026." The rationale given is not merely administrative simplification — it's an epistemic claim about what the levels were actually measuring: "widespread deployment is essentially the metric used for advancement from Draft Standard to Standard. The use of this same metric for advancement beyond Proposed Standard means that there is no longer a useful distinction between the top two tiers of the maturity ladder." I.e., they killed a rung because they discovered, empirically, that two rungs were measuring the same thing under different names — a genuine finding about their own ontology, not a re-org for its own sake.

Existing documents already at the abandoned Draft Standard level were **not** migrated automatically: "Any protocol or service that is currently at the abandoned Draft Standard maturity level will retain that classification, absent explicit actions." This is a real design choice — grandfathered documents keep an officially-deprecated-as-a-*category* status rather than being silently reclassified, which our taxonomy might want to think about: retiring a rung doesn't retroactively erase the atoms that were honestly placed there.

**Orthogonal, non-maturity categories** (not ranks — a document can be published directly into one without passing through the maturity ladder at all): Informational, Experimental, Best Current Practice (BCP).

**Historic** — a terminal status, explicitly and separately assigned (not a silent lapse), reachable from *any* prior maturity level including a Standard. The IESG issues a specific statement/pointer document justifying each Historic designation. This is a genuinely separate axis from the maturity ladder: "how mature was this" and "is it still the right thing to use" are different questions, and Historic answers the second one at any point on the first.

**Relationship metadata as first-class edges, not just a status field:** `Obsoletes` / `Obsoleted by` and `Updates` / `Updated by` headers let an RFC be superseded *piecemeal* (a later RFC updates one section) distinct from being wholesale retired (Historic). This is the single most structurally interesting feature in the whole sweep for us: supersession is not binary-current-or-not, it's a graph with typed edges, and partial supersession is a first-class, common case, not an edge case handled by prose.

**(b) What it's trying to be true about:** whether a technical specification has reached a state of consensus + demonstrated interoperable implementation sufficient that others should build on it as a fixed point. The ladder's underlying variable, per RFC 6410, is essentially "deployment evidence," though earlier RFC 2026 framing also weighted technical maturity and clarity of specification separately before the 2011 collapse suggested those had converged in practice.

**(c) Uncertainty/gap handling:** No explicit "unknown/uncertain" state in the ladder itself — a document simply sits at whatever level it has reached, and anything not yet a Proposed Standard is an Internet-Draft (expires after 6 months absent renewal — i.e., *drafts have a built-in decay clock*, an interesting default-to-death design rather than default-to-limbo). Gaps in implementation are surfaced via the "Implementation Report" required for advancement, not via a status label — evidence-for-promotion is a separate artifact from the state token itself.

**(d) Feature our atom-kinds would care about:** the maturity-axis / category-axis split, the typed supersession graph (Obsoletes vs Updates), Historic as an actively-justified terminal state reachable from any rung, and Internet-Drafts' default-expiry-not-default-limbo behavior.

**(e) Provenance:** IETF/IESG, formal standards body, extremely battle-tested (30+ years, governs the Internet's own specs). Verified via direct WebFetch of RFC 6410 primary text (quotes above are from the fetched document).
- https://datatracker.ietf.org/doc/html/rfc6410 (fetched, verified)
- https://datatracker.ietf.org/doc/html/rfc2026 (cited, not re-fetched this pass — recalled from prior search)
- https://en.wikipedia.org/wiki/Internet_Standard
- https://ietf.org/blog/iesg-statement-designating-rfcs-historic
- https://www.ietf.org/process/rfcs/

---

## 2. W3C Recommendation Track (W3C Process Document)

**(a) Structure, verified against the 2005 W3C Process tr.html (fetched):**

**WD (Working Draft) → CR (Candidate Recommendation) → PR (Proposed Recommendation) → REC (Recommendation)**, with **Working Group Note** as an explicit off-ramp: "Published by a chartered Working Group to indicate that work has ended on a particular topic" — i.e., a document can leave the track without ever becoming a standard, and that's a named, non-failure outcome, not silence.

Verbatim definitions confirmed by fetch:
- WD: "A document that W3C has published for review by the community, including W3C Members, the public, and other technical organizations."
- CR: "A document that W3C believes has been widely reviewed and satisfies the Working Group's technical requirements."
- PR: "A mature technical report that, after wide review for technical soundness and implementability, W3C has sent to the W3C Advisory Committee for final endorsement."
- REC: "A specification or set of guidelines that, after extensive consensus-building, has received the endorsement of W3C Members and the Director."
- Also present in this version: **Proposed Edited Recommendation** — "A Recommendation published for community review of changes, some of which may affect conformance" — i.e., an already-endorsed REC can re-enter review for a subset of changes without dropping all the way back to WD. A finer-grained "partial re-opening" mechanism than IETF's Obsoletes/Updates.

**Terminal decay, confirmed vs. unconfirmed:** **Rescinded Recommendation** is confirmed in this primary source, verbatim: "An entire Recommendation that W3C no longer endorses." I could **not** confirm "Obsolete Recommendation" as a formally adopted status in the process document I fetched (2005 snapshot) — the earlier search-derived finding claiming a clean Rescinded (wrong) vs. Obsolete (right-but-irrelevant) two-reason split is **only partially verified**. The W3C Process community-group issue-tracker link (w3process/track/issues/171) in the original findings appears to be a *discussion/proposal* about adding an Obsolete maturity level, not necessarily a ratified one — I did not fetch a version of the Process Document confirming Obsolete as a standing status, and flag this as **recalled, not verified**. Treat the "two distinct terminal-decay causes" claim as *plausible and worth citing as a discussion artifact*, but not as confirmed dual-status W3C policy the way Rescinded is confirmed.

**(b) What it's trying to be true about:** whether a web technology specification has achieved sufficiently wide implementer/community review and Membership consensus to be treated as an interoperability contract.

**(c) Uncertainty/gap handling:** CR explicitly exists to surface *not-yet-resolved* implementation gaps — its entire purpose is "we believe this is right, prove us wrong via implementation experience," and a CR can cycle back to an earlier CR or even WD if substantive issues surface (the Process Document permits looping backward, not just forward progression — this is an explicit non-monotonic ladder, unlike IETF's mostly-monotonic one).

**(d) Feature our atom-kinds would care about:** the WG Note off-ramp as a named non-failure exit; the Proposed Edited Recommendation as a partial-reopening mechanism finer-grained than full-Historic; the non-monotonic (can loop backward) ladder design; and the caution that not every plausible-sounding "clean split" (Rescinded vs Obsolete) survives contact with the primary source — a live example of exactly the verify-don't-recall discipline this sweep is supposed to enforce on itself.

**(e) Provenance:** W3C, formal standards consortium, decades of production use for HTML/CSS/etc. Verified by direct fetch of a primary Process Document snapshot (2005-10-14 version); the Rescinded definition is confirmed text from that fetch. Obsolete-status claim downgraded to unverified/recalled.
- https://www.w3.org/2005/10/Process-20051014/tr.html (fetched, verified — Rescinded confirmed; Obsolete NOT found in this version)
- https://www.w3.org/Consortium/Process/ProcessChanges (not fetched this pass)
- https://www.w3.org/community/w3process/track/issues/171 (discussion artifact, not confirmed policy)

---

## 3. ISO Harmonized Stage Code system (ISO/IEC Guide 69)

**(a) Structure — NOT independently re-verified this pass.** Both attempted fetches (iso.org/stage-codes.html — HTTP 403; the iteh.ai PDF mirror — returned corrupted/unreadable text) failed. What follows is carried forward from the prior search pass, **unverified against primary text in this session** — flagging explicitly per the steward's own discipline rather than asserting it with a confidence I don't have.

Recalled structure: a two-level numeric code `SS.SS` (main-stage.sub-stage). Main stages recalled as: 00 Preliminary → 10 Proposal → 20 Preparatory (WD) → 30 Committee (CD) → 40 Enquiry (DIS) → 50 Approval (FDIS) → 60 Publication, plus a maintenance loop (90 Review, 95 Withdrawal). Sub-stage digits recalled as event markers within a stage (`.00` registered, `.20` under review/ballot-open, `.60` close-of-main-action).

**(b)–(d):** unchanged from the prior pass's description — if accurate, the event-vs-status separation (state = stage number; event = sub-stage digit) would still be the standout structural feature for us. I am **not** confident enough in the recalled sub-stage semantics to present them as established without re-verification — a future pass should retry https://www.iso.org/stage-codes.html (403 may be a bot-block, worth trying via a different fetch path or the Wayback Machine) or locate a text-clean copy of ISO/IEC Guide 69:1999.

**(e) Provenance:** ISO, formal — but **confidence: recalled, not verified this session.** Downgrade accordingly; do not cite the specific sub-stage digit meanings as confirmed until re-checked.
- https://www.iso.org/stage-codes.html (403 — not fetched)
- https://cdn.standards.iteh.ai/samples/31383/885ae64e40cd49d6a19095fd3439ccfe/ISO-Guide-69-1999.pdf (fetched but unreadable/corrupted extraction)

---

## 4. ISO 15489-1 (Records Management — Concepts and Principles)

**(a) Structure — not independently fetched this pass; carried from prior search, flagged as recalled.** Lifecycle-of-custody model: creation → capture → maintenance/use → disposition, with an orthogonal **trustworthiness axis** defined by four properties: **authenticity, reliability, integrity, usability**. This four-property framing is widely cited in records-management secondary literature (archival science, DCC briefing papers) consistently enough that I hold moderate confidence in it despite not fetching the ISO text itself directly (ISO standards are paywalled, which is itself worth noting — unlike RFC/W3C, this primary source is not freely fetchable, a structural difference in how "battle-tested" translates into "independently verifiable by us").

**(b) What it's trying to be true about:** not "how mature is this document's content" but "can this record still be trusted to be what it claims to be" — i.e., custodial integrity over time, not epistemic consensus-building. This is a genuinely different *kind* of question from every other system in this sweep except possibly the Akoma Ntoso enacted-vs-revised split.

**(c) Uncertainty/gap handling:** disposition decisions (retain/destroy/ transfer) are themselves documented actions with stated authority and rationale — the standard is process-oriented (records of the record-keeping decisions) rather than a status-label system.

**(d) Feature our atom-kinds would care about:** the trustworthiness axis is a different *axis-type* than every maturity ladder in this sweep — relevant wherever our Accounts/References atoms care about "has this been tampered with / is the chain of custody intact" rather than "how vetted is this claim." Worth holding as a check against assuming all our atom-kinds want the same kind of axis.

**(e) Provenance:** ISO 15489-1:2016 (revision of 2001 original), formal, paywalled primary text — **confidence: recalled from secondary sources (DCC, CASRAI summaries), not verified against ISO primary text this session or prior.**
- https://en.wikipedia.org/wiki/ISO_15489
- https://www.dcc.ac.uk/guidance/briefing-papers/standards-watch-papers/iso-15489
- https://casrai.org/guides/iso-15489-records-management-concepts-principles

---

## 5. Wikipedia Content Assessment (article quality scale)

**(a) Structure, verified against Wikipedia:Content_assessment (fetched):**

Full ladder, highest to lowest, as actually listed: **FA (Featured Article), FL (Featured List), FM (Featured Media), A-Class, GA (Good Article), B-Class, C-Class, Start-Class, Stub-Class, List-Class.** Note this is *not* a strictly linear ladder — FA/FL/FM are parallel top-tier terminal grades for different content *types* (article vs list vs media) rather than one document progressing through all three, and List-Class sits as its own bottom-adjacent category for lists specifically rather than fitting the article-quality scale at all. My prior-pass summary ("Stub → Start → C → B → GA → A → FA") was a reasonable simplification but the actual fetched page shows more type-branching than that clean line suggests — worth being honest that even Wikipedia's own restatement is more tangled than the tidy version we'd want to cite.

**Grading-authority shift, confirmed verbatim:** "All editors, including editors who have written or improved an article, are encouraged to boldly set any quality rating that they believe is appropriate, except for the GA, FA, and A grades." GA requires "review... by an independent editor after a nomination"; FA requires review "by several editors"; A-Class "generally requires the agreement of at least two editors." So there are actually **three** distinct governance modes, not two as I'd summarized before: unilateral self-assessment (Stub/Start/C/B) → informal multi-editor consensus (A-Class, ≥2 editors) → formal independent nomination-and-review (GA, FA).

**(b) What it's trying to be true about:** an article's overall quality as a Wikipedia entry — a genuinely different target than the other systems here, which mostly measure "how ready/authoritative for external reliance," while this measures "how good is this piece of writing/coverage against a house style."

**(c) Uncertainty/gap handling:** no explicit uncertainty token; grades are periodically re-assessed by WikiProjects and can be downgraded (e.g., a GA can be delisted via Good Article Reassessment) — so it's a live, re-auditable status, not a one-way ratchet, which contrasts with IETF's mostly-monotonic ladder.

**(d) Feature our atom-kinds would care about:** rung-dependent epistemic authority-of-the-grader as an explicit, three-tier design (self → informal consensus → formal review), and the fact that even the "content quality" axis is itself internally scored on three separate sub-dimensions (prose, sourcing/technical rigor, coverage) collapsed into one letter — a real example of a two-or-more-axis reality being flattened to one rung for usability, worth treating as a cautionary structural precedent given our own open debate about one-axis-vs-two design.

**(e) Provenance:** Wikipedia community/WikiProject-governed — **not** a formal institution, lowest provenance-weight in this sweep, but a live, actively-audited, huge-scale, non-institutional grading system, which is itself a data point (mature quality-ladders don't require institutional backing to develop real governance-mode gradients). Verified via direct WebFetch of the primary page (quotes above are from the fetched document).
- https://en.wikipedia.org/wiki/Wikipedia:Content_assessment (fetched, verified)
- https://en.wikipedia.org/wiki/Wikipedia:Assessing_articles (not re-fetched)

---

## 6. Wikidata Statement Ranks + Qualifiers

**(a) Structure, verified against Help:Ranking (fetched):**

Exactly three ranks, confirmed verbatim:
- **Preferred:** "The preferred rank is assigned to the most current statement or statements that best represent consensus." Multiple statements *can* simultaneously hold Preferred rank (e.g., several co-existing "best" values for different contexts) — it's not a single winner-take-all slot.
- **Normal:** "The normal rank is assigned to all statements by default. A normal rank provides no judgement or evaluation of a value's accuracy and currency." Explicitly neutral, not "unreviewed" or "provisional" — the documentation is careful to say normal implies *no judgment*, not *pending* judgment. Worth noting for our own ladders: a "default/normal" state that carries zero epistemic content, vs. a "default" state that silently implies "not yet checked," are different designs, and Wikidata picked the former deliberately.
- **Deprecated:** "The deprecated rank is used for statements that are known to include errors (i.e. data produced by flawed measurement processes, inaccurate statements) or that represent outdated knowledge."

**Mandatory-by-convention justification qualifier:** "It is often useful to indicate the reason for a deprecation with a reason for deprecated rank (P2241) qualifier" — a specific, named property whose entire job is to require (by strong convention, not hard schema enforcement — "often useful," not "required") a *why* attached to the *deprecated* rank specifically. (Note my prior-pass finding also claimed a parallel "reason for preferred rank" (P7452) qualifier convention; I did not re-verify that specific property in this fetch, so hold that half at recalled-not-reverified confidence — the Deprecated-rank+P2241 pairing is the part directly confirmed by the fetch.)

**Deprecated statements are retained, not deleted** — confirmed by the fetch's caveat: "All statements, including deprecated ones, must be verifiable in the sense that a source makes the claim whether or not the claim is true." This is an explicit, stated design principle: *verifiability of "someone claimed this," not truth of the claim itself*, is the bar for keeping a statement in the graph at all — deprecation marks known-wrongness/outdatedness while the record of the claim having been made persists.

**(b) What it's trying to be true about:** which value, among possibly several sourced/conflicting values for the same property, should be treated as current-best for a given real-world fact, while preserving the historical and erroneous record rather than erasing it.

**(c) Uncertainty/gap handling:** ranks are a coarse 3-value scale, but the real uncertainty-handling machinery in Wikidata is elsewhere — in the statement's *references* (source citations) and *qualifiers* (e.g., point-in-time, applies-to-part) rather than in rank itself. Rank answers "how should conflicting values be prioritized," not "how confident are we" — those are different questions Wikidata deliberately keeps in different structural slots (rank vs. qualifiers vs. references), which is itself a notable design lesson: don't let one field try to answer both "which value wins" and "how sure are we."

**(d) Feature our atom-kinds would care about:** the small-alphabet (3-value) rank + mandatory-convention justification-qualifier pattern; the explicit "retain known-wrong statements, don't delete" principle; and the deliberate separation of *prioritization* (rank) from *confidence/sourcing* (references/qualifiers) into different structural slots rather than conflating them into one status field.

**(e) Provenance:** Wikimedia/Wikidata, part of the formal Wikibase data model, extremely battle-tested at huge scale (100M+ items). Verified via direct WebFetch of Help:Ranking (quotes above are from the fetched document); the P7452 "reason for preferred rank" half is recalled, not re-verified this pass.
- https://www.wikidata.org/wiki/Help:Ranking (fetched, verified)
- https://www.wikidata.org/wiki/Wikidata:Data_model (not re-fetched)
- https://www.wikidata.org/wiki/Property:P2241 (not re-fetched — P2241 existence/definition recalled, consistent with the fetched Help:Ranking text referencing it by name)

---

## 7. Legal instrument lifecycle (Akoma Ntoso / legislation.gov.uk)

**Not independently re-verified this pass** — carried forward from the prior search as the weakest-provenance entry, flagged as such originally and still flagged as such. Recalled structure: FRBR-style Work/Expression/ Manifestation/Item stack, with legislation.gov.uk layering an enacted-immutable vs. continuously-revised-expression split on top, plus "I-note" annotations for unresolvable in-force/commencement edge cases.

If accurate, the enacted-vs-revised split (immutable original + a maintained living projection incorporating amendments) is a strong structural precedent for keeping an atom's *original assertion* distinct from its *currently-in- force interpretation* — relevant to Norms/Decisions. But this entire entry should be treated as **recalled, not verified**, until a primary source is actually fetched; I did not attempt to this pass, given time, and it was already the weakest entry going in.
- https://en.wikipedia.org/wiki/Legislation.gov.uk
- https://cdn.nationalarchives.gov.uk/documents/cas-82049-legislation-date.pdf
- https://www.legislation.gov.uk/understanding-legislation

---

## Ranking, revised after verification

1. **IETF (RFC 6410 + Obsoletes/Updates/Historic)** — still the strongest entry, and now *more* confident after direct verification, including the detail (newly surfaced, not in the prior pass) that RFC 6410's own stated rationale for collapsing the ladder was an epistemic discovery ("no longer a useful distinction") — a real precedent for *why* a ladder rung might legitimately get retired, not just administrative tidying.

2. **Wikidata rank + P2241** — confirmed strong; the newly-surfaced nuance (rank answers "which value wins," not "how confident are we" — that's qualifiers/references' job) is a sharper and more useful finding than the prior pass had, and directly warns against conflating prioritization with confidence in our own atom design.

3. **W3C Rescinded vs. Obsolete** — **downgraded**. Rescinded is confirmed; Obsolete is not confirmed as ratified policy in the primary text I could reach, only as a community discussion artifact. The "clean two-cause terminal-decay split" from the prior pass should not be cited to Joseph as an established W3C structure without further verification — this is exactly the kind of plausible-sounding-but-unverified claim the steward asked this sweep to catch.

4. **ISO Guide 69 stage codes** — **downgraded to recalled-only**; both fetch attempts failed. The event-vs-status structural idea may still be worth exploring on its own merits, but should not be attributed to ISO Guide 69 with confidence until someone gets primary-source access (paywalled/blocked in this session).

5. **Wikipedia content assessment** — confirmed and *sharpened*: three governance tiers (not two), and the ladder is more branched (article/list/media-parallel) than a clean line — worth passing that correction along rather than the tidier but less accurate prior version.

6. **ISO 15489** and **Akoma Ntoso** — both remain recalled-only; ISO 15489 is structurally the most distinct (custodial-trust axis, not epistemic- maturity axis) and worth flagging to Joseph as worth a dedicated verification pass given how different its target question is from everything else here.

---

## What surprised me / what we did not ask about

**Surprise 1 — the IETF's own reason for retiring a rung was an empirical finding about their ontology, not administrative convenience.** RFC 6410 didn't just simplify for simplicity's sake; it stated that two rungs had converged to measuring the same underlying variable (deployment evidence) and that keeping them separate was no longer *meaningful*, not just inconvenient. That's a much stronger and more interesting justification pattern than "we wanted fewer states" — it's "we discovered our states weren't actually distinguishing anything real." Worth asking: does our own ladder have any rungs that, examined honestly, are measuring the same underlying thing under two names? This seems like a genuinely useful diagnostic question to carry into the taxonomy work, not just a historical curiosity.

**Surprise 2 — how much of the "clean structural story" collapses under direct verification.** The W3C Rescinded/Obsolete two-cause split — which read as one of the most directly useful findings in the original search pass — did not survive a primary-source check. This isn't a minor detail; it's a first-hand demonstration, inside this very task, of exactly the plausibility-vs-verification failure mode the steward's global instructions warn about. I want to flag explicitly: **the original search-pass findings document I was handed already had the shape of "confident structural claims that feel inevitable"** — several of its claims (ISO stage codes' sub-stage semantics, the W3C Obsolete status, Akoma Ntoso's structure, the P7452 qualifier) turned out to be unverifiable or unverified within this session's tool access, and I think it's more honest to say that plainly than to let the confident prose stand uncorrected just because it "sounds right" and matches a plausible narrative about how these systems probably work.

**Surprise 3 — rank and confidence are different structural slots in Wikidata, and conflating them would be a design mistake.** This wasn't explicit in the original findings at all — it emerged from actually reading the primary text. "Which value should win when several conflict" and "how sure are we this is true" are genuinely different questions, and Wikidata's architecture keeps them in different fields (rank vs. references/qualifiers) rather than trying to make one status token answer both. Given that verisectorium's whole premise is atoms carrying honest epistemic state, this seems like a load-bearing distinction worth surfacing hard to the coordinator: **does our current taxonomy conflate "current-best-among- alternatives" with "how confident/verified"?** If so, that's a structural gap this sweep found, not just a nice-to-have precedent.

**Surprise 4 — every system in this sweep handles "gap/uncertainty" by routing it to a *different* structural location than the main status field** (IETF: implementation reports, separate from level; W3C: CR's whole purpose is surfacing gaps, and it can loop backward; Wikidata: references/qualifiers, separate from rank; Wikipedia: reassessment as a live, repeatable process, not a one-way state). **None of the six verified systems put "we don't know" or "confidence" directly into the same slot as "current lifecycle state."** If our working epistemic-map ladders are putting maturity-status and confidence-in-that-status into one combined rung, every verified system in this sweep disagrees with that design, independently and by convergence — that seems like the single most load-bearing finding of this whole pass, and it wasn't explicitly named as a finding in the original search-findings document at all; it only became visible by laying the six verified systems side by side.

**What we did not ask about, and probably should:** none of these are software-artifact-lifecycle systems (semver, deprecation-warning conventions in package ecosystems, CVE lifecycle) or scientific/medical evidence-grading systems (GRADE, Cochrane risk-of-bias, systematic-review PRISMA states) — both of which are extremely well-established, heavily used ontologies for "how much should you trust this claim and why," arguably closer in *purpose* to verisectorium's Assertions family than any document-lifecycle system here is. This sweep's frame (given to me as "document lifecycle") pulled toward standards-track and records-management systems and away from evidence-grading systems, which may be the more directly relevant family for Assertions/Accounts specifically. Worth a dedicated follow-up sweep rather than treating this document as covering "how established institutions grade truth-claims" — it covers "how established institutions grade *documents*," which is a related but distinct question.
