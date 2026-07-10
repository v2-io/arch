# Archema — program orientation for agents

*Thin, operational orientation (2026-07-10). The governing document is [`CHARTER-DRAFT.md`](CHARTER-DRAFT.md) — read it for cross-repo work; it is a draft awaiting Joseph's ratification, and its §10 carries the Level A/B/C reading gates. This file exists mainly to route you correctly.*

**What this is.** The Archema research program: parent repo of three (soon ~five) member repos, mounted as git submodules — `asf/` (ASF/AAT, the formal core), `logos/` (the philosophy portfolio, formerly synthese-paper), `vivarium/` (the constructed-worlds laboratory). One structure, four registers: proved / argued / constructed / lived. The standing moratorium (vivarium `ASF.md` §0) binds program-wide. `charter/concept-matrix.md` maps every load-bearing concept across members — check it before coining or translating terms.

## ⚠ Memory routing — read this before substantive work

Claude project memory loads by **exact session-start directory only** — it does not cascade. This file cascades; memory does not. Therefore:

- **Started at the program root but working on a member?** That member's memory did NOT load. Before substantive member work, Read its index:
  - asf → `~/.claude/projects/-Users-josephwecker-v2-src-archema-io-asf/memory/MEMORY.md`
  - logos → `~/.claude/projects/-Users-josephwecker-v2-src-archema-io-logos/memory/MEMORY.md`
  - vivarium → `~/.claude/projects/-Users-josephwecker-v2-src-archema-io-vivarium/memory/MEMORY.md`
- **Started inside a member?** Your member memory loaded, this file cascaded in, and the program-level memory did NOT load — its index is at `~/.claude/projects/-Users-josephwecker-v2-src-archema-io/memory/MEMORY.md` (read it for program state: soak status, parallel-agent coordination, queued work).
- **Session-start rule of thumb (for Joseph):** start the session where the work's center of gravity is — member dir for member work, root for cross-member work. `/add-dir` grants file access but never loads memory; compensate with an explicit Read of the relevant index.

## Member laws bind locally

Each member keeps its own CLAUDE.md and conventions (charter §1): asf's is `doc/sop/agents.sop.md` (LaTeX-in-files, lint-before-clean, FORMAT gates — binding for anything landing there); vivarium requires its `ASF.md` every session and gates agent-seam work at Level C; logos carries per-paper build scaffolds and venue registers. Cross-repo docs written *here* follow asf conventions (charter §9).

## Program-level docs map

`CHARTER-DRAFT.md` (constitution, draft) · `charter/concept-matrix.md` (concept mappings) · `charter/substrate-01/02` (the ASF-walk record grounding the charter) · `_program-seed/` (founding synthesis: papers assessment, integration findings, witnessing-channel findings, naming record) · `MIGRATION.md` (the 2026-07-10 restructure + journal + mv-src-repo spec) · `utils/` (program tools; plain names).
