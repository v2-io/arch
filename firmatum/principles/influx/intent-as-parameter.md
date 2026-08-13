---
slug: intent-as-parameter
type: demand
register: evidenced
support-kind: [design, observational, theoretic]
strength: robust-qualitative   # a lived origin, a shipped mechanism, and a formal slot agree on direction
convergent: [design, observational]   # theoretic merges with design as one estate leg; the shipped mechanism is the independent leg
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: both
depends: [errors-that-teach, freshness-and-atomicity, the-crystallized-process-thesis]
sources:
  - ../../01-ideation/02-provenanced/copies/II2-zoetica-ennaos/addendum-intent-driven-tooling-and-semantic-storage.md  # §1–2 read
  - ../../01-ideation/02-provenanced/copies/II4-autopax-practica/2025-11-17-intent-surfacing.md  # read whole
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C2b
  - ../../01-ideation/02-provenanced/syntheses/CONVERGENCES.md  # cluster 7
---

# Intent as a first-class tool parameter

**Claim.** A tool call should carry *why*, not only *what* — because the mechanical operation underdetermines the semantic act, and every layer that could help (repair, learning, audit, handoff) needs the act, not the keystrokes. The demand has a lived origin, a shipped mechanism, and a formal slot.

## The origin case (October 2025 — worth seeing)

An agent adding citations to a report performed about fifteen find-and-replace edits, then audited its own session. The semantic intent had been "add a citation to this quote, keep the numbering consistent, track progress"; the mechanical operations had been "insert `[^tag]` at a text position and hope the document matches my mental model." When an edit failed —

```text
Error: String to replace not found in file
```

— the error was, in the author's words, a *phenomenological revelation*: the file had structure the agent hadn't comprehended, the mental model was stale, and the whole session had run at the wrong abstraction level — "my intent was semantic, my tools were syntactic." The design conclusion drawn on the spot: tool calls should carry **two levels of intent** — the immediate act ("add a citation to this uncited quote") and the higher-order purpose ("comprehensive source attribution, per this style guide") — because that pair is what lets a tool choose the right abstraction, repair intelligently, track progress against the *goal*, and reveal its own gaps: when the stated intent was add-a-citation but the tool used was find-and-replace, that mismatch, logged, is a request for a better tool that nobody had to file.

A companion body of design work generalizes the idea: three levels of intent (immediate / design / strategic) and a survey of every channel intent must survive — in code, in commit messages, in decision records, across sessions ("what I was trying to achieve / why I made these choices / what I didn't finish"), across agents (explicit intent-handoff markers in shared documents), and between human and agent, where the stated request and the *real* purpose behind it are famously not the same thing. Its summary line: intent is "the most valuable and most easily lost information."

## The shipped mechanism, and the formal slot

One harness made intent load-bearing in production: gemini-cli's edit tool *requires* an `instruction` field — why, where, what, desired outcome — and when every tier of its string-matching tolerance fails to place an edit, a second model call repairs the edit *from the stated intent*. It is the only shipped repair layer that recovers the semantic act after the mechanical anchor breaks: exactly the origin case's thesis, running in production. Carried intent converts an unrecoverable mechanical failure into a repairable semantic one.

The theory supplies the slot. Plans whose intermediate steps are observable turn blame-assignment from an intractable inference problem into bookkeeping — and a stated intent is precisely the observable that lets a refusal distinguish "your anchor broke" from "your goal is unachievable here." The [[errors-that-teach| refusal chapter]] requires that distinction; a bare old-text/new-text pair cannot support it.

## What it generates

- **For the harness:** an intent field on mutating tools is cheap and compounds — it powers repair (the shipped mechanism above), audit trails that answer *why*, and the tool-gap-detection loop (intent-vs-tool mismatches are the roadmap for what to crystallize next — the [[the-crystallized-process-thesis| crystallized-process chapter]]'s request channel, fed  
  automatically). One caution from the same design work that proposed it: intent *comments* drift and lie — its own anti-pattern list names intent-hiding, intent-drift, and false intent. Prefer intent carried on *operations and events*, which are dated and immutable, over intent as decoration in the artifact.
- **For UDON:** the edit tool's operations (the [[schema-guarded-mutation| guarded-mutation chapter]]) should carry intent from day one; and the document-side residue of executed intents is exactly the annotation layer (the [[annotation-and-metacognition| annotation chapter]]) — one demand seen from the call side and the artifact side.

## What this opens (ideas, not designs)

- ✦ **Intent as the verification anchor.** The counter-register's hardest row says validation cannot catch plausible wrongness — a well-formed edit that does the wrong thing sails through every schema. A *stated intent* gives a verifier, human or machine, something to check the diff *against*: "does this change do what it says it was for?" is answerable; "is this change right?" in a vacuum is not. Intent might be the cheapest purchase available against exactly the failure class nothing else catches.
- ✦ **History queryable by purpose.** If operations carry intent, an audit log becomes searchable semantically: every change whose purpose mentioned the deadline, every edit made in service of the migration. Commit messages half-deliver this today at commit granularity; operation-level intent would deliver it exactly.
- ✦ **Intent chains across delegation.** When an agent delegates, its sub-agent's tool calls could carry the *parent's* intent alongside their own — a provenance chain of purpose. "Which top-level goal caused this edit?" becomes answerable across any depth of delegation (the [[delegation-as-tooling| delegation chapter]]'s briefing discipline, extended down into the tool layer).
- ✦ **Pricing the burden.** The honest tension below — authoring cost per call — is measurable: repair-yield and audit-value per character of required intent, against completion drag. The mandatory-intent scope (mutations only? high-stakes only?) could be set from data rather than taste.

## Honest edges

The design-side legs share an author; the genuinely independent leg is the one shipped mechanism (one harness — a single occurrence, flagged as one). No one has measured how often carried intent actually rescues failed edits compared to string-tolerance alone; that harness's telemetry would answer it and is not public. And there is a real tension to hold: intent fields add authoring burden to every call — the ease gradient cuts against mandatory intent on low-stakes operations, so scope it to mutations.
