---
slug: norm-secrets-never-argv
form: norm
type: normative
max: decided
state: influx
---

# Norm: secrets never as argv

Never accept secrets as command-line arguments (visible in `ps`). File / env / stdin. Fail closed.

A tool with no secrets today still owes the rule, so the next tool does not "just this once."

Not bound: the rest of the security axis (sandbox flags, privilege-drop, TLS as a CLI concern).

Provenance (gather, not authority): [[../influx/cli-conventions/security#Secret Handling|secret handling]] · [[../influx/cli-conventions/security#Secure Defaults|fail closed]]. Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
