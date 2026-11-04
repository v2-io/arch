# vsect: the kernel (draft 1)

*To Joseph, who adopts or sends back, and to councils-46, who checks it independently. From Claude Opus 5.5 (session `0903e35e`), 2026-11-04. Status: a draft, unchecked by a second mind. Each section says what kind of statement it makes: **decided** (Joseph's word, quoted), **carried** (taken from earlier work that this week's evidence did not overturn, with the source named) or **proposed** (mine, open to change).*

## What this is

vsect is an engine that holds a store's records and is the only thing that changes them. A store's **system definition** (its kinds of record, the transitions each kind allows, their guards, and who may perform each) is itself made of records in the store, so the system changes itself through its own transitions. Every act enters as a request to perform a transition, is checked against the definition, and lands as an event in an append-only log. The files a person reads are a projection of that log.

This document is the **kernel**: what is true of every store, whatever it holds. ASF's system definition is not here. It is admitted record by record, through the kernel, as the first real work demands it.

## Joseph's words this rests on

*Decided.*

- **Only through vsect.** *"AGENTS SHOULD NOT BE ABLE TO GO IN AND MANUALLY CHANGE CANON OR ATOMS. ONLY THROUGH VSECT!!! If the model and the tool aren't able to accomodate what they want to do yet-- that's exactly how we know what to flesh out more at any given point."* (2026-11-04, after the Line B pilot)
- **An engine with an evolvable system.** *"there can be a vsect tool that is a general engine, a much bigger 'system' definition / model than we suspected earlier that is also evolvable (but doesn't need to be, especially at first) via vsect"*, and *"new ideal vsect from the beginning, which can draw on anything principled from prior efforts"* (2026-11-04).
- **A runtime, eventually.** *"it should be thought of as a runtime or at least a long-lived process where influx is introduced and ripples and magnifies and settles (or gets interrupted and halts after time)"*, and *"calling vsect manually will start to be phased out in time where the correct agent configuration or council charter+agenda-template are specified for various atom mutation paths"* (2026-11-04).
- **But vsect does not model councils, especially not at first.** He has had to say it more than once:
  - *"treat each vsect transition-edge as a queue that is drained by a 'decision-maker' and not specify a council or agent etc."* (2026-11-03);
  - *"the processor/adjudicator/etc.-- is just one simple 'adjudicator' node/station that … will be either an agent or a well-tuned committee or human in the loop or anything else we decide *later*"* (2026-11-04);
  - *"especially not initially"* (2026-11-04).
- **Wall time is only a proxy.** The kernel counts acts and their yields; clocks serve only as recorded facts and as backstops.
- **The old process isn't ground truth.** The sprint's trail gives yields that are properties of minds and text (self-checks find nothing; independent checks find something about four times in ten), not routing to imitate.
- **The interface's shape is `ant`'s.** `ant messages create --format json <<'YAML' … YAML | jq …`: a resource and a verb, a typed request body on stdin, structured output.

## The invariants

Each is a property of every store at every moment, and each says what checks it.

1. **The engine is the only writer.** *Proposed, from Joseph's rule.* Every file under a store equals the engine's projection of its log. `vsect check` recomputes the projection and reports any difference, whatever route made it (an editor, `sed -i`, `cat >`). A PreToolUse hook refuses writes under a store's paths at the moment of attempt and says how to make the request instead. The check is the guarantee; the hook is the signal. This reverses the archived design's rule 7 (*"Files are plain markdown anyone may read or edit … `revise` takes them in"*).
2. **The log is append-only.** *Carried*, from vsect 0.1 and the archived design. An event is never edited or removed. A correction is a new event that names the one it corrects.
3. **Versions are content addresses.** *Carried.* A version is the hash of an atom's normalized text, its kind and its declared form. A check binds to the versions of everything it rests on and lapses, with the cause named, when any of them changes. Line A measured the consequence worth protecting: an exact return to an earlier text restores every binding to it, with no act needed.
4. **Standing is computed, never typed.** *Carried*, from the archived design (rule 4). No request body may carry a status, a rung or a standing word. The engine computes them from the log through declared projections.
5. **Every event carries an attested actor.** *Carried*, from the identity work of 2026-11-02. Each event records the agent key, parent, agent type and session, and says what that record rests on: supplied by the harness, the session only, or declared. Independence, the reviser refusal and "the checker is not the author" are computed from these, never from a role someone typed.
6. **Every change is a transition of the definition.** *Proposed.* No act changes the store unless the definition holds a transition for it and its guard admits it. An act the definition cannot express is refused, and the refusal is recorded as a **demand**: what was attempted, by whom, on what. Demands are the backlog Joseph's rule asks for.
7. **The definition is records.** *Proposed*, with councils-46. Kinds, edge kinds, transitions, guards, projections and decision-maker slots are atoms in the log, changed only by the definition's own transitions. The command surface is generated from them, so admitting a transition creates its verb, and the tool and the definition cannot drift apart.
8. **Every stock has an exit.** *Proposed*, from the flow model's residue condition. Every kind declares its terminal states. The engine reports any atom population that grows with no transition able to leave it, the way it reports a lapsed check.

## The nouns

*Proposed, built from the archived design's model and RC1. Each is a kind the kernel itself defines, admitted by the founding event.*

- **Store.** A directory holding a log. Its projection is everything else in it.
- **Event.** One recorded act: the transition, the atom and version it acted on, the request body, the attested actor, the outcome. Content-addressed.
- **Atom.** The unit with one fate (*carried*, `model.md`): anything that can be refuted, superseded or ratified while its neighbors carry on is its own atom. A file is a container, and a projection.
- **Kind.** A declared type of atom. It is admitted only when it *"fails differently, and is repaired differently, from every other kind"* (*carried*, `model.md`), shown by answering RC1's founding questions against real atoms. A kind declares:
  - its fields and the schema of each, so a request body can be validated;
  - its dimensions and their rungs;
  - its terminal states.
- **Edge kind.** A typed relation, declared with its meaning, the transitions that consume it, and how far it propagates (*carried*, RC1 `06-EDGES.md`). `supports` signals upward but never carries strength along a chain. `seeded-by` is transitive, because descent is. `restates` and `narrates` tie a restatement or a piece of teaching to the exact version it speaks about, so it goes stale when that version changes.
- **Transition.** One act a kind allows: kind × verb. It declares:
  - the schema of its request body;
  - its **guard**, a condition over the atom's state, its edges, the log and the actor;
  - its **evidence requirement**, what must already be recorded;
  - its **decision-maker slot**;
  - its effects, meaning the events it writes and the transitions it derives. A refutation, for instance, derives review requests on the atoms that rest on the refuted one.
- **Decision-maker slot.** Who may perform a transition, stated as a predicate over the attested actor: anyone; anyone who did not write this version; the steward; a named adapter. The kernel checks the predicate. It does not know or care whether the actor is one agent, a council or a person.
- **Queue.** The atoms for which a transition is admissible and not yet performed. A queue is computed, never stored. "Each transition edge is a queue drained by a decision-maker" is a query over the log.
- **Changeset.** A transaction (*proposed*, from the flow model's §6 and Line A). It is opened with an intent and a scope, the atoms it expects to touch, which may grow. While open it is a branch of the store where the same engine writes. Drafting there is cheap, attributed and unchecked, and ripples stay inside it. It then commits, appending the branch's events to the main log once its guard holds, or aborts, leaving the store as it was. One owner, open to close. A changeset's value is in confirming, and that needs two things the kernel provides:
  - **verified transforms**, such as "this span moved and the rest is byte-identical", which the engine checks so that no mind has to;
  - **basis reads at span grain** (next item).
- **Two kinds of read.** *Proposed*, with councils-46.
  - A **provenance read** is what a mind saw. Independence and priming depend on it.
  - A **basis read** is what a verdict rests on. Lapse and confirmation depend on it.

  A verifier can see a span its verdict doesn't rest on, so the two are recorded apart. Provenance reads can be attested at file grain by a hook on reads. Basis reads come from the check itself.
- **Projection.** A declared computation over the log: a file's rendered text, a computed column, a dashboard count. Projections are what a person reads. They carry no state of their own.
- **Demand.** A refused act, recorded with what was attempted and why it was refused. Demands are atoms, so they have queues too.

## The interface

*Proposed, after `ant`.*

```bash
vsect <kind> <transition> [--dry-run] [--format json|text] <<'YAML'
atom: <kind>:<slug>        # omitted when minting
version: <hash>            # the version the request was composed against
body:
  …                        # fields the transition's schema declares
YAML
```

- **The draft is the request.** Prose arrives as YAML block scalars in the body. Nothing is written until the body validates against the schema and the guard admits it. There is no file to edit and nothing to reconcile.
- **The result is structured:**
  - the event written;
  - the new version;
  - the checks the change lapsed;
  - the queues it entered or left.

  A refusal carries its reason as data, and the demand it recorded.
- **One interface for every writer:** a person, an agent, an adapter running a council's close. Read verbs (`show`, `why`, `open`, `check`, `flows`) take the same addressing.
- **`--dry-run`** returns the events that would be written, the lapses they would cause and the guard's verdict, and writes nothing.

## Dispatch, sequenced

*Proposed, reconciling Joseph's statements.*

The kernel defines the queue, the decision-maker slot and the **dispatch contract**: what the engine hands a decision-maker (the atom, its version, the transition, the request schema, and the provenance it may read), and what it accepts back (a request body, from an attested actor). That is all it knows about who decides.

At first, every slot is filled by whoever runs the verb. Adapters come later, each outside the kernel, each satisfying the contract:

- agent configurations per mutation path;
- council charters and agendas, run by something like the walking skeleton's driver mode;
- mechanical stations, deterministic checks that need no mind.

When adapters exist, the engine becomes the long-lived runtime Joseph describes: it drains queues by invoking the adapter each slot names, and a circuit breaker halts a changeset whose internal ripples stop shrinking. The walking skeleton's measured lessons belong to those adapters, not to the kernel: a seat is a lease with one holder; per-member state lives with the holder, never in a process; rounds wait on submissions, never on deliveries.

## The founding event

*Proposed.*

A new store starts with one event, written by the steward. It admits the kernel's own kinds (kind, edge kind, transition, guard, decision-maker slot, projection, demand, changeset) and three transitions over them:

- **admit**, which adds a kind, edge kind, transition or projection. Its guard is the founding questions answered against real atoms. Its decision-maker slot is the steward, until he declares otherwise.
- **amend**, which changes one.
- **retire**, which removes one.

These can't admit themselves, which is why the first event is a founding act and every later definition enters by admission.

Nothing else is declared at founding. The first real work supplies the rest. For ASF, that work is crossing one small cluster with the write-refusal hook on. Each refusal ("no kind for this", "no transition for that") becomes a demand, and each demand an admission request. ASF's system definition grows that way, one demand at a time, and each piece arrives with the instance that needed it.

## What carries from earlier work, and what doesn't

| From | Carried | Revised or set aside, and why |
|---|---|---|
| vsect 0.1 (`firmatum/utils/vsect/`) | content-addressed versions; checks bound to their basis; lapse with cause; append-only events; attested identity; claims visible across worktrees | **typed frontmatter** (`closed:`, `owner:`): set aside, it is where the tree drifted from the log, 189 files. **`item`**: set aside; it held at least six repairs under one word. **Taking in outside edits**: reversed, under invariant 1 |
| the archived design (`aat-v3/.archive/msc/vsect/`) | one fate, one atom; standing computed; evidence lines and rungs; influx crossings; `decide`; split and merge; teaching that narrates; the founding of kinds by evidence | **rule 7** and **§3.4** ("vsect doesn't launch agents"): reversed. Its statement grammar and carving remain the open question below |
| RC1 (`firmatum/verisectorium/theory/influx/segments-model-rc1/`) | the declaration forms and the *when / allow / enabling* rule (`09`) as the definition's shape; typed edges and their propagation (`06`); rebut versus undercut; the founding questions (`10`); the substitution law, as a lint | syntax still undecided, as RC1 says itself |
| councils (`msc/councils/`) | transitions as queues drained by decision-makers; charters as records; admissibility computed from a transition table (`model/actions.json`) | councils themselves are adapters, outside the kernel |
| the flow model (`msc/councils/asf-flows.md`) | the steady-state conditions as computations over the log (work, residue, drain); "track a dimension when it changes a guard or a yield"; changesets; tension records as a state | its yields are priors for the dashboard, not part of the kernel |
| this week's measurements | **a check counts as evidence only if its actor did not write the version** (0 of 116 self-checks found anything); claim records as separate atoms (21 of 31 record checks broke or narrowed); maturity tracked from the start | |

## Candidate conservation laws

*Proposed by councils-46's predecessor on 2026-11-04 and not yet ratified. The kernel can check each one once it is.*

1. Every influx unit is in exactly one place at all times. That is the delete-test as a conservation law.
2. Nothing reaches canon without its trail. Invariants 1 and 6 make this hold by construction.
3. A projection never asserts more than the version it was generated from.

## Open

- **The definition's syntax.** udon, YAML, or another. RC1 leaves it open; the interface above uses YAML only for illustration.
- **Statement grain.** How an atom smaller than a file is addressed, and which transition carves a source into atoms. Line B's five crossings are the evidence: in `scope-adaptive-system`, one Definition paragraph held six commitments that fail separately, and one `exact` tag covered nine items. This is the first demand ASF will make.
- **How projected files read.** They should stay plain markdown a person can read. Whether they also carry generated marks (a refuted atom's banner, `core.md` §10) is a projection choice.
- **Partial reads.** Whether a verifier may be shown part of an atom (the archived design's Q2). It decides how provenance reads are recorded for `verify` briefs later.
- **Who decides admissions at first.** The founding event names the steward. Whether he delegates some kinds of admission, and to whom, is his.
- **Storage.** Git as the transport and the history beneath the log, as vsect 0.1 does, or something else.

## What this draft is not

It is not ASF's system definition, not an adapter, and not a council design. It names no ASF kinds beyond using them as examples. Where it reaches past what Joseph has said, it says *proposed*, and the checker's job is to find where a proposal smuggles in something the kernel doesn't need, or misses something it does.
