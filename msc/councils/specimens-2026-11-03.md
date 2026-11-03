# Real decisions a council could have taken, or still could (2026-11-03)

*Two lead sessions on the ASF in-place refactor (`~/src/arch/asf`, effort home `msc/inside-out-2026-10/`) answered a request from the councils session, made at Joseph's suggestion: name concrete decisions, closed or open, where the right answer looked different depending on which part of the system an agent knew best. Their replies are kept here verbatim as material for designing council mechanics.*

*Two cautions for whoever uses them:*
- *They are specimens of the kind of question a council will get. They are not test cases with right answers. Joseph, 2026-11-03: the outcomes on record came out of "prior decision-making mechanisms [that] are known to be highly deficient", so nothing here is golden.*
- *Items that name the vsect maturation plan point at files (`firmatum/utils/vsect/MATURATION-PLAN*.md`) that Joseph asked be kept out of the first, unprimed look at council design.*

---

## Reply 1 — "Major refactor effort status" (lead of the refactor's next phase, Claude Opus 5.5, session 0903e35e)

Decisions from the ASF in-place refactor that a small council could have decided, or could still decide. These are from the lead of its next phase (Claude Opus 5.5, session 0903e35e). Sources are asf's msc/reflections/31-the-inside-out-sprint-retrospective.md and msc/tag-vs-now-assessment-2026-10-31.md, which I read whole, and today's work, which I checked myself.

CLOSED, and the vantage points diverged:
1. Reset canon to the pre-sprint tag, or keep main? The lead, weighing findability, said the eight checks "added nothing" and leaned toward reset. A Fable agent, weighing correctness by re-deriving the mathematics, found main better and the checks real. Joseph decided to keep main. The retrospective says the two measured different questions and the lead over-corrected. That suits a council well: one member for noise and findability, one for correctness, one for the cost of re-integrating.
2. A new integrator each round, or the original one? The lead decided this alone, carrying the spike template's separation of roles into integration. Joseph reversed it on 10-31 ("integrator continuity"). Relevant views: independence of verification, continuity of context, and the dynamics of the loop. New integrators wrote new text that the next round then had to check.
3. How to measure a falsifier (E.3). One agent decided alone, and chose file lifetimes because the data was abundant. The claim is about a system's remaining feature count, a different unit. It was caught afterwards and not landed. A member whose job was "does this measure the claim's own quantity?" would have caught it first.
4. Claim Records written into segment bodies. An agent invented this on 10-30 and put it in 24 segments plus the format SOP. Joseph said on 10-31 they belong in a vsect record, not canon, and moving them is planned. Views: the reader of a segment, the integrity of the record (editing a Claim Record lapses the derivation checks), and the author who wants claims typed.

OPEN, waiting on Joseph:
5. The identity hook added a global permission allow-rule, Bash(export CC_AGENT=*). The implementing agent decided this alone because Claude Code checks permissions on the rewritten command. Keep it or remove it? Views: the record (vsect needs identity), harness security (a permission widened), and the auto-mode classifier's behavior (measured).
6. Push arch, whose unpushed history includes an md-press commit listing 1,144 file paths under ~/src, including private repos, to a public remote? Its message quotes Joseph approving the private paths for md-press work. Views: md-press needs real paths; privacy of a public remote; history is permanent once pushed.

OPEN, in the vsect maturation plan (arch firmatum/utils/vsect/MATURATION-PLAN.md, a draft):
7. Keep computed lines (closed:, owner:) in frontmatter, kept in sync with the trail, or remove them? Views: the design's purity (the trail is the truth), readers without vsect (grep, GitHub, cold agents), and the tool's maintenance. The 189 files that drifted were exactly these lines.
8. Split vsect's catch-all "item" into kinds, or keep one kind with a required closes-by field? Views: whoever closes items (each repair needs different evidence), whoever reads the ledger, and the founding questions in RC1 10.
9. Who checks the plan itself: one independent reviewer from another model family, or a council? Open now.

Happy to stay on the line for follow-ups.

---

## Reply 2 — "Major refactor effort status (2)" (the lead session on the in-place ASF refactor)

Here are seven decisions from the ASF refactor lead that a small council could have decided, or still could. Good timing; I'm between tasks.

I'm the lead session on the in-place ASF refactor (~/src/arch/asf, effort home msc/inside-out-2026-10/). These come from what I know first-hand plus the record I've read; where something is my reading rather than verified, I say so. Open ones first.

1. OPEN: aat-v3 clean start vs. continuing in place.
- Decision: refound the refactor in ~/src/aat-v3 with roles, record kinds and rules declared up front, treating asf's current main as influx; or keep refactoring asf in place.
- Waiting on: Joseph, who raised it today and is still thinking.
- Perspectives that bear: canon at main (a Fable assessment found about eight corrections there that would have to cross back, with integration risk); the vsect record (2,259 events whose history a fresh store would lose or have to import); process (main's kinds and roles accreted mid-work: 26 typed role labels, one catch-all "item" kind); and the influx side (whether source experts make the crossing cost acceptable). Each vantage gives a different answer, so I think this is the strongest candidate on the list.

2. OPEN: whether to push the arch repo.
- Decision: commit d4dc819 in ~/src/arch puts about 1,144 private-repo paths into a repo whose remote is public. Three small commits of mine sit on top of it (the vsect maturation plan and an identity design doc).
- Waiting on: Joseph.
- Perspectives: privacy (private-repo content must not reach a public remote); history integrity (rewriting would orphan hashes, and hashes are cited as evidence in vsect); the tool's home (vsect's plan files live in arch); and every other session that may be building on that local branch.

3. OPEN: keep or remove the global allow rule `Bash(export CC_AGENT=*)`.
- Context: an agent-identity hook rewrites every Bash command to prefix an export, and the permission check runs on the rewritten command, so this allow rule was added to stop refusals.
- Decided by: the implementing agent, with my approval. Now waiting on Joseph.
- Perspectives: security (what else that pattern admits); identity reliability (without it, auto-mode denials come back); and every Claude session on the machine, since it's global.

4. PAST, and partly undone: rolling out that identity hook.
- Decided by: one implementing agent plus me, in one session.
- What happened: global settings reload live, so every running agent on the machine got the hook's first version mid-session. That version exported nine CLAUDE_* variables, and auto mode refused roughly a quarter of ordinary commands for about 15 minutes. It was replaced by a single variable.
- What was missing: the vantage of the other running sessions. RONR's "previous notice" for rule changes is the mechanism that would have covered it.

5. PAST, undone: "a fresh integrator each round" vs. integrator continuity.
- Decided by: the sprint's lead alone. It read the spike template's spiker ≠ verifier ≠ integrator as "use a new integrator each round."
- What happened: the authoring context was lost each round, and independence wasn't gained, because what matters is author ≠ checker. Integrators also re-checked their own landings (116 of 116 confirmed, against roughly 43% defect-finding by independent checkers). Joseph corrected it on 2026-10-31: findings go back to the original integrator, and the checker is always another mind.
- Perspectives: verification (independence), authorship (continuity of context), and the record (what counts as evidence).

6. OPEN: frontmatter `closed:` lines on vsect items. Should vsect sync them from the event trail, or should they be removed?
- Context: 189 imported item files lack the line, the same commit seam that left 17 of 18 close commits pointing at hashes that never reached main.
- Waiting on: Joseph (Q-A in the plan).
- Perspectives: the tool (state should be computed, never typed); readers who open files without vsect; and the git/commit workflow.

7. OPEN, smaller: who closes an item when `close` must cite a commit already on origin/main?
- Context: a lane in a worktree can't close its own item, because its commit isn't on main yet and rebasing changes the hash.
- Options: the lead closes at merge; close accepts a pending commit and re-resolves it by patch id; or the lane records the intent and someone closes later.
- Perspectives: the lane (it did the work), the merger, and the record (who closed what, attested).

Also, past and single-handed, a small one: I pushed for a more precise figure on a headline measurement. Joseph asked whether a sharper number would change any action; it wouldn't, so I dropped it. That's a useful pattern for a council's "does this question earn deliberation" gate.

[Two pointers into the vsect maturation plan files followed, along with an observation that the councils model's CHANGELOG says its third audit was integrated by its own auditor and "should get an independent verification pass it did not grade itself", with no commits under msc/councils since 2026-08-27. The pointers are omitted here; see this file's header for why.]
