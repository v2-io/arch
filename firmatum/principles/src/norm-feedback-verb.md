---
slug: norm-feedback-verb
form: norm
type: normative
max: decided
state: influx
---

# Norm: the tool carries its own feedback verb

Every estate tool allows feedback and friction points to be reported using a verb on the tool itself, on the CLI — `<tool> feedback ...` or equivalent. The reporting surface travels with the tool, not with a file the user must know about.

Why a verb and not a footer: the aspectus `inbox.md` footer proved the value of soliciting specimens (command + cwd) at the moment of friction, but it depends on the user knowing the inbox path and being willing to append to a file in someone else's tree. A verb makes the true thing (reporting the friction now, in place, with context the tool can attach itself — version, cwd, the failing invocation) cheaper than the fabricated thing (carrying the annoyance silently, or paraphrasing it later from memory). The tool is present at the moment of friction; the report should be one command away.

What a minimal implementation carries: append-only, provenanced (who/when/version/cwd), lands where the tool's maintainers already look (an inbox file in the tool's repo, or the tool's own data tree), and never blocks — a feedback verb that can fail loudly discourages the marginal report.

Provenance: **steward** — Joseph, 2026-08-28, in-session: *"all of our tooling should allow for feedback/friction points to be reported using a verb on the tool on the cli itself :-)"* — given while directing feature requests toward relata after a first-contact use session. Prior art: the aspectus inbox footer convention ([[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] session-check line); this norm generalizes it from a per-tool file convention to a suite-wide CLI surface.
