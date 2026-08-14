---
slug: form-shared-flags
form: form
type: formulation
max: decided
state: influx
---

# Formulation: shared flag meanings

Shared meanings across the suite, even if each crate hand-rolls parsing:

- `-h` / `--help` — show help
- `--version` — show version and exit
- `--` — end of flags
- `-` — stdin (when the tool reads)

When the tool has the story: `--format`, `--color=auto|always|never`, `-q` / `--quiet`, `--dry-run` (when it can mutate or plan).

Not floor: stackable `-v`, `--debug`. Do not add them until the tool has something true to say at those levels. A tool with no verbose story must not take `-v` for verbose.

Not bound: `--pipe` as a synonym bundle (detection makes it unnecessary); `--no-color` as a second spelling of `--color=never`; `@filename` argument files; combined-short syntax as a requirement.

Provenance (gather, not authority): [[../influx/cli-conventions/command-line-interface#Universal Flags|universal flags axis]] · [[../influx/cli-conventions/command-line-interface#Flag Conventions|flag-syntax axis]]. Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
