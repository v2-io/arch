---
slug: norm-config-walks-from-locus
form: norm
type: formulation
max: decided
state: influx
---

# Norm: what discovery-from-the-tree is for (and not)

*(Rewritten 2026-08-14. The earlier form — "config discovery walks up from the locus" — was agent-carved from the convention pack and conflicts with [[norm-caller-tunes-the-channel]]; the slug is kept so links resolve, and now carries the corrected law.)*

Discovery that walks up from a path is legitimate for exactly one kind of config: **write-scope overlays** — facts the tree states about itself that bound what a *mutating* tool may touch (`.md-pressignore` at or above the named file; `.gitignore`). A tool invoked on `~/x/y` must see *y's* protections, wherever the invoker stands, so the walk is from the **target**, not only from CWD.

Discovery from the tree is **refused** for channel tuning: an observation tool never reads how-to-look parameters from the locus or its parents ([[norm-caller-tunes-the-channel]]). Explicit `--config=PATH` always overrides; it names a file, it is not discovered.

Document whichever walk a tool does (or refuses) in `--help`.

Provenance (gather, not authority): [[../influx/cli-conventions/configuration-management#Working Directory Behavior|config · cwd]], as corrected. Specimens: md-press ignore-walk (write-scope, walks); aspectus config (tuning, refuses — decoy-at-locus is a tested non-event). Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
