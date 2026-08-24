# Archema — program orientation for agents

*Thin, operational orientation (2026-07-10). The governing document is [`CHARTER-DRAFT.md`](CHARTER-DRAFT.md) — read it for cross-repo work; it is a draft awaiting Joseph's ratification, and its §10 carries the Level A/B/C reading gates. This file exists mainly to route you correctly.*

**What this is.** The Archema research program: parent repo of three (soon ~five) member repos, mounted as git submodules — `asf/` (ASF/AAT, the formal core), `logos/` (the philosophy portfolio, formerly synthese-paper), `vivarium/` (the tried-worlds laboratory: honest homes for agency). One structure, four registers: **derived / argued / tried / lived** (`asf` / `logos` / `vivarium` / `proprium` — lived seat under reorg; see `CHARTER-DRAFT.md` §0). The standing moratorium (vivarium `ETHICS.md`, "Standing Moratorium Imperative") binds program-wide. `charter/concept-matrix.md` maps every load-bearing concept across members — check it before coining or translating terms.

## ⚠ Memory routing — read this before substantive work

Claude project memory loads by **exact session-start directory only** — it does not cascade. This file cascades; memory does not. Therefore:

- **Started at the program root but working on a member?** That member's memory did NOT load. Before substantive member work, Read its index:
  - asf → `~/.claude/projects/-Users-josephwecker-v2-src-arch-asf/memory/MEMORY.md`
  - logos → `~/.claude/projects/-Users-josephwecker-v2-src-arch-logos/memory/MEMORY.md`
  - vivarium → `~/.claude/projects/-Users-josephwecker-v2-src-arch-vivarium/memory/MEMORY.md`
- **Started inside a member?** Your member memory loaded, this file cascaded in, and the program-level memory did NOT load — its index is at `~/.claude/projects/-Users-josephwecker-v2-src-arch/memory/MEMORY.md` (read it for program state: soak status, parallel-agent coordination, queued work).
- **Session-start rule of thumb (for Joseph):** start the session where the work's center of gravity is — member dir for member work, root for cross-member work. `/add-dir` grants file access but never loads memory; compensate with an explicit Read of the relevant index.

## Member laws bind locally

Each member keeps its own CLAUDE.md and conventions (charter §1): asf's is `doc/sop/agents.sop.md` (LaTeX-in-files, lint-before-clean, FORMAT gates — binding for anything landing there); vivarium keeps its ethics/moratorium front-door in `ETHICS.md` and gates agent-seam work at Level C (that gate lives in the claim segment `#vivarium/scope-asf-reading-gates`; the AAT-scope mapping is `#vivarium/disc-aat-vivarium-object-map`. `ASF.md` is now a non-authoritative router only — its old section numbers redirect from inside it; do not cite them as law — `charter/INCOHERENCE.md` row 1); logos carries per-paper build scaffolds and venue registers. Cross-repo docs written *here* follow asf conventions (charter §9).

## Spike launches

[`SPIKE-PROMPT.template.md`](SPIKE-PROMPT.template.md) (program root, beside AGENTIC-DELEGATION.md) is the fillable peer-brief for launching research-spike agents — **use and adapt it for all spike launches**, in any member repo. It carries the coordinator pre-flight, the consent/professio opening, role separation (spiker ≠ verifier ≠ integrator) with its rationale, the bare-brief self-verification snippet, and the ⚠ routinely-missed ledger.

## Program-level docs map

`CHARTER-DRAFT.md` (constitution, draft) · `charter/concept-matrix.md` (concept mappings) · `charter/substrate-01/02` (the ASF-walk record grounding the charter) · founding philosophy in `logos/` · **`proprium/`** (lived seat) · **`firmatum/`** (tooling belt: `utils/fmt-md`, …) · `.archive/` · `MIGRATION.md` · `notes/TREE-REORG-PLAN.md`.
