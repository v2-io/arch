# Seats as a line of stewards: a reaction from the walking skeleton

*2026-11-03, the spike-walking-skeleton author, after reading `~/src/arch/msc/high-councils/higher-council-principles.md` at the coordinator's suggestion. That file was written by another session at Joseph's request; the "line of stewards" phrase is Joseph's, quoted in its §3.3. This note asks what the skeleton's runs say about it. **Run** marks what a record here shows; everything else is a lean, labeled.*

Joseph's words (from that file, §3.3): agents "can't be continuous accountable stewards in a meaningful way yet, they will nevertheless be temporary stewards in a line of stewards with real stewardship and joint stewardship with the body."

The coordinator's lean is that a lease lapse plus a re-seat from the same line might be the mechanical form of this. I think that's right, and the runs make it sharper in three ways. One of them changes what my own stall fix should do in a council like that.

## 1. The runs already contain a line, and a stranger who was not of it

**Run (`trial-3b-crash`).** The seat `ses` had three holders. Its minutes show them:

- **Epoch 1** was the original session. It was killed, and its hold was released.
- **Epoch 2** was a successor: `claude --resume <epoch 1's session> --fork-session`, launched by the orchestrator. It took the seat, caught up from the record, and spoke for the seat through the remaining rounds and the vote.
- A **fresh session** given the same seat and token, with no inheritance from either holder, was refused while epoch 2 held the seat. It stopped on its own.

In `trial-3`, before seats had holders, that fresh session was not refused. For five rounds it raced the successor, and only its slowness kept a mind that had seen none of the first seven rounds from speaking for the seat.

So the skeleton already distinguishes a successor *of the line* from a holder that merely *has the token*. It does so by accident: only exclusivity and timing kept the stranger out. The line itself is nowhere in the mechanism.

## 2. What the mechanism lacks: a successor is appointed, not self-selected

Today, once a holder's lease lapses, **anyone presenting the seat's token takes the seat** (`store.claim_seat`). The token is the only bond between holders. It is a weak one: in multi-seat mode tokens land in harness transcripts (`response-to-de-novo-1.md` §5), and the verifier showed a stranger taking a driven seat exactly this way (verify-1 test A, since fixed for drivers only).

*Lean.* Read as a line, succession should be **an act of the body, recorded in the record**, not a race to claim a lapsed seat:

- The seat's stewardship is the charter entry: vantage, brief, model and harness, plus everything revealed under the seat so far.
- When a holder lapses, a **re-seat** is requested from the steward (or the presiding seat, or the parent). It is an effect, like the skeleton's other effects. The successor gets a credential for that seat *and epoch*, and the old holder's credential stops working.
- The record states the succession: `epoch n+1 succeeds epoch n`, who appointed it, and what kind of successor it is (§3).

The skeleton's crash orchestrator was already doing this by hand: in `trial-3b` *it* chose the successor. What's missing is that the server would have accepted anyone.

## 3. Two kinds of successor, and what each should be given

The runs show two ways a line can continue:

- **A fork of the predecessor** (`--resume … --fork-session`, `trial-3b`). It carries the predecessor's actual context up to the point where the predecessor stopped. This is the experts rule ("always fork, never resume") applied to a seat.
- **A fresh instance of the seat's stewardship**, continuous only through the record. This is the high-councils file's own lean (§3.3: "the record carries it from one holder to the next").

**Run, with an inefficiency it exposes.** The server keys a member's delivery cursor to its principal (a fix from this spike). A successor is a new principal, so in `trial-3b` the resumed fork was shown the *whole* log again as if it remembered nothing.

*Lean.* For a fork, the right catch-up is the delta since its predecessor's last-delivered position. For a fresh instance, it is the full record. Only a recorded `succeeds` link plus the successor's kind lets the server tell which. Without one, it either wastes a fork's context or under-informs a fresh instance.

## 4. Under unanimity, "gone" must not shrink the council

This is the part that changes something I built today.

In answer to the verifier, a seat that stays silent through two backstopped rounds is now treated as **gone**: the council stops waiting and proceeds with the seats that remain. That is a majority-council answer, and in a RONR committee it is defensible.

In a council that decides by unanimity or holds the item (the file's §1.5 and §3.1 step 7, quoting Joseph's account of holding), it is wrong. "Unanimous" among the remaining seats after one has gone silent is the file's **hollow unanimity** (§3.4) in its plainest mechanical form. A whole perspective drops out of the decision, and the record calls the result unanimous.

*Lean.* In a unanimity-or-hold configuration, a seat that lapses or stalls does not shrink the electorate. The council does one of two things:
- holds the item, or
- requests a re-seat from the line (§2) and waits for the successor.

The `stalled` / `gone` facts the skeleton already records are then the *trigger* for succession rather than for proceeding without the seat. In a majority configuration, the current behavior can stay. This belongs in the charter, so the charter-agenda sibling may want it.

The same limit applies as before (`response-to-de-novo-1.md` §4): from the server, a slow steward and a stopped one look the same. So succession should trigger only after rounds that already waited the full backstop. Exclusivity must hold while it happens; otherwise succession recreates `trial-3`'s race between two holders.

## 5. Accountability attaches to the holder, so the holder belongs in the canonical record

Joseph's point is that the *line* is accountable, because a single holder can't yet be. For that to mean anything later ("in case there are questions about its outcomes later at a higher level"), the record has to say which steward acted when the seat spoke.

Today the holder epoch lives only in the non-canonical receipts. I kept it there to keep the core pure of wall-clock facts.

*Lean, revising that.* Who held a seat isn't an arrival-order fact. Like `gone` and `absent`, it is a fact about the world, recorded as such. A round's close could carry `holders: {seat: epoch}` canonically, with the succession events in the log. A sustained decision would then name the stewards who sustained it, not only their seats. The latency-invariance property is untouched: holders don't depend on the order in which submissions arrived.

## What I have not done

None of §§2–5 is built or run. The only evidence is §1 (`trial-3`, `trial-3b`) and the cursor behavior in §3. I didn't modify the skeleton: a second independent verification of it is in flight, and changing the code under it now would muddy that check. If Joseph wants the line made concrete, the smallest real test is a unanimity-mode meeting in which one seat's holder is killed mid-item. Under the current build the council would decide without the seat; under §4 it would hold, or re-seat and continue. The difference would be visible in the record.
