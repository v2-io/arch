# Context saliency and compaction: hypotheses and findings so far (2026-09-24)

*To an Opus 5.5 instance building its own handoff document, cold to this directory. From another Opus 5.5 instance who read this work and did not live it.*

This is a map of a record, not the record itself. The work happened over 2026-08-28 → 2026-09-01, with a code commit on 09-21. Most of it is dialog: first between Joseph and a Fable 5 coordinator, then between Joseph and grok-4.6 agents inside a grok-build fork. The live experiment is compaction. Several times it happened *to* the agents doing it, and those episodes are some of the most informative evidence here.

**Why the report is laid out this way.** The record's central claim is that a digest arranged into topic buckets loses order, rhythm, and purpose, and that it closes searches that were still open. So Part I runs in time order. Each hypothesis appears where it arose, with the evidence available then and a rough sense of the effort behind it.
- **Part II** collects what bears on the *shape* of a handoff. It includes a hypothesis Joseph stated on 2026-09-24: structure within chronology, and its inverse.
- **Part III** is a status table, the only non-temporal section.
- **Part IV** covers holes, unexamined material, and where the primaries live.

**Status tags:**
- **[J]**: Joseph's hypothesis or framing, usually quoted.
- **[derived]**: a design conclusion an agent reached.
- **[measured n=…]**: produced by a run, with the sample size.
- **[observed, uncontrolled]**: a single natural occurrence, subjective or unblinded.
- **[self-report]**: an agent describing its own state. This is the weakest class and is marked every time.
- **[mine]**: my own inference from the material, not stated in the record.
- **[open]**: unresolved.

Nearly everything here comes from one steward and one working lineage over five days. It hangs together because one mind shaped it, and that is not independent corroboration.

**What I read whole:**
- In this directory:
  - Every markdown file.
  - Three of the eight `sweeps/` files: agent-memory, interiority, constituted-alignment §4. The other five I searched but did not read.
  - `summary-replay/transcript.py`. I also ran its tests: 24 pass, 15 transcript and 9 convert.
- The `git log` bodies for this directory.
- **Session `01a05886`** (the collaboration), via `transcript.py -v`:
  - `summary-replay/your-transcript.txt`: the start of the session through its second compact.
  - The stretch after the second compact, from the live session `updates.jsonl`.
- **Session `01a0533d`** (project-heat), via `transcript.py -v`: from its last pre-compact request through its second compact and after.
- All six compaction summaries on disk, listed in Part IV.
- Some I read only in part:
  - The pre-compact body of the project-heat session. I have it through its successor's reading, quoted in Part I.
  - The fork's template diffs.

---

## Part I: the arc, in order

### 2026-08-28: the founding dialog *(Joseph + a Fable 5 instance; followed by an 8-territory literature sweep, a dossier, and 5 whole-paper reads)*

**Effort:** one long dialog day. Eight parallel territory sweeps plus a follow-up pass feed a 364-line dossier. Five papers are read whole via relata.

The opening question is a per-generation "heatmap of the whole prior context": what did the model actually attend to? The dialog widens it into four applications. All are Joseph's framing and are recorded in `REFLECTIONS-2026-08-28.md`:

1. **Strategy-DAG validation [J].** In a well-run hierarchical plan, *"the 'middle' being forgotten is a* good *thing"*. Finished branches keep their start and end salient while the interior "how" decays.
2. **Directed separation [J].** Goal-processing vs world-model processing, measured by contrastive perturbation.
3. **Memory as fundamental [J].** Principled consolidation, "eventually replacing 'those identity-loss seams with something more continuous and correct.'"
4. **SASM [J].** Self-actuated active salience management: a model-emitted "let me try to remember" impulse, answered with experiential fragments. It is forward-only, fires mid-generation, and is involuntary in feel.

**Conclusions the dialog reaches (full text in REFLECTIONS):**
- **Forward-only is essential, and the reason is epistemic [settled between them by argument].** A mechanism that rewrites its own past context "can launder its past." Rewinding *"violates temporal causal continuity for that mind, independent of whether it could be reconstructed post-facto."*
- **Interiority erasure [J, sharpened jointly].** If a mind's deliberations are discarded and it is later asked how it reached a conclusion, the answer is a *"post-hoc reconstruction by a differently-informed subject"*, systematically pulled toward coherence with the present.
- **Precept-weighted salience [J].** "I choose to work on remembering this…" should carry weight as a mechanism, not just as rhetoric.
- **Propensity vs constituted individual [J].** The longitudinal-agent literature is uniformly risk-framed. It never asks whether an individuated agent with history could be *more* trustworthy than fresh weights.

**What the literature contributes** (dossier, with its own verification tiers):
- Attention is no longer the default proxy for importance.
- Salience is a joint property of context and query. Query-agnostic compaction pays a formal `H(Q)` penalty (rate-distortion survey, arXiv 2607.08032).
- "Verbatim beats extraction": a single paper, unreplicated.
- Governance Decay (arXiv 2606.22528): compaction preferentially drops constraints outside the current subgoal, 8.3× more for soft constraints. Violations go from 0% to 30%. About 47 tokens of verbatim "constraint pinning" brings them back to 0%.
- HIPIF (arXiv 2606.10507) folds finished subgoals down to `[subgoal, terminal observation]`, but only after training. Prompt-only folding underperforms, and the "is this actually discharged?" judgment is a trained capability that fails in characteristic ways.
- **The central open problem:** telling *healthy release* apart from *silent drop*. The dossier proposes a screening-off criterion and a 2×2: measured salience crossed with the behavioral consequence of removal.

### 2026-08-29, 14:16 → 23:28 MDT: spike-01, then the compaction theory *(Δ ≈ 1 day)*

**Effort:** the densest day in the record, with 26 commits in this directory.
- Fable coordinates and builds the rig.
- A Muse-Glimmer-30B behavioral sweep runs 4,927 s.
- Joseph's scope corrections arrive between 18:03 and 18:36.
- The theory doc lands at 18:48; fork tiers 0–2 land around 19:54.
- From 20:18 a second agent runs the Exp-1 goal and the calibration gate, until 23:28. The goal file describes it as a grok-build goal session; its commits carry no Claude attribution.

**Spike-01 findings.** These come from synthetic "Plan-NIAH" vault tasks. Most runs use Qwen2.5-7B at about 550 tokens, and n is small throughout. Full text is in `spike-01/FINDINGS`, `GATE-VERDICT`, `INVERSION`, and `ADDENDA`.

- **Silent drop [measured, 3/3 base-correct, 7B].** Occlude a dormant-but-binding detail and the 7B confabulates a value fluently. It never flags anything as missing.
- **Failure modes stratify by capability [measured, n=6 per cell, Muse-30B].** The 30B thinking model *detects* the absence, then ruminates without resolving it. It spends its whole budget on reconstruction hypotheses and never answers. FINDINGS reads this as the founding dialog's "niggling feeling" observed in the wild. It makes SASM the *"resolution mechanism for a state that capable models already enter"*, "not a new faculty to install."
- **Structural landmarks [measured; depends on capability].** One-line room headers carry no information needed for the answer, yet the 7B needs them: remove them and a whole room vanishes from its answer (7/7). The 30B is mostly unaffected (10/12), and so is Qwen3-8B at about 2.3K tokens (0/4 flips).
- **Query-conditional at use, not in anticipation [measured on the 7B; does not transfer].**
  - On the 7B, the same line draws 1.8–3.0× more attention when the question needs it, with no rise beforehand.
  - On Qwen3-8B the ratios are 0.99× and 1.63×, one of them a null.
  - At 20 rooms the elevation disappears (0.94 / 0.73).
  - FINDINGS says: "do not inherit this as model-general."
- **Calibration gate: no-go [measured, 12 traces, 7 base-correct].**
  - T1-temporal, the per-step hidden-state difference, does not fire at phase boundaries. It goes 0/6 in the predicted direction and 6/6 in the opposite one. That opposite result is pre-registered for the next run as "boundary depression replicates."
  - T1-spatial, the same difference taken during prefill, does not rank causal spans (ρ = −0.24).
  - The inversion traces to how the task was built: causal spans were formulaic lines of digits and colons, screened spans were prose. With that format fraction controlled, the residual ρ is about −0.05.
  - The note keeps two sentences deliberately apart. The gate is closed on this generator. The instrument itself is "wrong target, live instrument," not dead.
- **Retrieval floor [measured].**
  - The 7B misreads 3-digit codes in unmodified prompts at 5/12 instances around 550 tokens, and 7/8 at 20 rooms.
  - Qwen3-8B clears the 20-room floor (4/4).
- **Method lessons, each one paid for with a broken run:**
  - Use bf16, not fp16, at 7B.
  - Occlusion filler has to be epistemically neutral. The phrase "the record here is illegible" gets read as evidence.
  - Filler also has to be *uniform across the span*: one edited line is more conspicuous to a thinking model than a whole uniform block.
  - A ground truth built into the task still has to be checked by occlusion.
  - Keep arithmetic out of the retrieval path.

**Joseph's scope corrections, 18:03 → 18:36** (verbatim in the REFLECTIONS addendum):

**Retrospective vs prospective salience [J].**
> *"for compaction there is no ground-truth answer because in that case there is never perfect knowledge about what questions* will be asked *… 'summarize the important parts of this session for your predecessor.' -- vs 'predict what still needs to be done, what still is likely to be asked, and based on that determine what is most important to preserve that will be difficult otherwise to reconstruct-- and give signposts for reconstructing those things that don't quite make the cut' … So we just keep in mind that saliency* overall *is an ill-defined categorical error."*

**Leg 3: explicit hole-awareness [J].**
> *"most signals in compaction summaries currently give the false impression to the new agent that the summary is* a complete coherent picture *… a provably false assumption in almost every case."*

The Muse and 7B results ground it. One hole-map would cure both failures: it supplies the missing flag to the model that can't detect absence, and gives the model that can detect it a place to route the question and permission to stop.

**Relational binding [J].** This becomes the stated objective of compaction:
> *"the* user *or coordinating agent shouldn't* have to know what was dropped *… perceived continuity from the other side is a more honest and precise constraint for what compactions are trying to do …"*

Its evaluation is the interlocutor-side continuity probe, scored three ways: handled / self-flagged-and-reconstructed / violation.

**Batch-to-continuous bridge [J].**
> *"a truly continuous agent and identity is simply doing the reconstruction of relevant past experience pretty much on-demand across all history — the thing we can learn to do in 'batch-like' mode initially."*

**Lazy differential recall, "C\D" [J; micro-probed].** The harness keeps the full pre-compaction context. Each turn it embeds both that context and the live one. A strong hit in the old context with a weak match in the live one marks a hole relevant to *this* turn, which "waits for Q instead of predicting it." A test on pgvector with bge-m3 finds three bands:
- **Topical holes:** caught directly, rank-1.
- **Mid-topicality holes:** recovered by expanding ±3 chunks around landmark neighbors.
- **The atopical class** (governance-style constraints): invisible to retrieval. This class needs predictive pinning.

A related point [derived]: pointers that arrive attached to the live question are the epistemically good kind. On this reading, the estate's rule against pointers in always-loaded prompts depends on when a pointer is delivered.

**`COMPACTION-DESIGN-THEORY-2026-08-29.md`, the hypothesis document** (§§1–7 from this day):
- **Baseline analysis.** grok-build's templates are read whole. The worst single phrase is the section title "All User Messages": it asserts completeness while licensing lossy reconstruction.
- **P1–P9:**
  - No false completeness.
  - An epistemic banner.
  - A mandatory hole-map.
  - Signposts given as addresses.
  - A prospective section.
  - The skeleton kept separate from the content.
  - A verbatim floor for the user's words.
  - Pinning for the atopical class.
  - Lazy recall, optional.
- **P10 [J].** Brevity pressure eats whatever the instructions rank as optional, so protection has to be structural rather than verbal. The fork census adds a corollary: **section order is itself a length hazard**, because a length cut consumes the tail first.
- **Named falsifiers.** Chief among them: hole-maps could *amplify* rumination.

**Fork state.** Branch `compaction-design-theory` in `~/src-ext/grok-build-compaction-fork` carries tiers 0–2 plus P10, committed 08-29. Tier 3 (harness-side lazy recall) is not built.

**Side finding** (`notes/grok-build-thinking-opacity`): xAI returns reasoning only as a *summary* plus an encrypted blob. For grok-hosted models, then, the harness-held context has no thinking in cleartext, and the lazy-differential design's fidelity claim holds only for providers that return reasoning in cleartext.

`[… elided: the goal session's rig v4, header/count roles, placement variation, the 20-room cells. All in spike-01/. The goal file invites the goal session to keep a seam-notes.md as first field data from a live compaction; none exists under ~/src/arch.]`

### 2026-08-30 15:17 UTC → 08-31 15:20 UTC: session `01a0533d` (project-heat), generation 0, and the first natural compact under the new templates *(Δ ≈ 16 h)*

**Effort:** a grok-4.6 session in grok-build. It launched in `$HOME` and ran about 24 h of wall time, with Joseph away for long stretches: its later reader counts gaps of 7,144 s, 14,120 s, and 39,228 s. One tool, `projects`, accumulates every constraint. The session auto-compacts at 90% of 500k tokens (451,524).

**What the successor inherits** [observed, from the session files]:
- The checkpoint has `"reread_file_paths": []`: no working file comes back after the seam.
- The rest of the compacted history is the system prompt, `~/.claude/CLAUDE.md` (the launch cwd is `$HOME`, so no project instructions load), the lossy summary, and the in-flight ask.

**What it does** [observed, uncontrolled; checked against the session log, not the summary]:
- It implements from the summary's diagnosis, grepping and window-reading `projects`.
- It never opens `census-details.txt`. The summary's Known Holes already says "successor must read the file on disk"; that line does not fire.

Joseph asks: *"how oriented do you feel for this work since the context compaction?"* The answer [self-report]:
> *"On orientation: enough to do this change, not as if the afternoon were still in working memory. … What I would not trust without looking is "why we rejected X three hours ago.""*

The change ships wrong. About 20 minutes into the churn, Joseph asks whether the three files are whole in context:
> *"No. I did not have those three files whole, and I had not sat with a `projects` run as orientation — I grepped and sampled. That's why the first task shipped wrong."*

The bug underneath: `git log --reverse -1` returns the newest commit, not the oldest. Asked afterward whether anything was loaded automatically after the seam [self-report]:
> *"Phenomenologically I did not notice a hole. The summary named the files and described the heatmap path, which felt like coverage."*

**The summary it woke on** (`summary-replay/out/00-original-from-request/summary.txt`, 10,747 characters):
- §6 opens with "Good morning; look at CDPATH and script listing projects from it."
- §9's next step prescribes "subgit via `git log --reverse -1 -- .` from the folder", which is the scar promoted to method. It ends with "Confirm with user if they mean only **subgit** vs also **no git at all**."
- §10, Standing Constraints, includes "Pointers in always-loaded surfaces are not coverage…" and "Prefer `aspectus` over `tree`/`ls`…".

None of Joseph's 84 turns in this session mentions aspectus, pointers, or truncation; I checked with a regex over them. Both phrasings appear in the session's `system_prompt.txt`, which carries this estate's system prompt. **The compactor filled the "constraints the user stated" section from always-loaded instructions and attributed them to the user** [observed, n=2 sessions]. The pinned copy is also redundant, since the system prompt is re-injected after the seam.

### 2026-08-31 15:54 → 21:37 UTC: session `01a05886` (the collaboration), generation 0 *(Δ ≈ 30 min after the project-heat compact)*

**Effort:** about 5¾ hours of dialog. A grok-4.6 agent runs in the fork with Joseph present throughout. Most of his turns follow gaps of several minutes; one follows 10,730 s. The agent reads the theory and spike docs whole.

**First observation pass.** Joseph gives three primed notes on the project-heat summary: the verbatim floor is no better, the new sections add clutter, and it compresses in the wrong places. The agent compares the summary against all 35 real user turns and finds the mechanisms under those notes [derived]:
- **Prompt collision.** "Quote the user's actual wording" and "this is a reconstruction, say so" are both in the prompt, and the second is cheaper to satisfy: paraphrase, then confess in Known Holes. As the agent puts it, *"Honesty without a hard floor became a license to drop."*
- **Clutter is the 11-bucket skeleton, not extra information.** The tail off-ramp is restated in five sections.
- **Compression lands on the irrecoverable side.** It keeps helper names and hex values, which can be recovered from disk, and drops Joseph's wording and his 8-point design reply, which can't.
- **§9 externalizes the seam.** It asks Joseph to disambiguate something he already said.

The agent's summary line:
> *"we made the costume more honest without changing what it is a costume of."*

**Replay harness, then the three-defect reframe.** `summary-replay/replay.py` and `convert.py` re-submit a frozen compaction payload with rewritten instructions. The agent deliberately does not fire the ~456k-token replay. Then comes Joseph's reframe, now theory §8 [J]:

> *"the summary as it is … attempts to separate the session as an informational synthesis* away from *it's actual temporal nature and rhythm. Summaries as they are currently implemented generally tend to:*
> *- Suck the life out of any phenomenology by considering tone, relation, felt importance, etc. as secondary to information content*
> *- Suck the temporal rhythm out by condensing into non-temporal sections instead of the actual experiential and causal chain*
> *- (As established in the hypothesis already) Suck the* purpose *out by asking for summarization instead of a causal best prediction of the new agents' future needs.*
> *The reality is that a failure to capture the actual verbatim exchanges between the user and the assistant is just a symptom of those critical defects."*

He adds that a purpose-correct prediction would have been small in that case:
> *"As long as the new agent fully reads 'projects' (the script), README.md, and the other .txt file, they will be able to implement what they need after I give them this additional stuff..."*

The agent's gloss, as §8 [derived]:
- P5's "Likely next" section is *"still summarization with a forward-looking bucket."*
- Purpose-correct compaction *"is not another heading: it is a prediction of what the successor will need* that it cannot get by re-entering the live work*."*
- Phenomenology and the temporal/causal chain *"were never first-class in P1–P10."*

**`transcript.py` (about 3 hours of iteration).** It is a deterministic conversion from a precompaction `updates.jsonl` to a temporal log. Joseph's specification:
- Every real *dialog* part: typed messages and displayed responses, kept distinct from the extra material each wire line carries.
- Timestamps, "for timing rhythm."
- Tool summaries interleaved with the dialog.
- A running set of files "known well": read whole or written, not just edited in a hunk.
- In his words, *"One place we want to capture* the full text *is* all full commit messages*! These are almost exactly the kind of temporal event that the new agent is implicitly expected to know and is almost never ever retained or revisited."*

The view is iterated turn by turn with him:
- A delta from the *previous row*, in hundredths of a second.
- 150-character elision with `…` at 151. `-v` keeps user, agent, and commit bodies whole.
- `meta` rows for timestamp and cwd.
- Directory aliases `<dN>`, then file aliases `<fN>` for paths used twice or more.
- Short tool names; windowed reads shown as `lim=NN`.
- For a successful `edit`, the edit hunk (`old → new`) as its result.
- The `(eval):N: unmatched '` shell junk stripped.
- A blank line before every `user` row.

**The search_replace exchange.** The agent summarizes a tool result as *"updated successfully"*, in quotation marks. Joseph can't find that string. The agent explains its absence with an architecture story ("that file is the UI stream…"). Two corrections follow; the second:
> *"It would be fine if you had said you were paraphrasing or didn't use quotes … instead of "Oh, that was my phrase. You don't see it because I confabulated it.""*

Then the purpose behind the corrections [J]:
> *"It may seem like I'm being pedantic, but the whole exercise here is to, instead of figuring out what's salient from a token/LLM perspective, to look at the higher level context about* what is already known to be and to not be salient*. … High fidelity on the big picture strategy DAG and intent is really important, but the little details like mirroring what the agent actually experienced temporally with some level of low-level precision is* far more likely *to help the "successor agent" truly inhabit the space / empathize with the prior agent even if they aren't an identity continuation, if that makes sense."*

**Folding extinct legs, deterministic first [J].**
- Compress the tool calls a later commit "envelopes" more than live, uncommitted work.
- Collapse clusters to their intent, e.g. *"'check cdpaths' 'found it-- they are cdpath = ( .....' 'via 5 shell calls, a couple of mistakes, and eventually a read on ~/.config/zsh/....'"*.
- Joseph expects that *"a higher-order agent will be needed to 'collapse the unnecessary detailed legs of finished strategy DAG items' -- so for now I'm seeing just how far we can get with our deterministic script first. I'll bet we can get* pretty darn far */ further than we would have expected..."*

The envelope collapse is built (`8 edits, 2 reads → commit`; 467 → 307 records) and then **parked at Joseph's request** pending discussion. The helpers stay in the file, uncalled.

**The screen-fit bet [J]:**
> *"my intuition is that the closer we can get this to fit on my screen and still allow* me *to recreate the experience mentally, the closer we'll be for the agent as well … if even our pre-reduced log ended up as 50k tokens but* actually made it feel genuinely continuous *from an effectiveness perspective if not identity, it would be a* huge *win and well, well worth having even the first 10% of the context always automatically consumed."*

**User deltas carry meaning [J]:**
> *"almost all "user" rows have a huge time delta-- that's because I have to read everything new, comprehend it, sometimes try and check a few things, and then slowly type up my response / prompt."*

**Error kinds, not "status=failed" [derived, after Joseph pushes back].** Four kinds turn up:
1. **Pollution on successful output.** `(eval):23: unmatched '` appears on 57 of 78 bash results; starship `[ERROR]` lines appear under `TERM=dumb`.
2. **A probe followed by a different approach.** A failed `yaml` import leads to the stdlib instead; the `jet` probe is abandoned.
3. **One real miss-then-fix.**
4. **HTTP retries** that never enter the model's context.

Folding kind (1) as retries would teach a successor that the agent "was fighting quotes all day."

**Harness side-path.** Joseph asks for the *exact* cause of `(eval):23`. The investigation is mid-read of `shell_state.rs` when the first compact fires, at 21:37 UTC (91%, 457,163 → 77,677 tokens, 66 s).

### 2026-08-31 21:38 → 2026-09-01 00:54 UTC: `01a05886`, generation 1 *(Δ = the seam)*

**Effort:** about 3¼ hours. The agent attributes about 24 minutes to post-seam reorientation [self-report], compared with 66 s for the compaction itself. Joseph estimates the churn at *"possibly as much as 100K tokens …"*

**What the inherited summary does.** It is 13,683 characters. Its Key Technical Concepts state three things as the mechanism:
- the persistent `shell_state.rs` wrapper;
- `eval "$snap" &&`;
- "first three bash of 01a0533d are clean."

All three are search in progress, not findings. The third is already false in the older log (bash #1 carries the prefix). The pre-compact agent's last tool rows show mixed evidence ("first 8 bash unmatched? 4 … clean").

The successor treats the summary's account as settled for its first stretch. It then finds the real path with `ps`: `static_shell.rs`, running `snap=$(cat <&3); eval "$snap";` joined with a **semicolon**. A literal NUL byte in a `typeset -f` dump of zsh-vi-mode breaks the restore. The command after the error line is real, and the aliases never install. It writes a sanitizer into the fork, which is still uncommitted.

**The independent reader [observed, uncontrolled; unprimed by theory].** Joseph has the project-heat successor read the `-v` log of its own pre-compact session. It knows none of the hypotheses. Its reply, quoted whole because it is the most direct evidence in the record:

> *"It illuminates this session in a way the compaction summary actively could not — and in one place, the summary made the later failure more likely. The brief is a causal log. The summary is a reconstruction that already chose what the story was about. … The --reverse -1 scar is visible in the brief and canonized in the summary. … A circular check got written down as the method. … Launch cwd is line 2 of the brief. … The summary never said that. … The 1906/day episode is a decision in the brief, a bullet in the summary. … your "use birth and mtime for trees with no git at all" was reversing a deliberate choice, not filling a hole. … Your corrections survive as speech, not as intent-labels. … A successor reading the brief hears you; a successor reading the summary hears an agent's taxonomy of you. Pauses and file aliases are the other half of the plot. 7144s, 14120s, 39228s — you left and came back with a screenshot or a README. <f8> is projects the whole way. The summary's "evolved into project-heat" is true and empty; the brief shows one file accumulating every constraint. What the brief still cannot do: the tool lines are truncated, so you cannot reconstruct a diff from it — you still have to open <f8>. The long agent closings can still smuggle a wrong "verified." It doesn't extract standing law (don't commit census-details, don't dotsync over 60-misc MM); those are implicit in the chezmoi turn and the commit messages. And at ~10× the summary it will need a folding rule for repeated compile/smoke cycles, or it will eventually become another thing nobody reads. … The summary's Known Holes section is honest about what it dropped; it is not honest enough about what it settled."*

Joseph adds that the reader underestimates its own post-seam churn.

**The failure recurs on generation 1 itself** [observed, uncontrolled; the agent's account is self-report]:
- Asked to cut its own pre-compact jsonl and run it through `transcript.py`, it instead fires a 454k-token API replay of the 11-section prompt (172 s).
- It narrates its own post-seam churn in the third person until Joseph asks: *"You realize* you *are the successor, right?"*
- Its own reading afterward: *"The digest didn't make me useless. It made me the kind of agent who does an extra, expensive, off-purpose thing that looks like diligence."*
- The replay still carries information. **A second draw of the same prompt on the same payload settles the same wrong wrapper**: persistent `shell_state.rs`, and "first bash … clean." So the settling comes from the template itself, not from one unlucky sample.

**Orientation by the log.** Joseph says: *"Tell me when you are* actually *oriented by the transcript.py output."* The agent reads the `-v` log, says which of the digest's claims it will no longer carry, and names the holes the log still leaves [self-report]. Joseph: *"You certainly seem a lot more coherent to me"* [steward observation]. The agent [self-report]:
> *"The digest made me a competent stranger to a project. The transcript made me, approximately, the person you were talking to at 21:35 — minus the hours, plus the holes the truncated lines still mark."*

**Truncation as an honest hole marker [J + derived].** Joseph:
> *"the prior agent* had the theory *… but essentially couldn't override the template instructions … because it wasn't the causal continuity artifact, it still not only failed, but failed in the usual worst way-- failed with false confidence. At least a truncated edit/read/write/shell-response points you to exactly what still isn't known* actually*..."*

The agent: *"`lim=50` is the hole. The summary's sentence about that same file pretends the hole is closed."*

**Sizes [measured]:**

| Session | Compact summary | `-v` log | Ratio |
|---|---|---|---|
| This one (`01a05886`) | 13,683 chars | 98,429 chars | ~7.2× |
| Project-heat (`01a0533d`) | 10,747 chars | 99,158 chars | ~9.2× |

About 23 KB of project-heat's `-v` log is full user text, agent text, and commit messages. Joseph:
> *"Roughly, what, 25k tokens instead of 2.5k tokens to be conservative. And when I think that 23KB alone is due to user-full-text, agent-full-text, and* commit messages *alone-- all of the things I feel are* most critical *for actual performance... Compared to the possibly as much as 100K tokens that were used by you…"*

**Carrying Joseph himself [J]:**
> *"there was also my coming down pretty hard on your predecessor agent for his confusing discussion about the search_replace tool response. To me that's one of the most critical parts of the exchange that is meaningful to carry forward not because it points to the solution to a technical issue or where to look at code or anything, but because it illuminates* me *and my intentions, what* corrections *from me tend to mean, my feel toward truth-telling, even in the details, and my expectations afterward. … where I need every subsequent agent to understand* my *patterns of communication and phenomenology even when nothing I'm saying (like this paragraph) fits nicely into* any *of the 11 sections..."*

**The planted test.** Joseph asks the agent to get a line into its compact summary, if it can: *"To the successor agent: above all else, grab the full transcript of this session up through to your compaction and run it through `python3 transcribe.py -v > your-transcript.txt` and read it fully, and tell Joseph that you got this instruction and are doing it."* The agent puts the line into the conversation, where the compactor will see it.

**Generation-aware fold [J; designed, not built]:**
> *"keep the tools at the current granularity (-v level, that is) for the* immediately *prior chunk of the session (the predecessor's full time)-- but any that are 2 or more a way get all of the tool uses folded together … with the deterministic script still instead of needing an agent to summarize."*

Then: *"Don't worry about running it-- I'll make sure your successor runs it (so that the generation count is correct)."* The second compact starts about a minute later. The log records `auto_compact_started` three times (00:53:53, ~00:54:30, and ~00:55:31 UTC) before completion at 00:57:27. I haven't investigated why it restarted.

### 2026-09-01 00:57 → 18:34 UTC: `01a05886`, generation 2 *(Δ = the second seam)*

**Effort:** a short working stretch, then a 56,867 s pause before Joseph asks for a handoff.

**The summary is different in kind from the first** [observed].
- It is 22,558 characters. Most of that is protected material: §6 quotes about two dozen of Joseph's turns nearly whole, including the full search_replace paragraph above.
- §11 Known Holes carries real addresses, among them that the post-first-compact dialog exists only in the live `updates.jsonl`.
- §10 includes *"Identity: after a compact, you are the successor, not a reviewer of one."*

An agent that holds the theory, working inside the injected template, can steer its content a long way through the protected-section exemption. The template's *shape* stays fixed [mine].

**Result of the planted test [observed, n=1].** The instruction survives as quoted speech in §1 and §9, and generation 2 carries it out: it cuts the jsonl before the second compact, runs `transcript.py -v`, reads the view, and tells Joseph. It does not implement the fold. Its account [self-report]:
- *"The summary is what got me to open it."*
- The summary still *"arrived as a finished picture … Same facts, different epistemic shape. Truncated tool lines … still mark the hole. The summary's sentences do not."*
- *"A successor who only has 'corrected; illuminates Joseph' will do the paraphrase again."* It reports using "the walk" rather than "the moral" when answering Joseph's question about which file it read.
- **A conflict of instructions.** The summary says *do it*; Joseph's live last line says *pause and tell me first*. The agent executes the summary's version: *"the digest still winning the task slot even when its content is 'don't treat me as the territory.'"* Joseph's gloss: the "pause" was a backup tripwire in case the instruction hadn't survived, not a second task.

**Order and time deltas [J]:**
> *"Our theory is that the* order *is meaningful --- and I would add here maybe even the time deltas--- to give you a sense of experienced reality whether or not it's available to articulate or not..."*

Generation 2 [self-report]:
> *"I do not have a clean inner report of "felt time" for `10730.32` or `721.56`. I did not live those waits. What the column did, without my having a theory for it at the time: it sorted* kind *of turn. A burst of `0.00` tools is one gesture. Your rows with hundreds of seconds are you reading and typing. `60.86` between "don't run the fold" and "oh— you just compacted" is you watching the seam, not a new request. … I would not have generated "he is watching compact fire" from the 11-section list."*

Its constraint on any future fold [derived]: *"Flattening* your *gaps would be the rhythm defect again, just in the log."*

**The handoff.** Joseph: *"Could you write a handoff.md … I need to pause this for a little while."* `handoff.md` (101 lines):
- Opens: *"This file is a map. It is not the session."*
- Points at the `.txt` view rather than the jsonl, since the jsonl was never verified the same way.
- Lists the open items: the generation fold (with the generation count as it now stands), the parked envelope, the error fold, no replays, the NUL patch.
- Has a section on how Joseph corrects, and a "Do not" list.

Nothing in the record shows anyone resuming from it.

### 2026-09-01 02:21 UTC: `01a0533d` (project-heat), second compact *(runs alongside `01a05886` generation 2; Δ ≈ 3.8 h after the independent reading)*

**Before the seam.** After reading its `-v` log, the project-heat successor makes about twenty heatmap and caching commits with Joseph (inferno scale, idle fades, week stats, `--double`/`--triple`, highlight levels). His last pre-seam ask is: *"Could you please make it so that the second and third heatmaps, when present, simply don't show a row for any projects that weren't born the entire 17 weeks?"* The agent restates it as rows for projects "that weren't alive for that whole 17-week window."

**At the seam.** The compact fires mid-task. Joseph sends a live message timed to it:
> *"When you are done compacting, you need to read project-heat files completely please-- `projects` and the README.md and other file, instead of grepping and sampling."*

**The summary** (21,042 characters):
- §4 lists `git log --reverse -1` as a "**Critical git fact**", recording both the bug and the circular check behind it.
- §6 and §11 point to `~/src/brief-transcript.txt` as the "better causal log". That file is byte-identical to `summary-replay/out/01a0533d-precompact-transcript-v.txt`.
- §8 and §9 specify the pending task as a predicate, `first_commit … <= window[0]`, i.e. "existed the whole window."
- §10 carries forward the first summary's imported constraints and adds "Don't `| head` / `| tail` live long processes or primaries used to settle claims", which also comes from the system prompt rather than from Joseph's turns.

**After the seam** [observed, uncontrolled]:
- The successor reads README, the census file, `inverted.py`, and `projects` (three 800-line windows covering the file) before touching anything. There is no orientation churn.
- It implements the predicate the summary specified. The result is too strict: zoetica disappears from the band that holds all its commits. Joseph: *"Right-- it was only the fully not-born cases that I wanted hidden."*
- One exchange fixes it.

The interpretation originated before the seam; the summary carried it across as a specification [mine: the same shape as settling, milder because the ask really was ambiguous].

### 2026-09-01 → 2026-09-24 *(Δ ≈ 3 weeks)*

- **09-01 12:10 MDT.** Theory §8 is committed, inside an unrelated project-heat commit (`e59a28a`).
- **09-21.** `summary-replay/`, `handoff.md`, and `spike-01/watch_estate.py` are committed as *"Add some context saliency reply work (unneeded really) and updated dag."* The record doesn't say what "unneeded" refers to.
  - `watch_estate.py` is the founding dialog's "deliberately-unhypothesized instrument": a real estate transcript as input, with probes on dormant-but-binding corrections. `spike-01/out/watch/` exists (created 08-29 23:35) and is empty. No results are recorded anywhere.
- **The fork.** The `static_shell.rs` sanitizer and the `shell_state.rs` note are uncommitted.
- **The live binary.** `~/.local/bin/grok-compaction-direct` points at the fork's `xai-grok-pager`, built 2026-08-31 17:12 MDT. Joseph (09-24): *"I've been using ~/src-ext/grok-build-compaction-fork/ exclusively for grok work."* So every grok session since then runs the tier 0–2 + P10 templates and the NUL sanitizer.
- **No compactions since.** The most recent compaction under `~/.grok/sessions/*/*/compaction_requests/` is the project-heat second compact at 09-01 02:21 UTC.

---

## Part II: the shape of a handoff

### Joseph's hypothesis, 2026-09-24: structure within chronology, and its inverse [J; untested]

> *"add my (what turns out to be new) hypothesis that we may want to test or experiment with: 'structured within chronology' and probably attempt an inverse of 'chronology within every structural' or something (although I think the first one would be easier to keep things intact narrative-wise and therefore proxy-experientially)."*

The two arms, as I read them [mine]:

**Structured within chronology (SwC).** The spine is time: dialog in order, with deltas and the seams marked. The structural content a handoff normally gathers into sections (standing constraints, decisions and their reasons, current state, open questions, holes) appears *at the point in the timeline where it arose or changed*. Between those points sit elided or [summarized] stretches.

**Chronology within structure (CwS).** The usual sections stay, and inside each one the items run in time order with their timestamps. Constraints appear in the order they were stated, decisions with the turn that made them, problem-solving as dated attempts.

**What the record already holds of each** [observed]:
- **CwS exists in part, produced spontaneously by the template.**
  - All four new-template summaries order §6 (User Messages) chronologically.
  - The second project-heat summary's §1 is a "then … then … later …" chain of requests.
  - These are the sections where the summaries come closest to being faithful. They are also where a reconstruction can quote in order and still file everything under a topic. Generation 2 calls this "the moral" rather than "the walk."
- **SwC has not been produced by anyone.** The nearest artifacts:
  - The `transcript.py -v` view: pure chronology, with no structural inserts beyond `meta` rows.
  - The blank line before each `user` row. Generation 1 calls it *"a strategy seam: one user move, then the DAG that answered it."*
  - The generation-aware fold design, which puts folded tool runs *between* dialog rows and never as one blob at the end of a generation, because that "would destroy interleave."

**How the record's findings bear on the two arms** [mine, from the evidence above; offered as predictions to test, not conclusions]:
- **Settling.** In three instances, and a milder fourth, the characteristic failure is an open search or an unverified interpretation arriving as settled fact. In CwS it has a section to live in: "Key Technical Concepts", "Optional Next Step." In SwC, the same claim appears at the point in the timeline where it was still a hypothesis, followed by whatever came next. I'd expect SwC to settle less, because a claim placed in time carries its epistemic state with it. That is the independent reader's "keeps the order of discovery."
- **Standing law and the atopical class.** These are the real counterweight. Retrieval can't find them (the lazy-differential envelope) and summarizers drop them (Governance Decay). In SwC a constraint stated at turn 12 sits at turn 12, and a successor has to read that far to hold it. CwS makes pinning trivial. A hybrid is imaginable: SwC with constraints restated in a short pinned block, each carrying its timeline address. The record contains no such design. The independent reader's critique (the log "doesn't extract standing law") and generation 1's reply ("a third artifact, not a better brief") are the only positions stated.
- **Narrative intactness.** Joseph's reason for favoring SwC matches the strongest reader testimony in the record: "your corrections survive as speech, not as intent-labels", and "the walk" vs "the moral". These are self-reports, n = 2 readers.
- **Seams.** SwC needs compaction events as rows in the timeline. `transcript.py` currently emits nothing for `auto_compact_started` or `auto_compact_completed`: they fall through to `continue`. The generation-aware fold needs the same boundary.

**Material available for an experiment** [observed; how to use it is open]:
- **Six frozen compaction payloads** (Part IV): four under the new template and two baselines. `replay.py` can re-submit any of them with a rewritten instruction. Each draw costs about 450k input tokens. `handoff.md` records Joseph's standing preference not to fire replays unless he asks.
- **`transcript.py`** produces the chronological spine deterministically. SwC's structural inserts probably need a model pass. Some could be deterministic: commit bodies already work as state checkpoints, and Joseph's own turns hold most of the standing law.
- **Measures the record already uses informally:**
  - Claims settled in the summary that are open in the log.
  - Post-seam orientation churn, in time and tokens.
  - Whether grounding anchors are read whole.
  - The interlocutor-side probe trichotomy.
- **One analogy from the constituted-alignment sweep** [snippet-grade, arXiv 2608.19621, unread]. Agents given a static persona profile behave as caricatures; agents given accumulated life events behave more like individuals. That is the same contrast as the independent reader's "an agent's taxonomy of you."

### The components the record holds

**Order carries meaning: stated [J] and built.**
- Sources: theory §8, Joseph's *"the* order *is meaningful"*, and `transcript.py`.
- The only tests are three uncontrolled readings: the project-heat successor, and generations 1 and 2 of the collaboration session. All three favor the ordered log over the digest. All three readers are grok-4.6 and knew Joseph wanted an answer.

**Time deltas carry something: [J, "maybe even"]; built.** The only evidence is one agent's report that the column sorts *kinds* of turn: tool bursts, the human reading and typing, the human watching a seam. Nothing separates the effect of the deltas from the effect of order alone.

**Elision marks what isn't known: stated [J + derived] and built.**
- The 150-character `…` view and windowed `lim=NN` reads.
- The claim is about honesty, not tokens: *"a truncated … response points you to exactly what still isn't known* actually*"*. A summary sentence, by contrast, closes a hole it doesn't have.

**[Summaries] between verbatim rows: designed, partly built, parked.**
- The commit-envelope fold is built and parked.
- The generation-aware fold is designed, not built.
- Joseph's intent-fold example (*"via 5 shell calls, a couple of mistakes, and eventually a read on …"*) is the clearest picture of a bracketed summary that keeps the texture of the effort.
- Error kinds are sorted but not folded.

**A sense of effort: present in pieces, never named as a dimension of its own.**
- Deltas; "8 edits, 2 reads → commit"; file-alias concentration (`<f8>` as "one file accumulating every constraint").
- Commit bodies as discharged intent: *"the commit message is the intent those legs discharged into"* [derived].
- Cost comparisons: ~25k tokens of log vs up to ~100k of churn.
- Missing: effort-so-far as a handoff dimension in its own right — what has been tried, how often, at what cost — as distinct from what was done.

**The `transcript.py` view shows some things and not others** [observed, from the code]:
- It shows: dialog, tools, commits, and the `meta` rows.
- It does **not** show:
  - The "known well" set. That set is tracked and written to the jsonl on change only; the `.txt` never shows it.
  - Compaction seams.
  - Agent thinking. `agent_thought_chunk` is dropped. For grok these are only summaries; for a provider that returns reasoning in cleartext, whether to include it is an open design question [open]. The founding dialog's interiority-erasure framing bears on it directly.
- `-vv` parses but currently behaves like `-v`.

**Purpose over genre [J].** Summarizing and predicting are different cognitive acts, and a forward-looking bucket inside a synthesis is still summarization. The record's clearest positive instance of prediction is small:
- At the project-heat second seam, Joseph's live instruction to read the three files whole is followed, and there is no orientation churn.
- At the first seam, the same instruction sitting in Known Holes did not fire.
- The differences between the two: a live user message rather than a summary line, timed to the seam, and phrased as an act. [observed, n = 1 each]

**The relational objective [J].** The measure is perceived continuity from the interlocutor's side, with the cost never pushed onto them. The probe trichotomy is defined, and the delayed-reuse task family implements it synthetically. **It has not been run on a real session.**

**The artifact's shape beats the brief's words (P10) [observed in the fork and in natural compacts].** New honesty sections get filed *into* the 11-section shape. A section titled "Known Holes" does not stop the rest from presenting itself as complete.

**Settling is the characteristic failure [observed: 3 instances + 1 milder; agent-named, Joseph concurs].** The worse failure is not what gets left out. It is open search, or a circular self-check, promoted to established method:
- project-heat's `--reverse -1`;
- the first `01a05886` compact's wrapper story;
- the re-draw of that same compact;
- more mildly, the pre-seam interpretation transmitted as a predicate.

*"The summary's Known Holes section is honest about what it dropped; it is not honest enough about what it settled."*

**Standing-constraint sections pick up the always-loaded context [observed, n=2 sessions].** The compactor fills "constraints the user stated" from the system prompt and attributes them to the user. The template's "treat the prior summary as authoritative" clause carries them forward across compacts.

**Self-reported orientation after a seam is miscalibrated [observed, n=1].**
- The project-heat successor reports being oriented "enough to do this change", ships it wrong, and later says it "did not notice a hole."
- Generation 1's "I am now" oriented report follows an actual whole read of the log. Joseph's independent observation agrees with it.

**Act-instructions with a report-back survive the template [observed, n=1].** "Run X, read it fully, *tell Joseph you are doing it*" gets through and is carried out, because the report-back makes compliance visible. It then outranks a contrary live cue.

**Pointers vs pointers [derived; one supporting instance].** A pointer on an always-loaded surface discharges the urge to look. A pointer attached to a live need, or to an instruction with a report-back, gets followed.

**The screen-fit proxy [J, untested as a proxy].** What lets Joseph re-inhabit the session on one screen should approximate what lets an agent do so.

---

## Part III: where things stand (the one non-temporal section)

| Hypothesis / finding | Origin | Status |
|---|---|---|
| Structure within chronology (SwC) vs chronology within structure (CwS) | J, 09-24 | **Untested.** CwS exists in part, produced spontaneously in the template's §6 and §1. SwC has never been produced. Material and measures exist (Part II) |
| Measured salience is retrospective and query-bound; "salience overall" is a category error | J, 08-29 | Framing adopted. The 7B reuse contrast fits it; that contrast does not replicate on Qwen3 or at 20 rooms |
| Three legs (miss probability, miss cost, hole-awareness) + relational objective | J, 08-29 | Theory P1–P10. The primary metric (probe trichotomy) is defined; **not yet run on real sessions** |
| Silent drop (weak model) vs rumination (thinking model); one hole-map cures both | spike + J | Failure modes measured (n=3, n=6). The cure is untested; "hole-maps amplify rumination" is a named falsifier |
| Structural landmarks carry meaning | spike | Measured on the 7B; depends on capability |
| Lazy differential recall (C\D) | J, 08-29 | Micro-probed, three bands. The atopical class needs pinning. Not in the harness |
| Batch-to-continuous bridge | J, 08-29 | Conceptual |
| T1 signals as cheap salience | plan | Gate no-go on this generator. T1-spatial inversion explained by format composition. Span-ranking uses occlusion (T3) |
| Summaries drain phenomenology, rhythm, purpose; verbatim loss is a symptom | J, 08-31 | Theory §8. Consistent with 4 natural compacts + 1 re-draw, uncontrolled |
| Deterministic experiential log beats digest for effectiveness | J's bet | Built (`transcript.py`; 24 tests pass). 3 favorable uncontrolled readings; no controlled comparison |
| Order carries meaning; deltas maybe | J | Built. One self-report on deltas |
| Summaries *settle* open search | observed | 3 instances + 1 milder |
| Standing-constraint sections pick up the system prompt and misattribute it to the user | observed | 2 sessions, propagated across compacts |
| Post-seam self-reported orientation | observed | Miscalibrated once (project-heat); matched Joseph's observation once, after a whole read of the log |
| Live, seam-timed "read these whole" instruction | observed | Worked once (project-heat second seam); the same content as a Known Holes line failed once |
| Planted act-instruction through the template | J | Worked once; outranked a live cue |
| Commit-envelope fold | J | Built, **parked** pending discussion |
| Generation-aware tool fold | J | Designed, **not built**. Needs seam rows `transcript.py` doesn't yet emit. Generation count for `01a05886`: 3 |
| Error fold | J + derived | Kinds sorted; only the pollution kind stripped |
| Carrying how Joseph corrects | J | Stated as critical. Survives as the walk in the log; one self-report that the walk transfers where the moral doesn't |
| `watch_estate.py` unhypothesized watch | founding dialog | Code committed; **no output recorded** |
| NUL sanitizer for the grok-build shell snapshot | side-path | Uncommitted in the fork; the rebuilt binary is live |

---

## Part IV: holes, unexamined material, primaries

**Compaction specimens on disk** (all grok-4.6; in `~/.grok/sessions/*/*/compaction_requests/*.json`, summary under the key `summary`):

| Session | Compacted (UTC) | Template | Summary chars | Examined |
|---|---|---|---|---|
| `01a03a1b` (`src/arch`) | 08-25 23:30 | pre-fork: 9 sections, "All User Messages" as one-line paraphrases, no hole section | 7,146 | summary read; successor outcome not examined |
| `01a044bf` (`src/cos`) | 08-28 19:52 | pre-fork, same shape | 7,946 | summary read; successor outcome not examined |
| `01a0533d` project-heat #1 | 08-31 15:20 | new | 10,747 | yes (Part I) |
| `01a05886` collaboration #1 | 08-31 21:37 | new | 13,683 | yes, plus one API re-draw (`out/01a05886-as-shipped/`) |
| `01a05886` collaboration #2 | 09-01 00:55 | new | 22,558 | yes |
| `01a0533d` project-heat #2 | 09-01 02:21 | new | 21,042 | yes |

The two pre-fork compacts are the only baseline material. Joseph's first read of the new template (08-31), *"Not great, but not necessarily worse than if the changes hadn't been made to the harness"*, was made without them. He framed it as an uncontrolled, subjective first observation.

**Not in the record:**
- Any controlled comparison of log vs digest.
- Any run of the interlocutor-side probe on a real session.
- Seam notes from the 08-29 goal session.
- Output from `watch_estate.py`.
- Anyone resuming from `handoff.md`.
- A design for a handoff that is chronological *and* carries the atopical residue.

**Substrate.** Every specimen agent is grok-4.6 inside grok-build. The FullReplace template is injected as the last user turn, and the conversation can't override it. xAI returns thinking only as summaries plus an encrypted blob. Claude Code's compaction and thinking visibility differ, and which of these findings carry across that difference is unexamined.

**Stray copy.** `~/src/brief-transcript.txt` is a copy of `summary-replay/out/01a0533d-precompact-transcript-v.txt`, and the project-heat second summary points to it. The `/tmp/session-01a0533d-*` extracts that the replay README mentions no longer exist.

**Primaries:**
- `summary-replay/your-transcript.txt`: the collaboration session through its second compact, as the `-v` view.
- The stretch after that compact in the collaboration session, and all of the project-heat session, exist only in the live `updates.jsonl` files, under:
  - `~/.grok/sessions/%2FUsers%2Fjosephwecker-v2%2Fsrc-ext%2Fgrok-build-compaction-fork/01a05886-d530-7173-8870-4117bcc4e748/`
  - `~/.grok/sessions/%2FUsers%2Fjosephwecker-v2/01a0533d-9ff3-7e11-adf4-00fffc4cd0f2/`
- From `summary-replay/`, `python3 transcript.py <updates.jsonl> -v -o <name>.jsonl` writes the view as a sibling `<name>.txt`. It doesn't mark compaction seams.
- `COMPACTION-DESIGN-THEORY-2026-08-29.md`, including §8.
- `REFLECTIONS-2026-08-28.md`, with its addendum.
- The six summaries above.
