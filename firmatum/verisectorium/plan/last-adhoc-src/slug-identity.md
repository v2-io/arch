---
slug: slug-identity
type: form
depends:
  - atom
  - identity-regime-archaeology
  - provenance-rot-specimen
---

# Identity is the slug, and the slug carries nothing else

*An atom's identity names the thing and only the thing — never where it sits, what number it has, or when it is read. Everything that moves for exposition reasons is kept out of the identity, which is why an identity survives moves that break everything else.*

## The claim

The invariant is small and, as far as this pattern has a proven core, it is the proven part: **one stable name per atom, carrying no order, no path, and no presentation number.** Ordering lives in a view ([[outline-as-organizing-principle]]); references carry the identity rather than a location ([[identities-over-locations]]).

**Why the separation holds.** Identity and position change under different authorities, at different rates, for unrelated reasons. Truth-status moves when evidence moves. Position moves when exposition improves — when a chapter is resequenced, a section is promoted, an appendix is introduced, a directory is tidied. Fuse them and every ordinary exposition improvement becomes a rename cascade, and every ordinary reorganization becomes a silent break in every reference that encoded the old arrangement. The asymmetry is what does the work: housekeeping is frequent, low-attention, and touches many records at once; deliberate re-identification is rare, high-attention, and touches one.

**The failure is silent, which is what makes it expensive.** A location-bearing reference does not fail at the move — the move is clean, the commit message is mundane, nothing complains. It fails later, at the moment someone needs it, and it fails for everything at once. In the clearest measured instance the estate has, one routine tidying commit broke nearly every path-anchored provenance span in a corpus whose one guarantee those spans *were*, and disabled the checker that existed to catch exactly that — while every slug reference in the same corpus, over the same move, still resolved. Two reference designs, one ordinary event, opposite outcomes. Full account and counts: [[provenance-rot-specimen]].

**The estate has tried the alternatives and left them, and the evidence is still on disk.** ASF's TST component holds three identity regimes physically side by side — positional numbers, then typed labels, then slugs — and the migration between them cost a rewrite of filename, frontmatter, heading and every citation each time, because the identity had been carried in all four ([[identity-regime-archaeology]]). The corpus's own format rule states the reasoning in one line: *"Canonical ordering lives in each component's `OUTLINE.md`, not in filenames. The ordering will change as the theory develops; the slug is the stable identity"* (`~/src/arch/asf/FORMAT.md`).

**A slug is exact, not a prefix.** Where identities are used as references, the discipline that makes them work is that they resolve wholly or not at all: a truncation or a prefix is a dangling reference wearing a short form's clothes — vivarium's format states this outright, after finding that its lexicon references dangled at zero while its decision references dangled, *"because only one of the two had a stated form to lint against"* (`~/src/arch/vivarium/FORMAT.md` §5.1). An identity scheme with no stated form is an identity scheme with no instrument.

**The honest competitors.** Two designs give up the invariant deliberately and are worth stating rather than dismissing.

*Order in the filename.* Two live paper projects in this estate name sections `00-…`, `01-…` and let the build glob them in sorted order, so there is no manifest to maintain at all — cheap and legible for a fixed venue skeleton whose order is settled by convention. The cost is paid on insertion (a new section between two becomes `04a-`) and on reordering, which becomes a rename. This is not a mistake in that setting; it is the right trade where the ordering will not change, which is exactly the condition a living corpus fails.

*Pure graph, no view.* If identity is separate from order, one might ask why an ordering has to be stored at all — let the dependency graph be the structure. The answer is not about identity but about readers: a cold-start reader needs *some* exposition order, the dependency graph does not supply one, and a corpus that refuses to store one makes every arriving mind invent its own ([[outline-as-organizing-principle]]). And the graph is not universally available: not every corpus's claims form a partial order at all — vivarium says outright that it *"does not have that ordering, and it may never want the same gating"* — so ordering-by-dependency cannot be the shared answer even in principle.

**What this does not give.** Stability is not survival. This claim says an identity does not change for housekeeping reasons; it says nothing about what happens when an identity legitimately *must* change — a rename, a merge of two records found to be the same claim, a split. That needs an alias mechanism, and it is a separate thing the estate has built exactly once (registered as N1/R48 in `plan/TODO.md`; the ch. 1 gap row on restatements, families and aliases is its home). Nor does the claim settle what a good slug *is* — subject-noun conventions, role prefixes, length — which is deployment-local.

## Strength & grounds

**Heuristic, but the best-evidenced claim in this corpus.** It rests on three legs of different kinds: a measured natural experiment where two reference designs met the same ordinary event and one survived intact ([[provenance-rot-specimen]], whose counts were re-run first-hand at its drafting on 2026-08-05); a physical archaeology of two abandoned alternatives with their migration costs visible ([[identity-regime-archaeology]], counted first-hand 2026-08-06); and the convergent statement of the rule across independently-grown instances, read first-hand on 2026-08-06 — ASF's *"the ordering will change as the theory develops; the slug is the stable identity"* (`FORMAT.md`), vivarium's *"one claim per file… filename is the slug; no numbering in filenames"* (`FORMAT.md` §2), and the udon-needs corpus, whose format document does not state the rule but whose practice follows it (slug-named files under `02-tooling-needs/src/`, ordering in `OUTLINE.md`, and a stated reference form that is a bare slug with no path and no suffix).

The legs are not independent in the way that would license a stronger label. All three sit inside one estate under one steward, so their agreement shows a stable intent rather than corroborated design ([[turnover-solution]] makes the same caution about the same evidence base). The specimen leg is the strongest because it is not testimony at all — it is one event's differential effect on two designs that were both present — but it is a single event in one young corpus, and it shows path-anchors failing rather than slugs succeeding at anything hard. What would raise this: a differently-authored corpus reporting the same failure asymmetry, or any corpus that adopted positional identity deliberately and reported the reorganization cost it actually paid.

## Working Notes

- The competing-designs material is registered as R24 and N3 in `plan/TODO.md`; the exact-slug discipline is N4. The order-in-filename instances were verified live on 2026-08-06 (`~/src/behavioral-floor/src/`, `~/src/causal-language/paper/src/` — both `NN-name.md` series with no manifest).
- Open, and not answered by anything in the estate: whether a slug should be *pronounceable to the domain* or merely unique. Every live instance chose subject-noun slugs; nobody has reported what that buys over an opaque key, and relata — the instance with the most sophisticated identity machinery — uses citation keys instead.
- The prospective twin of provenance addressing is registered as N21: an identity-anchored address (slug + section + span words) serves both directions — proving a quoted span still exists, and proving a reader read it. The vivarium orientation quiz already uses the second; the first is [[provenance-rot-specimen]]'s unchosen repair.
