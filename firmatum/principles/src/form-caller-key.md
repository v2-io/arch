---
slug: form-caller-key
form: form
type: formulation
max: decided
state: influx
---

# Formulation: the caller declares itself with a key

A suite tool accepts a caller declaration — spelling today: `--caller KEY` — whose value is an opaque key, not a taxonomy. Config may keep an overlay per key (aspectus: `caller-<key>.toml`); a key with no overlay is not an error.

This is the suite's first instance of the **agent-identification convention** the headless-I/O gather names as an unbuilt gift ([[../influx/headless-io-contract|headless-io-contract]], "What this opens"): detection (TTY, `CI=`) infers *that* a machine is calling; the caller key lets the caller say *who* — and later, what it can consume (context budget, preferred formats).

Bounds, held deliberately:

- **Declaration is the contract; detection is best-effort courtesy.** A tribal `TOOL_AGENT_MODE=1` is not the happy path ([[norm-machine-path-is-detected]]). The undramatic path is the flag a tool-description passes automatically.
- **No harness zoo.** Do not invent a taxonomy of callers in advance; files keyed by a value wait until there are two real callers. The key's grain (kind → harness+model) is the caller's business.
- **The key selects config overlays; it does not replace presentation detection.** `--color=auto` still follows the TTY and machine-format selection still follows non-TTY/`CI=`/`--format=` regardless of any caller key ([[norm-color-follows-tty]], [[norm-machine-path-is-detected]]) — a declared caller with a TTY still gets color.
- **Per-caller state follows the key.** Anything a tool personalizes (quiet thresholds, after-image of prior looks) is keyed by caller, so one caller's habits never retune another's channel — the personal form of [[norm-caller-tunes-the-channel]].
- Second collision starts the shared spelling: when a second tool takes the flag, the name and env convention become a suite constant, not a per-tool invention.

Provenance: aspectus `--caller` (shipped 2026-08); [[../influx/cli-conventions/ai-agent-considerations|ai-agent-considerations]] (detection axis). Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
