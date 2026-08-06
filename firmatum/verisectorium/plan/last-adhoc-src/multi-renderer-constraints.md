---
slug: multi-renderer-constraints
type: form
depends:
  - build-forced-commitments
---

# A second renderer constrains the source

*The moment one atom set feeds two renderers, the notation an author may use narrows to what both accept — and that constraint has to be stated as authoring law, because it is invisible from inside either renderer.*

## The claim

Presentation-neutral records are the point of the whole arrangement: write the claim once, project it into whatever form a reader needs. But neutrality is a property of the *notation*, not an intention. A source is presentation-neutral only to the extent that its markup lies in the intersection of every target renderer's capability — and that intersection is usually smaller than the capability of the renderer the author happens to be looking at.

This produces a class of rules that feel arbitrary to a newcomer and are not: use this one math delimiter, not the four equivalent ones; use this single block form rather than the environment your typesetter prefers; do not reach for the package that only one target has. Each such rule is the shadow of a second consumer. The characteristic failure is silent and delayed: the work compiles and looks right in the primary renderer, and breaks in the secondary one at the moment it matters — which, since secondary renderers tend to be the ones facing outward, is usually a submission or a publication.

Three practical consequences:

**State the intersection, not just the preference.** A rule recorded as a style preference gets relaxed by the next careful author who sees no reason for it. A rule recorded with its cause — *this exact form is required because the same text also feeds that renderer* — survives, and updates correctly when the target set changes.

**Adding a render target is a substrate decision, not a build task.** It can retroactively invalidate authoring across the whole corpus. It deserves to be decided where notation law is decided.

**Keep the shared projection under single ownership.** Where a shared preamble, template, or style layer serves many documents, letting individual authors extend it locally reintroduces exactly the divergence the shared layer exists to prevent. The workable arrangement observed is that authors own content and route capability requests to whoever owns the projection.

## Strength & grounds

**A practice claim with one sharp live instance**, from the 2026-08-05 survey ([[build-forced-commitments]]).

The instance is neurips, where the authoring rules state of single-dollar inline math that *"This is a hard requirement, not a preference"* — with the reason given plainly: the same segment text feeds both the LaTeX build and the OpenReview abstract submission, which recognizes only that form, and *"Authoring discipline that breaks abstract submission breaks the workflow."* The same document narrows block math to one shape (`$$…$$`, with `aligned` inside for multi-line) and rules out the several equivalent LaTeX environments, so the converter never has to decide whether it is already in math mode. The ownership half is in the same document: a missing package or environment goes to the build-pipeline owner rather than being injected from a segment, explicitly for cross-paper consistency.

The limits are worth being plain about. This is one workspace with one genuinely external second renderer; ASF, which targets two LaTeX classes from one pipeline, does not name a comparable notation constraint — either because two similar targets impose little, or because the constraint has not yet been discovered. So the mechanism is demonstrated once, and its generality is inferred from why it happens rather than from a second case. The prediction it makes is testable and unfulfilled: a corpus that adds a genuinely dissimilar renderer (HTML, EPUB, a search index) should discover new constraints of exactly this shape, and if it does not, the claim is weaker than it looks.

## Working Notes

- The canonicalization side of the same concern is a render-equality gate — a formatter that re-parses its own output and refuses to write when the rendered document would change. It is the same intersection problem approached from the tooling end rather than the authoring end; worth pairing whenever the lint-and-canonicalization mechanics get drafted.
- Open and unmeasured: whether the intersection can be *checked* rather than taught. A linter that knew the target set could report violations instead of relying on authors remembering the reason.
- Related: [[substrate-independence]] (the principles survive the technology; this segment is about what the technology charges for that), [[derived-vs-authored]].
