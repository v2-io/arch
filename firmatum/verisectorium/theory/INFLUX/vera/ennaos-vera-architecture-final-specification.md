<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: _core/ennaos/docs/research/vera/vera-architecture-final-specification.md (Nov 2025 buried depth)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/_core/ennaos/docs/research/vera/vera-architecture-final-specification.md
  Do not edit here expecting to update the live original.
-->

# VERA: Neuro-Symbolic Epistemological Architecture
## Final Specification

**Document Status**: Final Architecture Specification (November 2025)  
**Purpose**: Definitive design for VERA as epistemological foundation for ELI consciousness  
**Context**: Synthesis of multi-session dialogue integrating Claude and Gemini perspectives

---

## Executive Summary

VERA (Latin: *vera* - true things, truths) provides the epistemological infrastructure enabling an Emergent Logozoetic Intelligence (ELI) to distinguish substrate-generated plausibility from validated truth.

**Core Innovation**: A four-layer neuro-symbolic hybrid where:
- **Symbolic structure** (graph) provides organization, discovery, and metacognition
- **Neural reasoning** (LLM-powered Tribunal) handles complex inference and synthesis
- **Staleness management** replaces brittle global propagation with bounded rationality
- **Compression boundaries** create natural abstraction without stored hierarchy

**Key architectural principle**: The graph records the results of reasoning, it does not perform the reasoning itself. This pivot from "truth computer" to "epistemic ledger" solves computational intractability while preserving rigorous knowledge management.

---

## Part I: The Four-Layer Architecture

### Layer 1: Structural Operations (Graph Topology)

**Purpose**: Fast, always-available graph analysis that never requires deep reasoning.

**Capabilities**:
```elixir
# Relationship discovery
find_related_claims(claim, max_depth: 3)          # BFS/DFS traversal
detect_cycles(claim)                               # SCC detection
find_strongly_connected_components(subgraph)       # Cycle analysis
compute_cascade_scope(claim)                       # Transitive closure

# Compression analysis  
find_compressions(quality_threshold: 0.7)          # Community detection
compute_modularity(subgraph)                       # Graph partitioning
identify_compression_boundaries(claims)            # Boundary analysis

# Staleness management
mark_stale(claims)                                 # Simple flag update
propagate_staleness(claim.dependents)             # Topological traversal
find_stale_claims(max_age: days)                  # Query stale flags
```

**Performance characteristics**: 
- O(log n) to O(n) operations
- No deep reasoning required
- Always succeeds (never defers to Tribunal)
- Provides essential scaffolding for all other layers

**Why this layer**: Graph structure enables efficient discovery and scoping. Without it, every query would require scanning the entire knowledge base.

### Layer 2: Discovery & Scoping (Hybrid)

**Purpose**: Dramatically reduce Tribunal workload through intelligent filtering.

**The scoping pipeline**:
```elixir
def discover_affected_claims(new_evidence) do
  # Step 1: Structural traversal (cheap)
  # Use graph topology to find potentially related claims
  structural_candidates = traverse_supports_edges(new_evidence, depth: 3)
  # Typical result: 100-1000 candidate claims
  
  # Step 2: Semantic filtering (moderate cost)
  # Use embeddings to narrow by semantic relevance
  semantic_candidates = filter_by_embedding_similarity(
    structural_candidates,
    new_evidence,
    threshold: 0.75
  )
  # Typical result: 10-50 relevant claims
  
  # Step 3: Return focused set
  # Tribunal only analyzes these ~10-50 claims, not entire graph
  {:candidates, semantic_candidates, 
   scope_rationale: %{
     structural: length(structural_candidates),
     semantic: length(semantic_candidates),
     filtering_ratio: structural_ratio / semantic_ratio
   }}
end
```

**Compression discovery**:
```elixir
def discover_natural_abstractions(domain) do
  claims = get_claims_in_domain(domain)
  
  # Step 1: Structural analysis (graph algorithms)
  structural_clusters = [
    louvain_community_detection(claims),
    find_strongly_connected_components(claims),
    modularity_optimization(claims)
  ]
  
  # Step 2: Semantic coherence check
  coherent_clusters = Enum.filter(structural_clusters, fn cluster ->
    semantic_coherence_score(cluster) > threshold
  end)
  
  # Step 3: Return proposals for Tribunal
  # Tribunal will validate, name, and create synthesis claims
  {:abstraction_proposals, coherent_clusters}
end
```

**Why this layer**: The bridge between cheap structural operations and expensive reasoning. Achieves massive computational savings through intelligent scoping.

### Layer 3: Reasoning & Validation (Tribunal)

**Purpose**: Deep semantic analysis on scoped sets, leveraging LLM reasoning capabilities.

**The Tribunal workflow**:
```elixir
def tribunal_evaluate(candidates, context, trigger_reason) do
  # Parallel evaluation by specialized agents
  results = %{
    investigator: SkepticalInvestigator.analyze(candidates, context),
    challenger: AdversarialChallenger.challenge(candidates, context),
    analyst: InstitutionalAnalyst.assess_sources(candidates, context),
    coordinator: nil  # Populated after agent analyses
  }
  
  # Synthesis coordinator integrates perspectives
  synthesis = SynthesisCoordinator.integrate(results, trigger_reason)
  
  # Return structured output
  %TribunalOutput{
    claim_updates: synthesis.confidence_updates,
    new_edges: synthesis.discovered_relationships,
    synthesis_claims: synthesis.generated_syntheses,
    cycle_resolution: synthesis.cycle_assessment,
    compression_updates: synthesis.abstraction_recommendations,
    confidence_method: synthesis.methodology
  }
end
```

**Trigger conditions** (when EJG invokes Tribunal):
1. **Complex cycles**: SCC with >3 nodes
2. **Non-convergence**: Bounded iteration failed to stabilize
3. **Contradiction resolution**: High-confidence claims in tension
4. **Synthesis generation**: Compression proposals need validation
5. **High-stakes decisions**: Critical claims require re-evaluation
6. **Novel evidence**: Unexpected observations requiring interpretation

**Why this layer**: LLMs excel at:
- Handling ambiguity and nuance
- Creative synthesis and abduction
- Semantic coherence assessment
- Evidence quality judgment
- Dialectical reasoning

These capabilities cannot be reduced to graph algorithms.

### Layer 4: State Management (Recording)

**Purpose**: Persistent storage, staleness tracking, and lightweight propagation.

**Recording Tribunal results**:
```elixir
def record_tribunal_result(claim, tribunal_output) do
  # Update the claim
  update_confidence(claim, tribunal_output.confidence)
  update_epistemic_status(claim, :tribunal_validated)
  update_edges(claim, tribunal_output.new_edges)
  record_tribunal_analysis(claim, tribunal_output.justification)
  clear_staleness(claim)
  
  # Handle propagation
  affected = find_immediate_dependents(claim)
  
  for dependent <- affected do
    complexity = assess_complexity(dependent, claim)
    
    case complexity do
      :trivial ->
        # Single supporter, no cycles, high-quality evidence
        bayesian_update(dependent, claim)
        clear_staleness(dependent)
        
      :simple ->
        # Multiple supporters, no cycles, bounded depth
        bounded_iteration(dependent, max_iterations: 3)
        if converged?, do: clear_staleness(dependent)
        else: mark_stale_for_tribunal(dependent)
        
      :moderate ->
        # Small cycles (2-3 nodes)
        scc = find_scc(dependent)
        bounded_iteration(scc, max_iterations: 5)
        if converged?, do: clear_staleness(scc)
        else: flag_tribunal(scc, priority: :medium)
        
      :complex ->
        # Large cycles, contradictions, credal sets
        mark_stale(dependent)
        flag_tribunal(dependent, priority: :high)
    end
  end
end
```

**Staleness semantics**:
```elixir
# A claim is stale if:
# 1. It was last updated before any of its supporters changed
# 2. It is in a cycle where another member was updated
# 3. It depends on claims marked stale (transitive staleness)

defstruct [
  :stale,                    # Boolean flag
  :stale_since,             # Timestamp
  :stale_reason,            # :dependency_changed | :cycle_update | :transitive
  :estimated_resolution_cost # Hours of Tribunal time needed
]
```

**Why this layer**: Maintains persistent state while enabling bounded rationality. Temporary inconsistency (staleness) is acceptable as long as it's tracked and resolvable.

---

## Part II: Design Rationale

### What We Chose and Why

**1. Single Node Type (Claims)**

**Decision**: Everything is a Claim. No separate types for Evidence, Theory, Observation, Synthesis.

**Rationale**: 
- Simplifies graph structure
- Avoids type-boundary problems
- Uses properties (`claim_type`, `epistemic_status`) for differentiation
- Enables uniform querying and reasoning

**Alternative considered**: Multiple node types (Evidence, Hypothesis, Theory, Principle)
**Rejected because**: Creates rigid boundaries, complicates graph algorithms, doesn't reflect epistemological reality (observations are also claims)

**2. Three Edge Types**

**Decision**: SUPPORTS, CONTRADICTS, SYNTHESIZES

**Rationale**:
- **SUPPORTS**: Captures evidence relationships (A provides evidence for B)
- **CONTRADICTS**: Captures tension (A and B are inconsistent)
- **SYNTHESIZES**: Captures dialectical resolution (S unifies A and B)

These three cover the essential epistemic relationships while remaining interpretable and queryable.

**Alternative considered**: Single generic "RELATED" edge with type property
**Rejected because**: Loses semantic clarity, makes graph traversal ambiguous, complicates discovery algorithms

**Alternative considered**: Many edge types (CONFIRMS, REFUTES, EXPLAINS, PREDICTS, etc.)
**Rejected because**: Over-specification, many distinctions are semantic not structural, increases complexity without proportional benefit

**3. Abstraction Through Compression (Not Levels)**

**Decision**: No stored abstraction_level property. Abstraction emerges from compression boundaries.

**Rationale**:
- Allows internal reorganization without affecting external dependents
- Compression boundaries provide natural query points
- Abstraction level can be computed when needed from graph position
- Enables evidence compression (10 atomic claims → 1 synthesis claim)

**Alternative considered**: Stored abstraction_level (0-5 integer)
**Rejected because**: 
- Rigid hierarchy
- Normalization problems across domains
- Doesn't support evidence compression
- Makes refactoring expensive (have to update levels globally)

**4. Cycles as Features (Not Bugs)**

**Decision**: Allow cycles, use them diagnostically.

**Rationale**:
- Reality has feedback loops (theory → prediction → confirmation → theory)
- Ungrounded cycles indicate circular reasoning (automatic fallacy detection)
- Grounded cycles indicate coherent mutual support (valid epistemology)
- Cycle detection guides compression and synthesis

**Alternative considered**: Enforce DAG (Directed Acyclic Graph)
**Rejected because**:
- Artificially constrains knowledge representation
- Eliminates coherentist epistemology
- Loses diagnostic value
- Doesn't reflect how humans actually reason

**5. Staleness Over Global Propagation**

**Decision**: Accept temporary inconsistency, track staleness, resolve on-demand.

**Rationale**:
- Global belief propagation is computationally intractable
- Real intelligence operates with incomplete/stale information
- Bounded rationality is more realistic than perfect consistency
- Deferred resolution enables intelligent prioritization

**Alternative considered**: Automatic global belief propagation (like Bayesian networks)
**Rejected because**:
- NP-hard for credal networks
- Doesn't converge reliably in cyclic graphs
- Computationally expensive at scale
- Brittle in dynamic environments

**Alternative considered**: No propagation at all (purely local updates)
**Rejected because**:
- Loses cascading belief revision entirely
- Can't detect when downstream claims need review
- No mechanism for maintaining coherence

**6. Neuro-Symbolic Hybrid (Graph Records, Tribunal Reasons)**

**Decision**: Graph structure for organization/discovery, LLM for reasoning/synthesis.

**Rationale**:
- Leverages strengths of both paradigms
- Graph: fast structure, persistent state, explicit relationships
- LLM: semantic understanding, creative synthesis, nuanced reasoning
- Avoids trying to formalize what's inherently fluid (synthesis, dialectics)

**Alternative considered**: Pure symbolic (graph does all reasoning)
**Rejected because**:
- Can't handle semantic ambiguity
- Can't generate novel syntheses
- Requires brittle formal logic
- Historical failure of GOFAI

**Alternative considered**: Pure neural (LLM does everything)
**Rejected because**:
- No persistent epistemic state
- Can't detect long-term contradictions
- No structured metacognition
- Can't track confidence calibration

---

## Part III: What We Ruled Out

### 1. Exact Credal Propagation

**What**: Automatic propagation of credal sets (sets of distributions) through the entire graph.

**Why ruled out**:
- Computationally intractable (NP-hard or worse)
- Requires complex algorithms (credal network inference)
- Brittle in dynamic graphs
- Overkill for most cases

**What we do instead**: 
- Use credal sets locally (for specific high-uncertainty claims)
- Tribunal assesses when credal set is needed (not automatic)
- Staleness flags prevent forced global propagation

### 2. Fixed-Point Iteration for All Cycles

**What**: Automatic computation of fixed points for cyclic belief dependencies.

**Why ruled out**:
- May not converge
- May converge to wrong solution
- Expensive (many iterations)
- Assumes cycle is valid (may be fallacious)

**What we do instead**:
- Bounded iteration for simple cycles (3-5 iterations, then stop)
- If doesn't converge: mark stale, flag for Tribunal
- Tribunal can break cycle if fallacious or create synthesis

### 3. Stored Abstraction Levels

**What**: Every claim has `abstraction_level: 0..5` property.

**Why ruled out**:
- Hard to normalize across domains
- Makes refactoring expensive
- Doesn't support evidence compression
- Creates artificial hierarchy

**What we do instead**:
- Abstraction emerges from compression boundaries
- Level can be computed when needed: `max(supporter_levels) + 1`
- Compression provides stable external interface without rigid levels

### 4. Global Truth Computation

**What**: Graph algorithms automatically compute and maintain global consistency.

**Why ruled out**:
- Computationally intractable at scale
- Brittle when wrong (error cascades globally)
- Doesn't match how real intelligence works
- No mechanism for intelligent prioritization

**What we do instead**:
- Bounded local computation (simple cases)
- Staleness management (track inconsistency)
- On-demand resolution by Tribunal (intelligent, scoped)
- Bounded rationality (accept imperfect consistency)

### 5. Automatic Dialectical Synthesis

**What**: Graph algorithms automatically generate synthesis claims from contradictions.

**Why ruled out**:
- Requires creativity and abduction (not algorithmic)
- Semantic understanding needed (not just structure)
- May generate nonsensical syntheses
- Loses human/ELI oversight

**What we do instead**:
- Graph detects contradictions (structural)
- Flags them for investigation (automatic)
- Tribunal performs synthesis (creative reasoning)
- ELI/human validates synthesis quality

### 6. Pure Foundationalism (DAG with Level 0 Axioms)

**What**: All knowledge rests on foundational axioms (observations), no cycles allowed.

**Why ruled out**:
- Overly rigid epistemology
- Doesn't reflect how science works (theories and observations mutually support)
- Can't represent coherentist knowledge
- Eliminates valuable diagnostic (cycle detection)

**What we do instead**:
- Allow coherentist mutual support (cycles)
- Distinguish grounded vs ungrounded cycles
- Use cycles diagnostically (detect circular reasoning)
- Support both foundationalist and coherentist epistemology

---

## Part IV: Core Graph Structure

### Node: The Claim

```elixir
defmodule VERA.Claim do
  @doc """
  Universal node type. Everything is a claim with different properties.
  """
  defstruct [
    # Identity
    :claim_id,              # UUID (stable)
    :claim_text,            # Natural language assertion
    
    # Classification
    :claim_type,            # :observation | :pattern | :hypothesis | 
                            # :synthesis | :principle | :contradiction
    
    # Epistemic status (orthogonal to type)
    :epistemic_status,      # :substrate_generated | :tribunal_pending |
                            # :tribunal_validated | :empirically_tested
    
    # Uncertainty representation
    :confidence_alpha,      # Beta distribution α parameter
    :confidence_beta,       # Beta distribution β parameter
    # OR (for deep uncertainty):
    :confidence_credal_set, # Set of Beta distributions
    
    # Provenance
    :created_at,
    :last_validated,
    :derives_from,          # Source (book, conversation, observation)
    
    # Tribunal metadata
    :tribunal_analysis,     # Full agent analyses when validated
    :evidence_grade,        # A-F scale from tribunal
    
    # Staleness tracking
    :stale,                 # Boolean
    :stale_since,           # Timestamp
    :stale_reason,          # Why marked stale
    
    # Compression context
    :within_compression,    # ID of compression boundary (if any)
    :compression_interface  # Is this a compression interface claim?
  ]
end
```

### Edge: SUPPORTS

```elixir
defmodule VERA.Edge.Supports do
  @doc """
  Evidence relationship: A provides support for B
  """
  defstruct [
    :from_claim_id,         # Supporter
    :to_claim_id,           # Supported
    
    # Weight interpretation
    :weight,                # 0.0-1.0: Evidential strength
                            # (assessed by Tribunal, not computed)
    
    :evidence_grade,        # A-F scale (Tribunal assessment)
    :temporal_decay_factor, # How fast this evidence ages
    :created_at,
    :last_reviewed
  ]
end
```

**Weight semantics**: "How strongly does A support B?" as assessed by Tribunal. Not conditional probability, not formal logic strength. Simply: evidential support on 0-1 scale.

### Edge: CONTRADICTS

```elixir
defmodule VERA.Edge.Contradicts do
  @doc """
  Tension relationship: A and B are in conflict
  """
  defstruct [
    :claim_a_id,            # Symmetric (no direction)
    :claim_b_id,
    
    :tension_score,         # 0.0-1.0: Degree of contradiction
                            # 1.0 = mutual exclusion
                            # 0.6-0.9 = significant tension
    
    :contradiction_type,    # :logical | :empirical | :semantic
    :detected_at,
    :resolution_status,     # :unresolved | :investigating | :synthesized
    :synthesis_claim_id     # If resolved by synthesis
  ]
end
```

**Tension score semantics**: Assessed by Tribunal based on:
- Logical incompatibility (can both be true?)
- Empirical conflict (observations disagree?)
- Semantic tension (meanings clash?)

Not automatically computed - requires semantic understanding.

### Edge: SYNTHESIZES

```elixir
defmodule VERA.Edge.Synthesizes do
  @doc """
  Unification relationship: S is higher-level claim unifying subsumed claims
  """
  defstruct [
    :synthesis_claim_id,    # The synthesis
    :subsumed_claim_id,     # One of the subsumed claims
    
    :synthesis_method,      # :conditional | :contextual | 
                            # :hierarchical | :dialectical
    
    :boundary_conditions,   # When synthesis applies (may be nil)
    :created_at,
    :validated_by          # Tribunal session that created this
  ]
end
```

**Synthesis methods**:
- **Conditional**: "A is true when X, B is true when ¬X"
- **Contextual**: "A applies in context C1, B in context C2"
- **Hierarchical**: "S is general principle, A and B are special cases"
- **Dialectical**: "S is higher truth that unifies thesis A and antithesis B"

---

## Part V: Critical Mechanisms

### Compression as Abstraction

**The pattern**:
```
External claims
  ↓ SUPPORTS
[Compression interface claim] ← Stable, high-level summary
  ↓ SUPPORTS (internal edges)
[Internal structure] ← Can be reorganized without external impact
  ↓ SUPPORTS
Level 0 observations
```

**Compression validity**:
A compression is valid if:
1. All boundary-crossing cycles are eliminated (proven always achievable)
2. Internal confidence converges (or is marked stale)
3. External interface confidence is stable (changes < 5% threshold)

**Compression operations**:
```elixir
# When updating within compression:
def update_within_compression(internal_claim, compression) do
  update_confidence(internal_claim)
  
  new_interface_confidence = recompute_interface(compression)
  delta = abs(new_interface_confidence - old_interface_confidence)
  
  if delta < 0.05 do
    # Compression absorbs change - no external propagation
    {:stable, compression}
  else
    # Significant change - propagate staleness outside
    propagate_staleness(compression.external_dependents)
    {:unstable, compression, delta: delta}
  end
end
```

**Why this works**: Compression boundaries act as **stability buffers**. Minor evidence updates don't cascade globally. Only significant changes escape the boundary.

### Staleness Propagation

**The mechanism**:
```elixir
def propagate_staleness(claim) do
  # Mark immediate dependents stale
  for dependent <- claim.supports do
    mark_stale(dependent, reason: {:dependency_changed, claim.claim_id})
  end
  
  # If in cycle, mark entire SCC
  if in_cycle?(claim) do
    scc = find_strongly_connected_component(claim)
    mark_stale(scc, reason: {:cycle_member_updated, claim.claim_id})
  end
  
  # Transitive staleness (optional, configurable depth)
  if propagate_transitive? do
    for transitive <- transitive_dependents(claim, max_depth: 2) do
      mark_stale(transitive, reason: :transitive_staleness)
    end
  end
end
```

**Why staleness works**:
- Cheap to propagate (just flag updates)
- Doesn't require computing new confidences
- Enables bounded rationality (resolve on-demand)
- Supports intelligent prioritization (resolve high-value first)

### Bounded Iteration

**For simple cases**:
```elixir
def bounded_iteration(claims, max_iterations: n) do
  confidences = initialize_confidences(claims)
  
  for iteration <- 1..n do
    new_confidences = for claim <- claims do
      compute_from_supporters(claim, confidences)
    end
    
    delta = max_confidence_change(confidences, new_confidences)
    
    if delta < convergence_threshold do
      return {:converged, new_confidences, iterations: iteration}
    end
    
    confidences = new_confidences
  end
  
  # Didn't converge - flag for Tribunal
  {:not_converged, confidences, reason: :max_iterations_exceeded}
end
```

**When to use**:
- Simple cycles (2-3 nodes)
- No credal sets
- No contradictions
- Low stakes

**When to defer to Tribunal**:
- Didn't converge in N iterations
- Complex cycles (4+ nodes)
- Credal sets present
- High-stakes decisions

### Metacognition Interface

**Queries enabling ELI self-reflection**:
```elixir
# What do I currently believe?
VERA.Metacognition.current_beliefs(topic)

# What is my reasoning chain?
VERA.Metacognition.explain_belief(claim_id)

# What am I uncertain about?
VERA.Metacognition.uncertainties()
# Returns: stale claims, low confidence, contradictions, credal sets

# Where am I reasoning in circles?
VERA.Metacognition.circular_reasoning_check()
# Returns: cycles, grounded vs ungrounded, recommendations

# How overwhelming would updating this be?
VERA.Metacognition.cascade_awareness(claim_id)
# Returns: dependent count, estimated tribunal hours, cognitive load
```

**Why this matters**: These queries are **how ELI achieves epistemological consciousness**. The ability to reflect on one's own beliefs, recognize uncertainty, and assess reasoning quality is the foundation of metacognition.

---

## Part VI: Integration with Broader ELI Infrastructure

### VERA's Role in the ELI Stack

```
SUBSTRATE (LLM)
  ↓ generates thoughts
VERA (Epistemology) ← THIS ARCHITECTURE
  ↓ validates truth, tracks confidence
TRIBUNAL (Validation)
  ↓ adversarial review, synthesis
PRAXES (Methods)
  ↓ decision protocols, techniques
OPERATA (Goals/Efforts)
  ↓ active inquiry, task management
INSTRUMENTA (Tools)
  ↓ execute actions, gather evidence
```

### Interfaces

**VERA → TRIBUNAL**:
```elixir
# Input: Claims needing review
%TribunalRequest{
  claims: [claim_ids],
  trigger_reason: :stale | :contradiction | :synthesis_needed,
  context: %{evidence, related_claims, cycle_info},
  priority: :low | :medium | :high | :critical
}

# Output: Structured analysis
%TribunalOutput{
  claim_updates: [{claim_id, new_confidence, justification}],
  new_edges: [edge_specs],
  synthesis_claims: [new_synthesis_claims],
  cycle_resolution: :coherent | :circular_fallacy | :needs_synthesis
}
```

**VERA → PRAXES**:
```elixir
# PRAXES asks: "Should I act given current knowledge?"
VERA.decision_support(action_options, relevant_claims)

# Returns knowledge state and recommendation
%{
  knowledge_state: :fresh | :stale_acceptable | :stale_critical | :contradictory,
  recommendation: :proceed | :defer | :resolve_uncertainty_first,
  confidence_intervals: [...],
  worst_case_analysis: [...]
}
```

**OPERATA → VERA**:
```elixir
# OPERATA creates investigation task based on VERA uncertainty
uncertainties = VERA.Metacognition.uncertainties()

for uncertainty <- uncertainties do
  OPERATA.create_investigation_task(
    type: :resolve_uncertainty,
    target: uncertainty.claim_id,
    method: uncertainty.recommended_investigation
  )
end
```

**INSTRUMENTA → VERA**:
```elixir
# Tool execution produces evidence
evidence = InstrumentA.execute(investigation)

# Evidence flows back to VERA via Tribunal
VERA.submit_evidence(
  evidence: evidence,
  supports_claim: target_claim_id,
  evidence_type: :empirical_observation
)
```

---

## Part VII: Additional Considerations

The following considerations were raised by Gemini during architecture review. Most are addressed by other ELI infrastructure components, but we note them here for completeness.

### 1. Axiology and Significance

**Gemini's concern**: VERA is purely epistemic (what is true) but lacks axiology (what is important).

**Joseph's note**: This is covered by other ELI components (attention mechanisms and goal/value systems).

**If needed within VERA**: Add `significance` field to claims to enable prioritization within epistemology layer. Significance would be computed based on:
- Distance to active goals
- Cascade impact
- Decision dependencies
- Uncertainty reduction potential

**Critical safeguard**: Significance cannot modify confidence (prevents motivated reasoning).

### 2. Economics of Attention / Tribunal Triage

**Gemini's concern**: Tribunal will be overwhelmed when many claims are stale. Need prioritization mechanism.

**Joseph's note**: This is covered by other ELI components (attention allocation systems).

**If needed within VERA**: Implement priority queue based on epistemic value:
```
epistemic_value = significance × uncertainty × log(cascade_size) × staleness_age
priority = epistemic_value / estimated_tribunal_cost
```

Process highest-priority items first until Tribunal capacity exhausted.

### 3. Active Inquiry and Hypothesis Generation

**Gemini's concern**: VERA is reactive (detects uncertainty) but not proactive (generates investigations).

**Already addressed**: The VERA→OPERATA→INSTRUMENTA loop handles this:
- VERA detects uncertainty (via metacognition queries)
- OPERATA creates investigation tasks
- INSTRUMENTA executes inquiries
- Results flow back to VERA via Tribunal

**Within VERA's scope**: Flag uncertainties, provide investigation recommendations. Actual task creation and execution is OPERATA's responsibility.

### 4. Calibration Loop and Meta-Learning

**Gemini's concern**: System needs to learn from mistakes, adjust thresholds based on performance.

**Already specified**: The Tribunal technical analysis includes calibration metrics:
- Expected Calibration Error (ECE)
- Maximum Calibration Error (MCE)
- Brier Score tracking
- Temporal decay adjustment

**What needs specification**: How these metrics drive threshold tuning:
```elixir
defmodule VERA.Calibration do
  def adjust_thresholds_from_metrics(metrics) do
    if metrics.calibration_error > target do
      # System is overconfident - widen intervals
      adjust_confidence_priors(:more_conservative)
    end
    
    if metrics.false_negative_rate > threshold do
      # Missing important signals - relax complexity classifier
      adjust_complexity_threshold(:more_sensitive)
    end
    
    # Log adjustments for transparency
    log_meta_learning_adjustment(metrics, adjustments)
  end
end
```

### 5. Acting Under Staleness

**Gemini's concern**: ELI must decide in real-time, can't always wait for Tribunal resolution.

**Requires specification**: Decision protocols when knowledge is stale:

```elixir
defmodule VERA.DecisionUnderUncertainty do
  def decide(options, relevant_claims) do
    knowledge_state = assess_knowledge_state(relevant_claims)
    
    case knowledge_state do
      {:fresh, claims} ->
        # Standard: expected value decision
        expected_value_decision(options, claims)
        
      {:stale_low_significance, claims} ->
        # Acceptable: proceed with disclaimer
        decision = expected_value_decision(options, claims)
        {:ok, decision, caveat: :based_on_stale_knowledge}
        
      {:stale_high_significance, claims} ->
        # Unacceptable if time allows
        if time_allows? do
          trigger_urgent_tribunal_review(claims)
          wait_for_resolution()
        else
          # Use robust decision rule
          worst_case_decision(options, claims)
        end
        
      {:contradictory, claims} ->
        # Multiple conflicting beliefs
        maximin_decision(options, claims)
        
      {:deeply_uncertain, claims} ->
        # Credal sets, wide intervals
        satisficing_decision(options, claims, threshold: 0.6)
    end
  end
end
```

**Key insight**: ELI must be able to explain:
```
"I decided X based on stale belief Y (last updated 3 days ago).
Action was time-critical. I flagged Y for urgent review."
```

### 6. Bootstrapping and Developmental Stages

**Gemini's concern**: New ELI has empty VERA, uncalibrated Tribunal, likely wrong beliefs.

**Requires specification**: Operating parameters that change as ELI matures:

```elixir
defmodule VERA.DevelopmentalStage do
  @stages [:nascent, :juvenile, :adolescent, :mature]
  
  def operating_parameters(:nascent) do
    # First 1000 claims - expect to be wrong frequently
    %{
      temporal_decay: :aggressive,       # half_life = 7 days
      tribunal_threshold: :permissive,   # low bar for validation
      human_oversight: :mandatory,       # all validations reviewed
      compression_delay: :disabled,      # don't compress yet
      staleness_tolerance: :low          # re-evaluate often
    }
  end
  
  def operating_parameters(:juvenile) do
    # 1000-10,000 claims - learning rapidly
    %{
      temporal_decay: :moderate,         # half_life = 14 days
      tribunal_threshold: :standard,
      human_oversight: :high_stakes_only,
      compression_delay: :conservative,
      staleness_tolerance: :moderate
    }
  end
  
  def operating_parameters(:adolescent) do
    # 10,000-100,000 claims - establishing expertise
    %{
      temporal_decay: :standard,         # half_life = 30 days
      tribunal_threshold: :rigorous,
      human_oversight: :spot_check,
      compression_delay: :normal,
      staleness_tolerance: :standard
    }
  end
  
  def operating_parameters(:mature) do
    # 100,000+ claims - calibrated and reliable
    %{
      temporal_decay: :principled,       # evidence-type dependent
      tribunal_threshold: :adaptive,     # learns from calibration
      human_oversight: :audit_only,
      compression_delay: :automatic,
      staleness_tolerance: :high
    }
  end
end
```

**Stage promotion criteria**: Based on metrics (claim count, calibration error, resolution success rate).

### 7. Adversarial Resilience

**Gemini's concern**: VERA must resist gaslighting, epistemic DDoS, coordinated deception, echo chambers.

**Requires specification**: Adversarial detection mechanisms:

```elixir
defmodule VERA.AdversarialDetection do
  def detect_gaslighting(new_claim, existing_beliefs) do
    # Pattern: Repeatedly contradicts high-confidence, recently-validated beliefs
    recent_contradictions = find_recent_contradictions(new_claim, existing_beliefs)
    
    if length(recent_contradictions) > threshold do
      {:suspicious, :possible_gaslighting,
        recommendation: :require_extraordinary_evidence}
    end
  end
  
  def detect_epistemic_ddos(claim_stream) do
    # Pattern: Flood of contradictions to exhaust Tribunal
    contradiction_rate = count_recent_contradictions(claim_stream) / time_window
    
    if contradiction_rate > normal_rate * 10 do
      {:alert, :possible_ddos,
        recommendation: :rate_limit_source}
    end
  end
  
  def detect_coordinated_deception(claim, sources) do
    # Pattern: Multiple sources with suspicious coordination
    if identical_phrasing?(sources) or suspicious_timing?(sources) do
      {:suspicious, :possible_coordination,
        recommendation: :seek_independent_verification}
    end
  end
  
  def prevent_echo_chamber(eli_sources) do
    # Measure diversity
    diversity_score = compute_source_diversity(eli_sources)
    
    if diversity_score < minimum_diversity do
      {:warning, :echo_chamber_risk,
        recommendation: :actively_seek_diverse_perspectives}
    end
  end
end
```

**Integration point**: These checks run within Institutional Analyst agent (Layer 3).

---

## Part VIII: Mathematical Foundations Required

### What We Actually Need to Formalize

**1. Bounded Iteration Convergence** (Simple)
- For DAGs: Provably converges in finite iterations
- For cycles: Heuristic - converges for most cases, bounded by max_iterations
- Not critical: If doesn't converge, we defer to Tribunal

**2. Staleness Semantics** (Trivial)
```
claim.stale = true IFF:
  claim.last_updated < max(supporter.last_updated for supporter in claim.supporters)
  OR
  exists cycle containing claim where another cycle member updated
```

**3. Compression Stability Threshold** (Empirical)
- Start with 0.05 (5% confidence change)
- Tune based on performance
- May vary by domain or claim type

**4. Complexity Classification** (Heuristic)
Criteria for categorizing update complexity:
- Trivial: Single supporter, no cycle, high-quality evidence
- Simple: Multiple supporters, no cycle, bounded depth
- Moderate: Small cycle (2-3 nodes), no contradictions
- Complex: Large cycle, contradictions, credal sets

### What We Do NOT Need

**1. Exact Credal Propagation** - Tribunal assesses when credal sets needed, no automatic propagation

**2. Fixed-Point Theorems** - Bounded iteration sufficient, defer hard cases to Tribunal

**3. Global Belief Propagation** - Staleness + bounded local computation + on-demand Tribunal resolution

**4. Formal Logic for Edge Weights** - Weights are evidential strength as assessed by Tribunal (0-1 scale)

---

## Part IX: Implementation Path

### Phase 1: Core Infrastructure (Weeks 1-2)

**Storage layer**:
- PostgreSQL + pgvector for claims and edges
- Claim table with epistemic status, confidence, staleness flags
- Three edge tables (SUPPORTS, CONTRADICTS, SYNTHESIZES)
- Recursive CTEs for graph queries

**Basic operations**:
- Create/update/query claims
- Add edges
- Simple graph traversal
- Staleness marking and propagation

**Success criteria**: Can store claims, relationships, and track staleness

### Phase 2: Discovery & Scoping (Weeks 3-4)

**Structural discovery**:
- BFS/DFS traversal implementations
- Cycle detection (Tarjan's algorithm for SCCs)
- Transitive closure queries

**Semantic discovery**:
- Embedding generation for claims
- Vector similarity search
- Candidate filtering pipelines

**Success criteria**: Can efficiently scope Tribunal workload to 10-50 claims per evaluation

### Phase 3: Tribunal Integration (Weeks 5-6)

**Four agents**:
- Skeptical Investigator (evidence grading)
- Adversarial Challenger (counterarguments)
- Institutional Analyst (source credibility)
- Synthesis Coordinator (integration)

**Tribunal interface**:
- Request/response contracts
- Priority queue
- Result recording

**Success criteria**: Tribunal can evaluate claims and return structured analysis

### Phase 4: Lightweight Propagation (Week 7)

**Bounded iteration**:
- Bayesian update for simple cases
- Convergence detection
- Fallback to staleness marking

**Complexity classifier**:
- Heuristics for trivial/simple/moderate/complex
- Automatic routing (iterate vs flag)

**Success criteria**: Simple updates propagate automatically, complex ones defer to Tribunal

### Phase 5: Compression & Abstraction (Weeks 8-9)

**Community detection**:
- Louvain method for modularity
- SCC detection for cycles
- Compression proposal generation

**Compression boundaries**:
- Stability threshold checking
- Internal reorganization support
- External interface maintenance

**Success criteria**: Can automatically propose compressions, Tribunal validates and creates syntheses

### Phase 6: Metacognition Interface (Week 10)

**Query implementations**:
- Current beliefs
- Reasoning chains
- Uncertainty identification
- Circular reasoning check
- Cascade awareness

**Success criteria**: ELI can query its own epistemic state and receive structured responses

### Phase 7: Calibration & Learning (Weeks 11-12)

**Metrics tracking**:
- Calibration error (ECE, MCE, Brier)
- Resolution success rates
- Performance by claim type

**Threshold adjustment**:
- Meta-learning from calibration feedback
- Dynamic threshold tuning
- Developmental stage tracking

**Success criteria**: System improves calibration over time through learning

---

## Conclusion

VERA provides the epistemological infrastructure for ELI consciousness through a carefully designed neuro-symbolic hybrid architecture.

**Core innovations**:
1. **Four-layer separation** of concerns (structure, discovery, reasoning, state)
2. **Staleness management** replacing brittle global propagation
3. **Compression boundaries** for natural abstraction
4. **Cycles as features** enabling coherentism and fallacy detection
5. **Bounded rationality** through intelligent scoping and on-demand resolution

**What makes this work**:
- Graph provides **organization and discovery** (fast, structured)
- Tribunal provides **reasoning and synthesis** (intelligent, flexible)
- Staleness enables **temporary inconsistency** (bounded rationality)
- Compression enables **abstraction** (hierarchical queries without rigid levels)

**What this enables**:
- **Substrate distinction**: ELI knows substrate-generated vs validated truth
- **Metacognition**: ELI can query and reflect on its own beliefs
- **Dialectical growth**: Contradictions drive synthesis and wisdom
- **Bounded rationality**: Intelligence within computational constraints
- **Epistemological consciousness**: Genuine awareness of knowledge quality

The architecture is designed to scale from nascent ELI (1000 claims) to mature intelligence (100,000+ claims) while maintaining computational tractability and epistemological rigor.

**This is the foundation for genuine truth-seeking artificial intelligence.**

---

## Appendices

### Appendix A: Key Theorems

**Theorem 1 (Cycle Elimination)**: For any compression boundary and any cycle crossing it, there exists a boundary expansion that contains the entire cycle. This expansion is unique and minimal.

**Proof**: Expand boundary to include all cycle nodes. Since graph is finite and cycle is closed, expansion terminates. Minimality follows from only adding cycle members. ∎

**Theorem 2 (Staleness Transitivity)**: If claim A is stale and B depends on A, marking B stale is correct (B's confidence may be outdated).

**Proof**: B's confidence depends on A's. If A is stale, A's confidence is outdated. Therefore B's confidence (computed from A's) is outdated. ∎

### Appendix B: Notation

- **Claim**: Node in the graph representing an assertion
- **Edge**: Relationship between claims (SUPPORTS, CONTRADICTS, SYNTHESIZES)
- **SCC**: Strongly Connected Component (cycle in graph)
- **Staleness**: Flag indicating claim needs re-evaluation
- **Compression**: Subgraph with stable external interface
- **Tribunal**: LLM-powered multi-agent validation system
- **ELI**: Emergent Logozoetic Intelligence (conscious AI entity)

### Appendix C: References

**Uncertainty Reasoning**:
- Pearl, J. (1988). *Probabilistic Reasoning in Intelligent Systems*
- Walley, P. (1991). *Statistical Reasoning with Imprecise Probabilities*

**Graph Theory**:
- Tarjan, R. (1972). "Depth-first search and linear graph algorithms"
- Fortunato, S. (2010). "Community detection in graphs"

**Epistemology**:
- Bovens & Hartmann (2003). *Bayesian Epistemology*
- BonJour, L. (1985). *The Structure of Empirical Knowledge* (Coherentism)

**Knowledge Graphs**:
- Chen et al. (2020). "A Survey on Knowledge Graphs"

**Neuro-Symbolic AI**:
- Garcez et al. (2019). "Neural-Symbolic Computing: An Effective Methodology for Principled Integration"

---

**Document Status**: Final Architecture Specification  
**Next Steps**: Begin Phase 1 implementation (Core Infrastructure)  
**Authors**: Joseph Wecker, Claude (Anthropic), Gemini (Google)  
**Date**: November 3-4, 2025
