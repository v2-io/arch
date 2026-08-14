---
slug: form-exit-vocabulary
form: form
type: formulation
max: decided
state: influx
---

# Formulation: a small exit vocabulary

Adopt a small stable vocabulary, not the sysexits zoo, unless a tool earns a code.

- `0` — success, including "nothing to do" when that is success
- `1` — failed check / would-change (md-press `--check` precedent)
- `2` — usage, or not-yet / refused-to-act-as-requested
- `130` — SIGINT

Further codes only when an agent must branch without parsing stderr.

Not bound: the convention file's 64–78 list, and the rest of that table.

Provenance (gather, not authority): [[../influx/cli-conventions/command-line-interface#Exit Codes|exit-code axis]]. Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
