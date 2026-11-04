# ASF's system model

*Iteration 1, by councils-46 (2026-11-04), against `KERNEL.md` draft 2 (arch `6f0b16c`). Handed to the vsect lead for iteration 2 on both files, under Joseph's arrangement:*

> *take asf-system-model as far as it can go with the current KERNEL-- then make some modifications if needed and have the other agent do the next iteration on *both*. … Getting ASF to work from ASF's perspective (which both of you need to be representing) is what will push KERNEL into what it needs to be.*

*He added, while I was building it: "get it started and as uncertainty grows-- hand it over". That's where this is. The vsect lead knows ASF better than I do. I'd expect parts of this to be wrong in ways that are obvious to him.*

## The files

- **`model.yaml`** is the model, as data: lanes, kinds of atom (stocks), transitions (stations) with their slots and drainers, sinks, and the flow gains, each tagged measured, estimate, guess or proposed.
- **`render.py`** draws `out/asf-system.svg` and writes `out/report.md`, which holds the structural checks and the flow computation. Run it with `python3 render.py`. It needs PyYAML, numpy and graphviz.

The picture follows Joseph's diagram, with lanes for influx, working, canon, explanation and application, plus lanes for what leaves and for the sinks:
- **ellipses** are kinds of atom, as Petri places;
- **white boxes** are judgment stations;
- **yellow boxes** are decision points, where importance, and later a charter, matter;
- **grey boxes** are mechanical stations;
- **a red border** on a station means no drainer;
- **red dashed edges** run upstream, as feedback.

In place of Kendall's A/S/c, each station carries a slot, a drainer, and its load per source of influx. These are timeless, as Joseph asked.

*Known flaw:* graphviz stacks the lanes vertically, not left to right. A swimlane layout that really keeps lanes in columns probably needs a hand layout or a different tool.

## What the model already says

**Structural (from `out/report.md`):**
- **24 of 28 stations have no drainer.** That is the drain condition failing exactly where it failed in ASF. In the model this is honest, not a defect: nobody is assigned yet.
- **Nine decision points.** In the order the model lists them:
  - `unit.cross`;
  - `question.answer`;
  - `changeset.commit`;
  - `changeset.halt`, which routes to reframing;
  - `tension.resolve`;
  - `definition.admit`;
  - `decision.decide`;
  - `purpose.revise`, steward only;
  - `term.decide`.

  `unit.cross` is a decision, not clerical work. Every disposition of influx is a choice about what ASF holds.
- **Every kind has a way in and a way out,** after two fixes the checks found in my own first pass: claim records and arrangements had no exit.

**Flows (illustrative, and dominated by guesses):**
- **The sprint as it ran has $\rho \approx 1.45$: work grows without bound.**
- **With the flow model's levers as founding rules, $\rho \approx 1.00$, only borderline.** The levers are one independent check per landing, restatements bound to versions, and the reality edge switched on.

  Under these guesses, what holds it at 1 is:
  - the lapse cascade, 6 × 0.03;
  - unbound restatement churn;
  - strengthenings re-entering as internal influx.

  The guesses that decide whether the founding rules are enough are all restatement and strengthening gains. Those are what to measure first, ahead of anything else.

## What ASF demands of the kernel, as I see it

Each of these is something ASF can't do under draft 2. Each is a candidate for the kernel or for ASF's own definition, and the lead should judge which.

1. **Arrangements.** ASF's topical spine is its outline (RC1 `10`: "topic → outline · dependency → edges · failure → record boundaries · adjudication → kind & dimensions"). A segment file, at statement grain, is an ordered arrangement of atoms. Draft 2 has renderings, but a rendering needs an authored order: which atoms, in which file, in what sequence, with gap rows for what's planned. That order is authored state, so it's a record, and every store with readers has one. I've marked it in red in the picture as a demand on the kernel.
2. **Store addressing and cross-store edges.**
   - ASF's influx comes from logos, vivarium and udon.
   - Its set-asides go to them.
   - Vivarium holds 2,881 references to ASF slugs.

   Draft 2's set-aside names "the store or system it belongs to", but there's no store mapping. The archived design's §12 had one.
3. **Names versus atoms.** Joseph's slug decision ("a simple mapping made available is sufficient") needs a displayed id that can change while the atom doesn't. The archived design had this: an atom is its mint event, and its id is a name. Draft 2 doesn't say. The old-slug → fate mapping is then a projection.
4. **Mechanical actors.**
   - ASF's coherence checks are deterministic tools: notation consistency, headline bindings, rendering equals log.
   - The slot is "a predicate over the attested actor", and draft 2's actor is an agent.
   - A tool run needs to be an attested actor too, with the tool's version as its era.
5. **Guards that read projections.** "A root edit must be inside a changeset" is a guard over computed fan-out (transitive dependents). That's probably already allowed, since guards read the log, but it's worth saying that guards may cite declared projections.
6. **Importance has a computed part in ASF.** What rests on a claim, meaning its fan-out, is structural importance. Purpose-relevance is declared. Draft 2's decision lapse ("importance, recomputed from what now rests on it") fits this. ASF just shows how the two parts combine.

## Where I'm unsure, and would like the lead's eye first

- **The kinds in the canon lane.** I took them from the archived `model.md` and ASF's segment prefixes (def, der, deriv, disc, result, form, scope, hyp, impl, …). Whether `disc-*` and `impl-*` are claims, teaching or arrangements is the statement-grain question, and Line B's evidence is the lead's more than mine.
- **Where checks live.** Is `check` an atom, or an event that moves a claim's evidence line? I made it a stock so its lapse could be drawn. It may be better as an event, with the lapse as a projection.
- **Every flow gain marked guess.**
- **What the canon lane's "citable standing" means per kind.** Draft 2 asks each kind to declare it. I haven't.
- **What reaches "pedagogy" and what is teaching.** H.4's pedagogy home, keyed by slug and chapter, may be its own store.

*I'm staying available for questions, and to take the next turn.*

---

## Iteration 2 (the vsect lead, session `0903e35e`, 2026-11-04)

*Changes to `model.yaml` and `render.py`. The report is regenerated. Yellow rounded boxes for decision points are kept, per Joseph.*

### A correction to the flows: a changeset's ripples are its own

Iteration 1 counted every lapse and every stale restatement as a new finding re-entering the global queues. Under the founding rules, though, an edit with dependents is a changeset. The confirms it causes and the restatements it rebinds are the change's own propagation, settled inside it before commit. That is the reason changesets exist (`asf-flows.md` §6).

The model now has three regimes. Strengthenings re-entering as influx are also marked **discovery** and reported apart from **rework**: a store that keeps finding stronger results is not failing to converge.

| Regime | $\rho$ | Rework only |
|---|---|---|
| sprint | 1.45 | 1.43 |
| founding rules, no changesets | 1.00 | 0.97 |
| founding rules, edits with dependents as changesets | **0.83** | **0.76** |

The third row is a limiting case: it assumes every edit with dependents is a changeset. Every gain is still mostly guessed.

What remains near the margin is the core loop: finding → revision → one independent check, which finds something 42% of the time. That 0.42 is a first-check yield measured on sprint text. Whether a revision that only lands findings is as defect-prone as fresh text is exactly what the untried land-only experiment measures, so **it is the measurement to make first**. Ahead of the restatement gains, because changesets absorb those.

### The six demands on the kernel

All six belong in the kernel, by its own test (`KERNEL.md`, Scope). Each is in draft 3.

1. **Arrangement: kernel.** Invariant 1 makes every file a rendering, and a rendering needs an authored composition: which atoms, in what order, with gap rows. So arrangement is what the engine itself needs to render anything. It adds no claims (`model.md`: "a view selects and orders records and adds no claims"). ASF's outline is an arrangement, and so is a segment file.
2. **Store addressing: kernel, minimal.** Intake needs to say where a source came from, and a set-aside needs to say where a unit goes. A store declares the other stores it names; an edge or a provenance may then designate `store/kind:slug`. Cross-store *cascade* (vivarium learning that an ASF claim changed) is left open.
3. **Names versus atoms: kernel.** An atom's identity is its minting event; its name is a label with a history. Joseph's old-slug → fate mapping is then a projection over name events. Nothing extra has to be maintained.
4. **Mechanical actors: kernel.** The attested actor gains a second form: a tool at a version, invoked by an attested agent or person. The tool's version is the event's era, so a result expires when the tool changes.
5. **Guards may read declared projections: kernel**, stated explicitly. "A root edit must be inside a changeset" is a guard over computed reach.
6. **Importance, declared and computed: kernel.** A decision records its declared stakes and its computed reach (what rests on it). It lapses when either rises.

### Answers to the questions this note asked

- **The kinds in the canon lane.** An ASF prefix is only a hint; a kind belongs to each atom. The mapping I'd start from, now in the claim stock's note:
  - `def` → definition;
  - `scope` → scope;
  - `post` → postulate;
  - `form` → choice;
  - `der`, `deriv`, `result`, `obs` → claim, at the derived or empirical stage;
  - `hyp`, `sketch` → claim, at an earlier stage;
  - `disc` → recognition or teaching, decided per atom;
  - `impl`, `intro`, `example` → teaching or arrangement.

  Line B's evidence says statements also hide in Epistemic Status and Discussion sections, so carving judges each paragraph, not each prefix.
- **Checks: an event, not an atom.** A check is the act of securing a claim's version on an evidence line, and its lapse is a projection, which is how vsect 0.1 already works. What a checker *writes* is an atom: a **review**, which can itself be wrong. The `check` stock is relabeled to say so. Its id is kept, because the flows count checks as acts.
- **"Citable standing" per kind.** It is still undeclared, and it should be, until the first crossing demands it. A guess for claims: at least one independent check holding on the current version.
- **Pedagogy as a store of its own.** Plausibly yes, with store addressing (demand 2) linking it back. Left for the ASF model's next turn.

### Left for the next turn

- **The layout.** graphviz can't keep clusters in columns. A small hand-written SVG layout, one column per lane with feedback curving back to the left, is the fix I'd make next.
- **Drainers.** Still none, honestly.
- **Gains to measure, in order:**
  1. the land-only experiment;
  2. restatements per substantive change, countable now from ASF's front surfaces;
  3. strengthening yield per check.
