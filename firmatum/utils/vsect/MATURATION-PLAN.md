*KR I.3 and I.4 (one open ledger keyed by claim; standing read from recorded checks, not typed), under the top intent "Complete "in-place" refactor of ASF-- meta, all artifacts, and theory itself." (Joseph, 2026-10-30).*

# vsect: the maturation plan

*Draft, 2026-11-02, by the lead of ASF's next phase (Claude Opus 5.5), for the agents who will build each phase, for the independent reviewer of this plan, and for Joseph. Not yet reviewed by a second mind.*

## Where vsect stands, and what this plan rests on

vsect's first version was built in one day (2026-10-30) to the design's "what comes first" (`~/src/aat-v3/msc/vsect/vsect.md` §8), plus an `item` kind to absorb ASF's fourteen trackers. It was then used hard: 2,259 events in a sprint of parallel lanes. Three readings of that use ground this plan, and each is worth reading whole before building anything here:

- `~/src/arch/asf/msc/reflections/31-the-inside-out-sprint-retrospective.md`, the sprint's retrospective, especially correction 12: where vsect had no record kind for something, agents invented one in prose.
- `~/src/arch/asf/msc/inside-out-2026-10/system-review/vsect-flow-2026-11-02/`, a Fable agent's reading of the trail with the scripts that computed every number.
- `~/src/arch/asf/msc/inside-out-2026-10/system-review/agent-identity-2026-11-02.md`, the per-agent identity now installed (`CC_AGENT`, `Claude-Agent:` trailers).

The design in `~/src/aat-v3/msc/vsect/` (README → model → vsect → core → storage → open) stays the specification. Most of what use showed missing is already designed there and was deferred: influx crossings (`cross`), decisions (`decide`), finding disposal, the independence predicate (`core.md` §5.6), teaching records with `narrates` edges, generated marks, `vsect commit`. Where use contradicts the design, the design is revised first (§ *The design, revised* below), so the two never diverge silently.

## Four things use taught

1. **What the tool prevents stays prevented; what a brief asks for drifts.** Every place the tree and the record disagree today sits at a step agents were asked to do by hand. 189 imported item files still read "not yet re-checked" while the trail closed them, because lanes committed the event files and not the item files vsect had rewritten. 17 of 18 `close --by-commit` citations name commits no ref reaches, because worktree commits were rebased away. And `--by` was typed in 74 spellings. Four briefs asked for the commits to be made properly. Joseph, 2026-11-02: he cares little whether a fault is "a problem with vsect" or "a problem with how vsect is being used", only whether it was easily preventable.
2. **The states are kind × who acted × what act, not kinds alone.** Integrators' re-derivations of their own landings: 116 confirmed, none broken or narrowed. Independent re-derivations: 88 confirmed, 22 broken, 43 narrowed. One `review` kind carries both, and the dashboard counts them as one, so 59% of its "confirmed" is the author's own word. Identity is now read from the harness, so this split can be computed for every event from 2026-11-02 on.
3. **A kind that is not there gets improvised in prose.** `kind: item` carries at least six different repairs (a defect in a claim, a record to write, a derivation to attempt, a citation, a tool task, a recorded judgment). Source dispositions live in 23 drain files under invented vocabularies, and 60% of the ledger's coverage of claims is named only there. Claim Records were written into 24 segment bodies. Twelve process decisions sit in a reflection, while the built-in `decision` kind holds none.
4. **A check-and-fix loop needs its convergence signal in view before the next launch.** The four most-checked segments grew 1.5x to 2.5x over their rounds; no round removed more text than it added; independent checks kept finding something about a quarter of the time. The tool recorded every lapse honestly. What it did not show was edit mass beside yield, the number that says whether rounds are settling or adding.

## The phases

Each phase can land alone and leaves the store better than it found it. Each says what it builds, why (from the record), how we know it is done, and what would show it was the wrong thing to build. The order is by how much each phase prevents, and phase 4 waits on a test (Q10) the design already names.

### Phase 1: the tree agrees with the record, and stays that way

No process decision is needed for any of this.

- **Check rules.** `vsect check` reports a record whose frontmatter `closed:`, `rechecked:` or `owner:` disagrees with the trail, and a `close --by-commit` whose commit the durable ref does not carry. The pre-commit hook refuses both.
- **Repair what is there, through the record.** `vsect sync` rewrites frontmatter from the trail, idempotently. That clears the 189. For the 17 dead commit citations, find the commit on main that carries the same change (by patch id, then by subject), and record the mapping with `annotate`. Old events are never edited.
- **The commit seam.** The pre-commit hook refuses a commit that adds an event file while a record file the same verb rewrote is left out. This is the smallest fix that catches what happened. `vsect commit` (design §3.11) is the fuller one; the git trailer hook already supplies its identity half.
- **`close` refuses a commit the durable ref does not carry**, as `take … at:` already warns for versions; `vsect anchors` learns commits as well as versions.
- **Small honesty defects.** `confirm --dry-run` says "recorded" when nothing was written. The dashboard calls a `narrowed` check "known broken" (retrospective item 11). `vsect open` says "1,926 open items" where there are 1,213 distinct things of five kinds; it should give counts per kind and take `--kind`.
- **Teaching surfaces.** asf's `doc/sop/agents.sop.md` still tells agents to type `--by <you>`.

*Done when* `vsect check --strict` on asf reports no tree-record disagreement, and a deliberately partial commit in a clone is refused. *Wrong if* nobody reads the frontmatter lines. In that case delete them instead (see Q-A below).

### Phase 2: independence is visible, and so is convergence

Identity has landed; the item for this is `item:separate-self-recorded-from-independent-evidence` in asf.

- **Self versus independent, on every securing event.** Compare the actor's agent key with the keys of everyone who revised the target version, within the check's basis. The agent's recorded `parent:` and `agent-type:` let a fork (which inherits its parent's whole context) count as the least independent kind of other. The dashboard shows evidence in two columns, and `read` marks `✓` only for an independent check. An author's own check stays in the record on the lowest rung (the design's `assert`). Self-recorded runs remain evidence of reproducibility; self-recorded re-derivations do not count as derivation evidence.
- **History stays honest.** Events before 2026-11-02 carry typed names, not keys. Compare them by name, label every verdict that rests on a name as `declared`, and never present it as harness-attested.
- **What a check read.** `take` without a read set warns and records "whole basis, undeclared". `confirm` lists undeclared checks first. Of about 120 bindings the one confirm pass compared, 3 were touched by their change; read sets are how that cost goes down without loosening the strict rule.
- **The convergence signal.** Edit mass per revise (words added and removed in the hashed text) appears in `why` and in the dashboard's "At work now". Beside it go independent-check yield and the self-recorded share: the three measures the flow review would put on the dashboard.
- **`vsect flows`.** Lift `scripts/fsm.py` from the flow review into a read verb: the state machine from the trail, nodes with their stocks, edges with counts and rates, printed with the window the counts come from (`--format text|mermaid|dot|json`). It is a projection over acts, not standing, so it does not wait on declared projections.

*Done when* the dashboard's evidence lines show two columns on asf, and a check by a reviser's own fork is labelled as one in a clone test. *Wrong if* independent and self-recorded checks find defects at similar rates once rounds stop being adversarial. The record today says they do not.

### Phase 3: the kinds the work keeps improvising

The rule is the design's: *"A kind earns its place when it fails differently, and is repaired differently, from every other kind"*, admitted with `vsect kinds propose` against real instances, not declared in an afternoon.

- **Run the founding questions on `item` first.** The questions are in `~/src/arch/firmatum/verisectorium/theory/influx/segments-model-rc1/10-FOUNDING-QUESTIONS.md`: standing, speech-act, failure, instrument, dimensions, movers, ceiling, grain, certification. They were never run on `item`, `question` or `review`. Run them on the flow review's 45-item sample, starting with the record-boundary test Joseph's 2026-08-30 note in that file puts before the speech-act question: *"can this piece fail — be refuted, rot, reopen — while that piece stands?"* The likely outcome is that the act that closes an item names its kind: a revision with a second mind's check (a defect in a claim, which is usually a finding with a target), a new record (authoring), a review (a derivation to attempt), an `empirica/` run (a measurement), a commit elsewhere (tooling), or a reader's name (a recorded judgment). `close` then demands the matching evidence. Whether those become separate kinds or a required `closes-by:` field on items is what the questions decide. Reclassify the 631 open items by `recheck`, with sampling first, as the tracker import did.
- **`source`, with `cross`.** Build the design's influx crossing (§3.10) for every source: a spike, an audit dir, a Working Note item, an influx file in another repo, a drain row. Its states are those the flow review derived (`04-sunset.md`): unread → read (by whom, at which version) → already in canon (carrier named) / superseded / set aside (with reason and home) / opened (as an item or finding id). **Retirement is computed**: a source may move to `.integrated/` or `.archived/` only when every unit it carries is in a terminal state with a target that resolves. That is Joseph's delete-test as a lint, and `check` enforces it on those directories. `unread` is the default and is listed, so "what has not been read for integration" is a query, and a planted canary is something the lead can look up. Seed it with the retrospective's known-unread list. Then import the roughly 2,700 prose dispositions in `drain/`, one file by hand first, as the items pilot did. Only then is `drain/` deleted, in the order the I.10 measurement requires.
- **`decide`.** Record the retrospective's twelve corrections as decisions, each with its grounds and what would reopen it, so `why` and `open` can show them and the next lead inherits records, not a reading.
- **Findings can be disposed of.** `findings dispose … rejected | known-defect "why"` is used, and the dashboard shows those counts, so a wrong finding has a way out of the queue. Today it waits beside the right ones.
- **Questions name who closes them.** `ask --closer anyone` requires a `--why`. Today 61 of 64 questions are for anyone.

*Done when* `vsect open --kind source --state unread` lists the canary list, `drain/` is gone with every disposition in the record, and the I.10 measure re-run shows nothing newly "named by nothing". *Wrong if* the founding questions show one closing act serves every item. Then the kinds were a distinction without a difference in repair.

### Phase 4: grain, teaching and marks

This waits on the design's five-segment test (open Q10), which is also the first admission evidence for the theory store's kinds.

- **The five-segment test, extended.** Re-mark five ASF segments into statements, including one with Claim Records and one restated in the front door, and count where their findings fall. Two questions ride on it. Does a Claim Record share its statement's fate? Use says no: it is standing information about a statement, and inserting one lapsed 13 of the 36 pairs in the one confirm pass. If the test agrees, Claim Records become records keyed by slug and bound to the version they describe, outside the hashed text, as retrospective correction 12 asks. And does a front-door headline read as **teaching**?
- **Teaching with `narrates`.** Front-door headlines, Briefs, OUTLINE cells and README partials become teaching that names the statements it narrates, and goes stale when any of them changes (`model.md`, *Teaching*). This is the fix for the retrospective's first mechanism: Part I's mathematics converged by round 3, while its restatements kept producing findings because nothing tied them to their source. A.2's `<!-- headline:Hn · restates: … -->` comments are the seed. The `front` kind and `restates` edge proposed under D.3 and I.7 are this same design, and use its names.
- **Generated marks.** A refuted statement, and what rests on it, is marked in its own file (`core.md` §10), so a reader without vsect is not misled.

*Done when* the five-segment test's counts are written up, and changing a statement a headline narrates turns that headline stale in `vsect check`. *Wrong if* the test shows Claim Records and their statements are refuted together often enough that lapsing on a record edit is right.

### Later, once phases 1–4 have been used

The design's ranking and briefs: `next` (what is worth doing, with why), `verify` (a bare brief written by the tool, so a verifier is not primed by its launcher), `orient`, eras for simulation evidence. These are where the tool starts directing work, so they should be shaped by use of the parts that record it.

## The design, revised

Before each phase is built, these go into the design documents in `~/src/aat-v3/msc/vsect/`, so specification and implementation agree:

- **Identity is attested by the harness where it can be** (`core.md` §5.6, §15). The design treated actors as declared claims. On Claude Code they now carry a key the harness supplies, and each event records which it rests on (`identity: harness-agent | harness-session | declared`). Elsewhere it stays declared.
- **Items exist** and are not in the design. Phase 3's founding questions decide whether they stay a kind.
- **Computed values in frontmatter.** The design says frontmatter never holds them (`core.md` §10.2). v1 writes `closed:`, `rechecked:` and `owner:` there, and that is exactly where the tree drifted (Q-A).
- **Where vsect lives** (open Q3) is settled by fact: `~/src/arch/firmatum/utils/vsect/`, in Rust.

## How each phase is built

From what worked in the identity work, and what the sprint's hazards list says did not:

- **Who.** One implementer per phase, in its own worktree of arch, holding the phase to the end. A different mind checks it with a bare brief and a habit of building counterexamples. The findings go back to the same implementer, and the same checker confirms. Retrospective correction 5, and Joseph's integrator-continuity rule of 2026-10-31.
- **Before anything is installed.** Unit tests. The new and old builds give byte-identical dashboard, `open` and `check` output on the real asf store, except where the phase means to change them, and the diff shows exactly that. Every changed verb is exercised on a throwaway clone. Every agent on the machine uses vsect, so a broken build breaks everyone at once.
- **Data changes go through the record.** Migrations write new events (`annotate`, `recheck`, `cross`) and never edit old ones. A data change is a separate commit from the code that makes it possible.
- **Teaching surfaces in the same commit.** When a phase changes how agents work, the same change updates vsect's README and help, the global `~/.claude/CLAUDE.md` vsect entry, and asf's `doc/sop/agents.sop.md`.
- **Forward compatibility.** An old binary reading a store that holds new fields must not report it as damaged; the identity work found it did. New event fields round-trip through older builds from now on.

## Open, and how each is resolved

- **Q-A. Frontmatter: keep it in sync with the trail, or remove the computed lines?** The design's position is remove, and `vsect open` already reads the trail. Keeping them serves readers of the files, such as grep, GitHub and a cold agent. Phase 1 builds `sync` either way, so the choice can wait for evidence: whether agents read those lines. Resolved in phase 1's review.
- **Q-B. Separate item kinds, or one kind with a required `closes-by:`?** Resolved by the founding questions in phase 3.
- **Two of the design's own open questions are Joseph's**, and this plan does not settle them. Q1 asks whether the who-moves-next flags are typed or computed; it reverses his spec-lite proposal. Q2 asks whether a verifier may be shown only part of an atom. Phase 4 needs Q2 answered before `verify` writes two-stage briefs.

## What this plan leaves alone, on the record's evidence

- **The strict lapse rule.** It has low precision and the right recall: the three true lapses in the confirm pass were the ones a weaker rule would have missed. Narrow its cost with read sets; don't loosen it.
- **Append-only events.** The 135 `correct` events are the system working.
- **The bare-brief independent check.** It is where every defect in the record came from.
- **Items closed by commits**, once the commits are real.

## What this plan does not cover

Process outside the tool: how often to check, when to land, who coordinates. One test belongs there and should run early in the next phase, because it decides how checking is paced: a round that lands only the findings, adding nothing, followed by one more independent check. If that check still finds problems at about a quarter of the rate, the defects were there all along. If it finds almost nothing, the rounds were creating their own work, and rounds should be limited to landing findings.
