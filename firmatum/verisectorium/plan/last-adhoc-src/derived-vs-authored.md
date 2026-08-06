---
slug: derived-vs-authored
type: form
depends:
  - build-forced-commitments
---

# Derived artifacts should be unmistakably derived

*Anything a build produces should announce that fact by construction — in where it lives and what it is called — because the failure mode is not that someone edits a generated file, it is that the edit works for a while.*

## The claim

A corpus that projects views produces artifacts: assembled documents, extracted bibliographies, generated lexicons, indexes. Each of them looks exactly like a source file, and a future agent has no way to tell the difference from the inside. The cost of that ambiguity is asymmetric: editing a source file is normal work, while editing a derived file produces an improvement that is real, is used, and disappears at the next build — often long after the person who made it has gone.

The rule that follows is that **derivedness is declared by construction, not by hoping**. The mechanisms are unglamorous and they compose:

- **Location** — derived output lives in a directory that exists for that purpose and is not tracked, so its status is visible from the path alone.
- **Naming** — where a derived file must sit beside sources, the name says so, deliberately and even a little awkwardly.
- **Banners** — a generated file states in its own first lines that it is generated and where its source is.
- **Clobber guards** — the generator refuses to overwrite something it did not produce, so an accident surfaces as an error rather than a loss.

There is a fourth failure this discipline addresses, and it is the subtler one: **a derived file that the build has stopped reading**. It keeps working as documentation, keeps being edited, and is now a fork of the source that nothing reconciles. When a pipeline stops consuming a file, the honest moves are to delete it or to rename it into visible obsolescence — not to leave it inert and correct-looking.

The same reasoning runs the other way for a *view*: an outline or manifest is authored, not derived, and carries content that exists nowhere else. Generated views can be thrown away and rebuilt; authored ones cannot, which is why the two must not be stored so similarly that a cleanup treats them alike.

## Strength & grounds

**A practice claim with one worked live instance and a documented near-miss**, from the 2026-08-05 survey ([[build-forced-commitments]]).

The instance is the neurips 2026-05-06 build refactor, which exists substantially to make derivedness legible: build output moved into per-manifest `.build/<stem>/` directories that are ignored by version control, and the per-paper bibliography became `<paper>/<stem>.extracted.bib`, of which the change record says the *"naming is explicit-on-purpose so it's clear by construction that it's a build artifact."* The near-miss is in the same record: the previous `<paper>/refs.bib` was a hand-editable file that the build had stopped reading, and the migration guidance is explicit that per-paper agents *"shouldn't have been hand-editing this file in the first place"* — the boundary was being crossed routinely while the file still looked authoritative. Elsewhere in the estate the same problem is met with different instruments: ASF gitignores its build directory, heads its generated lexicon with a banner (*"Auto-generated from `terminology/entries/` by `bin/term render`. Do not hand-edit."*), and carries a clobber guard in the terminology tool that refuses to overwrite a file it did not produce. Its generated README, by contrast, carries no banner at all — the same estate applies the discipline unevenly across its own derived files.

What this does *not* establish is that any one mechanism is sufficient or that the set is complete. Every instance surveyed uses convention — a name, a path, a banner — and none uses a *declaration* that a tool could check; there is no instance of "this file states its own derivedness in a form the build verifies." So the claim to carry forward is that convention has repeatedly proven necessary and has repeatedly been the thing that slipped, not that convention is the right final answer. And the instances are not independent of one another: the same steward and closely related agents built all of them, so their agreement shows a practice that survived repeated contact with the problem, not several parties discovering it separately.

## Working Notes

- The unbuilt piece worth naming: a machine-checkable derivedness declaration, so that "is this file authored?" is answerable without knowing the pipeline. Every current answer is a human-readable convention.
- Open: whether an assembled artifact that is *committed on purpose* (for citability, or so reviewers can diff it) should be treated differently from build scratch. The estate does both without distinguishing them.
- Related: [[selection-and-projection]] (a projection's output is derived by definition), [[write-safety]] (a clobber guard is the single-writer discipline applied to generated files).
