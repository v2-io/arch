# Proposed integration plan — suggestions with reasons, and a do-not-inherit list

*For the integrator, after independent verification. These are suggestions; the reasons are the content. Written by the spiker, so weigh them as such.*

## Suggestions

1. **Adopt the charter as the server's seating artifact, as an instance over `council.json`'s `committee` configuration** (`02 §1–2`). Reason: it keeps "many councils, one engine" true — the server loads a configuration and an instance, not a bespoke council type — and the checker shows two very different councils loading against one chain. The walking-skeleton's `{seats, quorum, agenda}` maps onto `roll.seats`, `quorum`, `agenda.items`; its YAML surface can stay and emit this shape.
2. **Make `kind: special | standing` the first field Joseph fills in**, and derive the agenda for `special`. Reason: the book's own distinction carries dissolution, session structure, and whether the council may touch its agenda; it is the one field that separates "quick council" from "RONR top-down", and it is worth more than any amount of whitelist editing.
3. **Fix `restrict_actions` before any council runs on it** (`model-defects.md` F1–F2): add `scope`, never whitelist duties, and derive the rights-critical floor (`object-to-unanimous-consent`, `withdraw-own-motion`, `demand-separate-vote`, point of order, appeal, `put-ignored-question`) from the model so no whitelist can drop it. Reason: the committee-of-the-whole overlay, read literally, cannot state a question; and the walking-skeleton's runs show that whatever a whitelist omits gets routed around in ways the model would refuse.
4. **Add SDC fields to added motions** (F3). Reason: the three charter-added acts carry thresholds; prose notes are what the INTERFACE forbids.
5. **Land the common-state condition as the server's stated invariant** (`01 §3`) and the four residuals as a required charter section named after 9:36 ("electronic meeting rules"). Reason: it is the principle (mutual deliberation before decision) in executable form, it is the property the latency-invariance test observes, and the book already names the list.
6. **Logical time as a charter section with the five mappings of `03 §2`**, and the `before(now, event-ref)` extension recorded in INTERFACE/CHANGELOG (F4). Reason: the inventory shows only day, session, corridor, hour, and duration need mapping; event-anchored scheduling is the only interface change the two in-meeting acts need.
7. **Mechanical chair as the default for ≤ 3 seats; elected chair as an option**, with the lost sua-sponte duties named in the charter (`02 §5`). Reason: the adjudication machine's "submitted" path already is the mechanical chair; the cost is real and should be visible where the choice is made.
8. **Model the two in-meeting acts as charter-granted incidentals with the generic privilege route as fallback** (`05 §1–2`). Reason: 13:8(d) is the warrant; the privilege route works today in the generic model but costs a ruling round every time (F8).
9. **Report shape per 51:7 with per-seat votes and reasons and minority views of right** (`02 §4`), landing as `deciders: council under grant; decided-by: delegated`. Reason: the udon ADR and vivarium norm are already law; the book's form is a good one; the reasons are the product for a truth-seeking council.
10. **Keep `tools/check-charter.py` (or fold its rules into `sweep.py`) as the acceptance sweep for charters** (F9). Reason: a charter that references a renamed guard should fail at load, loudly.
11. **Schema prerequisites in this order**: `Committee` record (`ctx.subcommittees`), then `Sitting`, then `effect.kind: act | recommendation` (F6–F7). Reason: the first unblocks `commission-inquiry` and recursion; the second unblocks standing councils; the third makes the report projection honest about what a committee adopted.
12. **Treat the polling fallback (51:2: unanimity only) as a first-class mode** the server can record. Reason: it is the book's own answer to "what if we just ask each agent separately", and it gives abdication-by-delegation a lawful floor instead of none.

## Do not inherit

Interpretive language I suspect is mine rather than the material's. Verify or drop; do not carry into canon on my say-so.

- **"common-state condition"** — my name and my formulation (01 §3). The book has "equivalent conditions of opportunity for simultaneous aural communication" and nothing more precise. The ten-row table is my enumeration; a verifier should look for an eleventh load I missed.
- **"same source = same member"** for re-forked seats (03 §3, 06 §4). A charter assertion, not a derivation; 06 §4 argues the opposite is defensible.
- **"a mechanical chair is a chair that is always in doubt"** (02 §5) — a reading of 23:18–21 and 12:21, not the book's phrase.
- **The three-axis growth claim** (whitelist / schema / effects, 02 §6) — my decomposition.
- **"roll call and ballot at once"** (01 §6) — the mechanism is real; the slogan is mine.
- **The executive-session guess** (04 §6) — explicitly a guess, disposition territory Joseph deferred.
- **The automatic voiding of inquorate action** (06 §7) — marked guess.
- **Whitelist contents** of both charters — illustrative; each exclusion has a stated reason, and the reasons, not the lists, are what to check.
- **The specimen walkthrough's round numbers and vote tallies** (05 §3) — illustrative; the *admitting conjuncts* column is the checkable content.
- **Every "[derived]" label is the spiker's own grading.** Role separation exists because that grading is contaminated by the goal; the verifier re-derives.
