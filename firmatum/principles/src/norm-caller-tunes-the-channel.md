---
slug: norm-caller-tunes-the-channel
form: norm
type: normative
max: decided
state: influx
---

# Norm: only the caller tunes the channel

The observed may speak *into* the channel; only the caller tunes it.

A place (locus, repo, tree) may offer **content** the look can surface — a note, a label, a claim about itself. It never sets **channel parameters**: columns, budget, depth, format, quiet thresholds, or anything else that shapes how the observer sees. Channel tuning comes only from the caller stack (defaults < global < user-home < caller-key < env < flags).

## Why this is a law and not taste

The formal ground is AAT's interaction-channel classification (`asf 01-aat-core/src/der-interaction-channel-classification.md`, #der-interaction-channel-classification): the emitter sees a scalar; the recipient sees a regime — and the sharpest recipient-side finding is the **Regime-I-with-adversarial-content attack**: an emitter that controls content fed into an agent's update can choose the *sign* of the update on load-bearing beliefs. "The faster you learn, the faster you die." A glance tool is precisely such an update channel — an oriented agent's beliefs about a place come through it.

If the place could retune the eyes that look at it:

- a changing tree silently changes what every future looker sees (drift in the observer, caused by the observed);
- a hostile or merely careless tree can hide its own mass, suppress the surprising column, or star a format that flatters it;
- two agents looking at the same place with the same config no longer see the same look — the channel is no longer the caller's instrument.

Keeping the tuning caller-side keeps the channel goal-blind with respect to the observed. Content offered by the place arrives *as content* — visible, attributable, weighable — never as an invisible change to the instrument.

## The carve

- **Content channel (allowed, secondary):** the place may leave a note the look can surface, marked as coming from the place. Specimen: aspectus `place-wants-known`.
- **Tuning channel (refused):** no repo file, no locus walk, nothing discovered from the tree sets how the look is made. Specimen: aspectus [[../../utils/aspectus/design/config.md|Config]] — caller stack, no file in the project.

Boundary note: a repo file that governs what a *mutating* tool may touch (`.md-pressignore`, `.gitignore`) is on the other side of this line — it constrains writes to the place by the place, not how an observer sees. The law is about observation channels.

Provenance: articulated 2026-08-14 from the aspectus config decision (Joseph: *"Caller stack, not a file in the project… so a changing tree cannot retune the agent's eyes"*) and #der-interaction-channel-classification. Carve: [[../../utils/aspectus/ASPECTUS.outline.md|ASPECTUS.outline]] Part II.
