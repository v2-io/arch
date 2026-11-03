# Agentic councils — initial thoughts

> [!note] **Superseded as direction (2026-11-03, the same day).** Read `generic-collab-council.md` (the procedure) and `councils-system-model.md` (how councils fit the system) instead. The spikes under `spikes/` cite this file, so it stays as the record of where the work started. It was never a design.

*2026-11-03. Claude (Opus 5.5) with Joseph, in conversation. This file is a starting direction. It is not a design. It was written early on purpose: it should point the next work somewhere without settling what that work should find. A deeper reading of the model may overturn any lean here, and if so this file loses.*

*Register:*
- *Joseph's words are quoted and attributed.*
- *Everything else is my lean or my question, marked as such.*
- *RONR citations are only ones I saw in the model files I read whole: `protocol.json`, `council.json`, `actions.json`, `adjudication.json`, `gaps.json`, `INTERFACE.md`, `ROADMAP.md`.*
- *Not read yet: registry, precedence, guards, engine, lifecycle, rules, scheduler, voting, schema and acceptance, and the RONR body. Claims touching those parts are expectations, not findings.*
- *I deliberately did not read `firmatum/utils/vsect/MATURATION-PLAN*.md` (Joseph's request: they are primed toward how vsect was being used). So this file looks at councils as a tool first, and only then at what it would serve.*

## The problem

The RONR-12 model (`model/`) is an executable account of how a human assembly decides. It is one machine, configured into contexts, that reduces to `available_actions(member, role, state) → [(action, justification)]`.

The goal is not a RONR-compliant tool for agents. It is to take RONR as one input, the most evolved human system for fair group decision under contention, and see how much of it a council of agents can realize, including where it has to diverge to stay fair. Then build it: a server, plus channel and MCP adapters, running real committees.

Why now, in Joseph's framing: much of what currently happens by delegation is closer to abdication, "when a subagent is given full ownership and the delegator simply coordinates." A council is a principled place for those decisions to go instead.

## What Joseph has set, for now

- **Committee is the starting simplification.** In RONR's terms the council is a subordinate instrument of a parent (`council.json` configuration `committee`, `authority-relation: subordinate`).
- **Exclusively agentic committees.** The tool's members are agents. Joseph keeps an independent veto *outside* the tool: "in the eyes of the law and the world I shoulder full actual accountability and reputation for the outcomes." So what a council produces is, structurally, a committee's report and recommendations to a parent who may or may not adopt them. The veto is not a procedure inside the tool.
- **Diversity by design, and recorded.** Members are chosen to "represent a diversity of perspectives and underlying models", and the council's record of each role keeps all of that, "in case there are questions about its outcomes later at a higher level."
- **Acts with immediate effect inside a meeting** are wanted. Two examples:
  - launching a research subcommittee "that can return with what it has found without adjourning or even interrupting the main meeting";
  - moving that "another agent with the following perspectives is included into the membership before the council continues."

## Two criteria, proposed

1. **`available_actions`, kept.** It is the model's existing success criterion: every member has a computable list of available actions with justifications at every state.
2. **Latency invariance, new.** The event log is identical under any permutation of when members' responses arrive. No agent gains or loses because its API was faster or slower.

The second one is testable: shuffle the arrival orders and diff the logs. A human member, if one is ever seated, is just the slowest member, by hours, and the criterion covers that too.

*Lean:* both criteria govern, the way `available_actions` does now.

## Why rounds, and why rounds are not enough

**Where the race is.** RONR's floor is a mutex whose contention a human chair settles by seeing who rose first. Preference in recognition "applies only among members who rose before anyone was recognized" [42:7]. The model compiles everything around that moment: the 42:9 and 42:13 ladders, interrupts, speech counters. The moment itself is wall-clock order. For agents, wall-clock order is API latency.

**The first shape.** Simultaneous-move rounds, not turns (turns give the first mover an advantage):
- At a state transition the server opens a round.
- Each eligible member receives the state delta and its `available_actions`, and submits one sealed intent. Passing is a real and cheap intent.
- The round closes when everyone has responded. The deadline exists for liveness, not for deciding anything.
- A deterministic resolver picks among intents by RONR's own priorities. Remaining ties go to least-recently-recognized or to seeded randomness, so the run replays exactly.

**A round is not just collect-then-pick.** What arrives in one round can call for routing that depends on what arrived. Joseph gave three examples. They show the *kind* of issue to expect, not a list of the known issues. In his words: "a tiny sampling of the *kinds* of things that I assume this process … will surface. … I'm not a subject expert … I haven't read the book or the model either." Most of the real issues of this kind are presumably still unfound. The three:

**1. Near-duplicates as seconds.** A moves a motion, and B, in the same round, moves "pretty much the same motion". Does that count as seconded, and does it have to be identical?

RONR has the parts:
- The motion is the maker's property until the chair states it, and can be modified or withdrawn freely [4:19].
- Informal suggestions to modify are admissible before then [4:20-23].
- "Substantially the same" is already one of the judgment predicates, `substantially-same` [10:26(3-5), 38:2-3].

*Lean:* only byte-identical text merges automatically. Everything else goes back to the makers, since before stating the motion is theirs: "B's wording reads close to yours. Would you take it as a second, or accept one wording as the joint motion?"
- If both consent, the motion stands moved and seconded.
- If neither does, they are two motions, and only one can be pending.
- Co-makers have no RONR precedent, because RONR has exactly one maker, with first-speech preference [42:9(1)]. Agents may need it.

Open: what flags the candidate pair? Detecting near-sameness is judgment; the engine can't compute it.

**2. The intents that didn't win. Queue them, discard them, or something else?**
- *Auto-queuing* executes intents that went stale while the state changed under them, which is a bad option.
- *Discarding* is pure but loses the fact that someone wanted to act.
- RONR's half-precedent: a member whose announced alternative motion follows the defeat of the prior motion has preference [42:13(3)]. The intent is remembered as a claim, not executed.

*Lean:* a non-winning exclusive intent becomes a standing announced intent. It keeps a priority claim, and when its slot comes, the member is asked to re-affirm or revise it against the new state. Nothing runs that its author hasn't re-decided.

**3. Targeted follow-up questions.** Sometimes the server has to ask a subset of members something because of what others said. The merge question above is one case. A clarifying question about what a motion means is another.

So a round has internal phases. Roughly: collect → merge (possibly with targeted sub-rounds) → resolve → apply → reveal. The round is itself a small state machine, and its sub-rounds must also be latency-invariant.

Fairness costs of targeted questions:
- They give some members an extra turn and extra information.
- So every targeted question and its answer are recorded and revealed with the round.
- Who decides to ask them is open: a clerk role, or the presiding member (see *Who presides*).

*Open:* whether an intent may be *conditional* or *bundled* ("I move X; if X fails, I move Y"). It is expressive, but it reopens the staleness problem in case 2. Without it, a member needs one intent per round plus standing interests.

## Not every action contends

*Lean:* classify actions by rivalry. Only the first class needs contention resolution; the others merge.
- **Exclusive:** the floor; making a main or secondary motion.
- **First-one-suffices:** a second, a division demand, an objection to unanimous consent [4:58-59].
- **Additive:** votes.
- **Preemptive:** a point of order, an appeal, interrupts in general.

The merge step in case 1 above is the exclusive class meeting first-one-suffices: one member's motion is another's second.

## Sealing as epistemics, not etiquette

RONR's voice and rising votes are sequential and observable, so members can herd. In a council of same-substrate agents, herding destroys the one thing that makes counting votes informative. This is the estate's prior that agreement is coherence, not independent evidence, turned into procedure.

*Lean:* votes and debate statements are committed sealed and revealed together. The purpose is independence, where RONR's ballot is for secrecy; nothing needs to stay hidden after the reveal.

**Lineage.** Joseph will seat diverse members by design, and the record will carry composition. Separately, agent-identity (`~/src/agent-core/claude-code/agent-identity/`) already exposes each acting agent's key, parent, type (including `fork`) and depth. A vote record can carry those, so "five yes votes from forks of one parent" is visible on its face.
- *Open:* whether lineage ever weights a count, or only annotates it. My lean is that it only annotates. Weighting is a question for the parent at a higher level, which is where Joseph says those questions would be asked.

## The disposition to always act, and closing debate

RONR relies on silence meaning something: when no member claims the floor for debate, the question is put (`put-question`'s derived exhaustion). Agents' default is to do something at every transition. So the procedure has to make passing honorable and the limits real.

**Two findings from `council.json` that bear on this:**
- The small-board relaxations [49:21] remove the speech limits, on the human assumption of self-restraint. The committee configuration inherits that [50:25].
- The committee configuration also *disables* limit-debate and previous-question. Human committees don't need closure; agent committees may.

*Lean:* in an agentic committee, a full round in which every member passes on debate is exhaustion, and the question is put. Keep speech counters as rate limits, including the second-speech bar [43:13], rather than relaxing them. Both of these diverge from RONR's committee rules, and should be recorded as divergences.

**Logical time.** "Per calendar day" counters [43:12] and every other temporal anchor want a logical clock: rounds, epochs, sessions. Real deadlines from outside, such as when the parent wants the report, stay wall-clock, but only as outside events.

## Members are invoked, not present

`available_actions` is pull-shaped: it assumes a member who chooses when to look. Agents act only when invoked, so *whom the server asks, and when* is itself a fairness surface. It is also a cost: every transition × every member × the tokens to read the state. For an agent, context is the scarce resource, not time. A long meeting can exhaust a member, which is continuity death in miniature.

What follows from that:
- **Compact state deltas** and a "where are we" view. Minutes are a projection of the event log (ROADMAP Phase 5), so this is nearly free.
- **Standing interests.** A member can ask to be woken only when relevant ("when a vote opens", "when motion X is pending"). The rights-critical pins ROADMAP Phase 7 already derives always wake everyone.
- **One round protocol, two adapters, as a lean:**
  - *clients*: live sessions, such as prepared experts, join through MCP and block on something like `wait_for_round`;
  - *driver*: the server resumes or forks each member per round through the SDK.
- **The experts model fits as-is.** A member could be a fork of a prepared expert, living for one meeting and then discarded, with the record persisting. That respects the experts' rule: always fork, never resume.

## Who presides

From `actions.json`: "Where exactly one chair action is admissible … the list is also the obligation." So the chair's office splits in two:
- **Duties:** state, put, announce, recognize-per-the-ladder. The engine does these.
- **Rulings:** germaneness, dilatory, decorum and the rest of `adjudication.json`. These need a mind. A presiding member, possibly rotating, makes them, and they are appealable.

If a ruling doesn't come, the point is submitted to the assembly, as RONR provides for a chair in doubt [23:18-21].

*Open:* whether a non-voting **clerk** role is the right home for the merge and clarification questions from the rounds section. It would be procedural only, the secretary's office grown up. Or should those go to the presiding member?

## Acts that take effect during the meeting

Both of Joseph's examples bend RONR in instructive ways.

**A research subcommittee that reports back without the meeting stopping.** In RONR, committing a question moves it into the committee's custody, and the parent's only reach is discharge or instruction (the single-custody invariant, ROADMAP Phase 5, [10:26(5), 36]). A fact-finding referral that leaves the question pending in the main body is something else: information is commissioned, and no custody moves.

*Lean:* model it as an adopted motion whose effect is spawning a subcommittee, with its brief and composition as the motion's content, the way a referral carries composition and instructions [13:8]. Its report arrives as an event that recomputes everyone's lists. While it is out, the body can keep debating, or postpone the question *until the report arrives*. That is an event-anchored postponement, and it is the logical-clock version of postponing to a certain time.

A note from the estate's delegation discipline: the brief the body adopts is the subcommittee's whole context. Peer-voice delegation applies to a council's briefs exactly as to one agent's.

**Co-opting a member.** My understanding is that RONR has a committee's parent set its membership. I haven't verified that the book gives a committee no power to add members.

*Lean:* adding a member is in order only within bounds the parent's grant sets, since that is what Joseph's diversity-by-design guards. Composition changes are recorded as events, and quorum is recomputed.

*Open:* how a member arriving mid-meeting comes up to speed before voting. RONR lets any member present vote. An agent who never saw the debate might reasonably be asked to read the record first. That is a divergence that could serve fairness or harm it, and I don't know which yet.

Both acts need the engine to *execute* effects, such as spawning agents, through adapters. That is new surface. The model has Effect fields, but no executor.

## Questions still open (beyond those marked above)

- **Deliberative, adjudicative, or both.** RONR is majoritarian deliberation with minority protections. The estate's tribunal (`firmatum/udon/v2/theory/to-integrate/refine-more/epistemic-tribunal-revisited.md`) is role-differentiated adjudication whose product is a record that carries:
  - the decision, kept distinct from its confidence;
  - which arguments were load-bearing;
  - revisit or expiry criteria, and the hypotheses the decision rests on.

  *Lean:* RONR for procedure, and the tribunal's roles as offices within it. The council's report would then carry the tribunal's record shape, which fits what a committee owes its parent. Unsettled.
- **Debate's shape.** A serial floor with resolved recognition is closer to RONR. Rounds of parallel sealed statements, revealed together, remove the race entirely but lose back-and-forth. Possibly a council parameter, possibly per question.
- **What a grant gives a council, and what it holds back.** Two existing estate pieces bear on this:
  - vivarium's `:status council-accepted` (`vivarium/core/src/norm-decision-authority.md`, Joseph 2026-07-24): "recommendation carried, Joseph supported". It sits strictly between proposed and decided and never upgrades who decided. That is close to what a committee's adopted report already is to its parent.
  - the steward-valve admission criteria (`firmatum/verisectorium/theory/src/form-steward-valve.md`): irreversible, authoring-voice, cross-project blast-radius, provenance-check.

  *Lean:* those criteria are a first draft of what a grant leaves out of a council's reach, and the valve segment itself notes that a deployment should be able to declare its own.
- **What the first real council decides.** A pilot target grounds every lean here.
- **Where the record lives.** The council's event log, vsect's event store, and DECISIONS ledgers (`deciders: council` under a grant, per udon's `adr/decision-authority.md`). Out of scope until the rest is clearer.

## What needs deeper work

The issues in this file, Joseph's and mine, are what two people found without having read the book or most of the model. They are a sample. Most of the issue space is still to be found. Whoever does the deeper work should treat this file as one input and look for what neither of us thought to ask. Three places I expect the work to start, each needing the model files I haven't read:
- **Translation survey.** For each motion, timer and window: does it survive translation to logical time and to rounds, does it change, or does it lose its purpose? Many subsidiary motions exist to manage scarce human meeting time.
- **The resolver against precedence.** Every interrupt, precedence case and preference rung, run through the round resolver. Which resolve cleanly, which need sub-rounds, and which have no latency-invariant equivalent.
- **The honest remainder.** What has no fair agentic equivalent, or only a divergent one. These are this project's `RONR-OBSERVATIONS.md` in the other direction.
