---
slug: name-collision-across-stores
type: emp
depends:
  - collision-staleness-detection
---

# Collision detection runs inside a store and is blind between stores

*Two statements about one thing collide, and that is how staleness surfaces. One name over four different things in four different stores collides with nothing — no mechanism in the estate can see it, and the only thing keeping them apart is a hand-written note.*

## The claim

[[collision-staleness-detection]] rests on an asymmetry that works: present-truth bodies can contradict each other, so a stale statement eventually meets a fresh one and the contradiction is the signal. That mechanism has a precondition nobody states — **the two statements have to be found together.** Within one verisectorium they are, because identity is the filing system: same slug, same place, statements adjacent by construction.

Across stores, identity is exactly what is missing. A name coined in one corpus and a name coined in another are not two statements about one thing; they are **one name over two things**, and no amount of present-truth discipline inside either store makes them meet. The failure is the dual of the one collision solves, and it is invisible to the same machinery:

|  | same identity, different statements | same name, different identities |
|---|---|---|
| Where it happens | inside a store | between stores |
| What surfaces it | the statements are adjacent; contradiction is visible | nothing — the referents never meet |
| Current estate answer | integration-is-replacement, present-truth bodies | a hand-written index, if anyone noticed |

**The consequence is not confusion in the moment — it is confusion at a distance.** Nobody inside comproprium is unclear about what `vera/` means. The cost lands on the next agent, arriving from a third project, reading a note that says "the VERA material" — and on any tooling that would resolve a reference by name.

## The specimen

Verified first-hand, 2026-08-05, by reading each primary. **Four live things in this estate carry the name "vera," and they are related enough to be confusable and distinct enough that conflating them would be an error:**

| Form | What it actually is | Live home |
|---|---|---|
| **VERA**, a PROPRIUM component | A named slot in the ELI architecture for factual belief held with explicit uncertainty — a component *name* with a brief mechanism sketch, not a designed subsystem | `~/src/arch/proprium/INGEST/msc-from-harness/canonical/PROPRIUM-{ONTOLOGY,ARCHITECTURE}-v2.md` |
| **`vera/`**, a comproprium directory | One of three segment directories, holding **precepts** — claims about methodology, split from `praxes/` and `exempla/` because the three fail differently | `~/src/arch/proprium/comproprium/vera/` (12 segments) |
| **VERA architecture**, ennaos | A Nov 2025 design-research specification for a neuro-symbolic epistemological architecture — substantial, and abandoned in place | `~/src/_core/ennaos/docs/research/vera/` |
| **vox-vera**, zoetica | Reference material on verbal-probability calibration and Bayesian knowledge graphs | `~/src/_core/zoetica/docs/refs/vox-vera/` |

They share a theme — truth held under uncertainty — which is why they were named alike and why the collision is easy to make. They are not versions of each other: no cross-reference exists between the PROPRIUM v2 documents and the comproprium precepts in either direction, and the ennaos specification is not ratified PROPRIUM law.

**Two facts make this more than a curiosity.** First, the collision was only kept straight because a gathering agent wrote a four-row table by hand and put it at the top of a note; nothing detected it, and nothing will detect the next one. Second, the estate already knows it has this problem and has recorded it as unresolved rather than fixed: comproprium's own README lists among its open questions, verbatim, that **"Collision with symbolic PROPRIUM / firmatum ontology remains a naming caution, not a layout blocker."** That is a *fifth* collision, on the neighbouring name, recorded by the people living in it — as an open question with no instrument proposed.

## What would actually catch it

The ch. 3 gap row on terminology substores already names the instrument: per-term records with **bounded-context sharing across the estate**. Bounded context is the load-bearing half for this failure specifically — a term store that only knows one corpus reproduces the problem one level up. What is needed is the ability to ask *what else in this estate answers to this name*, which is a query no current instance can serve, and which would have produced this table automatically.

Two boundaries worth keeping honest. Homonymy across genuinely separate contexts is **not itself a defect** — natural language runs on it, and forcing globally unique names across an estate would be its own pathology. What is a defect is homonymy that nothing can *report*. And this is one specimen family, found because someone went looking at one name; it is not a measured rate, and it says nothing about how often this happens estate-wide.

## Strength & grounds

**Empirical for the specimen, heuristic for the mechanism.** The four referents were each verified by opening the primaries listed above on 2026-08-05, and the absence of cross-reference between strands 1 and 2 was checked rather than assumed. The comproprium README quotation is verbatim from its live `README.md` §Open. The general claim — that collision machinery is structurally blind to the between-store case — is an argument from how the mechanism works, not a measurement; it would be defeated by any instance that does detect cross-store name reuse, and none is known.

An incidental second finding from the same reading, worth recording because it bears on a different segment: `~/src/arch/asf/terminology/entries/proprium-mapping.md` — a canonical terminology entry — points readers at `~/src/firmatum/PROPRIUM-ONTOLOGY-v2.md` and `~/src/firmatum/PROPRIUM-ARCHITECTURE-v2.md`. **Neither file exists; `~/src/firmatum/` does not exist**, the tree having moved under `~/src/arch/`. The same entry's ASF-internal reference is identity-shaped (`#def-proprium-mapping`) and resolves correctly. One file, two reference designs, opposite outcomes — an independent live replication of [[provenance-rot-specimen]], in a different corpus, from a different move, still broken today.

## Working Notes

- The repair for the incidental finding is a one-line path fix, but the interesting question is why nothing reported it: asf has a link checker; whatever it checks, it does not check prose paths into other trees. Routes to ch. 14's instruments-going-blind concern.
- Open, and genuinely: whether *aliases* are the right primitive here (one term record, several surface names) or whether these four want to stay four records with an explicit `do-not-confuse` relation. The asf terminology schema already ships a `do_not_confuse` field, which is evidence for the second reading and worth examining before designing anything.
- Deliberate decision not to split this into claim + appendix specimen, against the usual practice here: the verification load is a four-row table and two greps, and a companion segment carrying that would be ceremony rather than structure. The split earns itself when the evidence is too heavy for the claim to stay readable — which is the actual test — and it is not yet. **If a second collision specimen lands, split then**, and this row becomes the readable claim over both.
- This segment is deliberately narrow. The wider question — how concept coherence is maintained across members of a program — is the ch. 2 estate-layer gap, and should not be pre-empted from one specimen.
- Intake artifact behind this segment, back in `plan/INFLUX/vera/00-INDEX.md` (returned in the 2026-08-06 delete-test reversion) and **not** to be cited as warrant: its hand-written collision table is the thing this segment is *about*. That table's content is superseded by the table above; the live paths listed are what the body stands on.
