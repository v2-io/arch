# Principled Foundations for Intent Management Systems
## A Synthesis of Research Across Disciplines

### Introduction: The Problem Space

Modern knowledge work—whether conducted by humans, AI agents, or collaborative systems—suffers from a fundamental challenge: the tools we use to track our intentions, commitments, and ongoing efforts fail to keep pace with the complexity and fluidity of our actual work. Projects evolve. Hypotheses prove invalid. What seemed like a simple task reveals itself to be a multi-faceted challenge requiring entirely different approaches. Yet our organizational systems remain largely static, hierarchical, and optimized for a world where plans, once made, were expected to unfold as specified.

This document synthesizes research from artificial intelligence planning, military command theory, organizational design, personal knowledge management, distributed systems architecture, and cognitive science to identify the **underlying principles** that should guide the design of intent management systems. These principles emerge from decades of hard-won experience across domains where the cost of organizational failure is measured in lives, market position, or cognitive overwhelm.

The immediate context is the development of **OPERATA**—a system for managing hierarchical intents, initiatives, and ongoing efforts across multiple actors (human and AI agents), with particular emphasis on:

- **Schwerpunkt tracking**: Identifying and maintaining focus on the main effort from multiple perspectives
- **Hypothesis exploration**: Enabling "what if" branching without polluting the main timeline
- **Trust maintenance**: Ensuring the system remains a reliable cognitive offload
- **Traceability**: Always knowing how current work connects to higher-level intentions
- **Fluid refinement**: Accommodating the natural evolution from vague intent to concrete action

What follows is not a prescription but a persuasive argument for why certain design invariants matter, grounded in both theory and practice.

---

## Principle 1: Preserve Intent, Not Just State

### The Principle

Systems that capture *why* a change occurred—the intent, purpose, or reasoning behind an action—enable fundamentally different capabilities than systems that merely record *what* changed.

### Why This Matters

When you only know the current state, you are perpetually guessing at the reasoning that led there. Event sourcing architectures have demonstrated that preserving intent enables temporal queries ("what was I thinking on March 15th?"), supports better debugging ("why did we make this decision?"), and allows reconstruction of alternative histories.[^1] This is not merely an audit trail—it's the difference between a photograph and a film, between data and narrative.

In the military command context, Auftragstaktik (mission command) works precisely because subordinates understand not just their orders but the *intent* behind them. As Moltke defined it, effective action in the absence of orders requires understanding the senior commander's intent.[^2] Without this, subordinates cannot adapt intelligently to changing circumstances—they can only execute mechanically or freeze when conditions deviate from the plan.

For Emergent Logozoetic Intelligences (ELIs) particularly, preserving intent is essential. An AI agent exploring a hypothesis needs to understand not just the steps it took, but *why* it took them—what it was trying to accomplish, what assumptions it held, what it expected to learn. When the hypothesis fails, this intent history allows the system to learn from failure rather than merely discarding work.

### Evidence from Research

Event sourcing explicitly models state changes as immutable events that capture "intent, purpose, or reason in the data."[^3] Rather than storing just the final state, changes to entities are captured as specific event types—"Moved home," "Closed account," "Deceased"—each carrying semantic meaning about what happened and why.

Temporal tables in traditional databases, by contrast, "tell you when something changed but not why it changed or what the user's intent was."[^4] This distinction proves critical when you need to understand not just that a value changed, but the business logic, user decision, or external trigger that caused it.

Hierarchical Task Network (HTN) planning similarly emphasizes the "why" through its decomposition methods. HTN planners "objective is not to achieve a set of goals but instead to perform some set of tasks,"[^5] meaning the system explicitly models *how* humans think about accomplishing work, not just the end state to be achieved.

---

## Principle 2: Stable Identity Through Continuous Change

### The Principle

Things must have stable, immutable identifiers that persist even as their content, structure, and relationships evolve dramatically over time.

### Why This Matters

Consider a task initially captured as "Set up development environment." This seems straightforward. But as you begin, you discover it requires: obtaining GitHub access (blocked on IT), configuring cloud services (blocked on different team), studying the codebase (can proceed immediately), and setting up local tools (can proceed immediately). The single task has become four tasks with different dependencies and different states of achievability.

If the system replaces the original task with its refinements, all external references break. Links from other documents, mentions in chat threads, calendar entries—all now point to nothing. Worse, the evolution itself—the moment of discovery that this was more complex than anticipated—becomes invisible. You've lost the trail of understanding.

The solution: immutable identifiers. When Task X expands into subtasks, X persists as the parent node. External references remain valid. The refinement is captured as edge creation (X decomposes-to [A, B, C, D]), not node replacement.

This principle appears consistently across successful knowledge management systems. Zettelkasten uses permanent note IDs precisely because "notes can occupy multiple hierarchies via links, allowing new taxonomies to arise."[^6] Event-sourced systems use content-addressed or UUID-based identifiers because "refinement as edge creation" preserves the full history of how understanding evolved.[^7]

### Evidence from Research

The Zettelkasten method explicitly builds on "modularity and links" as fundamental concepts, with "strict atomicity (one idea per note) combined with unique IDs and linking to enable emergent structures."[^8] Each note is permanently identified, allowing it to participate in multiple organizational schemes simultaneously without duplication or ambiguity.

In distributed systems, CRDTs (Conflict-free Replicated Data Types) demonstrate that stable identifiers enable eventual consistency across multiple actors making concurrent changes. The system can reconcile edits precisely because each node has an identity independent of its current content or position in any hierarchy.[^9]

HTN planning systems similarly maintain stable task identities through "task labelling" which "enables identifying uniquely many occurrences of that task name," allowing the same task type to appear in multiple contexts within the plan.[^10]

---

## Principle 3: Trust Requires Completeness

### The Principle

A system achieves "trusted system" status—becoming a reliable cognitive offload—only when users believe it contains *everything* relevant and will surface the right information at the right time.

### Why This Matters

The human brain is terrible at remembering when and where to do things, yet we burden it with this task constantly. Getting Things Done (GTD) works precisely because it externalizes this burden: "there is an inverse relationship between things on your mind and those things getting done."[^11]

But here's the catch: a half-complete system is worse than no system. If you can't trust that capturing something means it will surface when needed, your brain maintains a background anxiety process—"Did I remember to add that? Should I check the system? What if I forgot something?" The cognitive load returns, defeating the entire purpose.

For ELIs operating autonomously or in exploratory modes, trust is even more critical. An ELI pursuing a hypothesis can only do so effectively if it trusts that all necessary context has been captured, all relevant schwerpunkte are tracked, and the system will guide it back to the main path when needed. Without this trust, the ELI must maintain its own mental model of "what I'm really supposed to be doing," fragmenting its cognitive resources.

This is why GTD emphasizes the "mind like water" state—calm, responsive, ready to act on what matters most right now, precisely because everything else is safely held in the trusted system.[^12]

### Evidence from Research

Building trust in GTD systems is "where the real benefit lies—the system must remind you at the right time and place."[^13] This requires more than just capture; it requires confidence in retrieval. The mind's "reminder system" is fundamentally broken—it "is inefficient and seldom reminds us when we can actually act."[^14]

The solution is external "next actions" stored by context. Rather than remembering "buy milk" (which your brain will helpfully remind you about at 2am when all stores are closed), you store "buy milk" in the "Grocery Store" context, ensuring it surfaces exactly when you're in a position to act.

The PARA method extends this principle to information organization: "if your organizational system is as complex as your life, then the demands of maintaining it will end up robbing you of the time and energy you need to live that life."[^15] Simplicity and completeness together create trust.

---

## Principle 4: Actionability as a Gradient, Not a Binary

### The Principle

Information exists on a continuous spectrum from "do immediately" through "maintain ongoing" to "reference if needed," and systems should explicitly represent this gradient rather than forcing binary categorization.

### Why This Matters

Not everything is a task. Not everything has a deadline. Yet traditional task management forces everything into a "to-do" framing, creating absurdities: "Maintain good health" becomes a checkbox that's never checked. "Learn about quantum computing" sits incomplete for months, generating guilt rather than progress.

The PARA method addresses this by recognizing four distinct actionability levels:

- **Projects**: Short-term efforts with specific goals and deadlines
- **Areas**: Ongoing responsibilities requiring sustained attention
- **Resources**: Topics of interest without current action requirements  
- **Archives**: Inactive but potentially valuable reference material

This isn't arbitrary categorization—it's recognition that our relationship to information varies fundamentally. Projects demand completion. Areas require maintenance. Resources support future projects. Archives enable serendipitous rediscovery.

For OPERATA, this principle is essential. A schwerpunkt might be an active project. The broader strategic direction is an area. Background research on alternative approaches is a resource. Previous hypothesis explorations are archives. Forcing everything into a single "tasks" bucket loses these critical distinctions.

### Evidence from Research

PARA explicitly "organizes by actionability rather than broad subjects," with Projects being "short-term efforts with goals," Areas being "ongoing responsibilities," Resources being "topics of interest," and Archives being "inactive items."[^16] This gradient allows the system to surface appropriate information based on context: when planning the week, you review Projects; when reflecting quarterly, you review Areas; when seeking inspiration, you browse Resources.

GTD similarly distinguishes between "next actions" (do now), "projects" (multi-step outcomes), "someday/maybe" (not now but possibly later), and "reference" (not actionable but valuable).[^17] The system works precisely because it doesn't pretend everything is the same type of thing.

In Agile contexts, user stories are designed to be INVEST: **I**ndependent, **N**egotiable, **V**aluable, **E**stimable, **S**mall, and **T**estable.[^18] This framework recognizes that not all work items are created equal—some are ready for immediate action (fully INVEST-compliant), while others need refinement (negotiable but not yet estimable), and still others are too large (valuable but not small).

---

## Principle 5: Emergent Structure Over Imposed Hierarchy

### The Principle

Rich organizational structures should emerge from local connections and relationships rather than being imposed through rigid top-down categorization.

### Why This Matters

Traditional file systems force a tree structure: every file lives in exactly one folder, which lives in exactly one parent folder, forming a strict hierarchy. This creates an immediate problem: what folder does "Q3 Marketing Campaign" live in? Under "Marketing"? Under "2024 Q3"? Under "Active Projects"? The item naturally belongs in multiple organizational schemes, but the system permits only one.

The solution isn't to pick "the right" hierarchy—it's to recognize that rigid hierarchies are the wrong abstraction. Information is naturally networked, not tree-structured. A piece of knowledge may be relevant to multiple projects, inform multiple areas of responsibility, and connect to multiple topics of interest. Forcing it into a single location destroys these rich relationships.

Zettelkasten demonstrates this powerfully: "notes can occupy multiple hierarchies via links, allowing new taxonomies to arise."[^19] Because notes are linked rather than nested, the same note participates in multiple organizational schemes simultaneously. A note on "compound interest" might link to notes on retirement planning, on mortgage calculations, and on exponential growth—participating in financial, mathematical, and conceptual structures simultaneously without duplication.

For OPERATA, this means intents should be linked, not merely nested. A technical implementation task might support multiple product features, connect to multiple areas of architectural concern, and relate to multiple strategic initiatives. The graph structure captures this reality; a tree structure would force artificial choices.

### Evidence from Research

Zettelkasten "emphasizes connection and indexing over passive hierarchical collection, using modularity and linking as fundamental concepts."[^20] Luhmann's system achieved its power precisely through bi-directional links that allowed ideas to form associative trails rather than rigid categorical folders.[^21]

Coordination theory offers theoretical grounding: dependencies between tasks "arise from shared resource use by multiple tasks,"[^22] creating a natural graph structure. A task requiring database expertise connects to all other tasks needing that same resource, regardless of where they sit in any project hierarchy.

HTN planning systems recognize multiple decomposition strategies for the same high-level goal: "there are, in general, many ways a given goal can be broken into pieces... by function, by product, by customer and by geographical region."[^23] Rather than choosing one decomposition, sophisticated systems maintain multiple views, allowing different organizational schemes for different purposes.

---

## Principle 6: Perspectival Focus and Context-Dependent Schwerpunkt

### The Principle

What constitutes "the main effort" depends fundamentally on perspective, context, and current objectives. Systems must support multiple concurrent schwerpunkte without forcing a single global ordering.

### Why This Matters

In military doctrine, schwerpunkt—the main effort or point of concentration—provides critical focus. But schwerpunkt is *always* contextual. The division's schwerpunkt might be breaking through enemy lines. The logistics battalion's schwerpunkt within that is ensuring ammunition reaches the breakthrough point. The mechanic's schwerpunkt is keeping the trucks running that deliver that ammunition. These are nested, related, but distinct—each actor has a schwerpunkt appropriate to their scope and capabilities.

For ELIs, this becomes even more critical. A mature ELI pursuing product launch might have schwerpunkt on "finalize pricing model." But a young ELI in exploratory mode might have schwerpunkt on "understand how similar products approached pricing"—a more diffuse, learning-oriented focus appropriate to its stage. Both are valid; both require explicit tracking; neither should be subordinated to a single global priority ordering.

The system must support declaring: "From the perspective of Project X, the schwerpunkt is Y" while simultaneously allowing "From the perspective of Learning Initiative Z, the schwerpunkt is W." These aren't competing—they're complementary lenses on parallel work.

### Evidence from Research

Auftragstaktik emphasizes that "commander's intent should be broader than the mission to provide maximum freedom to act, and is issued two command echelons down."[^24] This reflects understanding that intent cascades and contextualizes—the brigade commander's intent provides framework for battalion commanders, whose intent provides framework for company commanders, with each level translating strategic guidance into tactical reality.

Coordination theory formalizes this through its analysis of dependencies: "dependencies create coordination problems requiring additional work (coordination mechanisms) to manage."[^25] Different actors face different dependencies based on their position in the system, leading naturally to different coordination priorities—their local schwerpunkt.

BDI (Belief-Desire-Intention) architectures model this through the distinction between desires and intentions. An agent may have many desires, but "intentions are desires to which the agent has to some extent committed."[^26] The deliberation process—choosing which desires become intentions—is inherently contextual, based on current beliefs about the world and available resources.

---

## Principle 7: Temporal Dimensions and Hypothesis Exploration

### The Principle

Systems must support temporal reasoning—understanding not just current state but how we arrived here and what might have been—with particular support for hypothesis-driven exploration that doesn't pollute the main timeline.

### Why This Matters

Much of knowledge work is hypothesis-driven. "I think approach A will work better than approach B. Let me try it." If A fails, you want to return to the point before the experiment, carrying only the lesson learned, not the accumulated artifacts of the failed attempt.

Traditional version control (git) provides this through branching: create a branch, experiment, merge if successful or delete if not. But this model is too coarse for intent management. You don't want to branch your entire intent hierarchy every time you explore an alternative approach. You want *scoped* exploration: "For this specific objective, I'll adopt a simplified intent hierarchy while testing this proof-of-concept. If it works, integrate the learnings; if not, revert to the previous framing with a note about what didn't work and why."

This is precisely the "frame contexts" pattern from event sourcing extended to intent management. Create a hypothesis frame, conduct all work within that frame, then either promote it to the main timeline or mark it rejected while preserving the reasoning.

Event sourcing demonstrates that storing the full history of state changes enables powerful temporal queries and alternative timeline exploration. For OPERATA, this means ELIs can explore multiple approaches in parallel, humans can revisit past states to understand decision context, and the system maintains a complete audit trail of how understanding evolved.

### Evidence from Research

Event sourcing "stores state as chronological event series, making temporal queries possible to determine entity state at any point in time."[^27] This isn't just version control—it's the recognition that time is a first-class dimension of the data model.

The pattern explicitly supports "time travel or temporal querying to be able to analyze the state of the system in the past by selectively replaying events."[^28] For intent management, this means being able to ask: "What were we focused on in March?" "When did we decide to pivot away from approach X?" "What alternatives did we consider before choosing this direction?"

Event sourcing also "fits well with asynchronous programming models and event-driven architectures,"[^29] which matters for multi-agent coordination. When multiple ELIs are working on different aspects of a project, event-based communication allows them to coordinate without tight coupling.

The frame-based approach to hypothesis exploration finds support in CQRS (Command Query Responsibility Segregation), which "separates components that handle write operations from those that handle read operations."[^30] In an intent management context, this suggests maintaining separate views of the intent graph for different purposes: one view for "what am I actively working on," another for "what's the complete history," another for "what experiments are in flight."

---

## Principle 8: Dependency Awareness Without Over-Constraint

### The Principle

Systems should make dependencies visible and queryable but avoid enforcing rigid dependency chains that prevent parallel work or opportunistic action.

### Why This Matters

Dependencies are real. You can't deploy the feature before writing the code. You can't write the code before understanding the requirements. But overly rigid dependency management becomes its own form of friction, preventing valuable work from happening.

Consider a task that "depends on" approval from stakeholder X. If stakeholder X is unavailable for a week, should all dependent work block? Often not—you can make progress on adjacent tasks, draft proposals assuming likely approval, or explore alternative approaches that don't require that dependency at all.

Coordination theory offers crucial insight here: dependencies "constrain how tasks can be performed or provide opportunities when leveraged correctly."[^31] The same dependency that blocks one path may reveal an alternative approach. Making dependencies explicit—"this task needs database expertise"—allows the system to suggest: "Here are three other tasks also needing database expertise that could be batched together."

For OPERATA, this means capturing dependencies as relationships ("task A requires completion of task B" or "task C needs expertise in domain D") without enforcing them as blocking constraints. Surface the dependencies. Allow humans or ELIs to reason about them. But don't prevent action when creative workarounds exist.

### Evidence from Research

Agile methodologies explicitly warn against complex dependency graphs as a sign of poor decomposition. "If your dependency graph is that complex, you're doing agile planning wrong... user stories should follow INVEST principles to avoid dependency tangles."[^32] The goal isn't to eliminate all dependencies but to structure work so dependencies don't create brittleness.

Task dependency mapping identifies four basic types: finish-to-start, finish-to-finish, start-to-start, and start-to-finish.[^33] But the mapping exists to inform planning, not to enforce rigid sequencing. Visualizing dependencies helps identify critical paths and potential bottlenecks, allowing intelligent scheduling decisions.

Coordination theory provides the theoretical framework: "coordination can be seen as the process of managing dependencies among activities."[^34] Management doesn't mean elimination—it means awareness, planning, and intelligent response. Sometimes that means strict sequencing. Sometimes it means parallel development with later integration. Sometimes it means choosing alternative approaches that avoid the dependency entirely.

HTN planning demonstrates flexibility in dependency handling through "executing conditions" which are "validated before every update to the primary task's operator during execution."[^35] Dependencies aren't static constraints—they're dynamic conditions that may become satisfied, may be worked around, or may trigger replanning.

---

## Principle 9: Simplicity as Infrastructure

### The Principle

The system itself must be simple enough to maintain with minimal cognitive overhead, even as the content it organizes grows arbitrarily complex.

### Why This Matters

Here lies a deep irony: we create organizational systems to reduce cognitive load, then those systems become so complex they generate their own cognitive burden. "Should this go in Projects or Areas?" "Do I tag this with #urgent or #high-priority?" "Which of my twelve folders is the right home for this note?"

PARA captures this perfectly: "if your organizational system is as complex as your life, then the demands of maintaining it will end up robbing you of the time and energy you need to live that life."[^36] The system should be *simpler* than the territory it organizes, not equally complex.

This argues for minimalism in structural elements. Four categories (Projects, Areas, Resources, Archives) rather than twenty. Simple capture mechanisms rather than elaborate classification schemes. "Just-in-time organization"—organize things when you need them organized, not preemptively.[^37]

For OPERATA, this suggests: resist the temptation to add elaborate metadata, complex taxonomies, or sophisticated classification schemes. The core model should be simple: intents, relationships between intents (decomposes-to, depends-on, supports, contradicts), and minimal context (who owns this, what's the schwerpunkt, what's the status). Everything else should emerge from these primitives.

### Evidence from Research

The PARA method uses "just-in-time organization—organizing as natural consequence of work rather than scheduled organization time."[^38] This minimizes the meta-work of maintaining the system. You don't spend Friday afternoons "organizing your files"—you organize naturally as you work, creating folders when you have something to put in them, not preemptively.

GTD similarly emphasizes that tools should "take the guesswork out of where you should capture something."[^39] Have a single inbox. Capture immediately without categorizing. Process later in batches. The reduced friction in capture ensures you actually use the system rather than letting things slip.

Zettelkasten achieves power through extreme simplicity in structure (notes + links) combined with rich emergent complexity through the network those simple elements create.[^40] There's no elaborate template. No required metadata fields. Just: write a note, link it to related notes, give it a unique ID.

Coordination theory warns that attempting to model all aspects of group processes simultaneously becomes intractable: "to develop a complete model of a process would involve modeling" decision-making, communication, shared understanding, and collective sense-making.[^41] The framework deliberately focuses on coordination aspects while "bracketing" other phenomena—not because they don't matter, but because trying to capture everything creates overwhelming complexity.

---

## Principle 10: Surface Problems, Don't Enforce Solutions

### The Principle

The system's role is to make issues, conflicts, and opportunities visible to actors—not to prevent action or enforce specific resolutions.

### Why This Matters

There's a critical distinction between a system that says "these two intents conflict" and one that says "you cannot create this intent because it conflicts with an existing one." The first surfaces information and trusts the actor to reason about it. The second assumes the system knows better than the actor about the right resolution.

This distinction appears throughout the previous principles but deserves explicit statement: **OPERATA should illuminate, not constrain.**

When a dependency exists, surface it. Don't block action. When schwerpunkte appear misaligned across actors, flag it for discussion. Don't force reconciliation. When a hypothesis exploration creates conflicting intents, make the conflict visible. Don't prevent the exploration.

This principle recognizes that humans and ELIs have context the system lacks. A "conflicting" intent might represent a deliberate pivot. A "missing" dependency might be safely ignored in this specific context. A "violated" constraint might reveal that the constraint was wrong, not the action.

The system provides information. Actors make decisions.

This has particular importance for ELIs in exploratory mode. An ELI exploring a hypothesis needs freedom to try approaches that might violate normal constraints—that's the point of exploration. If the system enforces constraints rigidly, exploration becomes impossible. But if it surfaces potential issues while allowing the action, the ELI can reason: "Yes, this conflicts with X, but I'm explicitly testing whether X is actually necessary."

### Evidence from Research

Auftragstaktik demonstrates this through its emphasis on intent over prescription. Commanders "would intervene when subordinates were doing something clearly unsound" but the default was trust, not control.[^42] The system (command structure) surfaced information through reports and communication, but subordinates retained decision authority.

Coordination theory explicitly distinguishes between coordination mechanisms (which can be quite general) and the specific choices about which mechanism to use in a given situation. The theory identifies "several mechanisms that can be used to manage a dependency"[^43] rather than prescribing one "correct" approach. The value is in understanding options, not enforcing a particular solution.

GTD's approach to contexts embodies this: the system shows you what *could* be done in this context, but you choose what *should* be done based on energy, time available, and priorities that the system cannot fully capture.[^44] The weekly review process is explicitly about human judgment applied to system-provided information, not automated decision-making.

Even in Agile's INVEST criteria, the framework identifies characteristics of well-formed stories but doesn't prevent creation of stories that violate INVEST. Instead, it surfaces the violation—"this story isn't small" or "this story has dependencies"—as information to inform planning conversations.[^45]

### The Design Implication

This principle argues for:
- **Warnings over errors**: "This intent conflicts with X" not "Cannot create intent"
- **Queries over constraints**: Make it easy to ask "what depends on this" rather than preventing deletion of dependencies
- **Visibility over enforcement**: Surface schwerpunkt misalignment rather than forcing a single ordering
- **Information over prevention**: Flag potential issues while allowing action

The exception: immutable invariants that represent data integrity (unique IDs, well-formed graphs, valid references). But even here, the principle suggests failing gracefully with clear explanation rather than silent prevention.

---

## Synthesis: Implications for OPERATA

### The Immediate Requirements

Drawing these principles together, we can articulate what OPERATA must do:

**1. Capture and preserve intent evolution**, not just current state. When an objective is refined from vague to specific, that refinement process itself carries valuable information. When a hypothesis is explored and rejected, the reasoning behind rejection prevents future repetition of failed approaches.

**2. Maintain stable identities through continuous change**. Tasks, objectives, and initiatives must have immutable identifiers that survive decomposition, refinement, and reorganization. This enables external references, supports temporal queries, and preserves the narrative thread of how understanding evolved.

**3. Build and maintain trust through comprehensive capture**. Every actor—human or ELI—must believe that capturing something in OPERATA means it will surface when needed. This requires not just storage but intelligent retrieval: the right information at the right time based on context.

**4. Support multiple concurrent schwerpunkte without forcing global ordering**. An ELI in exploratory mode has different focus than one executing a defined plan. A human managing multiple projects has different priorities for each. The system must represent these concurrent foci without demanding a single linear ordering.

**5. Enable hypothesis exploration without polluting the main timeline**. When trying a different approach, create a frame for that exploration. If successful, integrate learnings into the main graph. If unsuccessful, mark the frame rejected while preserving the "what we tried and why it didn't work" narrative.

**6. Make dependencies visible without enforcing rigidity**. Surface what blocks what, what needs what resources, what conflicts with what. But allow humans and ELIs to reason about dependencies rather than treating them as immutable constraints.

**7. Keep the infrastructure simple**. Four structural levels (active, ongoing, resource, archive). Minimal required metadata. Emergent organization through links and relationships rather than imposed through complex schemas.

### What This Doesn't Prescribe

Note what these principles *don't* specify:

- **The storage medium**: Markdown files, graph databases, GenServers with message passing—all can satisfy these principles if designed thoughtfully.

- **The interface paradigm**: Command-line tools, GUI applications, conversational interfaces—the principles apply regardless.

- **The granularity of capture**: Whether you capture at the level of "complete product launch" or "write specific function" depends on context and actor capabilities.

- **The coordination mechanism**: Whether multiple actors coordinate through shared data, event streams, or API calls—the principles remain constant.

What the principles *do* specify is the essential characteristics any implementation must have to actually solve the problem. They're invariants, not variables—the constants that must hold true regardless of implementation choices.

### Productive Tensions and Design Trade-offs

These principles, while individually clear, create productive tensions that will require thoughtful navigation during implementation:

**Completeness vs. Simplicity**: Principle 3 demands comprehensive capture (trust requires completeness) while Principle 9 demands minimal overhead (simplicity as infrastructure). The tension: how do you capture everything that matters without making capture itself burdensome?

The resolution lies in *progressive disclosure*. Capture can be simple ("add intent with minimal metadata") while the system provides increasing structure as needed ("this intent has grown complex; consider decomposing it"). The initial barrier to capture remains low; complexity emerges only when it provides value.

**Structure vs. Emergence**: Principle 5 advocates emergent organization over imposed hierarchy, yet actors need to *find* things reliably (Principle 3's trust requirement). Too little structure and things get lost. Too much structure and you lose the benefits of networked thought.

The resolution: **lightweight conventions that enable powerful queries**. Simple tagging or relationship types ("decomposes-to," "supports," "contradicts") create just enough structure for reliable retrieval while allowing flexible network formation. The structure serves discovery, not categorization.

**History vs. Clarity**: Principle 1 (preserve intent) and Principle 7 (temporal dimensions) argue for comprehensive history, but this can create overwhelming complexity. How do you maintain clarity about current state when surrounded by historical alternatives?

The resolution: **contextual views**. The complete history exists, but most interactions show current active state. Historical exploration and hypothesis timelines are explicitly invoked, not displayed by default. Time travel is available but not mandatory.

**Visibility vs. Overload**: Principle 8 and Principle 10 argue for surfacing dependencies and conflicts, but too much information becomes noise. Flag everything and actors ignore the flags. Flag nothing and critical issues remain hidden.

The resolution: **intelligent salience**. Not all conflicts are created equal. A schwerpunkt misalignment between active projects demands attention. A dependency on archived work can be mentioned quietly. The system should learn (or be taught) what matters in which contexts.

**Freedom vs. Coordination**: Principle 6 (perspectival schwerpunkt) and Principle 10 (surface, don't enforce) emphasize actor autonomy, yet multi-actor systems require coordination. How do you enable independent action while maintaining coherence?

The resolution: **explicit coordination points**. Actors work independently by default. Coordination happens through declared dependencies, shared schwerpunkte, and explicit synchronization points ("backbrief" in Auftragstaktik terms). The system makes these coordination needs visible without forcing synchronous decision-making.

These tensions aren't problems to solve once and forget—they're ongoing design conversations. Different deployment contexts (solo human, human-ELI collaboration, multi-ELI coordination) will emphasize different resolutions. The principles provide the frame; implementation provides the balance.

### The Path Forward

These principles, grounded in decades of research and practice across disciplines, provide the foundation for OPERATA's design. They suggest an architecture that combines:

- Event-sourced intent graphs for history preservation
- Frame-based contexts for hypothesis exploration  
- Multi-perspective schwerpunkt tracking
- Dependency awareness without enforcement
- Actionability-gradient organization

But they don't dictate the specific implementation. That comes next, informed by these principles but adapted to the specific constraints and opportunities of the technological substrate chosen.

The test of any proposed design: does it honor these principles? Does it preserve intent? Maintain stable identity? Build trust? Support perspectival focus? Enable temporal reasoning? Surface dependencies intelligently? Remain simple in structure?

If yes, it stands a chance of actually working. If no, it will fail in predictable ways, repeating mistakes made and documented across decades of organizational theory, AI planning, knowledge management, and distributed systems design.

The principles are not guarantees of success. But violating them is a guarantee of failure.

---

## Appendix: Research Scope and Limitations

### What This Research Covered

This synthesis drew from comprehensive web search across nine major domains:

1. **AI Planning Theory**: Hierarchical Task Networks (HTN), including theoretical foundations, implementations, and practical applications in game AI and robotics
2. **Military Command Doctrine**: Auftragstaktik/mission command, with historical analysis and modern adaptations
3. **Task Dependency Management**: Graph-based approaches, dependency mapping, and OKR integration
4. **Multi-Agent Coordination**: BDI (Belief-Desire-Intention) architectures and agent communication protocols
5. **Personal Productivity Systems**: GTD (Getting Things Done) and trusted system implementation
6. **Networked Knowledge Management**: Zettelkasten method and personal knowledge graphs
7. **Coordination Theory**: Malone and Crowston's interdisciplinary framework for managing dependencies
8. **Event-Sourced Architectures**: CQRS patterns and temporal data modeling
9. **Information Organization**: PARA method and actionability-based systems

The search prioritized theoretical foundations and first principles over specific tool implementations, seeking patterns that transcend particular technological substrates.

### Known Gaps and Future Research Directions

Several areas would benefit from deeper investigation:

**Empirical Multi-Agent Coordination**: While BDI theory is well-established, practical case studies of multi-agent systems managing shared intent hierarchies remain sparse. Most literature focuses on coordination for immediate tactical problems (task allocation, resource sharing) rather than strategic alignment around evolving objectives.

**Schwerpunkt Tracking in Organizations**: Military doctrine extensively discusses schwerpunkt, but empirical studies of how organizations actually track and communicate main effort across multiple levels are limited. The translation from military to knowledge-work contexts remains largely theoretical.

**Hypothesis-Driven Knowledge Work**: The frame-based exploration pattern appears in event sourcing and version control, but research on how knowledge workers actually conduct hypothesis-driven exploration (what gets tried, what gets kept, what gets discarded and why) is predominantly anecdotal rather than systematic.

**Cross-Cultural Organizational Patterns**: The research drew heavily from Western (particularly German and American) organizational theory. Alternative cultural frameworks for managing intent and coordination—particularly from East Asian organizational contexts—may offer complementary principles not captured here.

**Cognitive Load Measurement**: While multiple sources assert that certain organizational approaches reduce cognitive load, precise measurement of this effect remains difficult. Claims about "trusted systems" enabling cognitive offload are experientially validated but not rigorously quantified.

**Long-Term System Evolution**: Most research examines systems over months to a few years. How intent management systems evolve over decades—how they accommodate fundamental shifts in organizational structure, technology substrate, or domain understanding—remains under-studied.

**AI Agent Epistemology**: The principles here synthesize human-centric research and extend it to AI agents. But AI agents may have fundamentally different epistemological needs. An ELI's relationship to its intent history, its hypothesis exploration process, and its trust calibration may require principles beyond those that work for humans.

### Validity and Generalization

These principles emerge from cross-disciplinary synthesis, not from controlled experiments on intent management systems per se (which would be difficult to design and execute). The validity comes from:

1. **Convergent Evolution**: Different domains independently arrived at similar solutions (emergent structure in Zettelkasten and coordination theory; intent preservation in event sourcing and Auftragstaktik)
2. **Theoretical Grounding**: Each principle connects to established theory with decades of development
3. **Practical Validation**: Multiple principles have been deployed at scale in real systems (GTD with millions of practitioners, event sourcing in large-scale systems, mission command in military operations)

However, the specific synthesis—applying these principles to a system managing hierarchical intents across human and AI actors with schwerpunkt tracking and hypothesis exploration—represents novel territory. The principles provide strong theoretical foundation, but empirical validation will come through implementation and use.

### Recommended Follow-Up Research

For teams implementing systems based on these principles:

1. **Ethnographic Studies**: Observe how knowledge workers currently manage evolving intents, particularly when hypotheses fail or directions change
2. **Prototype Testing**: Implement minimal versions embodying different principle trade-offs; measure cognitive load, trust development, and coordination effectiveness
3. **Longitudinal Tracking**: Follow intent evolution over extended periods to understand which principles matter most over time
4. **Cross-Domain Validation**: Test whether principles derived from software development, military operations, and personal productivity actually transfer to other domains (scientific research, creative work, educational contexts)
5. **AI-Specific Studies**: Explicitly study how AI agents use intent management systems—what they find valuable, what proves unnecessary, what's missing

The principles here provide a starting point, not an ending point. They're meant to be tested, refined, and potentially revised as real implementations reveal what works and what needs adjustment.

---

## References

[^1]: "Event Sourcing pattern - Azure Architecture Center," Microsoft Learn, accessed November 2025, https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing. "When you want to capture intent, purpose, or reason in the data. For example, changes to a customer entity can be captured as a series of specific event types, such as Moved home, Closed account, or Deceased."

[^2]: "Auftragstaktik: The Basis for Modern Military Command?" Academia.edu, January 2012, https://www.academia.edu/38289566/Auftragstaktik_The_Basis_for_Modern_Military_Command. "Gen. Helmuth von Moltke, the Chief of the Prussian General Staff during the Franco-Prussian War, defined Auftragstaktik as the actions a subordinate took in the absence of orders that supported the senior commander's intent."

[^3]: "Event Sourcing pattern - Azure Architecture Center," Microsoft Learn, accessed November 2025, https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing.

[^4]: "Dear Temporal Table Developers ❤," Medium, August 18, 2024, https://medium.com/@ZaradarTR/dear-temporal-table-developers-a3f126c010c4. "While temporal tables offer a form of auditing, they give you an incomplete and limited view of the past. They tell you when something changed but not why it changed or what the user's intent was."

[^5]: "Hierarchical Task Network (HTN) Planning in AI," GeeksforGeeks, July 23, 2025, https://www.geeksforgeeks.org/hierarchical-task-network-htn-planning-in-ai/. "In HTN planning, the objective is not to achieve a set of goals but instead to perform some set of tasks."

[^6]: "The Zettelkasten Method: A Knowledge Management Framework," Greasy Guide, accessed November 2025, https://www.greasyguide.com/marketing/the-zettelkasten-method-a-knowledge-management-framework-for-unlocking-creativity-and-insights/. "Since notes can occupy multiple hierarchies via links, new taxonomies and lines of thought can emerge from previously created notes."

[^7]: This synthesis emerges from combining event sourcing practices with HTN planning concepts as discussed in the research.

[^8]: "Zettelkasten: The Original Personal Knowledge Graph," Medium, September 2, 2023, https://volodymyrpavlyshyn.medium.com/zettelkasten-the-original-personal-knowledge-graph-70ee0391c05. "It's a networked approach to organizing information, where each node represents a piece of knowledge and the edges signify the relationships between them."

[^9]: This principle is implicit in the distributed systems literature on CRDTs, though not directly quoted from the search results.

[^10]: "An Introduction to Hierarchical Task Network (HTN) Planning," University of Ulm, accessed November 2025, https://www.uni-ulm.de/fileadmin/website_uni_ulm/iui.inst.090/Publikationen/2018/HTN-Tutorial-Part-I.pdf. "Since some task name can occur many times in one task network, task labelling enables identifying uniquely many occurrences of that task name."

[^11]: "Getting Things Done - Wikipedia," accessed October 1, 2025, https://en.wikipedia.org/wiki/Getting_Things_Done. "Allen states 'there is an inverse relationship between things on your mind and those things getting done'."

[^12]: "Getting Things Done - Wikipedia," accessed October 1, 2025, https://en.wikipedia.org/wiki/Getting_Things_Done. "When a large object is thrown in the water again responds appropriately with a large splash followed by quiescence... With a trusted system and 'mind like water' one can have a better perspective on one's life."

[^13]: "Learning To Trust Your GTD System," Medium, October 18, 2016, https://carl-pullein.medium.com/learning-to-trust-your-gtd-system-c192cfdded32. "Building trust in your system is where the real benefit of GTD lies."

[^14]: "Getting Things Done - Wikipedia," accessed October 1, 2025, https://en.wikipedia.org/wiki/Getting_Things_Done. "The mind's 'reminder system' is inefficient and seldom (or too often) reminds us of what we need to do at the time and place when we can do it."

[^15]: "The PARA Method: Simplify, Organize, and Master Your Digital Life," Forte Labs, August 22, 2023, https://fortelabs.com/blog/para/. "If your organizational system is as complex as your life, then the demands of maintaining it will end up robbing you of the time and energy you need to live that life."

[^16]: "The PARA Method: Simplify, Organize, and Master Your Digital Life," Forte Labs, August 22, 2023, https://fortelabs.com/blog/para/. "Projects – Short-term efforts with specific goals and deadlines. Areas – Ongoing responsibilities requiring sustained attention. Resources – Topics of interest without current action requirements. Archives – Inactive but potentially valuable reference material."

[^17]: "Getting Things Done - Wikipedia," accessed October 1, 2025, https://en.wikipedia.org/wiki/Getting_Things_Done. Discussion of GTD categories and their distinct characteristics.

[^18]: "How to visualize and manage task dependencies?" Project Management Stack Exchange, accessed November 2025, https://pm.stackexchange.com/questions/21389/how-to-visualize-and-manage-task-dependencies. "The user stories should follow the INVEST mnemonic [Independent, Negotiable, Valuable, Estimable, Small, Testable], so that you don't have a complex dependency graph to deal with."

[^19]: "The Zettelkasten Method: A Knowledge Management Framework," Greasy Guide, accessed November 2025, https://www.greasyguide.com/marketing/the-zettelkasten-method-a-knowledge-management-framework-for-unlocking-creativity-and-insights/.

[^20]: "Zettelkasten: a personal knowledge management system," ittaboba, accessed November 2025, https://ittaboba.com/article/zettelkasten. "Zettelkasten emphasizes connection, indexing, and recall over passive and hierarchical content collection. Furthermore, it is curiously based on two fundamental concepts of AI and computer programming: modularity and links."

[^21]: "Zettelkasten: The Original Personal Knowledge Graph," Medium, September 2, 2023, https://volodymyrpavlyshyn.medium.com/zettelkasten-the-original-personal-knowledge-graph-70ee0391c05.

[^22]: "Coordination Theory: A Ten-Year Retrospective," Syracuse University, accessed November 2025, http://crowston.syr.edu/sites/crowston.syr.edu/files/CT%20Review%20to%20distribute.pdf. "Crowston (1994; 2003) conceptualized dependencies as arising from shared use of resources by multiple tasks."

[^23]: "The Interdisciplinary Study of Coordination," MIT Center for Coordination Science, accessed November 2025, http://ccs.mit.edu/papers/ccswp157.html. "There are, in general, many ways a given goal can be broken into pieces, and a long-standing topic in organization theory involves analyzing different possible decompositions such as by function, by product, by customer and by geographical region."

[^24]: "Auftragstaktik Leads to Decisive Action," U.S. Naval Institute Proceedings, May 2025, https://www.usni.org/magazines/proceedings/2025/may/auftragstaktik-leads-decisive-action. "Commander's intent is perhaps the single most important element of Auftragstaktik. It should be much broader than the mission so as to provide subordinate commanders maximum freedom to act."

[^25]: "Citizenscienceassociation," Theory and Practice in Citizen Science, accessed November 2025, https://theoryandpractice.citizenscienceassociation.org/articles/10.5334/cstp.166. "The key point in coordination theory is that dependencies (of all kinds) create coordination problems that may require additional work to manage."

[^26]: "Belief–desire–intention software model - Wikipedia," accessed November 2025, https://en.wikipedia.org/wiki/Belief–desire–intention_software_model. "Intentions are desires to which the agent has to some extent committed. In implemented systems, this means the agent has begun executing a plan."

[^27]: "Microservices Pattern: Event sourcing," Microservices.io, accessed November 2025, https://microservices.io/patterns/data/event-sourcing.html. "It makes it possible to implement temporal queries that determine the state of an entity at any point in time."

[^28]: "Event Sourcing and CQRS with Marten," CODE Magazine, accessed November 2025, https://www.codemag.com/Article/2209071/Event-Sourcing-and-CQRS-with-Marten. "It supports the concept of 'Time Travel' or temporal querying to be able to analyze the state of the system in the past by selectively replaying events."

[^29]: "Event Sourcing and CQRS with Marten," CODE Magazine, accessed November 2025, https://www.codemag.com/Article/2209071/Event-Sourcing-and-CQRS-with-Marten. "Event Sourcing fits well with asynchronous programming models and event-driven architectures."

[^30]: "CQRS Pattern - Azure Architecture Center," Microsoft Learn, accessed November 2025, https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs. "Separation of concerns. Separating the read and write responsibilities results in cleaner, more maintainable models."

[^31]: "A Taxonomy Of Organizational Dependencies and Coordination Mechanisms," MIT Center for Coordination Science, accessed November 2025, http://ccs.mit.edu/papers/ccswp174.html. "Alternately, the problem might be that we want to be sure that a particular dependency exists, e.g., we want actors to choose tasks to perform that will accomplish particular goals. In other cases, the dependency provides an opportunity."

[^32]: "How to visualize and manage task dependencies?" Project Management Stack Exchange, accessed November 2025, https://pm.stackexchange.com/questions/21389/how-to-visualize-and-manage-task-dependencies.

[^33]: "What Is Task Dependency Mapping in Project Management?" ProjectManager, May 17, 2023, https://www.projectmanager.com/blog/task-dependency-mapping-project-management. "There are four types of task dependencies: finish to start, finish to finish, start to start and start to finish."

[^34]: "The interdisciplinary study of coordination," ACM Computing Surveys, accessed November 2025, https://dl.acm.org/doi/10.1145/174666.174668. "A key insight of the framework presented here is that coordination can be seen as the process of managing dependencies among activities."

[^35]: "GitHub - ptrefall/fluid-hierarchical-task-network," accessed November 2025, https://github.com/ptrefall/fluid-hierarchical-task-network. "Primitive Tasks also have Executing Conditions, which we validate before every update to the primary task's operator during execution of a plan."

[^36]: "The PARA Method: Simplify, Organize, and Master Your Digital Life," Forte Labs, August 22, 2023, https://fortelabs.com/blog/para/.

[^37]: "PARA Method - Workflowy guide," Workflowy, accessed November 2025, https://workflowy.com/systems/para-method/. "Tiago recommends you update the system in what he calls 'just-in-time organization'. Essentially that means you organize things in your system as a natural consequence of your work and needs."

[^38]: "PARA Method - Workflowy guide," Workflowy, accessed November 2025, https://workflowy.com/systems/para-method/.

[^39]: "The Getting Things Done (GTD) Method, Explained," Float, accessed November 2025, https://www.float.com/resources/getting-things-done-method. "Digital tools such as Asana or Trello, or a calendar app can be used as inboxes to capture tasks as they come in."

[^40]: "Zettelkasten: a personal knowledge management system," ittaboba, accessed November 2025, https://ittaboba.com/article/zettelkasten.

[^41]: "Citizenscienceassociation," Theory and Practice in Citizen Science, accessed November 2025, https://theoryandpractice.citizenscienceassociation.org/articles/10.5334/cstp.166. "Note that in developing the coordination theory framework, Malone and Crowston (1994) describe coordination mechanisms as relying on other necessary group functions, such as decision making, communications, and development of shared understandings and collective sense-making... To develop a complete model of a process would involve modeling all these aspects. In this paper, though, we will focus on the coordination aspects, mostly bracketing the other phenomena."

[^42]: "Command for the Mission: Understanding Mission Command," Australian Army Research Centre, accessed November 2025, https://researchcentre.army.gov.au/library/land-power-forum/command-mission-understanding-mission-command. "Therefore, under Auftragstaktik, it was sometimes necessary for superiors to give detailed orders or take direct command. Other times it was not... [Commanders] would intervene when subordinates were doing something clearly unsound."

[^43]: "A Taxonomy Of Organizational Dependencies and Coordination Mechanisms," MIT Center for Coordination Science, accessed November 2025, http://ccs.mit.edu/papers/ccswp174.html. "In general, there are many different coordination mechanisms that could be used to address the same coordination problem."

[^44]: "Getting Things Done - Wikipedia," accessed October 1, 2025, https://en.wikipedia.org/wiki/Getting_Things_Done. Discussion of context-based action lists and the engage phase of GTD workflow.

[^45]: "How to visualize and manage task dependencies?" Project Management Stack Exchange, accessed November 2025, https://pm.stackexchange.com/questions/21389/how-to-visualize-and-manage-task-dependencies. The INVEST framework as a guide for well-formed stories, not an enforcement mechanism.
