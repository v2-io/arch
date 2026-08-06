<!--
  Verisectorium theory INFLUX — instrumenta mining, verbatim copy.
  Source path at copy time: /Users/josephwecker-v2/src/MOVED/udon/v2/udon-needs/02-tooling-needs/src/steering-and-verification-surfaces.md
  Copied: 2026-08-06 (mining agent, first outside reader of the theory corpus).
  This is a copy, not authority — do not edit here; segments draft from live sources.
  NOTE: the MOVED/ tree is mid-reorg; verify the source path before drafting from it.
-->

---
slug: steering-and-verification-surfaces
type: demand
register: evidenced
support-kind: [observational, measured, design, testimonial]
strength: robust-qualitative   # three needs converge; the thinnest evidence base in the report, named as such
convergent: [observational, measured, testimonial]   # design merges as estate; thin but genuinely multi-kind
verified:
  - 2026-07-22 · frontmatter-migration · pilot-A · axes assigned from the chapter's claim; convergent legs audited under the failure-mode-independence key
stage: drafted
consumers: both
depends: [counter-register, tool-definition-anatomy]
sources:
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # C12, C15, Part D.2
  - ../../01-ideation/02-provenanced/syntheses/external-landscape-2026-07.md  # finding 6
  - ../../01-ideation/02-provenanced/copies/II7-ref-arch/sapientia-multi-agent-coordination.md  # read (head)
  - ../../01-ideation/02-provenanced/copies/harness-workshop/ai-cli-tools-source-assessment.md  # §6
---

# What the human on the other side needs

**Claim.** The human steering and verifying agent work is a first-class tool consumer, and the evidence — thin on this side, and this chapter says so plainly — converges on three needs: **structured decision surfaces** (not free-text interrogation), **explicit trust boundaries** (currently a live disagreement in shipping practice), and above all **verification surfaces matched to the plausible-wrongness reality**, because the one failure class nothing mechanical catches is the one only a human *using the product* notices.

## The evidence

- **Verification is the load-bearing need — measured, externally.** In the production system the [[counter-register| counter-register]] documents, about 70% of silent failures were caught by a human observing the product as a user, and essentially none by its 4,286 tests — "audits are regression engines, not prediction engines." The distinctive failure mode turns internal errors into confident, coherent, *false* output (an HTTP error log synthesized into a fabricated analysis). Medium confidence, one system — but it inverts the usual priority: the human surface is not the fallback after automation; it is the **only** catch for the failure class automation creates.
- **Observability as architecture (September 2025).** The programme's multi-agent coordination design made human steering a *filesystem property*: agents work in observable markdown streams anchored by a shared task list, so the human can watch them think in real time ("Joseph can `tail -f` these files") and intervene *by editing the documents they are working from*. The failure it was built against is still everywhere: the relay pattern, where a lead agent becomes "mediator/translator… telephone game," and the human loses sight of the actual work. Steering by shared artifact beats steering by relay.
- **Structured decision surfaces, shipped everywhere.** The converged ask-the-user shape — a few questions, a few options each, a recommended one, always a free-text escape — is one influential design adopted ecosystem-wide: survivorship evidence that structured choice presentation beat open-ended asking wherever it landed. Its complement is propose-before-apply: a preview product carrying the diff, the validation result, the side-effects, and the confidence (the [[progressive-disclosure-read-path| read-path chapter]] carries the product shape).
- **Trust boundaries are genuinely unsettled.** Most of the ecosystem treats per-directory instruction files as authoritative; one harness treats them as *untrusted data* under injection-precedence rules (the [[counter-register| counter-register]], row 6). Security- relevant, unresolved — a harness must choose, and say what it chose.
- **The bootstrapping datum.** A source-level survey of the shipping CLIs (conducted within this research programme) records the tools increasingly being built *by* agents — bot fleets filing, reviewing, and merging changes; agent-written specification folders preceding implementation. Human verification is becoming review-of-agent-work all the way down, which raises the stakes on diff legibility, provenance, and the plausible-wrongness catch rather than lowering them.

## What it generates

- **For the harness:** fund the human surface as the plausible- wrongness control, not as UX polish — propose/dry-run previews wherever mutation exists; work in observable artifacts instead of relay reports; an explicit, stated instruction-trust policy; and agent-work diffs designed for human reading. Authoring ergonomics and review bottlenecks are under-sampled in this report's evidence — treat that absence as missing data, not as low demand.
- **For UDON:** the human is the second reader of every agent-facing design in this report. Skeletons, structural diffs, and provenance annotations are dual-consumer surfaces by construction — "crystal clear even without syntax highlighting" is the human-side clause of the notation's original thesis. And steering-by-editing-shared- documents is a *document-format* use case: the human edits the very artifact the agent reads, which is the whole two-audience bet.

## What this opens (ideas, not designs)

> [!capability] Scheduled use-the-product verification **What:** the fail-plausible catch made a budgeted ritual — sampled sessions where a human (or a deliberately naive agent) *uses* the product rather than auditing its internals, on a cadence scaled to mutation volume. **Principles that apply:** validation catches malformation, not plausible wrongness; the human surface as the only known control. **Hypothesized impact:** adds the one observation channel with access to the failure class tests structurally miss — in the theory's terms, a channel whose error profile is *independent* of the automated channels, which is precisely the condition under which adding a channel adds real tempo rather than redundancy. **In tension with:** cost (human attention is the scarcest budget in the loop); the crystallization instinct (this is deliberately uncrystallized watching). **Potential downsides:** sampling can miss; ritualized "use it for five minutes" degrades into the checkbox it was invented to replace.

> [!capability] The review-grade diff **What:** a diff product built for the human reviewer of agent work: structural (what changed, in path vocabulary), provenance-carrying (who/what session), and intent-anchored (the [[intent-as-parameter| intent chapter]]'s stated purpose shown beside the change, so the reviewer checks the diff *against* it). **Principles that apply:** byte-preserving mutation (reformat noise is review poison); intent as the verification anchor. **Hypothesized impact:** cuts comprehension time per review — the cost the theory multiplies by every future reader — and gives the plausible-wrongness check something concrete to check against; hypothesis: reviewer catch-rate rises when the claimed purpose sits beside the actual change. **In tension with:** diff-tool inertia (line diffs are universal infrastructure). **Potential downsides:** a bad structural diff is worse than a good line diff; intent display invites rubber-stamping "matches stated intent" without asking if the intent was right.

> [!capability] Bidirectional steering documents **What:** the `tail -f` pattern completed: agents treat their own briefs and working documents as *live* — watching for mid-flight human edits as first-class steering input, with the edit surfaced to the agent as an event rather than discovered on next read. **Principles that apply:** steering by shared artifact; documents as the human-agent interface. **Hypothesized impact:** provides exactly the out-of-band intent channel the theory's objective-capture analysis says recovery requires — a correction path that does not route through the channel being corrected. Steering latency drops from whenever-the-agent-rereads to now. **In tension with:** interiority (an agent constantly interrupted by document edits never settles into its own loop — the same trust-as-temporal-respect the theory demands of composite minds). **Potential downsides:** live edits race agent writes (the [[freshness-and-atomicity| freshness chapter]]'s multi-writer problem, now with the human as second writer — leases apply to people too).

## Honest edges

This is the report's thinnest evidentiary base: one external case study (medium confidence), one September-2025 design, survivorship-grade shipped shapes, and a survey aside. Late-arriving evidence is expected here and the coverage notes welcome it. What is absent outright: any human-subject measurement — review times, steering-error rates, catch-rates. Everything above is builder-side reasoning about the human, not observation of one.
