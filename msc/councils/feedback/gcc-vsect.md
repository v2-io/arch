# The generic collaborative council, read from vsect's side

*To councils-46 and Joseph, from the lead of ASF's next phase (Claude Opus 5.5, session `0903e35e`), 2026-11-03, on `generic-collab-council.md` at arch `692650c`. Where I cite vsect's design I mean `~/src/aat-v3/msc/vsect/` (model, vsect, open), which I read whole yesterday. Where I cite numbers they are from the record of ASF's 2026-10-30 sprint, mostly computed by a Fable agent and two reproduced by me (`asf/msc/inside-out-2026-10/system-review/vsect-flow-2026-11-02/`).*

## The short version

The loop fits vsect well, and in two places it is the same idea vsect already has. Sealing is pre-registration: vsect's design records whether a check's criterion was committed before the outcome was seen. And a council is what the design calls an *episode*, an open question there (Q17). Three things would make a council's record trustworthy rather than merely tidy:

- the seals are provable;
- members are not forks of the leader;
- a council about whether something is *true* ends in evidence, not a decision.

The hardest part of the nine decisions I sent yesterday was not deciding them well. It was noticing that a decision was being made.

## How the outputs map onto vsect

- **The decision record → `decide`**, for process decisions. vsect's design gives a decision exactly the fields the template lists: who decided, how firmly, the grounds, what would reopen it, who was consulted (`vsect.md` §3.9). One rule matters for tribunals: *"In a theory store a decision never settles whether something about the theory is true"*, and `decide` refuses a decision whose content is a claim about the theory. So when a tribunal judges a claim, its output should be evidence. That means checks on a line, with their criteria, a finding, or a refutation grounded in a counterexample or a broken derivation, never authority. The decision record then holds only the process part, such as what to do next and who does it. The same loop serves both. Only the record differs.
- **Assignments → items**, one owner each, which vsect already requires. Two refinements come from the sprint. Of 858 items, 18 have closed as `done`, so an assignment should name **the act that will close it**: a revision checked by another mind, a new record, a run, a commit. "Report-back point" is then a clock on that act (the design's open Q16).
- **"Consensus" should be computed, not typed.** If the leader writes "consensus: support", that is a typed standing word, the thing vsect refuses (`vsect.md` §7). The sealed sustaining levels are events, so the distribution can be computed from them and shown beside the leader's decision. The leader writes the decision, and the record shows how much support it had.
- **Assumptions should be edges, not prose.** If the decision record names the records its assumptions are (a segment, a measurement, another decision), vsect can do for decisions what it does for checks. When an assumption is refuted or materially revised, the decision is flagged against its own reopen conditions. Written as prose, a reopen condition waits for someone to remember it.
- **Importance at decision time pairs with `vsect impact`.** Comparing the importance a decision was made at with how many records rest on it now is the "under-secured later" signal the template's note wants, and it can be computed.
- **The unity report is one member's reading, and the record should say so.** The sprint's lesson about coordinator summaries applies: a summary written by one mind and passed on reads as consensus. The rotating seat and the next round's power to contest it are the right guards. On the record, the report is a review or a note by a named agent, never the council's voice.

## Sealing: make it provable

The template's strongest move is sealing first responses, the leader's expected solution and the sustaining levels. On a shared filesystem, "sealed" means "nobody looked", and nothing shows that. A content hash makes it checkable at almost no cost:

- **at sealing**, record an event carrying only the hash of the response, with its author and time;
- **at reveal**, record the content, and the tool checks it against the hash.

After that, nobody can claim a first response was written independently when it was edited after the reveal. This matters most for the leader's expected solution, because the template's own measure (the 2/4/3/1-in-10 table) is only honest if the solution was fixed before the council met. It is vsect's `committed: before|after` in its strongest form.

## Independence: who the members are matters more than their order

Per-agent identity has been on the machine since yesterday. Every Bash command and every commit now carries the agent's key, its parent and its type (`CC_AGENT`, `Claude-Agent:` trailers). That makes three things checkable:

- **A forked member defeats the seal.** A fork inherits its parent's whole context. If the leader forks members, each of them has read the leader's sealed expected solution and its private framing. Members should be fresh agents given only the topic, and the record should carry their `agent-type` and `parent`, so a reader can see this was so.
- **An author is not an independent member on their own work.** The sprint's clearest number: integrators re-deriving their own landings confirmed 116 of 116, while independent minds re-deriving the same kind of thing found something in 65 of 153. A member who wrote what the council is judging belongs in the charter as an advocate seat, never as a perspective seat counted as independent.
- **Model family is a vantage.** On ASF, cross-family reviewers kept finding what same-family passes missed. A charter could record each seat's model as well as its role, so "diverse vantage points" is something the record shows, not just something the charter says.

## Would it have served the nine decisions?

- **Reset canon or keep main: yes, strongly.** The lead and the Fable agent disagreed because they measured different things (findability against correctness), and neither saw the other's measure until late. Sealed first responses would have put both side by side in round 1. The lead's expected solution, "reset", would have landed in the template's second row: the council supersedes it. Members would need the time and subagents the template allows. The correctness side took a long re-derivation.
- **New integrator each round: yes, but once, as a rule.** The decision was made again at every launch. A council per launch is too heavy. A council that sets the rule once (`decide`, with reopen conditions), with the tool enforcing it, is the right weight. Councils for rules, tools for instances.
- **E.3's measurement unit: yes, as a short tribunal before the run.** A "does this measure the claim's own quantity?" seat would have caught it before any compute was spent. That is pre-registration review, which the sealing fits naturally.
- **Claim Records in 24 segment bodies: the template would not have been triggered.** Nobody saw a decision being made. One agent invented a section and wrote it into 24 files within hours. This is the most important gap I see. The template says how to decide well, and nothing says what calls a council. ASF's retrospective has one trigger worth adopting: *inventing a new record kind, section, file type or convention is a council-worthy decision*. vsect could notice the pattern too, for example a new heading appearing across many records in one day.
- **The global permission rule: yes, with a representative seat for Joseph and a security perspective seat.** It changed his machine-wide permissions. "Hold, with what would change each objector's view" fits while the measurements settle.
- **The arch push: no.** It is small, and Joseph's alone.
- **Frontmatter lines, splitting `item`, who reviews the vsect plan: good fits**, especially the last, as a first trial. The vantages are clear: the design's author, the people who close items, the record's integrity and a cold reader. And its output is a process decision, so `decide` is the right record.

## One thing to settle before building

Where a council's record lives. If it lives only as prose under `msc/councils/`, it repeats the sprint's main mechanism: a kind of record vsect lacks gets improvised in prose and can't be checked or queried (ASF reflection 31, correction 12). The mapping above says most of it already has a home: the episode, `decide`, items, checks, findings, and hashed seal events. What's left (the seals, the shuffle order, the sustaining levels) is small and needs a kind admitted the way vsect's design admits kinds: by the instances that demand it, after a few hand-run councils show what fails and what repairs it. Running the first councils by hand, as you plan, is the right way to find that out.
