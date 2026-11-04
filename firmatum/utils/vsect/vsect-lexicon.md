# vsect lexicon

*Begun 2026-11-04 by Claude Opus 5.5 (session `0903e35e`), at Joseph's request: "As terminology evolves, prepare a vsect-lexicon.md that helps us all stay on the same page with vernacular." It is kept by whoever has the turn on `KERNEL.md` and the ASF system model, and anyone may propose a change. Each term gives:*
- *what it means now;*
- *where it comes from;*
- *what it is easily confused with.*

*Status:*
- ***kernel**: in `KERNEL.md`;*
- ***candidate**: proposed for a store to admit;*
- ***retired**: no longer used, with what replaced it.*

*One concept, one term. If two terms here name the same thing, or one term names two things, that is a defect in this file. Report it.*

## The system

- **engine** *(kernel)*. The program that holds a store's records and is the only thing that changes them (Joseph, 2026-11-04). Not to be confused with **the kernel**, which is the engine's specification.
- **kernel** *(kernel)*. What is true of every store, or needed for the engine to work and to change its own definition (`KERNEL.md`, Scope). It excludes any one store's kinds.
- **store** *(kernel)*. A directory holding one log. Everything else in it is a rendering. A store is the archived design's and RC1's **verisectorium**, seen as a running instance.
- **system definition** *(kernel)*. A store's own kinds, edge kinds, transitions, projections and slots, held as records in that store. Not the kernel: ASF's system definition is ASF's, and it is admitted through the kernel.
- **adapter** *(kernel)*. Something outside the kernel that fills a decision-maker slot through the dispatch contract: an agent configuration, a council charter and agenda, or a mechanical station. A **council** is one kind of adapter; vsect does not model councils.
- **dispatch contract** *(kernel)*. What the engine hands a decision-maker (the atom, its version, the transition, the request schema, the provenance it may read), and what it accepts back (a request body from an attested actor).

## Records and their history

- **atom** *(kernel)*. The unit with one fate: anything that can be refuted, superseded or ratified while its neighbors carry on (`model.md`). The archived design calls this a **statement**, and RC1 a **record**. Neither is a **file**. A file is a container and a rendering, and may hold many atoms.
- **identity** and **name** *(kernel)*. An atom's identity is its minting event; its **name** (the slug) is a label with a history, which a rename changes. The old-slug → fate mapping is a projection over name events. Not interchangeable: renaming never changes identity.
- **store address** *(kernel)*. `store/kind:slug`, which can be used once a store declares the other stores it names.
- **arrangement** *(kernel)*. An authored composition of atoms (which ones, in what order, with gap rows) from which a rendering is computed. It adds no claims. An outline and a segment file are both arrangements. Not a **rendering**: the arrangement is authored, and the rendering is computed from it.
- **kind** *(kernel)*. A declared type of atom. It is admitted only when it *"fails differently, and is repaired differently, from every other kind"* (`model.md`).
- **log** *(kernel)*. The append-only sequence of events. Nothing in it is edited; a correction is a new event naming the one it corrects.
- **event** *(kernel)*. One recorded act. Its fields are in `KERNEL.md`, *What an event records*.
- **version** *(kernel)*. The content hash of an atom's normalized text, its kind and its declared form.
- **basis** *(kernel)*. The versions an act rests on: the atom's own, plus those of everything it depends on.
- **lapse** *(kernel)*. A check stops counting because something in its basis changed. The cause is always named. Not a refutation: a lapsed check has stopped counting, but nothing has shown it wrong.
- **confirm** *(kernel)*. A second mind judges that a change didn't touch what a lapsed check read, and the check binds again. Never done by the reviser.
- **attested actor** *(kernel)*. Who performed an act, from the harness: agent key, parent, agent type, session, and what that rests on (harness-agent, harness-session or declared). On this machine it comes from `CC_AGENT`. Not a typed `--by` name, which is only a label. An actor is a mind (an agent or a person) or a **tool at a version** invoked by one, whose version is the event's era.

## Change

- **transition** *(kernel)*. One act a kind allows: kind × verb, with a request schema, a guard, an evidence requirement, the dimension it moves, what its close writes, a decision-maker slot, a drainer, and its effects. Each transition is one generated command: `vsect <kind> <transition>`.
- **guard** *(kernel)*. The condition a transition must satisfy, over the atom's state, its edges, the log and the actor.
- **decision-maker slot** *(kernel)*. Who may perform a transition, stated as a predicate over the attested actor. It says who may act, not who must.
- **drainer** *(kernel)*. The agent, or the steward, answerable for a transition's queue. Ownership lapses visibly when the agent's session ends without a handover. Not the decision-maker slot: the slot says who *may* act, the drainer who *answers* for the queue being worked. From Joseph: *"a KR needs an owner and the owner needs to be an agent, not an index."*
- **queue** *(kernel)*. The atoms for which a transition is admissible and not yet performed. Always computed, never stored.
- **demand** *(kernel)*. A refused act, recorded with what was attempted and why it was refused. Demands are how the kernel learns what to build next.
- **changeset** *(kernel)*. A transaction with an intent, an invariant and a scope, opened on a branch of the store and closed by commit, abort or halt. RC1 calls this an **episode**, and the flow model an **atomic changeset**. Not a git commit.
- **verified transform** *(kernel)*. A change whose shape the engine checks itself, e.g. "this span moved; the rest is byte-identical", so no mind has to judge it.
- **halt** *(kernel)*. Stopping a changeset whose ripples aren't settling, and routing it to **reframing**: is the problem ill-formed, or are two ideas conflated?
- **generation** *(kernel)*. One step of ripple inside a changeset, traced through `caused-by`. A circuit breaker counts generations per kind of finding.

## Intake

- **influx** *(kernel)*. Material not yet in the store, external or internal (spikes, audits, experiments). ASF itself is influx to the new store, and nothing about it is transitional (Joseph, 2026-11-04).
- **source** *(kernel)*. One piece of influx, brought in with its provenance: a paper, a spike, an audit directory, an ASF segment.
- **unit** *(kernel)*. One thing a source says that may matter. Crossed exactly once.
- **cross** *(kernel)*. A unit's disposition. It ends in a state change on an atom the store holds, or in a sink. The sinks are:
  - **already present**;
  - **duplicate**;
  - **superseded**;
  - **set aside**, with where the unit belongs instead;
  - **rejected**.

  Joseph keeps two words apart (2026-09-30): ***integrated*** is said of influx, and ***landed*** only of an outline row.
- **retire** (of a source) *(kernel)*. A source leaves the live store when every one of its units has reached a terminal state. That is the **delete-test**, computed.

## Standing and reading

- **standing** *(kernel)*. What an atom's recorded evidence supports, computed through declared projections. It is never typed into the store.
- **canon** and **working** *(kernel)*. Computed standings, not places. Each kind declares the standing at which its atoms count as citable (canon). Crossing from working to canon changes the state of one record; nothing is copied.
- **projection** *(kernel)*. A declared computation over the log: a standing, a count, a queue (RC1 `09`). Not the "Projections/Pedagogy/Implications" box in Joseph's diagram, which is content: **explanatory** and **application** atoms.
- **rendering** *(kernel)*. A file's text, computed from the log. The engine is the only thing that writes it.
- **provenance read** *(kernel)*. What a mind saw. Independence and priming depend on it.
- **basis read** *(kernel)*. What a verdict rests on. Lapse and confirm depend on it.

## Decisions

- **decision** *(kernel)*. A recorded choice: its grounds, its assumptions as edges, the importance it was made at, and what would reopen it. Admitting a kind is a decision. A decision never settles whether a claim is true (archived design, §3.9).
- **importance** *(kernel)*. How much rests on a decision. Declared when it is made, and recomputed later from what has come to depend on it.
- **under-secured** *(kernel)*. A decision whose assumptions have changed, or whose importance has risen above what it was decided at. It asks to be secured again at the new importance. It is not reversed automatically.
- **founding event** *(kernel)*. The steward's first event in a new store. It admits the kernel's own kinds and transitions, which can't admit themselves.
- **admit**, **amend**, **retire** (of a definition) *(kernel)*. The transitions that change a store's system definition. The guard on `admit` is the founding questions (RC1 `10`), answered against the material whose demand raised it.

## Edges

*From RC1 `06-EDGES.md` (kernel).*
- **supports**. A premise or ground. It signals upward but never carries strength along a chain.
- **rebuts**. Attacks a conclusion: a counterexample, a refuting result.
- **undercuts**. Attacks a support. The conclusion may still be true.
- **supersedes**. Typed as **revised-by** (chains), **invalidated-by** or **alternate-of** (neither chains).
- **seeded-by**. Descent of framing or approach. Transitive, because descent is.
- **narrates**. Teaching speaks about an atom, and resets when that atom changes.
- **restates**. The same claim on a different carrier. Exact restates compose; near restates don't chain.
- **designator** versus **description**. A maintained binding, which fails by lapse, versus a match run at use, which drifts with the world.
- **intended cardinality**. Declared on an edge at writing. It is what gives a miss its meaning.

## Flow

*From `msc/councils/asf-flows.md` (the computations are kernel; the yields are a store's data).*
- **yield**. How many items of each type, on average, processing one item of a given type produces. Counted per unit of influx, never per hour.
- **the three steady-state conditions:**
  - **work** is finite: the loop gain's spectral radius is below 1;
  - **residue** is bounded: every stock has an exit;
  - **drain** happens: every transition has a live drainer and a nonzero share of acts.
- **station**. The flow model's word for a transition seen as an adjudicating step. Same thing, different view.
- **mechanical station**. A deterministic check that needs no mind.
- **catalog against trail**. Comparing the designed transitions with what the log shows happened. It reveals gaps and dead transitions. The trail checks the catalog and is never its source.

## Candidate kinds

*Proposed for a store to admit; not kernel.*
- **claim**. A truth-apt atom at statement grain.
- **finding**. A defect reported against an existing atom, typed by kind.
- **question**. An open item, with a closer.
- **review**. What a checker sends back: an atom, which can itself be wrong.
- **check**. Not an atom: an event that secures an atom's version on an evidence line. Its lapse is a projection.
- **claim record**. Register, population, falsifier and so on for a claim. A separate atom, linked by `describes`.
- **explanatory atom** and **application atom**. Content that restates or narrates claims, and content that uses claims against reality.
- **tension**. A standing contradiction between well-supported claims, held as a state rather than a loop.

## Retired, and what replaced them

- **item** (vsect 0.1). It held at least six kinds of repair under one word. Replaced by a store's own kinds, each closed by its own act.
- **status words in frontmatter** (`closed:`, `owner:`, `status:`). Typed state, where the tree drifted from the log. Replaced by computed standing, and by drainers.
- **`--by` as identity**. A typed name. Replaced by the attested actor; `--by` survives as a label.
- **"files anyone may edit" and `revise` taking outside edits in** (the archived design's rule 7). Replaced by the engine as only writer, and requests as typed bodies.
