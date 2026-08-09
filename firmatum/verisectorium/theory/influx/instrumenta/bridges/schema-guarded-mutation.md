<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/02-tooling-needs/src/schema-guarded-mutation.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

---
slug: schema-guarded-mutation
type: demand
register: evidenced
support-kind: [design, observational, theoretic, measured, synthetic]
strength: robust-qualitative   # the report's strongest demand; every kind of evidence here asks for it
convergent: [design, observational, measured, synthetic]   # theoretic merges with design as one estate leg; the absence-of-guarantee observation, external fault data, and the stress-test runs each fail differently
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: both (udon-primary)
depends: [edit-representation-landscape, errors-that-teach]
sources:
  - ../../01-ideation/02-provenanced/syntheses/CONVERGENCES.md  # clusters 1, 9; singleton yaml-spike
  - ../../01-ideation/02-provenanced/commentary/spikes/agent-utility-NOTES.md  # §2, P-C
  - ../../01-ideation/02-provenanced/syntheses/external-landscape-2026-07.md  # findings 4, 7
  - ../../pipeline-discussion.md  # S1 context; Joseph ~L537
---

# The demand: schema-guarded structural mutation

**Claim.** The clearest single demand in this entire report: an edit surface where an agent mutates a document **structurally** — the target named by a path, spans spliced precisely, indentation and escaping owned by the tool — with the mutation **validated inside the write**: no change is accepted that would make the document violate its declared schema, and a refused mutation returns a law-rich, menu-shaped error and touches nothing. The previous chapter established that no shipping tool provides this — every shipped editor works at the text level with no validity guarantee. Every kind of evidence this report holds asks for it.

## The demand, witness by witness

- **Stated directly by the project's owner** during the deliberations that reoriented this design effort (mid-2026): "a critical tool IMO for agents is a specialized edit tool that makes edits very easy without needing to worry about indent-levels for prose or raw code blocks, while simultaneously guaranteeing that no mutation that would cause the document to now violate the schema is accepted. The tool itself will need the machinery to do jq/yq like span-sensitive changes to the AST and have it checked against a static schema." The same demand runs through years of his design work in other dress: a document store that validates structure at the write boundary, with the schema declaration part of the document's own identity; a pattern-language whose first layer is "make invalid states unrepresentable"; write-path rules that insist on validating *inside* the write rather than after it, on one-call resolution rather than edit-check-revert loops, and on the file's own declared schema governing ("declared ≠ theater").
- **The shipping ecosystem has the gap and the near-misses.** The whole tolerance-ladder apparatus is compensation for unguaranteed text edits. One harness bolts secret-scanning onto edits *after* they land (guard-after — the weaker form). The yq tool demonstrates structural path-assignment with no schema guard. A popular linter's own maintainers admit its rules don't compose. Nobody has the validated-transaction shape.
- **The measured wound** (December 2025; [[yaml-stress-test| the YAML stress test]] reproduces it whole). Three  
  agents, an adversarial protocol: one writes valid data, a second introduces a specific corruption, a third — fresh context, no human — attempts recovery. Six corruption scenarios; their outcomes, verbatim from the test:

  | Corruption | Recoverable? | Method | Without backup |
  |---|---|---|---|
  | Truncated write (crash mid-write) | ✓ | restore from backup | ✗ failed |
  | Invalid YAML syntax (generation bug) | ✓ | auto-fix heuristic OR backup | ⚠ maybe |
  | Schema violation (valid YAML, wrong shape) | ✓ | backup, after a validation layer catches it | ✗ failed |
  | **Duplicate keys** | ✗ **silent failure** | none — undetectable | ✗ failed |
  | Partial update | ✓ | binary-search salvage OR backup | ⚠ maybe, loses data |
  | Circular reference (anchor misuse) | ✓ | backup, after cycle detection | ✗ failed |

  Headline: **recovery drops from ~100% to 16% — one scenario in six — without backup infrastructure.** And the one failure backups don't fix is the quiet one. Duplicate keys parse cleanly: the parser raises nothing, the last value wins, the earlier values are silently gone —

  ```yaml
  - id: "task-1"
    name: "Implement feature X"
    status: pending
    name: "Implement feature Y"   # parses fine; X is lost forever
  ```

  "Can agent detect this? NO — YAML parser doesn't warn about duplicates. Can agent recover? NO — earlier values are gone forever. Human intervention required: YES (to notice data inconsistency)." The team's four resulting requirements — backup before every write, validation after every read, salvage heuristics, human escalation — amount to ~500 lines of infrastructure built around a format that did nothing to help. Read from the wound side, the demand is exact: every scenario a schema-checking write gate refuses at the door is a scenario the next agent never has to recover from — and the undetectable one is refusable *only* at the door, because after the write there is nothing left to detect. (UDON's own attribute law is the other half of this defense: repeated keys are *kept, in order*, by core rule — the silent last-wins that destroyed task X is the behavior UDON already refuses.)
- **Lived, adjacently.** An agent's own post-mortem of a system it broke three times: chaining unverified edits "was the easiest path." The tool's own audience asking for the verified path to be the easy path — this chapter's demand, in first person.
- **The theory.** A schema converts an interpretive observation ("does this look right?") into a pass/fail one — the sharpest-signal move the [[tools-are-observation-infrastructure| observation chapter]] prices; a typed write boundary is one of its two separation mechanisms; and the [[errors-that-teach| refusal chapter]] makes atomicity of refusals an epistemic requirement, not politeness.
- **External research keeps rediscovering the absence from outside:** malformed-call and fabricated-parameter failures attributed to insufficient schema grounding (the [[structured-output-two-mechanisms| structured-output chapter]] carries the numbers and their scope), and the largest execution-failure subcategory in a 2026 fault-taxonomy study of the Model Context Protocol ecosystem is schema-serialization mismatch.

## The shape the evidence pins (and what stays open)

Pinned — consistent across the design sources:

```text
re-resolve path against current file → mutate assembly model
→ syntax-validate → (if schema bound) conformance check
→ span-splice write   — one atomic transaction; refusal mutates nothing
```

with a staged first version honestly available: syntax-valid, indent-correct, atomic first; schema conformance next.

Open, deliberately — the design work ahead must decide: whether the schema is static or composable ("can schemas be nested or otherwise composable?… or is the schema static?" — the owner's own open questions, from the same discussion as the edit-tool demand); the path language itself (the [[addressing-is-the-long-pole| addressing chapter]]); the serialization substrate that makes span-splicing exact (the [[round-trip-and-span-splice| round-trip chapter]]); and where guard strictness lives — the design work sketches enforcement *profiles* (casual / careful / critical: same notation, different strictness), with the agent edit tool as the careful gatekeeper.

## What this opens (ideas, not designs)

- ✦ **One guard, three doors.** The same conformance engine could stand at generation time (grammar/schema-constrained emission), at edit time (this chapter's transaction), and at review time (batch CI checking) — one schema, three enforcement points, so a document is guarded whether it was born, changed, or merged into shape. Nothing requires three separate validators; nobody has built the unified one.
- ✦ **Refusals that carry the repair.** A guarded refusal knows exactly which constraint failed. It could return the *nearest conforming version* of the attempted edit as a diff — "not this, but here is the closest thing that would be accepted." The refusal chapter's error-as-menu, upgraded to error-as-counteroffer.
- ✦ **Schema-aware merge.** Two agents editing the same document concurrently is a documented collision class. Structural mutation suggests structural *merge*: both edits applied to the model (not the text), conformance checked on the merged result, conflicts reported in path vocabulary rather than diff hunks. The multi-writer problem inherits the guard.
- ✦ **Guarding the markdown-era estate.** The harness's divergence note below has an answering idea: conventions-as-schema. The harness's existing plain-markdown artifacts follow implicit rules; stated as a lightweight schema, the same write gate could guard them *today*, before any notation migration — which would also measure how much guarding is worth, on the corpus that motivated it.

**Who reads this and when:** UDON reads it as the flagship utility the design decisions ahead must enable — it pulls paths, schema, spans, and round-trip at once, which is *why* it is the long-pole customer. The harness reads it as the write-path contract for any document-shaped state agents maintain (memory files, tracking documents, identity-defining files), where "schema" may be the harness's own invariants rather than a UDON schema. Divergence to keep visible: the harness needs this for its *plain-markdown-era* artifacts too — a UDON-only tool leaves its present corpus unguarded.

## Honest edges

The demand's breadth is real but its legs differ in kind: the owner's design line is decided intent; the stress test is one measured protocol on one format; the first-person account is adjacent (it asks for verification-made-easy, not for schemas by name); and the external studies indict ungrounded structure generally, not this design specifically. No one has yet built the guarded transaction and measured what it saves — that experiment is the point of the design work this chapter exists to inform.
