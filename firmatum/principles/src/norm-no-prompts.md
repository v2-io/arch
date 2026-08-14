---
slug: norm-no-prompts
form: norm
type: formulation
max: decided
state: influx
---

# Norm: no prompts

Never block on stdin for confirmation when not a TTY. Fail, or take the safe default, and say so on stderr.

`--interactive` / `--batch` are overrides of detection, not the primary interface.

Provenance (gather, not authority): [[../influx/cli-conventions/core-design-philosophy#AI Agent Design Principles|no interactive prompts]] · [[../influx/cli-conventions/input-output-handling#Interactive vs Non-Interactive|tty detection]]. Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
