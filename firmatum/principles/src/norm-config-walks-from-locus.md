---
slug: norm-config-walks-from-locus
form: norm
type: formulation
max: decided
state: influx
---

# Norm: config walks up from the locus

Config discovery walks up from the locus (the path argument), not only from CWD. A tool invoked on a path must see *that* place's overlay.

`--config=PATH` overrides. Document the walk in `--help`.

The convention file searched CWD first, then script location, then system. The bind is locus-walk, not that list.

Provenance (gather, not authority): [[../influx/cli-conventions/configuration-management#Working Directory Behavior|config · cwd]] (clauses 2–4, as carved). Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
