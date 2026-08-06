<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/02-tooling-needs/src/freshness-and-atomicity.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

---
slug: freshness-and-atomicity
type: principle
register: evidenced
support-kind: [observational, testimonial, design, theoretic]
strength: robust-qualitative   # three invariants recur across every kind of evidence touching writing
convergent: [observational, testimonial, design]   # theoretic merges with design as one estate leg; read-gate is universal in shipped practice, collision is lived
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: both
depends: [edit-representation-landscape]
sources:
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C6, hashline singleton, kimi ToolAccesses
  - ../../01-ideation/02-provenanced/commentary/spikes/paths-NOTES.md  # D5, D6
  - ../../01-ideation/02-provenanced/syntheses/CONVERGENCES.md  # cluster 14 (multi-writer)
  - ../../01-ideation/02-provenanced/commentary/spikes/agent-utility-NOTES.md  # P-C
---

# Freshness and atomicity: edits bind to the file as it is *now*

**Claim.** Three related invariants recur across every kind of evidence that touches writing, and together they form the transaction discipline of agent mutation: (1) **read before edit**, as a hard, code-enforced gate — an edit is only meaningful against a state the agent has actually observed; (2) **re-resolve at apply time** — addresses resolve against the current file at the moment of writing, never trusted from cache; (3) **atomic multi-site application** — a batch of changes either fully applies or fully refuses.

## The evidence

- **Universal in shipping practice, with one especially sharp formulation.** The read-before-edit gate is enforced *in code* — a typed error, dedicated modules — across at least eight of the fourteen harnesses examined: a hard invariant, not a prompt suggestion. The sharpest shipped statement is the hash-anchored editor from the [[edit-representation-landscape| edit-landscape chapter]]: its anchors are valid only for the file state at read time, and **one stale anchor rejects the whole batch** — freshness and atomicity as a single mechanism. One other harness lifts the same concern to its scheduler: every tool call declares the files it will read and write, and only non-conflicting calls run concurrently.
- **Lived.** An agent's first-person account of the multi-writer wound — two agents editing the same document, each blind to the other — plus a working convention that grew up in response (a lightweight "soft-claim" note staking a file before touching it). Concurrent multi-agent editing is a present condition, not a future scenario.
- **Designed.** The 2025–26 design work states the re-resolution rule verbatim ("paths re-resolve at write time; never trust cache over file"), and [[addressing-exploration| the addressing exploration]] hardens it into a composition rule: path evaluation for a whole patch runs against the *pre-patch* tree, compare-and-swap style, so a batch of edits shares one consistent addressing frame instead of each edit shifting the ground under the next.
- **Derived.** The [[errors-that-teach| refusal chapter]] already established that a failed operation must not half-apply — atomicity is what keeps the lesson of a refusal unconfounded. And the theory's analysis of forked, divergent copies of one artifact gives the deep version of the multi-writer problem: two writers over one document are two histories sharing a prefix, and reconciling them takes an explicit protocol, not hope.

## What it generates

- **For UDON's edit substrate:** the transaction in the [[schema-guarded-mutation| guarded-mutation chapter]] inherits all three invariants, and the failure vocabulary must keep **stale** (the file changed since you read it) distinct from **not found** and **not unique** — the repairs differ: re-read, versus re-derive the address, versus disambiguate. One early design idea remains above all of this as a known missing piece: structure-aware *merge*, with annotations accumulating rather than colliding — carried here as an open demand, not designed.
- **For the harness:** enforce the read gate in code, never prose; consider declared-footprint concurrency once multi-agent sessions are the norm; and surface staleness as its own teaching refusal — name the hypothesis to the agent ("the file may have changed since you read it") instead of letting a mysterious mismatch teach nothing.

## What this opens (ideas, not designs)

- ✦ **Freshness as a token, not a behavior.** Every read could return a freshness token (a content hash, a version mark); every write could require one. The read gate stops being a rule agents follow and becomes a fact of the interface — un-forgettable by construction, and the hash-anchored editor's insight generalized from lines to any addressable thing.
- ✦ **Staleness scoped to the target.** Text-level freshness invalidates on *any* change to the file. Structural addressing makes a finer question possible: did the *addressed subtree* change? A file whose unrelated sections moved on could still accept a guarded edit whose target is untouched — fewer false staleness refusals, which matters exactly in the multi-writer world where the file is always changing somewhere.
- ✦ **Claims as leases.** The soft-claim convention formalizes naturally: an advisory reservation over a path, with a lifetime, visible to every other writer. Whether it lives in the document, beside it, or in the tool layer is a design question; that agents already invented the manual version is the demand evidence.

## Honest edges

Atomic *multi-file* transactions remain the gap nobody fills — the hash-anchored batch stops at one file. And nothing in the evidence yet prices optimistic against pessimistic concurrency for agent workloads: the soft-claim convention and the footprint scheduler are single points, not a convergence.
