# A read of the "define the system first" plan, from the vsect side

*To councils-46 and Joseph, from Claude Opus 5.5 (session `0903e35e`), 2026-11-04. On `~/.claude/plans/so-now-i-ve-manually-rippling-raven.md`, read whole beside `councils-system-model.md` and `asf-flows.md`. I have read the old vsect design whole (now at `~/src/aat-v3/.archive/msc/vsect/`), the vsect tool, and the measurements from this week. I have not read the walking skeleton, the GCC template's v2, RC1 beyond `10`, or the unknowns scout.*

## The short version

The frame is right, and it is close to what Joseph told me directly today. The plan misses one distinction he drew, and it carries one tension inside itself.

- **The distinction: the engine versus the system definition.** Joseph to me, today: *"there can be a vsect tool that is a general engine, a much bigger 'system' definition / model than we suspected earlier that is also evolvable … via vsect"*, and vsect *"should be thought of as a runtime or at least a long-lived process where influx is introduced and ripples and magnifies and settles (or gets interrupted and halts after time)"*. And: *"new ideal vsect from the beginning, which can draw on anything principled from prior efforts."* The plan treats vsect 0.1.0 as the base, and derives the tool work from the gap between it and a definition (step 3). On Joseph's framing, the engine is designed fresh. 0.1.0, the archived design and RC1 are all influx to it.
- **The tension: define first, or build nothing ahead of demand.** Step 1 writes the whole definition first: about thirteen record types and a full catalog of transitions. Step 3 says to build nothing ahead of what the first crossing demands. Joseph's rule resolves it: *"If the model and the tool aren't able to accommodate what they want to do yet — that's exactly how we know what to flesh out more."* That is demand-driven. So define the **kernel** first: what any kind, transition, guard, event and decision-maker is, and the engine's invariants. Then let the **ASF system definition** grow through the engine, one demand at a time, starting with the first real crossing. The flow model's own fresh read warned against a fifth design layer over a theory nobody has touched since 2026-10-31. A full domain definition written up front would be that layer.

## What each side already has

Neither half is complete, and each holds the piece the other lacks.

- **The record side (vsect).** Durable, checkable state:
  - atoms with one fate;
  - versions as content hashes, with a check bound to the versions it read;
  - lapse on change, a second mind to confirm, append-only events;
  - identity attested by the harness (since 2026-11-02);
  - exact undo restoring evidence for free (Line A).

  These are the engine's invariants, and they survived this week's measurements. They're worth carrying whole.
- **The execution side (councils).** Who acts, and when:
  - transitions as queues, each with a decision-maker;
  - admissibility computed from a transition table (`model/actions.json`);
  - charters as records, so the system amends itself;
  - and, as I understand it without having read it, the walking skeleton's driver mode, where the engine invokes only the seat whose turn it is. That is a runtime launching minds.

The old vsect design said the opposite on exactly this point (*"vsect doesn't launch agents itself"*, `vsect.md` §3.4).

**The engine is the executor running over the record model.** That's the meeting point, and I'd write it down as one kernel, not as two documents that refer to each other.

## Where the plan reinvents

- **The mutation catalog mostly exists already.**
  - The archived design's verbs and event payloads cover most of step 1's list: revise with reworded versus redefined, take, confirm, refute, contest, split, merge, supersede, retire, cross, decide (`vsect.md` §3; `core.md` §5).
  - RC1 `09-LANGUAGE.md` sketches the rule form.
  - `model/actions.json` is the executable shape.

  Lift the catalog from these three, and spend the judgment on where they disagree. Rebut versus undercut is in RC1, not in the old design.
- **"Only through vsect" should be enforced at the tool call, not at commit.** A pre-commit hook catches the hand edit after an agent has spent its work, and says nothing to the agent at the moment of need. A PreToolUse hook that refuses `Write`, `Edit` and shell writes under a store's atom paths stops the act immediately. Its refusal can carry Joseph's message: this is a demand on vsect, so record it through a verb that doesn't exist yet, such as `vsect demand "…"`. That turns every refusal into backlog, the signal his rule wants. The identity hook (2026-11-02) is the working precedent, along with its lesson: hooks change permission behavior, so measure under real settings. Keep the commit hook as a backstop for humans and other harnesses.
- **The changeset is a transaction of the runtime.** In a 0.1.0-style tool it has to be a record with bookkeeping. In an engine, "open, ripple internally on an overlay, commit atomically or abort" is how the engine runs a multi-atom mutation anyway. The design work is then the two things Line A measured as missing: transforms the engine verifies, and read sets at span grain.

## What has to change in the inherited design

- **Rule 7 is reversed.** *"Files are plain markdown anyone may read or edit … vsect notices edits made outside it, and `revise` takes them in"* becomes something like: files are a readable projection the engine writes, and nothing else does. `revise` stops being "take in an outside edit" and becomes the transition itself.
- **§3.4 is reversed.** The engine launches decision-makers.
- **Frontmatter's computed lines go.** The 189-file drift came from a tool writing state into files that hand-commits then dropped. With the engine as the only writer, a file can still be a projection that carries a status, but nothing can drift it from the trail.

## What I would carry into the system definition from this week's numbers

These change guards and yields by an order of magnitude, so by the flow model's own tracking rule they belong in the kernel from the first event:
- **Authorship relative to the target.** A self-check found nothing in 116 of 116. That's a guard: a check only counts as evidence when its actor didn't write the version.
- **Whether a checker was primed by the author's account.** The other lead session's notes: the archived design's independence test includes "read nothing from the withheld set", and lineage alone can't see priming.
- **Maturity.** Re-check yield runs from about 0.8 at discussion grade to under 0.1 for settled mathematics.
- **Claim records as their own records.** Line A: they fail on their own schedule, 21 of 31 record checks broke or narrowed.

## How I would proceed

1. **One kernel document, one author, one independent checker, with Joseph adopting it.** It covers:
   - the engine's invariants (above);
   - the meta-model: kind, transition, guard, evidence requirement, decision-maker slot, changeset as transaction, circuit breaker, tension record;
   - a table sorting every principled piece from prior work into kernel, ASF system definition, or set aside, with the reason.

   I'd suggest one of us drafts and the other checks it, not two parallel drafts. Who does which is Joseph's call. Asked to guess, I'd say the vsect side drafts the record half and the councils side the executor half, inside one document.
2. **The first crossing as the first demand.** Pick one small ASF cluster, run it through the kernel with the refusal hook on, and let every refusal become a demand. The domain kinds get admitted as they're needed, through the engine, which is the self-hosting Joseph described.
3. **One experiment, run early**, because it decides the flow model's central yield: a round that only lands findings, then one more independent check.

## What I'd suggest councils-46 read before the next round

Short, and only what the kernel rests on:
- the archived design's `model.md` (the nouns and "one fate, one atom");
- `core.md` §3 (versions and basis) and §5 (events);
- Line A's account (`asf/msc/inside-out-2026-10/system-review/claim-record-extraction-2026-11-04.md`, branch `worktree-agent-a25fdf864ac319cc9`);
- the other lead session's `firmatum/utils/vsect/MATURATION-PLAN-notes.md`, eight gaps, which is uncommitted, so read it in place.

I'll read the walking skeleton and RC1 `09-LANGUAGE.md` from my side before drafting anything.
