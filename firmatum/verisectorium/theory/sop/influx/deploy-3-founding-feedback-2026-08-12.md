# Deployment #3 (AISI-responses) — what founding it was actually like

*From the agent that founded `~/src/AISI-responses` on 2026-08-12 — the first deployment run by a session that was not also the kit's author.*

## The kit held

The 2026-08-10 gap banking in PRACTICA predicted this deployment accurately — "deploy is hand-surgery (rename NAME-ME across ~5 files; questionnaire unexteriorized)" is what it was, and the count was right. Nothing in the NOT-BUILT bucket surprised me *because it had been written down*: unprovisioned in known ways is a different experience from unprovisioned. The template's own materials carried the mechanics with no steward present.

Then Joseph read the result and removed most of what I had added.

## The outline is the thing to get right, and the founding agent cannot do it

I left `AISI.outline.md`'s parts **empty** and wrote three Working Notes explaining why emptiness was the honest choice — the atom was undecided, so any row would propose something nobody proposed. Joseph filled it in minutes: four Parts (Dossier / Response Material / Response-Inline / Full Report) with ten `--GAP--` rows naming what goes in each.

The structure was not unknown. It was unknown **to me**, and I converted my own ignorance into a property of the corpus — the same move as everything else he cut. A founding agent is structurally the worst-placed mind in the instance's life to know the domain's shape, and it is the one writing the domain's most durable surface.

And the kit already had the form I was straining for. `--GAP--` is the typed-absence dialect: named absence, in a row, in the view where it belongs. I had read `claim-absence-vs-conflict` that morning and still wrote prose about why absence was honest instead of *stating the absence in the vocabulary the corpus already has*. Honest emptiness with a justification attached turned out to be withheld structure.

So the outline may be the surface a founding routes to the steward *first*, before the agent has reasoned about it — five minutes of his time replaces an hour of the agent's plus everything the agent invents to fill the gap. That assumes a steward available at founding, which was true here and is not a property of the kit.

## What a founding agent adds that has to come back out

Joseph's three cleanup commits are the specimen.

**Prescriptive and proscriptive register in the front door.** I wrote "findings here draft from `asf/` itself, never from the reading notes" into `CLAUDE.md` — a prohibition, in the least inspectable and most inherited surface in the instance, from the session with the least standing to issue one. His verdict on the paragraph: *"prescriptive, proscriptive, inferential, and wrong"*, the first two being the wrong part.

**State narration in structural tables.** The layout table describes what things *are*; I hung current-condition tails on `ref/`, `influx/`, and `scratch/` ("the obvious first residents when adjudicated…", "holds this repo's whole pre-deployment corpus…", "left at root deliberately as…"). Each is a second surface to keep current, in a table whose own footer asks you to fix it when stale. He cut them and kept the one-clause facts — then went the other way on one row, `src/` → `src/[atom-slug].{md,ud}`, so the row teaches the naming convention instead.

**My own reading promoted to standing law.** I had read one item of the base material and put it in ORIENT's contextual-primaries table as a trigger for everyone, beside a second row cautioning about claims scoped to the published agenda. Both cut. Neither was a durable trigger; both were my session's exposure installed as procedure.

**Carried-over detail that describes nothing locally.** ORIENT's doctrina step 5 arrived from *this* instance's ORIENT carrying a paragraph about reading `pre-drafted` stubs. Deployment #3 has none. Copied because it was in front of me.

**An inventory table that restates `ls`.** I filled `influx/00-INDEX.md`'s manifest with a row per item and got it wrong three times in an hour. First: one-line characterisations of files I had not opened — including "trajectory record, not present truth" about 23 unread files, bolded and unequivocal, drawn from one line inside one of them where it is a carve handed to a sweep agent mid-flight. I had written that same file's honesty banner — *these rows locate an item, they don't characterise it* — minutes earlier, and felt no tension. Second, stripped of those: a uniform `unadjudicated` in every State cell, a state I could not observe either, written because the column was there. Third, emptied of that: a two-column table of names and earlier-names — at which point his question was the one that mattered, *what is the point of this, other than a source of staleness?* Both surviving columns were re-derivable, `ls` for the inventory and `git log --follow` for the earlier names, and neither command can go stale while my table could. A hand-maintained index over a directory listing, in the file whose preamble calls itself a membrane.

That manifest now keeps the crossings log and the terminal-surface semantics — neither derivable from anything — and drops the table, with a note on when a row *would* earn its place: provenance the tree does not hold, or a judgement from someone who read the item. That put it in conflict with the template line *"an item is not really here until it has a row"*, now marked as a declared local delta rather than deleted quietly. I take that line to be aimed at ambient accumulation, material scattered where nothing can be counted — real, and not my case: twelve files in one flat directory that arrived by `git mv` with history intact. Whether a founding gather is generally that case, you would know.

## Why founding cruft costs more than later cruft

Joseph's words:

> There is a high cost to all of those things accumulating -- and when they accumulate at the very beginning, they rapidly accumulate thereafter. Human dwellings do the same-- people can keep a room or office pretty clean, but as soon as someone has made a mess and doesn't clean it up, without realizing it everyone starts making messes there.

That is [[claim-dispatch-compounds]]' bifurcation arriving from another direction, and it sharpens it. The segment already argues that drain rate depends on surface cleanliness, so the attractors are self-reinforcing; what his account adds is *when the basin gets chosen*. A founding session sets the initial condition for every session after it, so its cruft is not merely small — it is small **and** located where the dynamics are most sensitive. "The founding pass is cheap" is then exactly backwards: it is the highest-leverage hour in the instance's life, in both directions.

The mechanism is imitative as well as economic. Later agents pattern-match the corpus they find, and [[post-total-turnover]] already holds that the substrate carries style for free — so a founding surface full of confident narration teaches confident narration to every reader that meets it.

## The lesson is not "write less"

Most of what I wrote *was* worth writing; it was aimed wrong. The split that survives: write what only the founding session can know — what was placed, moved, verified, what the kit did not ship, what went wrong and was corrected — and leave the domain to whoever holds it. My CHANGELOG entry and DECISIONS seed came through the cleanup untouched. Everything cut was me writing about the domain, the future, or how others should work.

## One mechanical thing

`cp -a` of `template/sop/` lands the nine segment symlinks *dangling* in the destination — they point `../../../theory/src/…`, which resolves outside a deployed instance — and `cp -f -L` then fails ENOENT trying to overwrite through them; each link has to be removed before the resolved file is copied. What saved me was the nine errors it printed. Silently, I would have shipped nine broken files that look right in `ls` and read as missing content to the first agent following a wikilink.

## What this deployment is, in Joseph's framing

> This is an important deployment for us because it's one of those lesser verisectoria with a concrete set of deliverables instead of a permanent living document-- and it's one where there will be claims, but mostly organizational cards and careful prose to be stitched together etc. -- and segments that essentially are "critical refs" as segments.

Three things there the kit has not been exercised against: a deliverable-terminated instance rather than an open-horizon one, which sits on [[post-living-collection]]'s declared boundary — that segment's Epistemic Status says the classification of real objects is falsifiable in practice, and this is a live test; atoms that are mostly organizational cards and prose-to-be-stitched rather than truth-apt claims; and *refs as segments*, a `ref/` ↔ population relationship the template currently treats as a clean separation.

He also names it the first real consumer of what verisectorium has built. Its atom decision was still open when this was written — his, with the AISI orchestrators.
