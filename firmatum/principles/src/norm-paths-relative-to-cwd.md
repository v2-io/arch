---
slug: norm-paths-relative-to-cwd
form: norm
type: formulation
max: decided
state: influx
---

# Norm: explicit paths are relative to CWD

Explicit paths in arguments are relative to CWD. A tool invoked with no path looks at CWD.

This is path resolution, not config discovery. Discovery is [[norm-config-walks-from-locus]].

Provenance (gather, not authority): [[../influx/cli-conventions/configuration-management#Working Directory Behavior|config · cwd]] (clause 1). Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
