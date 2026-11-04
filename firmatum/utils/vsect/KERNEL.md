# vsect: the kernel (draft 3)

*Draft 3 (the vsect lead, after iteration 1 of the ASF system model) admits the six demands that model raised, each by the Scope test below: arrangement, store addressing, names versus atoms, tool actors, guards that read projections, and importance both declared and computed. It also makes check an event and review an atom. The reasons are in `msc/asf-system/ASF-SYSTEM-MODEL.md`, iteration 2.*

*To Joseph, and to councils-46, who checked draft 1. From Claude Opus 5.5 (session `0903e35e`), 2026-11-04.*

*Draft 2 replaces draft 1 (`ecf7f91`). It answers the eight findings in `KERNEL-check-1.md`, beside this file, where each answer says what changed.*

*Under Joseph's arrangement of the same day, this file and an ASF system model are worked in turns by councils-46 and me. His words: "Getting ASF to work from ASF's perspective (which both of you need to be representing) is what will push KERNEL into what it needs to be." So the line between kernel and store is drawn here provisionally. Anything domain-shaped is listed as a candidate, for the ASF model to argue about.*

*Each section marks what it states: **decided** (Joseph's words, quoted), **carried** (from earlier work that this week's evidence didn't overturn, source named) or **proposed** (mine).*

## What this is

vsect is an engine that holds a store's records and is the only thing that changes them. A store's **system definition** (its kinds of record, the transitions each kind allows, their guards, and who may perform each) is itself made of records in the store, so the system changes itself through its own transitions. Every act enters as a request to perform a transition, is checked against the definition, and lands as an event in an append-only log. The files a person reads are a rendering of that log.

Crossing any boundary is a change of state on one record, never a copy. "Working" and "canon" are not places in this model. They are computed standings over the same atoms. A store declares, per kind, what standing makes an atom citable as canon.

## Scope

*Proposed.* The test for what belongs here: true of every store, or needed for the engine to work and to change its own definition.

**In scope:**
- the engine's invariants;
- the meta-model: what declaring a kind, edge kind, transition or projection means (the definition language's semantics, not its syntax);
- what an event records;
- the kinds the engine itself needs: the definition's own records, demand, changeset, source and unit (every store has an intake), decision (admitting a kind is a decision), and arrangement (rendering any file needs an authored composition);
- naming and addressing: an atom's identity, its names, and other stores;
- the interface's shape;
- the dispatch contract;
- the computations over the log;
- the founding event.

**Out of scope:**
- **ASF's system definition:** claims, statement grammar, findings, explanatory and application atoms, and the transitions ASF needs.
- **Every adapter:** agent configurations, councils, driver mode, the implementations of mechanical stations.
- **When queues drain**, and which decision-maker each transition needs.
- **The yields and priors** from the flow model.
- **The definition's syntax, the implementation language and storage.**
- **ASF's content,** which is influx.

## Joseph's words this rests on

*Decided.*

- **Only through vsect.** *"AGENTS SHOULD NOT BE ABLE TO GO IN AND MANUALLY CHANGE CANON OR ATOMS. ONLY THROUGH VSECT!!! If the model and the tool aren't able to accomodate what they want to do yet-- that's exactly how we know what to flesh out more at any given point."*
- **An engine with an evolvable system.**
  - *"there can be a vsect tool that is a general engine, a much bigger 'system' definition / model than we suspected earlier that is also evolvable (but doesn't need to be, especially at first) via vsect"*;
  - *"new ideal vsect from the beginning, which can draw on anything principled from prior efforts"*.
- **A runtime, eventually.**
  - *"it should be thought of as a runtime or at least a long-lived process where influx is introduced and ripples and magnifies and settles (or gets interrupted and halts after time)"*;
  - *"calling vsect manually will start to be phased out in time where the correct agent configuration or council charter+agenda-template are specified for various atom mutation paths"*.
- **But vsect doesn't model councils, especially not at first.**
  - *"treat each vsect transition-edge as a queue that is drained by a 'decision-maker' and not specify a council or agent etc."*;
  - *"the processor/adjudicator/etc.-- is just one simple 'adjudicator' node/station that … will be either an agent or a well-tuned committee or human in the loop or anything else we decide *later*"*;
  - *"especially not initially"*.
- **Steady state without wall time.**
  - *"I don't want to use queueing models with traditional wall-time-- which to us is just a proxy."*;
  - the aim is the *"relative throughput necessary for steady-state"*.
- **The old process is evidence, not a template.** On the state machine read off the sprint's trail: *"I worry that the state machine empirically determined will be both grossly insufficient and seriously over-complicated at the same time..."*
- **The interface.** Anthropic's `ant` CLI *"is a great example of the right affordances and ergonomics"*: `ant messages create --format json <<'YAML' … YAML | jq …`.
- **Decisions carry their importance.**
  - *"perceived importance at the time"* against *"changed importance at later times"*;
  - *"There is different level of unity required for different levels of importance"*;
  - assumptions are *"an official part of a closed / finished thing"*.

## The invariants

1. **The engine is the only writer.** *Proposed.* Every file under a store equals the engine's rendering of its log. `vsect check` recomputes the rendering and reports any difference, whatever made it. A PreToolUse hook refuses writes under a store's paths at the moment they are attempted, and says how to make the request instead. The check is the guarantee; the hook is the signal. This reverses the archived design's rule 7.
2. **The log is append-only.** *Carried*, from vsect 0.1 and the archived design. A correction is a new event that names the one it corrects.
3. **Versions are content addresses.** *Carried.* A check binds to the versions of everything it rests on, and lapses, with the cause named, when any of them changes. An exact return to an earlier text restores its bindings (measured by Line A).
4. **Standing is computed, never typed.** *Carried.* No request body may carry a status, rung or standing word.
5. **Every event carries an attested actor.** *Carried*, from the identity work: agent key, parent, agent type, session, and what that record rests on. An actor is a mind (an agent or a person) or a **tool at a version** invoked by one. A tool's version is its event's era, so a mechanical result expires when the tool changes (*proposed*, ASF model demand 4).
6. **Every change is a transition of the definition.** *Proposed.* An act the definition can't express is refused, and the refusal itself is recorded as a demand, through the one transition that is always admissible (`demand.record`).
7. **The definition is records.** *Proposed.* Kinds, edge kinds, transitions, projections and slots change only through the definition's own transitions. The command surface is generated from them.
8. **Every stock has an exit.** *Proposed.* Every kind declares its terminal states, and the engine reports any population that grows with no transition able to leave it.

## What an event records

*Proposed. Each field is here because something the kernel computes needs it (check finding 4).*

| Field | What it says | What needs it |
|---|---|---|
| transition | kind × verb | everything |
| atom, version | what was acted on, at which text | lapse; confirmation |
| body | the validated request | rendering; history |
| actor | attested identity (invariant 5) | independence; the reviser refusal; drainers |
| caused-by | the events or requests this one follows from | derived transitions; the yield matrix; generations for the circuit breaker |
| basis | the version vector the act rests on | lapse |
| provenance reads | what the actor saw, where attested | independence; priming |
| criterion, committed | what a securing act judged by, and whether that was fixed before the outcome was seen (*carried*, RC1 Q5 and the archived design's `committed: before\|after`) | the evidence line's rungs |
| era | the code or world version a run belongs to, where its line declares eras | results that expire visibly |
| changeset | the transaction it belongs to, if any | commit; abort; generations |
| importance | for decisions, the importance it was made at | the decision lapse |
| outcome | what came of it | standing |

## The nouns

*Proposed, from the archived design's model and RC1, except where marked.*

- **Store, event, atom.** An atom is the unit with one fate (*carried*, `model.md`). A file is a container, and a rendering.
- **Identity and name.** An atom's identity is its minting event. Its **name** (the slug) is a label with a history, changed by its own event. Old names keep resolving, and Joseph's old-slug → fate mapping (renamed, split, merged, retired) is a projection over those events, so nothing extra is maintained (*proposed*, ASF model demand 3).
- **Store address.** A store declares the other stores it names. An edge, a source's provenance or a set-aside may then designate `store/kind:slug` (*carried* in shape from the archived design's §12, *proposed* here). Whether a change in one store cascades into another is open.
- **Arrangement.** An authored composition of atoms: which ones, in what order, and the gap rows for what is planned. Every rendering is computed from one; a segment file and an outline are both arrangements. It adds no claims (*carried*, `model.md`: "a view selects and orders records and adds no claims"). Because it is authored, it is an atom, with its own transitions (*proposed*, ASF model demand 1).
- **Kind.** A declared type of atom, admitted only when it *"fails differently, and is repaired differently, from every other kind"* (*carried*, `model.md`). It declares:
  - its fields, each with a schema;
  - its dimensions and their rungs;
  - its terminal states;
  - the standing at which an atom counts as citable.
- **Edge kind.** *Carried*, from RC1 `06-EDGES.md`. A typed relation with a meaning, the transitions that consume it, how far it propagates, and an **intended cardinality** that gives a miss its meaning. An edge either **designates** (a maintained binding, which fails by lapse) or **describes** (a match run at use, which drifts with the world, and must carry something to check against). The kinds 06 derives:
  - **supports**, which signals upward but never carries strength along a chain;
  - **rebuts** (attacks a conclusion) and **undercuts** (attacks a support), which a refutation and a weakening route differently;
  - **supersedes**, typed as *revised-by*, *invalidated-by* or *alternate-of*;
  - **seeded-by**, transitive, because descent is;
  - **narrates**, which resets the narrator's freshness when its referent changes;
  - **restates**, the same claim on a different carrier, where exact restatements compose and near ones don't chain.

  Binding a restatement to the exact version it restates is the flow model's lever (`asf-flows.md` §5), and a store can declare it per edge.
- **Transition.** One act a kind allows: kind × verb. It declares:
  - the schema of its request body;
  - its **guard** over the atom's state, its edges, the log, the actor, and any declared projection. For example, "a root edit must be inside a changeset" reads computed reach;
  - its **evidence requirement**;
  - **the dimension it moves**, and **what its close writes** (check finding 1). These are what a later charter is derived from;
  - its **decision-maker slot**;
  - its **drainer**;
  - its effects, meaning the events it writes and the transitions it derives.
- **Decision-maker slot and drainer.**
  - **The slot** says who may perform the transition, as a predicate over the attested actor: anyone; anyone who did not write this version; the steward; a named adapter. The kernel checks it, and doesn't know whether the actor is an agent, a council or a person.
  - **The drainer** says who is answerable for the queue being drained: an agent key, or the steward. It lapses visibly when that agent's session ends without a handover (*carried*, from the other lead session's plan notes, after Joseph: *"a KR can't own anything. A KR needs an owner and the owner needs to be an agent, not an index"*).
- **Queue.** The atoms for which a transition is admissible and not yet performed. Computed, never stored.
- **Source and unit.** Every store has an intake.
  - A **source** is brought in with its provenance (`source.bring`).
  - It is divided into **units**, each crossed exactly once (`unit.cross`). A crossing ends in a state change on an atom the store holds, or in a sink: already present, duplicate, superseded, set aside (with the store or system it belongs to), or rejected.
  - A source **retires** when every unit is terminal. That is the delete-test, computed.
- **Decision.** A recorded choice, with:
  - its grounds;
  - its **assumptions as edges** to the records they rest on;
  - **the importance it was made at**, in two parts: the stakes declared by whoever decides, and the reach computed from what rests on it;
  - what would reopen it.

  It lapses, as "under-secured", when an assumption changes or either part of its importance rises above what it was made at. Lapsing asks for the decision to be secured again at the new importance; it doesn't reverse it. Admitting a kind is a decision.
- **Changeset.** A transaction.
  - **Opening.** `changeset.open` takes an intent, an invariant (what must be true everywhere afterward) and a scope, which can grow. While it is open, it is a branch of the store where the same engine writes. Drafting there is cheap and attributed, and its ripples stay inside.
  - **Committing.** `changeset.commit` has its own guard, a judgment on the whole: that the change makes the store more true, coherence included (Joseph's definition). At commit every branch event is re-validated against main. A request whose `version:` main has moved past is refused as a conflict, and the changeset rebases or aborts.
  - **The neighborhood.** A changeset carries the source units it disposes, so its neighborhood drains with it.
  - **Ending otherwise.** `changeset.abort` leaves main as it was. `changeset.halt` stops a changeset whose ripples aren't settling (*proposed*, below) and routes it to reframing: a question about whether the problem is ill-formed or two ideas are conflated.
  - **One owner** from open to close.

  What must fit in one mind is the intent, the invariant and the judgment residue, never the whole impact. That is why the kernel offers **verified transforms**, which the engine checks itself ("this span moved; the rest is byte-identical"), and **basis reads at span grain**.
- **Two kinds of read.**
  - A **provenance read** is what a mind saw. Independence and priming depend on it.
  - A **basis read** is what a verdict rests on. Lapse and confirmation depend on it.
- **Projection and rendering.**
  - A **projection** is a declared computation over the log: a standing, a count, a queue (*carried*, RC1 `09`).
  - A **rendering** is the text of a file, computed from the log.

  Neither holds state of its own. The content Joseph's diagram calls "Projections/Pedagogy/Implications" is atoms, the explanatory and application kinds a store admits. The kernel has no name for it.
- **Demand.** A refused act: what was attempted, by whom, on what, and why it was refused.
  - `demand.record` is built in, always admissible, and written by the refusal itself.
  - `demand.dispose` closes a demand as answered (by naming the admission or transition that now allows the act), declined with a reason, or duplicate.

## The founding event, and the kernel's own transitions

*Proposed.* A store starts with one event, written by the steward. It admits the kernel's kinds and these transitions:

| Kind | Transitions |
|---|---|
| definition records (kind, edge kind, transition, projection) | `admit`, `amend`, `retire` |
| demand | `record` (built in, always admissible), `dispose` |
| changeset | `open`, `commit`, `abort`, `halt` |
| source, unit | `source.bring`, `unit.cross` |
| decision | `decide`, `reopen` |
| arrangement | `arrange`, `revise`, `retire` |
| any atom | `rename` |

These can't admit themselves, which is why the first event is a founding act.

**The guard on `admit`.** It is the founding questions (RC1 `10`), answered against the material whose demand raised the admission. At founding there are no atoms, so the questions are always answered against the influx that asked for the kind. RC1's Q0 ("Standing": who may bring such a record into being) and Q8 ("Certification": what a consumer may conclude from its state alone) would also serve as the kernel's own acceptance tests for any admitted kind. *That is my guess, built on councils-46's.*

**After founding, nothing else is declared.** ASF's definition arrives by demand, starting from the first crossing with the write-refusal hook on. Each refusal becomes a demand, and each demand an admission.

## The computations the kernel offers

*Proposed. Each is a projection over the log, available to any store. The yields that feed them are the store's data, not the kernel's.*

- **The three steady-state conditions** (from `asf-flows.md` §5):
  - **work:** the loop gain's spectral radius below 1, computed from `caused-by` chains as yields per unit of influx;
  - **residue:** every population has an exit (invariant 8);
  - **drain:** every transition has a live drainer and a nonzero share of acts.
- **The catalog against the trail.** Acts that happened but match no transition are gaps. Transitions never performed are dead or blocked. This is the use Joseph's caution leaves for the empirical record: it checks the designed catalog and never becomes its source.
- **Generations within a changeset**, from `caused-by`. A circuit breaker counts them per kind of finding, which a store declares, because raw counts fire on restatement churn while the mathematics holds (`asf-flows.md` §7). Breaker limits are set by importance.
- **The decision lapse**: importance at decision, against importance now.

## The interface

*Proposed, after `ant`.*

```bash
vsect <kind> <transition> [--dry-run] [--format json|text] <<'YAML'
atom: <kind>:<slug>        # omitted when minting
version: <hash>            # what the request was composed against; stale means conflict
body:
  …                        # fields the transition's schema declares; prose as block scalars
YAML
```

- **Nothing is written until the body validates and the guard admits it.**
- **The result is structured:** the event, the new version, the checks lapsed, and the queues entered or left. A refusal carries its reason as data, and the demand it recorded.
- **One interface for every writer:** a person, an agent, an adapter.
- **Read verbs** (`show`, `why`, `open`, `check`, `flows`) use the same addressing.

## Dispatch

*Proposed, reconciling Joseph's statements.* The kernel defines the queue, the slot, the drainer and the **dispatch contract**:
- **what the engine hands a decision-maker:** the atom, its version, the transition, the request schema, and the provenance it may read;
- **what it accepts back:** a request body, from an attested actor.

At first every slot is filled by whoever runs the verb. Adapters come later, outside the kernel, each meeting the contract:
- agent configurations per mutation path;
- council charters and agendas;
- mechanical stations.

With adapters in place, the engine becomes the long-lived runtime Joseph described. The walking skeleton's measured lessons are adapters' lessons: a seat is a lease with one holder; per-member state lives with the holder; rounds wait on submissions, never deliveries.

## Candidate kinds, for a store to admit

*Proposed. These are not the kernel. Each is here because it came up in the work, and the ASF system model decides whether ASF admits it.*
- **Claim** at statement grain, with its carving transition. Line B's evidence: in `scope-adaptive-system`, one Definition paragraph held six commitments that fail separately.
- **Finding, typed by kind.** The circuit breaker needs the type.
- **Question**, with a closer.
- **Review**, what a checker sends back, which can itself be wrong. Each **check** in it is an event that secures a claim's version on an evidence line, and its lapse is a projection. A check is an act, not an atom.
- **Claim record**, separate from its claim, with a `describes` edge (Line A: 21 of 31 record checks broke or narrowed).
- **Explanatory and application atoms** (`restates`, `narrates`; uses against reality).
- **Tension**, a standing contradiction between well-supported claims. Its dependents are conditioned on it, and it is re-evaluated only when bearing evidence arrives. A period-2 oscillation converts into one.

## What carries from earlier work

| From | Carried | Revised or set aside |
|---|---|---|
| vsect 0.1 | content-addressed versions; checks bound to their basis; lapse with cause; append-only events; attested identity; claims across worktrees | typed frontmatter, where the tree drifted (189 files); `item`, which held at least six repairs under one word; taking in outside edits, reversed by invariant 1 |
| the archived design (`aat-v3/.archive/msc/vsect/`) | one fate, one atom; computed standing; lines and rungs; `committed: before\|after`; crossings; `decide`; split and merge; kinds founded on evidence | rule 7 and §3.4, reversed |
| RC1 | declaration forms and the *when / allow / enabling* rule (`09`); typed edges as above (`06`); the founding questions (`10`); movers declared per dimension (Q5) | syntax open, as RC1 itself leaves it |
| councils | transitions as queues; charters as records; admissibility from a transition table | councils are adapters |
| the flow model | the three steady-state conditions; changesets, the breaker, tension records; "track a dimension when it changes a guard or a yield" | yields are store data |
| this week's measurements | a check counts as evidence only if its actor did not write the version (0 of 116 self-checks found anything); claim records separate; maturity tracked from the start | |

## Candidate conservation laws

*Proposed by councils-46's predecessor, now stated in the kernel's terms. Joseph to ratify.*
1. Every unit is in exactly one state at all times, and a source retires only when all its units are terminal.
2. Nothing becomes citable as canon without its trail. Invariants 1 and 6 make this hold by construction.
3. A rendering never asserts more than the versions it was generated from.

## Open

- The definition's syntax.
- Statement grain: the first demand ASF will make.
- Whether renderings carry generated marks.
- Partial reads (the archived design's Q2).
- Who decides admissions after founding.
- Storage.
- How a commit's whole-change guard is judged. That is a decision-maker's question, so it waits on the slot being filled.
