---
slug: form-config-precedence
form: form
type: formulation
max: decided
state: influx
---

# Formulation: config precedence

High to low: flags → env (`TOOL_*`) → repo file → user XDG → built-in defaults.

No system `/etc` until a tool is actually installed that way.

Filename of the repo file is a tool decision, not this clause.

Not bound: the convention's legacy `~/.mytool` / `~/.mytool.conf` locations as a requirement.

Provenance (gather, not authority): [[../influx/cli-conventions/configuration-management#Precedence Order (highest to lowest)|precedence axis]] · [[../influx/cli-conventions/configuration-management#Configuration File Locations|XDG axis]]. Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
