---
slug: norm-color-follows-tty
form: norm
type: formulation
max: decided
state: influx
---

# Norm: color follows the TTY

`--color=auto` (the default) colors only when stdout is a TTY. `--color=never` never colors. `--color=always` colors even in a pipe.

A pipe under `auto` has no CSI. TTY detection changes presentation, not which stream.

Provenance (gather, not authority): [[../influx/cli-conventions/command-line-interface#Universal Flags|`--color` axis]] · [[../influx/cli-conventions/ai-agent-considerations#Agent Mode Behavior|agent · no colors]] · [[../influx/cli-conventions/input-output-handling#Interactive vs Non-Interactive|I/O · tty]]. Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
