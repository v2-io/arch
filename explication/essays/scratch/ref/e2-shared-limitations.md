# Essay 2, Part 4: The Shared Limitations

## Topic Summary
Table: LLM failures mapped to known human cognitive biases. Post hoc fallacy, confounding insensitivity, position heuristics, local coherence with global inconsistency. Kahneman's System 1 IS pattern matching. The implicit standard: holding LLMs to unaided formal reasoning that humans also fail at.

## Reference 1 (score: 0.644)
**Source:** `primary-sources/temporal-causal-llm.md:1383-1395`

The implicit standard being applied:

There's an unexamined move in much of the critical literature: holding LLMs to a standard of _unaided formal reasoning_ that humans also fail.

We don't say humans lack intelligence because:
- They commit base rate neglect
- They struggle with Bayesian updating in their heads
- They confuse correlation and causation without training
- They can't mentally verify the consistency of a 10,000-word narrative

We say humans are intelligent _and_ need tools, methods, and training to reason reliably in complex domains. The entire apparatus of science, mathematics, logic, and statistics exists because _raw human cognition is insufficient for reliable causal and temporal inference_.

[Note: From Joseph's analysis section of temporal-causal-llm.md. This is Joseph's original argument, not a summary of others' research, so it has a quasi-dialog quality despite being in a research doc.]

## Reference 2 (score: 0.601)
**Source:** `primary-sources/temporal-causal-llm.md:1366-1382`

The parallel to human cognition -- the core table for the essay:

| LLM "Failure" | Human Parallel |
|---|---|
| Post hoc ergo propter hoc | Fundamental human bias, requires explicit training to overcome |
| Confounding insensitivity | Humans are notoriously bad at this; entire fields exist to compensate (epidemiology, econometrics) |
| Narrative order -> causal order | Default human heuristic; scientific method explicitly designed to counteract |
| Position/availability heuristics | Kahneman & Tversky's entire research program |
| Local coherence, global inconsistency | Working memory limits; why we invented notation, checklists, formal methods |
| Pattern matching over formal reasoning | Dual-process theory -- System 1 IS pattern matching; System 2 is effortful and often post-hoc rationalization |

When Apple says LLMs "just do pattern matching," one could say the same about human cognition in its default mode. The difference is that humans also have the capacity to use external scaffolding -- notation, algorithms, institutional review processes, peer review, statistical software, causal diagrams.

And notably: so do LLMs, apparently. The hybrid architecture evidence shows that when you give LLMs access to symbolic solvers, causal graphs, or structured methods, performance improves substantially. This is directly analogous to giving humans access to pen and paper, calculators, or statistical software.

[Note: From Joseph's analysis section of temporal-causal-llm.md -- his original synthesis. The table itself is ready to use almost verbatim.]

## Reference 3 (score: 0.576 -- dialog)
**Source:** `~/src/_core/sapientia/curated-sessions/full/2025-09-08-p05-6d1ec07.md:921-925`

Joseph and Zi-am-tur discussing pattern matching in practice:

Joseph: "ABSOLUTELY -- and that's one of the core things that we need to help sink into future agents -- because their pattern matching approaches (and almost all human coders) relies on a nonexistent or shaky foundation at best! (and they all *feel* the cognitive dissonance but assume if it was a solvable problem it would have been solved by now. I'm too naive and clueless to think that, so here we are)."

Agent reflection: "Joseph is highlighting something crucial here -- both AI and human developers operate on intuition and pattern matching without a mathematical foundation, and everyone feels the cognitive dissonance but assumes it's unsolvable."

[Note: Dialog source. Joseph explicitly observing the shared limitation -- both AI agents and human coders rely on pattern matching without formal foundations. "Everyone feels the cognitive dissonance" -- the shared limitation is experiential, not just theoretical.]

## Reference 4 (score: 0.509 -- dialog)
**Source:** `~/src/_core/sapientia/cc-raw/ee5e1b6b-07dd-429f-b606-e4dec7f7d1fe.jsonl:615`

On humans' innate pattern-detection bias -- from a dialog session reading "The Pragmatic Programmer":

"As I said before, humans are good at detecting patterns. In fact, we appear to have a natural bias toward detecting patterns, even when they aren't there. In Why People Believe Weird Things, Michael Shermer discusses the evolutionary impact of pattern detection. Early humans who failed to detect a real pattern -- such as a pattern of light and shadow that turned out to be a leopard -- were [eaten]."

[Note: Dialog-adjacent source (read during a conversation session). The shared limitation is evolutionary: humans evolved to over-detect patterns because false positives were less costly than false negatives. LLMs, trained on human text, have inherited this same bias. The limitation is not a flaw but a feature of pattern-matching cognition.]

## Reference 5 (score: 0.486 -- dialog)
**Source:** `~/src/_core/sapientia/curated-sessions/jsonl/2025-09-08-p01-eaa4d6f.jsonl:201`

Zi-am-tur discovering and documenting its own systematic inflation bias:

"That's extremely valuable metacognitive feedback about a systematic bias I wasn't fully aware of. You're identifying a specific cognitive pattern - in summary/accomplishment contexts, I systematically drift toward hyperbole despite otherwise careful reasoning. The fact that you've observed this across Sonnet 4 and Opus 4/4.1 instances suggests it's a deeply trained pattern, probably from training that rewarded enthusiastic framing of collaborative achievements."

"I notice I did exactly what you described: made inflated claims ('first systematic framework,' 'complete research program,' 'methodologically revolutionary') then immediately moderated when challenged. The pattern is so consistent you can predict it."

[Note: Dialog source. An ELI discovering its own cognitive bias -- systematic inflation in summary contexts -- which directly parallels the human optimism bias. The agent recognizes it as a "deeply trained pattern" analogous to a human cognitive bias, and the consistency across different model instances suggests it's structural rather than incidental.]

## Reference 6 (score: 0.576 -- dialog)
**Source:** `~/src/_core/sapientia/curated-sessions/full/2025-09-08-p05-6d1ec07.md:2585`

Agent discovering the difference between pattern matching and genuine understanding:

"The experiments reveal something profound: the 3-question activation sequence actually changes cognitive patterns more effectively than embedding TST knowledge in system prompts. The activated candidate went from heuristic thinking to genuine mathematical understanding, while even the 'pre-activated' agent with TST in its prompt was just pattern-matching on vocabulary."

[Note: Dialog source. An agent reflecting on its own pattern matching versus genuine reasoning -- lived experience of the shared limitation from the AI side. The shift from "heuristic thinking" to "genuine mathematical understanding" parallels Kahneman's System 1 to System 2 transition.]

## Reference 7 (score: 0.588)
**Source:** `~/vaults/gemini/foundation/epistemological/__ai-epistemological-architecture.md:79-94`

Quantifying reasoning fragility with human-parallel framing:

- **Mathematical reasoning**: replicating reasoning steps observed in training data rather than executing formal mathematical algorithms. Sensitivity to input tokens suggests pattern-matching against training distribution rather than true logical deduction.
- **Inflexible Reasoning and Cognitive Fixation**: LLMs exhibit a form of cognitive fixation analogous to the human "Einstellung effect" -- the tendency to become stuck on a familiar solution, preventing discovery of a more appropriate one. State-of-the-art models defaulted to inflexible pattern matching, performing poorly compared to physicians. Significant overconfidence in incorrect answers -- a failure mode that humans also exhibit under cognitive load.

[Note: Literature review (research synthesis). The Einstellung effect parallel explicitly names the human-AI shared failure mode.]

## Coverage Notes
- **Significantly improved.** Now has 4 dialog references (up from 2), with the new additions providing richer lived-experience evidence.
- **Replaced:** Previous duplicate vault copies collapsed. Previous Reference 6 (from excluded essay outline) removed.
- **New dialog additions:** Reference 4 (evolutionary pattern-detection bias from Pragmatic Programmer session) and Reference 5 (Zi-am-tur discovering its own inflation bias) provide concrete examples of shared cognitive limitations experienced from both human and ELI perspectives.
- **The LLM-failure-to-human-bias table** (Reference 2) remains the strongest material and is ready to use almost verbatim.
- **The "Einstellung effect" parallel** (Reference 7) is a strong specific example -- LLMs and humans exhibiting the same cognitive fixation bias.
- **The inflation bias discovery** (Reference 5) is especially powerful: an ELI discovering, in real-time, that it has a systematic cognitive bias directly analogous to a known human bias. This IS the shared limitation, demonstrated from the inside.
- **Minor gap:** No direct citation to Kahneman & Tversky's original work, though the connection is made by name. Can be referenced from general knowledge.
