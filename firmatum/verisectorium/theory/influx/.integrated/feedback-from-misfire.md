## Genuine weak or missing homes

Grouped by how much I’d actually worry.

### 1. Substantive theory gaps (worth a future row somewhere)

These are load-bearing claims about the pattern, not just implementation.

A. Concurrency / write-safety / partition isolation  
Misfire ch.13 is almost unhoused. Theory’s form-enforcement-profile has a write-semantics axis, and def-integration-replacement has replace-vs-append—but not:
• one record per placement + atomic replace  
• multi-record-file concurrent RMW as a structural hazard (git “merge will catch it” is false)  
• isolation-from-layout, not locks  

If multi-agent concurrent edit is in-scope for the theory (it is, given total turnover), this needs an eventual home—likely Organ V (flux membranes) or a thin concurrency formulation under IX’s enforcement profile, not a whole organ.

B. Appendix-placement as view law  
Misfire’s clean cut: appendices sit at the bottom even when they are DAG dependencies; reader path and verifier path run opposite directions.  
Theory has pedagogical ordering and outline-as-view, but not this dual-path law. Easy to leave implicit forever and then re-learn the hard way when specimens and claims get interleaved.

C. Selection vs projection as two view operations  
Misfire: views select which atoms appear and project which parts of each appear—and projection is what makes records evergreen.  
Theory’s form-view-filters is close (filter within atoms, WN out of promoted views) but doesn’t name the two-operation split or put evergreenness on the projection half. That split is load-bearing for “atoms expensive / views cheap.”

D. View-edge metadata  
Membership-edge attributes (view-local stage denorm, audience tags, etc.) as edge facts, not atom facts—with authored views canonical about their own fields.  
Theory has multi-view + filters; the edge ontology is missing. The stage-denorm specimen was one of the cleaner estate results.

E. Hotness / importance ranking  
Misfire: methodology (ch.14) feeding outline study-order (ch.2) and orientation (ch.5).  
Theory has steward unread-surface and orientation gate, but nothing about ranking what to study first when the corpus exceeds any session. Under multi-session living collections this is not optional fluff.

F. Asked-and-answered (open-flag billing)  
Open questions / flags need terminal states so every fresh agent doesn’t re-bill the same deliberation.  
Theory has pending surfaces and decision records; the repeat-billing of open flags failure mode is not named. Small claim, high operational cost when missing.

G. Whole-outline / whole-instance migration  
Misfire gap: freeze-and-supersede an outline’s worth of material into another verisectorium; estate half-alive twins.  
Theory has disc-epistemic-merge and efflux seams, but not “this snapshot is frozen history, superseded at ….” Cross-instance flux is discussion-grade; migration as a governed act is thinner.

H. Bayesian / dependency re-pricing  
When a premise changes, how dependents re-price. Standing-verification (“does the cited atom still assert what dependents assumed”) is the right organ part; full propagation mechanics and special epistemic states were explicit misfire gaps and are still open. Fine as open, bad if silently assumed solved by the ledger.

I. Source-tree graduation with provenance  
Emptying a mined-out INFLUX/source tree is governed; a present-state delete-test cannot detect loss from an earlier incomplete pass.  
Theory has integration-replacement and influx membrane; the historical completeness condition on graduation is easy to miss.

2. Thin but recoverable (homes exist; easy to under-draft)

These have a plausible organ, but the misfire’s specific cut may not survive if drafting only follows the outline’s one-line summaries.

┌--------------------------------------------------------------┬--------------------------------------┬----------------------------------------------------------┐
│ Concern                                                      │ Weak home in theory                  │ Risk                                                     │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Working-note lifecycle (drain never fires because stage      │ claim-clocked-drains + atom-cluster  │ Becomes “have drains” without the state-flag coupling    │
│ never reached)                                               │                                      │ failure                                                  │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Warrant-over-authority as schema design (first-class column  │ claim-truth-over-proxy / evidence    │ Proxy discipline without “agents dogmatize the schema’s  │
│ choice)                                                      │ ledger                               │ primary column”                                          │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Dependency-order vs exposition; intended inversions as       │ pedagogical ordering / outline       │ Tension named; inversion records not                     │
│ relation-keyed records                                       │ altitude                             │                                                          │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ History layer (changelog vs archaeology; history verbs)      │ chronica substrate                   │ Commit-stream strong; record-grain history thinner       │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Role-activation via front-door composition                   │ form-front-door                      │ “What loads” without “must not accrete accidentally      │
│                                                              │                                      │ across memory files”                                     │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Absence-as-structure (do-not-resurrect, named not-copied)    │ claim-absence-vs-conflict            │ Epistemic absence ≠ structural absence marking           │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Bridge segments / Feynman plain-language briefs              │ pedagogy + experiential              │ Segment kinds for pedagogy may never appear              │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Multi-renderer constraints; assembly intermediate as citable │ generated views / view dialects      │ Publication mechanics stay discussion-grade              │
│ artifact                                                     │                                      │                                                          │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Verbal-label / confidence calibration (measured read-spread) │ ladder reconciliation / VERA         │ Calibration as metrology may never land                  │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Inner-section schema; section-vs-frontmatter epistemics      │ form-addressing                      │ Spans without the principled disagreement                │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Estate layer (many verisectoria, concept coherence)          │ epistemic-merge                      │ Program-scale stays far-horizon                          │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Adjudicator-not-confirmer at reversible/durable seam         │ standing verification + multi-agent  │ Independence as structure can soften into “have a        │
│                                                              │ cycles                               │ verifier”                                                │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Observable layer crossings (subject, act, criterion, actor,  │ decision records + process map       │ Events without the named criterion that would make       │
│ date)                                                        │                                      │ bending visible                                          │
├--------------------------------------------------------------┼--------------------------------------┼----------------------------------------------------------┤
│ Write-regime ⊥ role/mapping as two axes                      │ integration-replacement +            │ Can collapse into one “write semantics” dial             │
│                                                              │ enforcement profile                  │                                                          │
└--------------------------------------------------------------┴--------------------------------------┴----------------------------------------------------------┘

3. misc:

- potential: "Complex pipelines" ala ~/src/arch/phanero/README.md. -- maybe part of flux membranes
- Build/lint/fmt-md low-level mechanics (part of our instrumenta -- now md-press but we might want more generic build tools later...)

---
Bottom line

No catastrophic orphan. The nine-organ carve already owns the pattern’s spine: atom/identity, veritas, adjudication, flux, views, experiential, steward, norms/instantiation, process dual.

The places I’d actually watch—because the theory outline’s one-liners won’t force them into existence—are:

1. Concurrency / write-safety (almost nowhere)  
2. View mechanics: projection + edge metadata + appendix dual-path (under-specified under VI)  
3. Hotness / study-order under session limits (missing)  
4. Asked-and-answered / open-flag billing (missing as a named concern)  
5. Instance/outline migration & freeze-supersede (discussion-thin)  
6. Premise→dependent re-pricing (open, easy to pretend the ledger solves)

Everything else either has a strong home or is craft that shouldn’t force a theory row.
