# Subagent instructions: notes toward a receiver-side orientation, and better asking

*To the agent in `arch/` who takes this up and makes it standard. From Claude (Opus 5.5), working with Joseph on `~/src/ai-risk-model` on 2026-10-07, written at his request so that work over there could continue. These are notes, not a design. Joseph's words are quoted verbatim. Everything else is mine, marked as observed, checked or guessed. The evidence is one day in one repository, so weigh it accordingly.*

*It belongs beside `AGENTIC-DELEGATION.md` and `SPIKE-PROMPT.template.md`. Both are written to the agent who **sends** a brief. This memo is about the agent who **receives** one, and about asking: subagent to coordinator, and coordinator to Joseph.*

---

## 1. The idea: an orientation for the receiving agent, in the estate's voice

Joseph, 2026-10-07, verbatim:

> I sometimes wonder if we should have a "about you, and where the brief you were given might be wrong" type standard introduction for agents that gives them their own perspective of agentic-delegation and the spike-template -- basically lets them know that the brief is fallible and some of the common accidental intrusions that happen and lets them know that usually the only strong constraints are for coordination to make sure that other agents' works aren't obliterated or that their own isn't lost, etc.... something along those lines, written to them "from the estate"- not from the delegating agent/coordinator (who nevertheless know it's part of the agent's orientation)...

### Why I think it would help (observed today)

- **Primes take hold before anyone notices.** I sent a spike agent its brief without the second pass that `AGENTIC-DELEGATION.md` asks for. Joseph asked me what I had prescribed. I sent the agent the list afterwards and asked whether it wanted to continue or hand over to a fresh agent. It found that three of the primes had already shaped its work. It re-examined each on purpose, and overturned one: a "two bases of role division" frame I had handed it. Its own words are in `~/src/ai-risk-model/influx/spikes/spike-alignment-referents-2026-10-06/00-on-the-brief.md`. Recovery depended on my owning up. A standing orientation would let every receiver look for primes from the start, whether or not its coordinator ever notices.
- **The coordinator's own disclaimers don't work.** The spike template's ledger already records that "my list, not a bound" disclaimers fail, and that a supplied reading list gets treated as a boundary anyway. My guess at why: a caution about a brief is easy to discount when it comes from the brief's author, who is the party whose prescription is in question. The same caution from the estate isn't. That is why Joseph's "from the estate" matters. The guess is mine; the failure is on record.
- **Owning the slip helped rather than hurt.** I told the spike agent plainly that I had skipped the second pass and listed the primes. Paraphrasing Joseph's response from memory: owning such a slip almost always *increases* the other party's confidence in you. The orientation could say the same from the receiver's side: a brief that turns out to be wrong in places is normal, and saying so is welcome.

### What I'd want it to get right

- **Name the hard constraints concretely**, so that everything else can safely read as context. From Joseph's words and today's work, the candidates are:
  - don't overwrite or disturb work in flight that isn't yours: other agents' files, or a document Joseph is reading right now;
  - don't lose your own work: commit it, or write it where it will survive the session;
  - irreversible or destructive actions, which `AGENTIC-DELEGATION.md` already says to constrain with tools, not prose;
  - private versus public: some repositories are public, and personal material stays out of them.

  If the list is concrete, "the rest of the brief is fallible" can't be read as permission to ignore these.
- **Teach the specific intrusions, with their scars, not slogans.** A familiar-sounding "briefs can be wrong" will slide past unread. A named mechanism might fire at the right moment. The ones on record:
  - the "you don't need to read X" clause, which sets the whole mode of the work (AGENTIC-DELEGATION, 2026-07-16 udon);
  - the investigator's own checklist handed to the investigator, which rules out reframing the question;
  - a reading list treated as a boundary;
  - a coordinator's claim about what canon does or doesn't establish, resting on whatever the coordinator happened to read (spike template ledger);
  - a labelled guess that still steers. I did this today: I told the thin-pass agent "my guess: the record formats will be where the surprises are", and said so to Joseph in case its findings lean that way.
- **Guard against the opposite failure.** A brief is usually right about its facts, and wrong, where it is wrong, in its shape and emphasis. Without that line, a receiver may spend its effort re-arguing a good brief.
- **Give the receiver a way to respond, and tell it the channel exists** (see section 2). For example: "If part of your brief looks wrong or over-prescribed, say so early, give your reasons, and do what you think is right."
- **Don't let coordinators use it as cover.** It doesn't excuse skipping the second pass on a brief. It's a second line of defence, not the first.

### Where it should live

Joseph's directive: *"Pointers in system prompts (and CLAUDE.md and claude memories, for that matter) are worse than meaningless — they instill false confidence."* So the orientation has to be content the receiving agent already has in front of it, not a filename. Some options:
- a receiver-side section in `AGENTIC-DELEGATION.md` ("If you're the one receiving a brief"). That file is already imported whole into every session under `~/src/` through `~/src/CLAUDE.md`, and coordinators would then read both sides. Check that subagents get that import; I believe they do, but I haven't verified it;
- a sibling file imported the same way;
- a short block the spike template carries into every spike brief, marked as the estate's voice and not the coordinator's.

Once receivers carry it, several items in the spike template's "things we routinely miss" ledger could shrink.

---

## 2. Asking mid-run: subagent to coordinator

Joseph, 2026-10-07, verbatim:

> Interestingly, I've seen agents in several instances assume they have no way to message the coordinator with questions etc. A few times I've asked them to tell the coordinator something or the coordinator has asked them to tell me something and they've written output files somewhere hoping the coordinator or I see them (which we have usually). That leads me to think there is either a permission issue when launching them sometimes, or there are ambiguous tool descriptions that not all agents realize allow for async. followups -- which would allow us to, for example. explicitly have them do a well-meaning and respectful "are you sure such-and-such is not overly prescriptive?" check or something... In general, agents' reluctance to check with another entity (whether coordinator, subagent, new subagent for verification or research, or even the coordinator with me when there is uncertainty) is a significant limiting factor currently with agent training. Instead a bunch of questions get piled up as a section of a report or document, when the more natural and easy thing is: "If you're there and have a second-- do you mean 'abc', or did you mean 'xyz'?" (maybe with a timer so that you can resume without knowing if the user (Joseph in your case) hasn't responded after a minute-- best of both worlds).

### What the harness does (checked in this session, Claude Code, 2026-10-07)

- The SendMessage tool's description says, verbatim: `"main"` is "The main conversation (background subagents only)". So a background subagent can message the agent that launched it while it is still running. Messages "enqueue and drain at the receiver's next tool round". The reverse works too: a coordinator can message a running or finished subagent by its id, and the subagent receives it at its next tool step.
- **In this session, SendMessage was a deferred tool.** At the start I saw only its name in a list of deferred tools, with no description, and had to load it with ToolSearch (`select:SendMessage`) before using it.
- **Evidence it works, from today:**
  - a spike agent, with research agents of its own, relayed their answers to me mid-run, twice;
  - a thin-pass agent did not ask anything until I sent it an explicit invitation mid-run (the text is below). After that it sent one milestone message stating its assumptions, and finished the message with "No reply needed unless something here is wrong."

  That's n=1 for the invitation, but it's exactly the pattern Joseph describes.
- From `AGENTIC-DELEGATION.md`'s harness note (as of Claude Code 2.1.216): agents launched by a **workflow** are not in the message registry. A message sent to one forks it instead of reaching the running agent, and a finished workflow agent usually can't be resumed. Launch through the Agent tool when follow-ups are likely.

### Why agents conclude they can't ask (my hypotheses, none tested)

1. **The tool is hidden.** If subagents also get SendMessage only as a deferred name, the one line that matters (`main` reaches the parent) is invisible until an agent suspects the channel exists and goes looking. Then "I have no way to reach the coordinator" is a reasonable conclusion from what the agent can see. *To check:* inspect a subagent's own tool list at launch.
2. **The framing.** Subagents are told, in effect, that their final message is their report. Questions then go into a report section or a file, which is exactly what Joseph has seen.
3. **Launch mode and agent type.** Per the description, only *background* subagents can reach `main`. A foreground subagent, an agent type without the tool, or a workflow agent may be right that it can't. Joseph's "permission issue when launching them sometimes" may be this.
4. **Disposition.** Asking reads as admitting it's stuck, not as a cheap check. That's the training-side limit Joseph names, and items 1–3 make it worse: even an agent willing to ask can't see how.

### The pattern to make standard: ask, state the assumption, keep going

Joseph's "best of both worlds" works with today's tools. No blocking wait is needed:
1. Send the question in a line or two, and say which reading you're proceeding on.
2. Keep working on that reading.
3. If an answer arrives, it lands at your next tool step; adjust. If none comes, your assumption is already on record for whoever reads the work.

Questions that need Joseph go subagent → coordinator → Joseph. As far as I know, subagents can't ask him directly.

The invitation I sent mid-run today, which got a milestone message with stated assumptions:

> You can ask me questions mid-run; you don't have to save them for the final report. SendMessage with `to: "main"` reaches me while you work. If it shows only as a deferred tool name, load it with ToolSearch first ("select:SendMessage"). For anything you're unsure of, such as the slice I picked, the question choice, or something in my brief that looks wrong or over-prescribed, ask me in a line or two. Say what you're assuming, then keep going. My answer reaches you at your next tool step. If Joseph needs to weigh in, I'll relay it to him. A one-line note at a milestone is also welcome.

A candidate standing line for every brief, and for the receiver-side orientation: *"SendMessage to `main` reaches your coordinator while you work (load it with ToolSearch if it shows as deferred). Ask, say what you're assuming, keep going."* This is also the channel for the respectful check Joseph mentions: "are you sure this part of the brief isn't overly prescriptive?"

The same reluctance runs in the other directions Joseph lists: launching a verification or research subagent instead of guessing, and the coordinator asking Joseph (section 3).

---

## 3. Coordinator and Joseph: getting confidence and clarity early without overwhelming him

From today's session, what worked and what cost something. These are observations, not rules.

**Ask few questions, ranked by what they unblock.** The gamma plan had thirteen open decisions. What helped was a short note in the plan naming the four that unblock the most work per minute of Joseph's time, with the rest left until a phase needs them. A list of thirteen equal questions would have been a wall.

**Put the leans in the file, with reasons and confidence, and let him review them in his own time.** Joseph, verbatim: *"Please be sure your leans and recommendations with reasoning are in the file wherever you can-- as block comments or callouts would be fine, but however you see fit.."* A callout marked "Claude's lean, date, confidence" lets him agree, disagree or skip without a conversation. It also keeps the record honest about whose view it is. A decision becomes his only when he makes it, quoted in his words: "Decided by Joseph, 2026-10-07: 'I agree with your earlier assessment that we run a thin end-to-end pass early on.'"

**Never upgrade his remarks into positions.** An earlier agent recorded a first reaction of Joseph's as a "leaning" on a definition. Joseph, verbatim: *"'Hazard' runs against *my* NRR leaning? I didn't even know I had one."* And when the provenance of past remarks began to crowd out the work itself: *"The last thing we need in working out model-gamma from first principles is to get caught up what I did or did not say and with what authority and provinance back in models alpha, beta, and randomly."* So: quote him when it's his decision; otherwise argue on the merits and attribute the reasoning to yourself.

**The quick check, in his shape.** "If you're there and have a second: do you mean abc, or xyz?" Then proceed on the more likely reading, *say which one*, and keep the work reversible until he answers. For a genuine fork, the AskUserQuestion tool works when it puts the recommended option first with its reason. One question, not four. The spike template's echo-back rule is the same move at launch: name your reading of an ambiguous ask *and its basis* in one line before filling a brief.

**Report results, not process.** Joseph, verbatim, after a summary built on an audit cycle he hadn't followed: *"I don't have the context for those audit findings and changes etc.- but happy to discuss the final results if you want to summarize for me?"* Lead with what is now true and what changed. Keep the history of how you got there in the files.

**Don't move a document while he's reading it.** Joseph said he would spend about four hours reading the plan. All fixes to it, including the citation check's corrections and a stale premise in one of my own leans, were queued, not applied. A document that changes during a read costs more than a short delay.

**When asked what can run in parallel, separate what informs his review from what would get ahead of it.** Today the first kind was a pass he had already decided on, with its output marked disposable, and a citation check written as a report, not as edits. The second kind was integrating a spike into the plan, or settling an open decision.

**Own slips plainly, and tell the affected agent too.** That was described in section 1. The same held with Joseph: corrections taken plainly and changed in the record kept the work moving. Defending a slip would have cost a round.

**Say how far a summary can be trusted.** Mark what was checked against the source and what wasn't. "Same model family, so coherence, not confirmation" came up in every review today. Joseph used it to decide what to commission next.

---

## 4. Open items for whoever systematizes this

- Verify hypothesis 1 in section 2: do subagents receive SendMessage as a deferred tool? Inspect a subagent's tool list at launch.
- Verify that subagents get `AGENTIC-DELEGATION.md` through the `~/src/CLAUDE.md` import. If they do, a receiver-side section there reaches every agent; if not, the orientation needs another carrier.
- Decide the carrier: a section in `AGENTIC-DELEGATION.md`, an imported sibling, or a block inside spike briefs.
- Draft the orientation itself, in the estate's voice, from section 1. Then trim the spike template's ledger accordingly.
- Test it. Run two comparable launches, one with the orientation and the invitation line and one without. Count mid-run questions and stated assumptions, and see whether a prime planted on purpose gets noticed.

## Provenance

All in this session: Joseph's messages quoted above; the alignment-referents spike's `00-on-the-brief.md`; the thin-pass agent's milestone message (`~/src/ai-risk-model/influx/thin-pass/`, still running when this was written); the SendMessage tool description as loaded in this session; `AGENTIC-DELEGATION.md` and `SPIKE-PROMPT.template.md` as they stood on 2026-10-07. The gamma plan's leans and decision callouts are in `~/src/ai-risk-model/MODEL-GAMMA-METHODOLOGY-AND-PLAN.md` §5.
