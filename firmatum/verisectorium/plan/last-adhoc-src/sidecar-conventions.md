---
slug: sidecar-conventions
type: form
depends:
  - basename-manifestation-survey
  - identities-over-locations
---

# A store should be callable by one name while its shape matures underneath

*The identity discipline that keeps references from rotting applies one level up, to stores as well as records: name the operational set, not the file it currently happens to be — because the file is exactly what changes as the set outgrows each form.*

## The claim

A corpus's operational surfaces do not stay one shape. A todo list starts as a markdown file with mixed conventions, gets converted to something schema-bearing, splits by subject when one file becomes unwieldy, and eventually becomes a directory with one record per file. The same trajectory runs for lexicons, changelogs, intake queues, decision logs. Each step is an improvement and each step **renames the thing on disk**.

That is the same asymmetry [[identities-over-locations]] identifies at record grain, and it has the same repair. Every reference to the store — in prose, in briefs, in tooling, in conversation — should name the **store**, not its current manifestation. "The project's TODO" resolves to whatever shape it has reached; `TODO.md` resolves to a file that will stop existing.

**The load-bearing rule is the union rule, and it is the one that is easy to skip.** If a base name names an operational set, then every manifestation carrying that base name is *part of the same set*, not a sibling set. Two files named for one base name are two parts of one thing. The practical consequence runs backwards from that: a genuinely separate concern needs a **separate base name**, because there is no other way to say "these are different things" once they share one. A store that is partitioned and a store that has been forked look identical on disk and are distinguished only by the name.

**The maturation path matters less than the invariant across it.** The forms — top-level file, transitional single file, typed single file, directory-as-store, partitioned set — are a plausible ordering, and nothing requires an instance to walk them in order or at all. What the convention actually buys is that *walking them is invisible to callers*: a store can be promoted from a file to a directory without touching a single reference, provided the references never named the file. An instance that pays this cost gets its restructurings for free; an instance that does not pays a rename sweep per promotion, and rename sweeps are where references quietly break.

**Sidecars are their own base names.** Alongside a store there is usually a companion set: the processed items, the archived items, the audit trail, the payload directory. The proposal is that these carry their own base name (`.BASENAME-side`) rather than being unnamed appendages — because a sidecar accretes structure exactly the way its parent does, and an unnamed set cannot be referred to, partitioned, or promoted. It leaves the main-to-sidecar linkage unspecified, and in practice the estate's linkage is *positional*: the sidecar sits inside or beside the thing it serves. That is not obviously wrong — containment is not a reference, and a directory that moves takes its children with it — but it does mean a sidecar cannot be relocated independently of its parent, which is the constraint nobody has had to test yet.

## The half that governs the non-atom files

A closely related question has a better-evidenced answer, and it is worth keeping in the same segment because it is the same instinct: **what governs the surfaces that are not claims?**

Two rules are stated, independently, in two live corpora:

*Convention is inherited by reference, never restated.* One instance's format document declares itself divergences-only — *"The conventions here are ported by reference, not restated… Restating it here would fork it — an inlined definition is not a copy, and there is nothing holding two copies in step"* — and then names what is carried unchanged, so nobody has to diff. Its sibling states the same rule from the failure side: a document that restated a definition by hand thereby used a term the dictionary had retired four days earlier, invented two access channels that do not exist, and dropped two that do.

*And the format rules bind every file, not only the atoms.* The same corpus splits its format law explicitly: the early sections govern claim segments, the later ones — cross-references, prose voice, notation — govern *every file in the repository*, with the exemption named and refused: *"'It is only a working document' is not an exemption."* This is not pedantry about memos. It is the recognition that a corpus's references and vocabulary are corpus-wide properties, so a surface exempted from them becomes the place where drift enters and is not caught.

## Strength & grounds

**A proposal, honestly labelled as one, plus two better-supported inherited rules.** The base-name convention is Joseph's own working brainstorm (`~/src/arch/notes/NORMS.md`) — provisional throughout, carrying unresolved questions in its body and an open TODO to name the concept at all. It is cited here **as a proposal to measure against**, and the measurement is not flattering to adoption: exactly one estate surface has reached the convention's compliant form, one is transitional, three trees reached directory-as-store by other routes without the naming, and the deprecated form carries essentially everything else ([[basename-manifestation-survey]]). So the trajectory is real and observed; the *naming discipline* is not in force anywhere and has therefore never been tested by a promotion.

The union rule's cost is the one thing here with live evidence: the partition affix runs in both directions within one estate (`FORMAT-TODO` versus `TODO-META`), which under the union rule means the two name partitions of *different* base names while being spoken of identically. That is an observed ambiguity, not a hypothetical one — and it is also only an ambiguity, with no recorded damage from it yet.

The inheritance-by-reference rule is stronger: stated in two corpora, with a recorded failure behind the second (a hand-restated definition going stale in four days). The everything-binds rule is one corpus's stated law with the same incident as its warrant, so the two rules share a specimen and should not be counted as two independent supports.

## Working Notes

- The falsifier for the naming half is cheap and nobody has run it: promote one store from a file to a directory and count what breaks. If nothing breaks under the current filename-referencing practice, the convention is buying less than it claims here and more of the value sits in tooling that does not exist yet.
- Open, and genuinely undecided: whether positional linkage of a sidecar to its parent is a violation of [[identities-over-locations]] or an exemption from it. The norm's own boundary — sometimes a path *is* the honest anchor — plausibly covers containment; nobody has stated the rule.
- Not carried: what belongs *inside* a sidecar at atom grain ([[working-notes-sidecar]]) and how intake sidecars are typed ([[influx-queues]]). This segment is about naming and governing the sets, not their contents.
- Watch: this project's own surfaces are `.un` — a sixth form the convention does not list, glossed in the source as Udon Notation. Whether that is a stage on the path or a divergence is undecided, and this instance is the datum.
