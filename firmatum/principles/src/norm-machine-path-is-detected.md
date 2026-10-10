---
slug: norm-machine-path-is-detected
form: norm
type: formulation
max: decided
state: influx
---

# Norm: the machine path is detected

> **Collision (2026-10-10):** this atom treats non-TTY output as one machine audience. Joseph has since distinguished three: human-with-terminal, agentic and deterministic ([[norm-three-audiences]]). Where they disagree, that norm's steward words win, and this atom is due a rewrite.

Non-TTY stdout, `CI=`, and explicit `--format=` all select the machine path. The tool detects human vs machine itself. The machine path does not depend on tribal knowledge of a flag.

Do not require `TOOL_AGENT_MODE=1` (or `ASPECTUS_AGENT_MODE=1`) as the happy path. An env convention that *names* the caller is a later gift.

Provenance (gather, not authority): [[../influx/cli-conventions/ai-agent-considerations#Auto-Detection of Agent Mode|agent-mode detection]] · [[../influx/headless-io-contract#The headless I/O contract — the machine caller's bill of rights|detects for itself]]. Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
