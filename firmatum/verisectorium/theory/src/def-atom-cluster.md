---
slug: def-atom-cluster
form: definition
type-expected: definition
status: axiomatic
max: axiomatic
state: [drafted]
depends: [def-atom]
---

# Definition: The Atom Cluster

An atom is a cluster, not a blob: a present-truth body plus working notes, events, and companions — with distinct write-semantics per part, and views projecting subsets.

## Formal Expression

*[Definition (atom-cluster)]*

An atom ( [[def-atom]]) comprises up to four parts, each with its own write-semantics:

| Part | Carries | Write-semantics |
|---|---|---|
| **Body** | Present truth: the claim/term/precept itself, its formal expression, its epistemic status, its discussion | Replacement — the collision surface; always states current truth with correct bounds ( [[def-integration-replacement]]) |
| **Working notes** | Forward-work only: open pointers, regression guards, dead-end warnings | Editable free attachment — not canon; never backward narration |
| **Events** | Append-only trail: verifications, decisions, dispositions, calibrations — each with actor, date, criterion, outcome | Append-only; "latest wins" is an operational projection, the trail is never deleted |
| **Companions** | Supporting artifacts under the atom's identity: derivations, figures, payload sidecars, run records | Per-companion; declared derived-vs-authored ( [[form-generated-views]]) |

Views select atoms and **project** parts ( [[form-view-filters]]): a promoted view projects the body without working notes; an audit view may project events; the projection freedom is what lets every part stay with its atom permanently — the cluster is how atoms are evergreen.

## Epistemic Status

Definitional in its cluster-ness; formulation-flavored in its part-inventory. That atoms *are* clusters is observed across independent-pressure instances: asf's cadence (body sections + Working Notes + Findings), relata's entries with per-key append-only verification directories and artifact sidecars, terminology's entries with decision-event directories, udon-needs' in-frontmatter `verified:` event logs. That the parts are exactly *these four* is a carve — alternatives exist (events folded into working notes, as most asf segments currently do; companions treated as separate atoms) — chosen because the four have provably distinct write-semantics, and two labels earn separation where they route to different repairs. Same-estate caveat: all instances share one steward; this is coherence plus pressure-diversity, not independent replication.

## Discussion

**The write-semantics are the argument for the carve.** Body-content in an event trail cannot collide (append-only history is collision-free by construction, so staleness hides); events in a body get overwritten (replacement semantics destroy the trail); history in working notes becomes vanity-changelog that pins artifacts in place. Each misplacement is a *different observed failure*, which is exactly the test for the parts being genuinely distinct. The estate's field data supplies the specimens: hand-set status tokens with no event behind them; working-note deluges carrying undrained history; dispositions performed as acts that left no record.

**The working-note law, stated once.** A working note earns its place only if it assists future work: forward pointers, regression guards (a deliberately-corrected-away form recorded so the messier-but-truer present is not "fixed back"), dead-end warnings. Not past-work narration — that is the history layer's job — and not spike references that pin archivable artifacts in place. Transmitted whole from asf's FORMAT discipline, where it is the settled resolution of the working-note-as-overloaded-queue failure.

**Events are the veritas substrate.** [[form-evidence-ledger]] makes status a projection of evidence; the cluster's event part is where that evidence physically lives, at the atom's own identity — per-atom event storage is what makes concurrent verifiers collision-free (relata's `verifications/<key>/` is the shipped form). An instance without the events part can still carry status tokens; it just cannot answer *who checked what, when, against which criterion* — which is the difference between veritas and assertion.

**Companions and coverage.** A companion rides under the atom's identity but is not the atom: a derivation backing a claim, a figure, an influx payload sidecar. Where a companion partially stands in for something larger (an excerpt, a sample), the atom's coverage declaration ( [[def-coverage-honesty]]) is the honesty mechanism.

## Working Notes

- Frontmatter schema provisional pending the epistemology decision.
- Open: physical realization of the cluster is a substrate/deployment choice the theory should not fix — sections within one file (asf today), sidecar files, per-atom directories (relata), frontmatter fields (udon-needs events). The kit needs at least one worked realization per substrate.
- Open: event grain and schema — the minimal event tuple (subject, act, criterion, actor, date, outcome, revisit-when?) is sketched at [[form-decision-records]]; whether *all* four cluster parts appear in a minimum-viable instance, or events/companions are upgrade-path organs, is a kit question ( [[form-instantiation-kit]]).
- Open: whether a Findings-style external-surfacing section (asf's catalog feed) is a fifth part or a body layer picked out by projection — currently read as the latter.
