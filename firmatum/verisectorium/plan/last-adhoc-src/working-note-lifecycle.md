---
slug: working-note-lifecycle
type: form
depends:
  - working-notes-sidecar
  - working-notes-deluge
---

# What earns a note, and how notes leave

*A per-atom note store is only as good as its exit: admission rules are easy to write and cheap to follow, while retirement needs an occasion that actually arrives, and a store without one silently becomes the largest thing in the corpus.*

## The claim

**Admission is the easy half, and it is worth getting right anyway.** A note earns its place if it assists *future* work on this atom. Three kinds do: a **forward pointer** (an open question, a follow-on, a dependency waiting on something else), a **regression guard** (a form that was tried and disconfirmed, or deliberately corrected away, recorded so it is not re-attempted by the next agent who has the same good idea), and a **dead-end warning**. One kind does not: narration of what already happened — *this used to say X, the audit recommended Y*. That is history, it belongs in the history layer, and the pull toward writing it is strongest exactly when the change was a deletion and there is no other artifact to point at.

**Retirement is the half that decides whether the store works.** A note is finished when its content has gone somewhere: resolved (the question is answered and the answer is in the atom's body), promoted (the note was really claim material and is now stated as such), or expressly deferred (it is still live, and saying so is a decision rather than an omission). All three are typed dispositions, not deletions — and this is where the design usually fails, because retirement has no natural occasion. Authoring produces notes; nothing produces their removal unless something *asks*.

**So a note store needs a real occasion for retirement, and the occasion cannot be a stage nobody reaches.** The common design attaches the drain to the top of a promotion ladder: at the final stage, empty the notes. This is elegant and it does not run, because the ladder's top is a destination the daily work never needs ([[state-flags-not-gates]]). Occasions that plausibly do arrive: any substantive edit to the atom (the notes are already open); a scheduled sweep over the largest note stores; the completion of whatever the note was waiting on. What they share is that they are triggered by work that was going to happen anyway.

**A note store also should not be a queue for something else.** When notes are the only per-atom place to put things, decisions awaiting a steward and findings awaiting routing get parked there, because there is nowhere better. They then have no queue semantics — no owner, nothing watching, no way to ask what is outstanding — and they inflate the store past the point where anyone reads it. Material that is waiting on a *person* belongs in a decision surface; material waiting on *work* belongs in the notes.

## Strength & grounds

**A design formulation with one strong negative specimen and no positive one.** The specimen is [[working-notes-deluge]]: on the ASF corpus, 2026-08-05, working notes are 27.3% of total prose volume across 232 of 243 segments, with 57.9% of that volume arriving from one unfinished harvest process, and the documented drain gate has fired zero times. That corpus's admission rule is unusually well-written and its drain is unusually well-specified — which is the useful part of the evidence: the failure is not sloppiness about what a note is, it is the absence of an occasion on which notes leave.

What is genuinely established is the negative claim — a drain attached to an unreached terminus does not drain, and a note store doubling as an unowned decision queue grows. The positive claims about which occasions work are **untested**: no instance in this estate has run edit-triggered or sweep-triggered retirement, so the recommendation is reasoning from the failure, not a demonstrated alternative. It is also one corpus, and the analysis shares authorship with the corpus — which means the diagnosis and the thing diagnosed were not arrived at independently, so a reader should weigh the *measurement* (which anyone can re-run) far more heavily than the *explanation* offered for it.

## Working Notes

- The measurement that would settle the design: the forward-pointing share of a mature note store. If most notes are still doing their job, the volume is inventory and the drain matters less than it appears; if most are retrospective, the drain is the whole problem.
- Open: whether a retired note should leave a trace. A typed disposition that is recorded rather than merely performed would make the retirement rate observable, and would let an undispositioned disappearance be detectable at all — that is the seam with [[observable-crossings]].
- Adjacent: [[decision-records]] (where steward-blocked items should go instead), [[history-layer]] (where the excluded narration belongs), [[atom-as-cluster]] (why the notes are on their own clock in the first place).
