---
slug: form-config-precedence
form: form
type: formulation
max: decided
state: influx
---

# Formulation: config precedence

High to low: flags → env (`TOOL_*`) → caller-key overlay → user XDG → global/machine → built-in defaults.

Two carves govern what may sit in this stack at all:

- **Channel tuning** (columns, budgets, depth, formats, thresholds — anything shaping how an observer sees) comes *only* from this caller-side stack. There is **no repo-file layer for tuning**: [[norm-caller-tunes-the-channel]] — the observed never retunes the observer's eyes. (An earlier form of this atom carried a "repo file" layer, agent-carved from the convention pack; superseded 2026-08-14.)
- **Write-scope overlays** (`.md-pressignore`, `.gitignore`-like: what a mutating tool may touch in *this* tree) legitimately live with the tree — see [[norm-overlays-are-config]]. They are not a precedence layer of the tuning stack; they are a different kind of fact with a different owner.

No system `/etc` layer is *read as required*: a tool defines its global path but treats absence as normal, until a tool is actually installed machine-wide.

Not bound: the convention pack's legacy `~/.mytool` / `~/.mytool.conf` locations.

Provenance (gather, not authority): [[../influx/cli-conventions/configuration-management#Precedence Order (highest to lowest)|precedence axis]] · [[../influx/cli-conventions/configuration-management#Configuration File Locations|XDG axis]]. Specimen: aspectus [[../../utils/aspectus/design/config.md|Config]] (defaults < global < user-home < agent-type < env < flags). Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
