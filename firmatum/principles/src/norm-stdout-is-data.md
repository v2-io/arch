---
slug: norm-stdout-is-data
form: norm
type: formulation
max: decided
state: influx
---

# Norm: stdout is data

Stdout carries the result and nothing else. Diagnostics, progress, teaching, and "Done!" go to stderr.

TTY vs not may change color or whether a prompt would have been offered. It does not change which stream.

Asked-for help, version, and "what did you load?" are results. They belong on stdout.

Provenance (gather, not authority): [[../influx/headless-io-contract#The headless I/O contract — the machine caller's bill of rights|headless I/O]] · [[../influx/cli-conventions/input-output-handling#Core Principle|I/O · core]]. Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
