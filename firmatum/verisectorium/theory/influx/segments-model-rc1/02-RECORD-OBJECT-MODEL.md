# The record object-model

*Register: **proposed**. The model's logical objects, stated tightly; physical realization is a deployment matter. This file is deliberately minimal and grows only as the other files need objects it lacks — additions are logged in Working Notes. Style after the udon reference defs (one-line definitions; relations with cardinality; invariants only where they must hold; failure modes inside the definitions).*

**RECORD** — one atom of one kind: the unit of adjudication, under a stable identity nothing mutable participates in.

- rels: `of-kind` → KIND {1,1} · `has` → BODY {1,1}, WORKING-NOTES {0,1}, TRAIL {1,1}, COMPANION {0,N}
- invariants:
  - Identity is the slug; ordering, numbering, and location are never identity.
  - The adjudication grain and the record grain coincide — a record carrying two independently-adjudicable things is two records.
- fails by: the undeclared mixed composite — part claim, part decision, part history, adjudicable as none of them.

**KIND** — what a record *does* (its speech-act), individuated by failure-and-repair: two kinds are distinct exactly when they go wrong differently and route to different repairs.

- rels: `adjudicated-on` → DIMENSION {1,N} · `declares` → WRITE-SEMANTICS {1,1} (replacement | append-only)
- invariants:
  - Kind is stable for the record's life; honest movement of anything mutable never forces a rename.
  - Write semantics follow from the kind's truth-conditions, never from style: present-truth kinds replace; accounts and events append.
- fails by: a kind admitted without an answer to "what should go wrong with this, and what should repair it?"

**DIMENSION** — the unit of orthogonality in epistemic state: one independently-movable value with its own field, its own ladder, and its own movers, accumulating its own subrecords. *The thing that is orthogonal as it is created.*

- named: `namespace/dimension` — `evidence/derivation` · `evidence/testimony` · `evidence/carriage` · `decision/grounding` · `decision/accountability` · `decision/consultation` · `decision/falsifiers` · `salience/freshness` · `salience/disposition` · … The namespace is part of the *name*, not an object: pure organization, re-cuttable per deployment as a rename, bearing no invariants by construction. Any namespace-level value a consumer sees is a declared PROJECTION over member dimensions, never a stored field.
- rels: `moved-only-by` → EVENT-class {1,N} · `laddered-by` → LADDER {0,1}
- invariants:
  - Two values that can move independently never share a field or a ladder.
  - A dimension's value is moved only by its own movers; no dimension's value ever substitutes for another's.
  - The dimension-set is open: a new dimension is admitted by exhibiting an independently-movable value no existing dimension carries — never by enumeration being "done."
- fails by: **fusion** — the tell is the unrolled-permutation ladder (`ruled-groundless | consulted-independent-agent-grounded | …`), values that are cross-products of independently-movable parts; and by substitution (authority read as truth, endorsement as firing, chain as adequacy…).

**LINE** — one record's accumulating engagement with one dimension over time: the diachronic program (derive it · ratify it · measure it · check the instance · watch it fire), holding that dimension's subrecords as they accrue.

- rels: `on` → RECORD {1,1} · `of` → DIMENSION {1,1} · `emits` → EVENT {0,N}
- invariants:
  - A claim's Evidence-family line-set derives from its declared foundation rungs; the instrument follows the ground.
  - Lines may inform each other through question and design channels; a line's check corroborates only if its criterion was committed before contact with the outcome.
  - Two lines corroborate beyond the stronger only when their failure modes are independent — descent from one source defeats the independence whatever the method labels say.
- fails by: verdict-channel contamination — what-counts-as-passing adjusted after seeing the answer.

**EVENT** — one recorded securing act: *channel · actor (with authority) · criterion · criterion-commitment (pre-committed | post-hoc) · date · outcome · era-key where measured*.

- rels: `on` → RECORD or LINE {1,1} · appended to TRAIL
- invariants:
  - Append-only; corrections chain beneath, never overwrite.
  - An event names the criterion it was performed against, or it cannot later show whether the criterion is being bent.
- fails by: infidelity — omission, fabrication, misrepresentation — which unwarrants every status computed above it.

**TRAIL** — a record's append-only event history: the substrate every status stands on, and what an audit reads.

**PROJECTION** — any computed summary over trails: a status cell, an outline row, a composite strength.

- invariants:
  - Every status a consumer reads is a projection; hand-set status is inexpressible in the model.
  - Order-invariant: the same whatever order events arrived in.
  - A projection is declared (strongest-line | every-line-sustains | shown-independent-agreement | …); one unqualified scalar is not a default.
- fails by: the label-lie — a cell asserting what no events support, or surviving events that invalidated it.

**LADDER** — a dimension's ordered rungs with the channel vocabulary that moves between them and a ceiling law.

- invariants:
  - Rungs move only by named channels, signed — refutation channels are first-class, and a break that narrows scope is a result.
  - The ceiling is assignable from kind before work, and names the action that would raise the record toward it.
  - A certificate at any rung decays with its warrant: edit, referent change, era bump. One that cannot decay is a lie in waiting.

**ERA** — a registry-scoped fact: *key → current era*, so that measured results carry the era they were true under and an era bump is an event that expires them visibly.

**Store-scope objects** *(named, not yet developed)*: the **ambient prior** (calibrated external material, with per-record descent edges — the input to independence checks), and the coordination surfaces (in-flight work; reconciliation findings).

## Working Notes

- **Grown-as-needed discipline:** objects are added here only when 03–09 need one this file lacks; each addition logged with its demanding file. *Log:* DIMENSION re-grained to the orthogonality unit (2026-08-14, steward correction via 03 — the earlier cut had let a grouping carry ontology); a FAMILY object existed for one commit before the steward reduced it to a **namespace in the dimension's name** — organization that structurally cannot bear invariants; LINE re-related from foundation-rung to dimension in the same pass, resolving its overlap question. Deliberately absent so far: view/outline objects (carried by [[claim-outline-as-view]] and the cluster definition in theory canon — reuse, don't restate), edge objects (06's subject — typed relations land here once 06 adjudicates them), question/disposition objects (07's subject).
- **To-do, carried from the charter's earlier anatomy sketch:** the uniform-anatomy test per dimension (does each dimension's adjudication fit record+trail+projections without strain? — checked as failure-mode bursts land); crumb-routing as the working-notes → trail drain; max/floor/shown-independent-agreement as the named projection set; descent edges' minimal form; the era registry's key vocabulary.
- **Feedback loop noted:** the udon reference defs this file's style follows are themselves future records of a deployment this model will describe — when 09's language exists, those defs are among its first test articles.
- Open: whether WORKING-NOTES is an object or a body-part with editable-free semantics; whether TRAIL and the Fidelity dimension are one thing seen twice (substrate vs priced question) or genuinely two.
- **Forward, steward-noted (2026-08-14), deliberately not minted now:** a namespace is a strong indication of a desired **accumulator** — evidences, saliency-factors, trust — so the namespace-level thing may earn objecthood later *as* the accumulator its projection reads over; and the name-level cut affords cheap **sub-subdivision** (`evidence/testimony/…`) when a dimension's population wants finer grain. Both wait for a demanding file; the easy cut stands until then.
