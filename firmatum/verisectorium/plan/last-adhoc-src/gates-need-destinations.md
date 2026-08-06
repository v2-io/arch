---
slug: gates-need-destinations
type: form
depends:
  - state-flags-not-gates
  - gate-profile-divergence
---

# A gate is an assurance attached to a destination

*Where nothing is moved, promoted, or treated differently on the far side, there is no gate — there is a state flag with ceremony attached; and where there genuinely is a destination, four further things have to be true or the gate is decoration.*

## The claim

**(A) The definition, and the test.** A gate is the assurance that something qualifies to cross into a place where it will be treated differently — cited as settled, published, assumed disposable, given write access. The test is to name the far side. If crossing changes nothing about how the material is treated, what is being described is *state*: a resettable flag recording that a check ran, which is useful and is not a gate ([[state-flags-not-gates]]). The reason to keep these apart is that they fail differently. A state flag that nobody sets is untidy; a gate that nobody passes is a **destination nothing reaches**, and everything designed to happen on the far side silently never happens — which is how a promotion ladder ends up with a documented drain that has fired zero times.

**(B) A gate fires at an act, never at a disposition.** This is the failure mode that is hardest to see from inside, because a disposition-shaped check *runs* and *returns clean*. The recorded instance is worth its length: six checks written as stances — be careful about X, watch for Y — fired on schedule through a morning and returned clean while the agent was producing exactly what they watched for; one of them did fire, was read, and was answered *"yes, but it is warranted."* Rewritten as moments paired with acts, the same list returned three real findings inside a minute. The general form: *a check with no moment to attach to attaches to all of them equally, which is to say none.* So the gate's specification names the act it interrupts — before this write, before this move, before this claim — and a gate that cannot name one is not yet a gate.

**(C) A gate needs teeth, and teeth are tooling.** Filesystem distance enforces itself by friction; procedural indirection does not. A declaration that cannot fail a build is a wish, and a rule that asks an author to remember is a rule that will be followed exactly as often as it is remembered. Where the estate's gates have force, something runs: a linter, a CLI that writes the event rather than trusting the author to, a sealed test. Where they do not, the gate is a paragraph.

**(D) Whether a check gates or warns belongs to the deployment, not to the schema.** The same machinery, the same fields, the same checker can be a hard stop in one instance and an advisory in another, and the difference tracks the consuming deployment's **stakes and reversibility** — what it costs to be wrong, and whether being wrong can be undone after the artifact ships. This is measured, not argued: two stores in one estate run identical verification machinery at 70/97 and 13/188 coverage, and the difference is that one gates anonymization before submission while the other warns ([[gate-profile-divergence]]). Neither is a defect. It follows that enforcement strength is a **profile axis** an instance selects, not a property the schema can carry — and that a generator has to ask about it rather than choose ([[instance-profiles]]).

**(E) More gates do not sum to more assurance.** Independent checks add confidence; correlated ones do not. A schema constraint, a linter rule, and a CI job that all read the same declaration and can all be wrong in the same way buy approximately **one** channel, at three times the cost and with the appearance of three. The design consequence is that a proposed additional gate should be asked what it can catch that the existing ones structurally cannot — and if the answer is nothing, the honest move is to strengthen one channel rather than to add a second reading of it.

**(F) A gate is legitimate only where the artifact would otherwise assert false confidence.** A canon landing, a status elevation, a claim of completion — these can lie, and a gate is what stops the lie. Releasing something at an honestly lower tier cannot lie, and gating *that* is where gate-machinery turns into regress. The full statement of the counterweight, with its self-check, is [[honest-incompleteness-discharge]]; it belongs beside this segment and is not restated here.

**(G) Prefer the crossing to the wall.** Where a gate exists to keep unsettled material out of a slow layer, the indirection it needs may be *temporal* rather than *spatial* — a recorded, adjudicated crossing rather than a barrier ([[observable-crossings]]). A wall taxes the comprehension and concurrency that proximity was providing; a crossing costs an event. Both make the transition deliberate; only one of them makes it auditable afterwards.

## Strength & grounds

**A formulation assembled from parts at unequal strength, and the parts should be weighted separately.**

(A) is definitional and (F) is quoted from the estate's own process law. (D) is the best-supported: one measured pair of stores on identical machinery, first-hand ([[gate-profile-divergence]]), plus the steward's own statement of the rule. (B) rests on a single first-hand incident report from one corpus, and it is a strong specimen precisely because it includes the *negative* half — the checks ran and returned clean — but it is one morning, one agent, self-reported. (E) is imported from theory about independent channels and is stated here as a design constraint rather than a derived bound; whether real gate-correlation in document corpora is as high as the argument assumes has not been measured anywhere in this estate.

(C) and (G) are the weakest and should be read as recommendations: the estate has instances of toothless gates and instances of tooled ones, but no controlled comparison, and no corpus has yet run a crossing-as-event design at all.

## Working Notes

- The measurement that would test (E) directly and cheaply: take one corpus's checks, and for each pair ask what defect one catches that the other cannot. A matrix that is nearly all-shared is the failure this predicts, and would be the first real evidence for it here.
- Open, and a genuine tension with (C): teeth make a gate real and also make it a bottleneck under concurrency — one estate gate is a sealed per-checkout test, which means parallel agents on one tree clobber each other. Enforcement and parallelism are trading against each other and nothing here states the exchange rate.
- Adjacent: [[influx-queues]] (a gate's non-happy outcome must split into at least two, and be stored), [[orientation-gate]] (the estate's one gate with a genuine destination — write access — and a graduated incentive design), [[layer-speeds]] (the membrane a gate sets permeability on).
