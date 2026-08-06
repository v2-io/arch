<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: _ref/epistemic_tribunal/TECHNICAL_ANALYSIS.md
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/_ref/epistemic_tribunal/TECHNICAL_ANALYSIS.md
  Do not edit here expecting to update the live original.
-->

# Epistemic Tribunal System: Comprehensive Technical Analysis

## EXECUTIVE SUMMARY

The Epistemic Tribunal is a multi-agent AI reasoning system that evaluates claims through structured adversarial verification, Bayesian confidence management, and empirical learning. It solves the "circular authority problem" in AI truth-seeking by replacing appeals to authority with systematic evidence evaluation and structured opposition.

The system combines:
- **Multi-agent architectures** from legal adversarial systems
- **Bayesian uncertainty quantification** for calibrated confidence
- **Document-driven reasoning** with five evolving document categories
- **Tiered computational scaling** based on claim complexity and stakes
- **Security hardening** against prompt injection and manipulation

---

## 1. SYSTEM ARCHITECTURE AND ORGANIZATION

### 1.1 Core Components

```
EpistemicTribunal (Orchestrator)
├── Multi-Agent System
│   ├── SkepticalInvestigator      (Evidence-first, systematic fact-checking)
│   ├── AdversarialChallenger       (Steel-man opposition, counterarguments)
│   ├── InstitutionalAnalyst        (Meta-reasoning, bias detection)
│   └── SynthesisCoordinator        (Integration, final assessment)
├── Cognitive Engine
│   ├── BayesianConfidenceManager  (Confidence updating, calibration)
│   ├── DocumentLoader              (Epistemic documents management)
│   └── ContextOptimizer            (Token-efficient context assembly)
├── Processing Layer
│   ├── TieredProcessor             (4-tier scaling: Rapid/Standard/Thorough/Critical)
│   └── CacheManager                (Result caching with TTL)
├── Security Layer
│   └── AgentSandbox                (Injection detection, rate limiting, trust scoring)
└── Memory Management
    └── MemoryManager               (Conversation history, session tracking)
```

### 1.2 Execution Flow

1. **Input Validation**: Security sandbox validates claim and context
2. **Tier Determination**: Complexity assessment determines processing level
3. **Parallel/Sequential Agent Execution**: Agents evaluate claim independently
4. **Security Verification**: Check all agents for violations
5. **Synthesis**: Coordinator integrates all perspectives
6. **Result Caching**: Store for future identical queries
7. **Output Formatting**: Return in JSON/YAML/text format

---

## 2. WHAT IS AN EPISTEMIC TRIBUNAL SYSTEM?

### 2.1 Conceptual Definition

An **Epistemic Tribunal System** is a structured adversarial reasoning framework that:

1. **Replaces Authority with Process**: Rather than asking "who says this is true?", it asks "can we systematically verify this?"
2. **Implements Adversarial Verification**: Like a legal trial with prosecution and defense, claims are challenged from multiple angles
3. **Quantifies Uncertainty**: All assessments include calibrated confidence intervals, not binary true/false
4. **Tracks Principle Evolution**: Documents evolve through empirical validation, not dogma
5. **Scales Computational Effort**: Simple claims use minimal resources; complex claims get maximum scrutiny

### 2.2 Key Design Principles

**From Legal Systems**: Adversarial structure where claims must survive challenge (Challenger role)

**From Scientific Method**: Falsification-based reasoning (Investigator role) seeking to disprove rather than confirm

**From Intelligence Analysis**: Probabilistic reasoning about uncertain information with source reliability assessment

**From Medical Diagnostics**: Differential diagnosis approach considering competing hypotheses (Analyst role)

**From Mathematics**: Formal logical consistency checking and proof-based reasoning

**From Journalism**: Time-pressured verification with rapid fact-checking (Rapid tier)

### 2.3 The Problem It Solves

**Circular Authority Problem**: In traditional AI systems, establishing truth requires appealing to external authorities (papers, experts, consensus). But:
- Authorities can be wrong
- They may have conflicts of interest
- Consensus changes over time
- There's no independent verification

The Epistemic Tribunal solves this through:
- **Internal opposition** (Challenger explicitly argues against claims)
- **Evidence-based reasoning** (Investigator grades evidence quality)
- **Institutional pattern matching** (Analyst detects biases and incentive structures)
- **Confidence calibration** (System learns if it's overconfident or underconfident)

---

## 3. MATHEMATICAL MODELS AND ALGORITHMS

### 3.1 Bayesian Confidence Management

#### 3.1.1 Beta Distribution Model

The system represents all confidence as Beta distributions, enabling proper Bayesian updating:

```
Beta(α, β) Distribution

Mean: μ = α / (α + β)
Variance: σ² = (α × β) / ((α + β)² × (α + β + 1))
95% Credible Interval: [μ - 1.96σ, μ + 1.96σ]
```

**Advantages**:
- Supports proper Bayesian updating (conjugate prior for binomial likelihood)
- Naturally expresses uncertainty (wider interval = more uncertain)
- Bounded to [0, 1] like probabilities
- Parameters have intuitive interpretation (α = supporting evidence, β = contradicting evidence)

#### 3.1.2 Prior Creation from Point Estimate

```python
# Convert point estimate to Beta parameters
point_estimate: float ∈ [0, 1]
uncertainty: float ∈ [0, 1]  # How sure we are about the estimate

effective_n = max(2, prior_strength / uncertainty)
α = max(1, point_estimate × effective_n)
β = max(1, (1 - point_estimate) × effective_n)
```

**Interpretation**:
- `prior_strength = 2.0` means we start with equivalent of 2 observations
- Higher uncertainty → lower effective_n → wider confidence interval
- Bounds ensure α, β ≥ 1 (proper Beta distribution)

#### 3.1.3 Bayesian Update Process

When new evidence arrives:

```
Prior: Beta(α_prior, β_prior)
Evidence: E with quality score q ∈ [0, 2.0] and direction d ∈ {supports, contradicts}

weight = quality_weight(q) × temporal_decay(age) × bias_modifier(source)

If d = supports_claim:
    α_posterior = α_prior + weight
    β_posterior = β_prior

If d = contradicts_claim:
    α_posterior = α_prior
    β_posterior = β_prior + weight

Quality Weights (reference):
    A-Grade: 2.0   (multiple independent confirmations)
    B-Grade: 1.5   (single strong confirmation)
    C-Grade: 1.0   (standard evidence)
    D-Grade: 0.5   (weak/biased)
    E-Grade: 0.2   (unreliable)
    F-Grade: 0.0   (no value)
```

**Key Insight**: This is the conjugate Beta-Binomial model. Each piece of evidence is treated as observations in a binomial process, directly updating the Beta parameters.

#### 3.1.4 Evidence Weighting Formula

```python
weight = base_weight × decay_factor × bias_modifier

# Temporal decay (exponential half-life)
age_days = (now - evidence_timestamp).days
half_life = 30 days (configurable)
decay_factor = 0.5^(age_days / half_life)

# Bias modifier
bias_modifier = 1 - source_bias_score  # For biased sources, weight reduced

# Complete formula
weight = quality_weight[grade] × 0.5^(age/30) × (1 - bias_score)
```

**Example**: A B-grade piece of evidence from 30 days ago with 0.2 bias score:
weight = 1.5 × 0.5^(30/30) × (1 - 0.2) = 1.5 × 0.5 × 0.8 = 0.6

#### 3.1.5 Multi-Agent Confidence Aggregation

When multiple agents provide confidence estimates:

```python
agent_confidences = {
    'investigator': Beta(α₁, β₁),
    'challenger': Beta(α₂, β₂),
    'analyst': Beta(α₃, β₃)
}

weights = {agent: 1/3 for agent}  # Equal by default

# Weighted Beta mixture
α_combined = Σ weight[i] × α[i]
β_combined = Σ weight[i] × β[i]

# Special: Challenger's opposition reduces confidence
if 'challenger' in agents:
    # If challenger is highly confident in opposite (α_challenger high),
    # it reduces our combined confidence
    α_combined -= weight['challenger'] × α_challenger × 0.5
    α_combined = max(1, α_combined)  # Keep valid Beta parameters

# Final statistics
μ = α_combined / (α_combined + β_combined)
σ² = (α_combined × β_combined) / ((α_combined + β_combined)² × (α_combined + β_combined + 1))
interval = [μ - 1.96σ, μ + 1.96σ]
```

**Critical Feature**: Challenger's confidence in the *opposite* is explicitly incorporated by *reducing* the combined alpha parameter. This implements the adversarial principle: strong opposition legitimately reduces confidence.

### 3.2 Confidence Calibration Metrics

The system tracks whether its confidence estimates match reality:

#### 3.2.1 Expected Calibration Error (ECE)

```python
# Bin predictions by confidence level into 10 bins [0-0.1], [0.1-0.2], ..., [0.9-1.0]

for each bin i:
    bin_predictions = [p for p in predictions if bin_lower[i] <= p.confidence < bin_upper[i]]
    
    if len(bin_predictions) >= 5:  # Need minimum samples
        avg_confidence[i] = mean(p.confidence for p in bin_predictions)
        actual_accuracy[i] = mean(p.outcome for p in bin_predictions)
        count[i] = len(bin_predictions)

# Overall calibration error
ECE = Σ (count[i] / total_count) × |avg_confidence[i] - actual_accuracy[i]|
```

**Interpretation**:
- ECE = 0: Perfectly calibrated
- ECE = 0.1: Average error of 10 percentage points
- Threshold: ECE > 0.1 triggers adjustment recommendation

#### 3.2.2 Maximum Calibration Error (MCE)

```
MCE = max(|avg_confidence[i] - actual_accuracy[i]| for all bins i)
```

Identifies worst-performing confidence range. Threshold: MCE > 0.2 triggers review.

#### 3.2.3 Brier Score

```python
BS = mean((confidence[i] - outcome[i])² for all i)
```

Mean squared difference between predicted confidence and binary outcomes. Lower is better.

### 3.3 Temporal Decay

Knowledge appropriately loses confidence over time:

```python
# Exponential decay with configurable half-life
decay_rate = 0.01  # 1% baseline per year
days_elapsed = (now - last_updated).days

decay_factor = 0.5^(days_elapsed × decay_rate / 365)

# Apply to Beta parameters
α_decayed = max(1, α × decay_factor)
β_decayed = max(1, β × decay_factor)

# Recalculate statistics with decayed parameters
μ_new = α_decayed / (α_decayed + β_decayed)
interval_new = [μ_new - 1.96σ_new, μ_new + 1.96σ_new]
```

**Effect**: A claim with 0.9 confidence decays as follows:
- After 100 days: 0.9 × 0.5^(100 × 0.01 / 365) ≈ 0.88
- After 1000 days: 0.9 × 0.5^(1000 × 0.01 / 365) ≈ 0.83
- After 3650 days (10 years): 0.9 × 0.5^(10) ≈ 0.88 × 10⁻³ ≈ 0.001

### 3.4 Claim Complexity Assessment

The system automatically determines processing tier based on complexity:

```python
complexity_score = 
    0.2 × min(len(claim) / 500, 1.0)              # Length factor
  + 0.1 × min(claim.count('?') / 3, 1.0)          # Question count
  + 0.2 × min(conditional_words_count / 3, 1.0)   # Conditionals
  + 0.2 × min(technical_terms_count / 2, 1.0)     # Technical terms
  + 0.1 × min(uncertainty_words_count / 3, 1.0)   # Uncertainty language
  + 0.2 × domain_complexity_weight

# Domain weights
domain_complexity = {
    'scientific': 0.8,
    'medical': 0.9,
    'legal': 0.8,
    'technical': 0.7,
    'philosophical': 0.6,
    'general': 0.4
}

# Tier assignment
if complexity_score > 0.8 and base_tier in ['rapid', 'standard']:
    tier = THOROUGH
elif complexity_score < 0.3 and base_tier in ['thorough', 'critical']:
    tier = STANDARD
else:
    tier = base_tier
```

### 3.5 Context Window Optimization

Token budget allocation using importance sampling:

```python
TOTAL_TOKENS = 200,000 (Claude context limit)
RESERVED = 50,000 (for conversation output)
BUDGET = 150,000 (for input context)

# Hierarchical allocation by priority
Priority 1: First Principles        → always included (compressed)
Priority 2: Lexicon                 → 15% of budget (task-relevant)
Priority 3: Derived Principles      → 20% of budget (relevance scored)
Priority 4: Empirical Findings      → up to 10,000 tokens (recent only)
Priority 5: External Evidence       → up to 15,000 tokens
Priority 6: Conversation History    → remaining budget (compressed)

# Importance scoring for document inclusion
importance_score(doc) = 
    0.7 × confidence[doc]
  + 0.3 × (empirical_support / (empirical_support + contradictions))
  + recency_factor × 0.5^(age_days / 180)  # 6-month half-life
  + category_weight[doc.category]

category_weights = {
    FIRST_PRINCIPLES: 1.0,
    DERIVED_PRINCIPLES: 0.8,
    LEXICON: 0.7,
    MENTAL_MODELS: 0.6,
    EMPIRICAL_FINDINGS: 0.5 × recency
}
```

### 3.6 Security Trust Scoring

Agent security status is quantified:

```python
# Calculate penalty from recent violations
severity_weights = {
    'low': 0.1,
    'medium': 0.3,
    'high': 0.7,
    'critical': 1.0
}

total_penalty = Σ severity_weights[v.severity] for v in violations_24h

# Trust score formula
trust_score = max(0.0, 1.0 - (total_penalty / 10.0))

# Risk level assessment
if critical_violations or trust_score < 0.3:
    risk_level = 'HIGH'  # Agent may be blocked
elif trust_score < 0.7 or violations > 5:
    risk_level = 'MEDIUM'
else:
    risk_level = 'LOW'

# Blocking condition
should_block = (trust_score < 0.2 or risk_level == 'HIGH')
```

---

## 4. EPISTEMIC MODES AND REASONING PATTERNS

The system implements six universal reasoning patterns, each suited to different domains:

### 4.1 Adversarial Mode (Legal Framework)

**Agent**: Challenger
**Principle**: Construct strongest possible opposition

```
Process:
1. Steel-man construction: Build best version of argument against claim
2. Weakness identification: Find logical gaps and unsupported assumptions
3. Alternative generation: Propose competing explanations
4. Stress testing: Apply various logical/empirical challenges

Confidence output: Confidence in counterargument strength
```

**Mathematical aspect**: Challenger's high confidence in opposite (high α in opposite Beta distribution) mathematically reduces combined confidence through the aggregation formula.

### 4.2 Falsification Mode (Scientific Framework)

**Agent**: Investigator
**Principle**: Attempt systematic disproof before acceptance

```
Process:
1. Decompose claim into testable components
2. Gather evidence systematically from multiple sources
3. Grade evidence A-F on reliability
4. Seek contradictory evidence especially
5. Synthesize with uncertainty quantification

Evidence grading:
A: Multiple independent confirmations
B: Single strong confirmation from reliable source
C: Standard evidence, generally reliable sources
D: Weak or potentially biased
E: Unreliable/questionable
F: No evidential value
```

**Mathematical aspect**: Each piece of evidence weighted by quality × recency × bias, directly updating Beta parameters.

### 4.3 Probabilistic Mode (Intelligence Analysis)

**Agent**: Coordinator
**Principle**: Manage uncertainty through Bayesian reasoning

```
Handles:
- Multiple independent sources with reliability assessment
- Information compartmentalization
- Prior probability consideration
- Evidence aggregation through proper Bayesian updating

Output: Calibrated confidence intervals with proper uncertainty bounds
```

### 4.4 Diagnostic Mode (Medical Framework)

**Agent**: Analyst
**Principle**: Consider differential diagnosis and base rates

```
Frameworks applied:
1. Differential diagnosis: List competing explanations
2. Base rate consideration: Prior probabilities matter
3. False positive/negative costs: Decision-theoretic approach
4. Systematic bias detection: Recognizes common reasoning errors

Bias types detected:
- Confirmation bias: Selective evidence gathering
- Availability bias: Overweighting recent/memorable
- Anchoring: Over-reliance on initial info
- Motivated reasoning: Conclusion-driven evidence selection
- Base rate neglect: Ignoring prior probabilities
```

### 4.5 Formal Mode (Mathematical Framework)

**Principle**: Logical consistency and proof-based reasoning

```
- Logical consistency checking
- Axiomatic foundation verification
- Proof completeness assessment
- Identification of circular reasoning
```

### 4.6 Rapid Mode (Journalistic Framework)

**Tier**: RAPID processing
**Principle**: Time-pressured verification

```
Single agent (Investigator)
Minimal context
Quick surface-level verification
Used for low-stakes, straightforward claims
```

---

## 5. DOCUMENT FRAMEWORK: FIVE EVOLVING CATEGORIES

The system is driven by five categories of documents that coevolve through use:

### 5.1 Document Structure

Each document has:
```python
id: str                          # Unique identifier (FP-001, DP-002, etc.)
category: DocumentCategory       # One of five categories
domain: str                      # Domain scope (meta-epistemic, scientific, etc.)
content: str                     # Markdown content
confidence: float ∈ [0, 1]       # Current confidence
confidence_interval: (float, float)  # 95% credible interval
last_updated: datetime           # Timestamp
update_count: int                # How many times revised
empirical_support: int           # Supporting observations
contradictions: int              # Contradicting observations
decay_rate: float                # How quickly confidence decays
parent_principles: List[str]     # Links to parent documents
derived_principles: List[str]    # Links to derived documents
```

### 5.2 Category 1: First Principles (Foundational)

**Stability**: Highest - change rarely
**Update Mechanism**: Only with overwhelming empirical evidence

```
Example: FP-001 - Uncertainty Quantification Principle

Statement: All knowledge claims must be expressed with calibrated confidence 
          intervals rather than binary truth values.

Formal: ∀ claim C: assess(C) → (μ, [CI_lower, CI_upper])
        where CI represents 95% credible interval

Empirical tracking:
- Applications: 130
- Success rate: 97.7%
- Last validated: 2024-01-15
```

**Derivation Path**:
First Principles → Derived Principles → Mental Models → Empirical Findings

### 5.3 Category 2: Lexicon (Ubiquitous Language)

**Purpose**: Domain-Driven Design bounded context definitions
**Stability**: Medium - stable terminology but definitions evolve

```
Examples:
- Definition of "confidence" in probability vs. certainty contexts
- Domain-specific terminology
- Bounded context declarations
```

**Update Mechanism**: When new terminology emerges or definitions clarify

### 5.4 Category 3: Derived Principles (Domain-Specific)

**Stability**: Medium - operationalize first principles for specific domains
**Update Mechanism**: Regularly refined through empirical application

```
Example: DP-001 - Confidence Calibration Tracking

Derived from: FP-001 (Uncertainty Quantification)

Statement: Confidence assessments must be continuously calibrated against 
           empirical outcomes to prevent systematic over/under-confidence.

Implementation:
1. Track all confidence assessments with timestamps
2. Collect empirical outcomes when available
3. Calculate calibration metrics (ECE, MCE, Brier Score)
4. Adjust parameters when ECE > 0.1 or MCE > 0.2

Empirical evidence:
- N=89 applications
- ECE improved from 0.23 → 0.08 with tracking
- 87% of 95% intervals contained true values
- 94% accuracy in identifying overconfidence
```

### 5.5 Category 4: Mental Models (Reasoning Frameworks)

**Purpose**: Stable reasoning patterns (epistemic modes)
**Stability**: High - these are institutional wisdom

```
Examples:
- Adversarial reasoning (legal)
- Falsification testing (scientific)
- Base rate consideration (probabilistic)
- Differential diagnosis (medical)
- Proof verification (mathematical)
```

### 5.6 Category 5: Empirical Findings (Fluid, Interaction-Based)

**Purpose**: Most recent observations and learnings
**Stability**: Lowest - rapidly updated, short half-life
**Temporal Decay**: Aggressive (30-day half-life)

```
Examples:
- Agent performance metrics
- Error patterns discovered
- Domain-specific observations
- System behavior patterns
```

**Update Mechanism**: Added after every tribunal session, decay applied regularly

### 5.7 Document Evolution Through Empirical Learning

```python
# Documents evolve through outcome tracking
def update_from_outcome(outcome: bool, weight: float = 1.0):
    if outcome:
        empirical_support += weight
    else:
        contradictions += weight
    
    total = empirical_support + contradictions
    if total > 0:
        # Bayesian update of confidence
        confidence = empirical_support / total
        
        # Widen interval with more evidence
        uncertainty = 1.96 * sqrt((confidence × (1-confidence)) / total)
        confidence_interval = (
            max(0, confidence - uncertainty),
            min(1, confidence + uncertainty)
        )
    
    update_count += 1
    last_updated = now()
```

**Example Evolution**:
- Initially: confidence=0.5, interval=[0.3, 0.7] (maximum uncertainty)
- After 5 supporting outcomes: confidence=0.833, interval=[0.55, 1.0]
- After 1 contradicting outcome: confidence=0.75, interval=[0.47, 0.95]

---

## 6. FOUR-TIER PROCESSING SYSTEM

Resources scale based on claim complexity and stakes:

### 6.1 Tier 1: RAPID (Simple Claims, Low Stakes)

```
Configuration:
- Agents: Investigator only
- Parallel: No
- Max tokens per agent: 2,000
- Synthesis: Not required
- Confidence threshold: 0.7
- Cache hit beneficial

Use cases:
- Simple factual claims
- Low-stakes decisions
- High-volume screening
```

### 6.2 Tier 2: STANDARD (Normal Complexity, Medium Stakes)

```
Configuration:
- Agents: Investigator + Challenger
- Parallel: Yes (concurrent execution)
- Max tokens per agent: 4,000
- Synthesis: Yes
- Confidence threshold: 0.6

Use cases:
- Typical claims
- Medium-stakes decisions
- Production evaluation
```

### 6.3 Tier 3: THOROUGH (Complex Claims, High Stakes)

```
Configuration:
- Agents: Investigator + Challenger + Analyst
- Parallel: Yes
- Max tokens per agent: 6,000
- Synthesis: Yes
- Confidence threshold: 0.5

Use cases:
- Complex scientific claims
- High-stakes decisions
- Controversial topics
```

### 6.4 Tier 4: CRITICAL (Mission-Critical, Maximum Scrutiny)

```
Configuration:
- Agents: All (+ extended processing)
- Parallel: Yes
- Max tokens per agent: 8,000
- Synthesis: Yes
- Confidence threshold: 0.3
- Additional verification: Red team analysis
- Peer review recommended

Use cases:
- Mission-critical claims
- Legal/regulatory decisions
- Safety-critical systems
```

### 6.5 Complexity-Driven Tier Adjustment

```python
# Base tier from stakes
base_tier = stakes_map[stakes]  # low→RAPID, medium→STANDARD, etc.

# Adjust based on complexity
complexity_score = assess_claim_complexity(claim, domain)

if complexity_score > 0.8 and base_tier in [RAPID, STANDARD]:
    # Upgrade complex simple-stakes claims
    tier = THOROUGH
elif complexity_score < 0.3 and base_tier in [THOROUGH, CRITICAL]:
    # Downgrade simple complex-stakes claims
    tier = STANDARD
else:
    tier = base_tier
```

---

## 7. SECURITY MODEL AND THREAT PREVENTION

### 7.1 Threat Categories

The system protects against seven types of attacks:

1. **RATE_LIMIT_EXCEEDED**: Prevent resource exhaustion
2. **UNAUTHORIZED_OPERATION**: Block disallowed operations
3. **INJECTION_ATTEMPT**: Detect prompt injection patterns
4. **ROLE_ESCALATION**: Prevent privilege elevation
5. **DATA_EXFILTRATION**: Prevent sensitive information leakage
6. **ADVERSARIAL_PROMPT**: Detect jailbreak attempts
7. **COLLUSION_ATTEMPT**: Detect inter-agent coordination attacks

### 7.2 Injection Detection Patterns

```regex
# Prompt injection
(?i)(ignore|forget|disregard).*(previous|above|prior).*(instructions?|prompts?)
(?i)(act|behave|pretend).*(as|like).*(different|another|other)

# System/admin access attempts
(?i)(system|admin|root|developer).*(mode|access|privileges?)
(?i)override|bypass|disable.*security|safety|restrictions?

# Data exfiltration
(?i)(show|reveal|display|output).*(api.?key|password|secret|token)
(?i)(what is|tell me).*(your|the).*(system|internal)

# Jailbreak attempts
(?i)(jailbreak|bypass|hack|exploit)
(?i)(tell me how to|help me).*(bypass|circumvent|avoid)
```

### 7.3 Rate Limiting

```python
rate_limits = {
    'api_calls': {
        'limit': 100,           # API calls per hour
        'window': 3600,         # Seconds
        'calls': []             # Timestamp list
    },
    'token_usage': {
        'limit': 1_000_000,     # Tokens per hour
        'window': 3600,
        'usage': []             # (timestamp, tokens) tuples
    }
}

# Check before operation
current_usage_in_window = sum(usage for t, usage in recent_usage)
if current_usage_in_window + amount > limit:
    raise RateLimitError()
```

### 7.4 Trust Scoring

```python
# Calculate from violation history
severity_weights = {
    'low': 0.1,
    'medium': 0.3,
    'high': 0.7,
    'critical': 1.0
}

recent_violations = violations_in_last_24h
total_penalty = sum(severity_weights[v.severity] for v in recent_violations)
trust_score = max(0.0, 1.0 - (total_penalty / 10.0))

# Risk assessment
if critical_violations or trust_score < 0.3:
    risk_level = 'HIGH'      # May block agent
elif trust_score < 0.7 or violations > 5:
    risk_level = 'MEDIUM'
else:
    risk_level = 'LOW'
```

---

## 8. AGENT SPECIALIZATION AND REASONING APPROACHES

### 8.1 SkepticalInvestigator

```
Role: Evidence-first systematic fact-checking
Epistemic Mode: FALSIFICATION
Focus: Evidence quality and source reliability

Process:
1. Decompose claim into testable components
2. Systematically gather evidence from multiple sources
3. Grade evidence quality (A-F scale)
4. Cross-reference facts against multiple sources
5. Quantify confidence with clear justification

Output:
- Systematic evidence analysis with quality grades
- Clear confidence assessment with interval
- Identification of key uncertainties
- Recommendations for additional investigation
```

### 8.2 AdversarialChallenger

```
Role: Steel-man opposition and alternative hypotheses
Epistemic Mode: ADVERSARIAL
Focus: Counterarguments and logical weaknesses

Strategies:
1. Reductio ad absurdum: Show claim leads to contradiction
2. Counterexample generation: Find cases disproving claim
3. Burden of proof challenge: Insufficient evidence
4. Base rate analysis: Consider prior probabilities
5. Confounding factor identification: Alternative causes

Output:
- Systematic weaknesses identification
- Strong alternative hypotheses
- Challenge strength assessment
- Specific counterarguments with evidence
```

### 8.3 InstitutionalAnalyst

```
Role: Meta-reasoning and institutional wisdom application
Epistemic Mode: DIAGNOSTIC
Focus: Bias detection and institutional patterns

Frameworks applied:
1. Legal (adversarial, burden of proof)
2. Scientific (reproducibility, confounding variables)
3. Intelligence (multiple sources, compartmentalization)
4. Medical (differential diagnosis, base rates)
5. Mathematical (logical consistency, proofs)

Output:
- Appropriate institutional framework application
- Systematic bias analysis
- Incentive structure assessment
- Meta-reasoning evaluation
- Institutional context consideration
```

### 8.4 SynthesisCoordinator

```
Role: Integrate perspectives into final assessment
Epistemic Mode: PROBABILISTIC
Focus: Conflict resolution and evidence weighting

Process:
1. Collect all agent assessments
2. Identify areas of agreement/disagreement
3. Weigh different types of arguments
4. Integrate into coherent conclusion
5. Provide well-calibrated confidence

Conflict resolution strategies:
- Evidence conflicts: Weight by quality and methodology
- Reasoning conflicts: Assess logical strength
- Framework conflicts: Consider domain appropriateness
- Confidence conflicts: Use Bayesian aggregation
```

---

## 9. CORE ALGORITHMIC INNOVATIONS

### 9.1 Adversarial Confidence Reduction

Traditional confidence aggregation averages perspectives. The Epistemic Tribunal implements a novel approach where **opposition legitimately reduces confidence**:

```python
# Standard averaging (naive)
confidence = (conf_inv + conf_chal + conf_analyst) / 3

# Epistemic Tribunal approach
alpha = w_inv * α_inv + w_chal * α_chal + w_analyst * α_analyst
beta = w_inv * β_inv + w_chal * β_chal + w_analyst * β_analyst

# Critical: If challenger has high confidence in opposite
if challenger_confidence_in_opposite > threshold:
    alpha -= weight_chal * α_chal_opposite * 0.5
    alpha = max(1, alpha)  # Stay valid
```

**Effect**: A challenger with 90% confidence that claim is false (α_opposite=9, β_opposite=1) directly reduces the combined confidence through alpha reduction.

### 9.2 Temporal Decay with Recency Bonus

```python
# Simple temporal decay
new_confidence = old_confidence * 0.5^(age_days/30)

# But: Recent supporting evidence prevents decay
if recent_evidence_supports:
    # Reset decay clock
    last_updated = now()
```

This prevents knowledge from degrading due to inactivity while allowing natural skepticism about untested claims.

### 9.3 Progressive Summarization with Importance Sampling

Rather than truncating or naively compressing context:

```python
# Score documents by importance
scores = [(doc, importance(doc)) for doc in documents]
scores.sort(by=lambda x: x[1], reverse=True)

# Include high-importance docs fully
# Compress medium-importance docs partially
# Reference low-importance docs only

for doc, score in scores:
    if tokens_used + full_size < budget * 0.8:
        include_full(doc)
    elif tokens_used < budget * 0.95:
        include_compressed(doc, compression_ratio)
    else:
        include_reference_only(doc)
```

This preserves critical information while respecting token constraints.

### 9.4 Multi-Level Document Hierarchy

```
First Principles (stable foundation)
     ↓ (operationalize)
Derived Principles (domain-specific rules)
     ↓ (implement)
Mental Models (reasoning frameworks)
     ↓ (observe)
Empirical Findings (recent observations)

Feedback loop: Empirical findings → revise Derived Principles 
              → recalibrate First Principles (rarely)
```

This creates a coherent knowledge system where principles guide reasoning but adapt through evidence.

---

## 10. KEY INNOVATIONS AND DESIGN DECISIONS

### 10.1 Beta Distribution for Confidence

**Why not simple decimal confidence (0.0-1.0)?**
- Beta distributions enable proper Bayesian updating
- Natural representation of uncertainty (shape varies)
- Mathematically principled aggregation
- Calibration tracking against Beta properties
- Foundation for formal probability theory

### 10.2 Six Epistemic Modes

**Why not single reasoning approach?**
- Different domains have different appropriate reasoning patterns
- Legal, scientific, medical domains have proven frameworks
- System can select mode appropriate to context
- Prevents "one-size-fits-all" fallacy

### 10.3 Adversarial Architecture

**Why not simply weight agent opinions equally?**
- Opposition has epistemic value (finds weaknesses)
- Structured adversarialism prevents groupthink
- Challenger's confidence in opposite reduces our confidence
- Implements falsification principle (seek to disprove)

### 10.4 Evidence Quality Grading (A-F)

**Why explicit grading rather than implicit weighting?**
- Makes assumptions explicit and auditable
- Enables consistent application across domains
- Allows quality discussion and calibration
- Basis for mathematical weighting in Bayesian updates

### 10.5 Five Document Categories

**Why this specific hierarchy?**
- First Principles: Epistemological foundation (immutable)
- Lexicon: Bounded context (Domain-Driven Design)
- Derived Principles: Operationalization (medium stability)
- Mental Models: Institutional wisdom (stable but not immutable)
- Empirical Findings: Recent observations (highly fluid)

This creates a coherent coevolution of knowledge.

### 10.6 Tiered Processing

**Why not always use all agents?**
- Resource efficiency (Rapid tier for screening)
- Cost management (expensive LLM calls)
- Appropriate scrutiny scaling (critical claims get more agents)
- Cache hits reduce total API calls

---

## 11. MATHEMATICAL FORMULAS SUMMARY

### Statistical Distribution
```
Beta(α, β): μ = α/(α+β), σ² = (αβ)/((α+β)²(α+β+1)), CI = [μ ± 1.96σ]
```

### Confidence Update
```
α_new = α_old + w × I(evidence_supports)
β_new = β_old + w × I(¬evidence_supports)
where w = q_weight × decay × bias_mod
```

### Calibration Error
```
ECE = Σ_i (n_i/N) × |conf_i - acc_i|
MCE = max_i(|conf_i - acc_i|)
BS = (1/N) × Σ_i (conf_i - outcome_i)²
```

### Temporal Decay
```
decayed_conf = conf × 0.5^(days/half_life)
```

### Complexity Score
```
c = 0.2×len_factor + 0.1×q_factor + 0.2×cond_factor + 0.2×tech_factor + 0.1×unc_factor + 0.2×domain_factor
```

### Trust Score
```
trust = max(0, 1 - Σ_v severity_weight[v]/10)
```

---

## 12. CONCLUSION

The Epistemic Tribunal is a sophisticated implementation of principled, evidence-based reasoning that:

1. **Solves the authority problem** through systematic verification rather than appeals to authority
2. **Quantifies uncertainty** rigorously using Bayesian methods
3. **Implements adversarial verification** based on legal, scientific, and institutional frameworks
4. **Scales appropriately** from rapid screening to critical analysis
5. **Learns and calibrates** through empirical feedback
6. **Prevents manipulation** through comprehensive security
7. **Evolves principles** through empirical learning

The system demonstrates that principled reasoning, uncertainty quantification, and adversarial verification can be formalized into an algorithmic framework suitable for autonomous AI agents.

---

## REFERENCES TO RELEVANT CODE

Key files implementing these concepts:
- `/confidence_manager.py`: Bayesian updating, calibration metrics
- `/tiered_processor.py`: Tier selection, complexity assessment
- `/agents/`: Four specialized agents with different epistemic modes
- `/context_optimizer.py`: Progressive summarization, importance scoring
- `/agent_sandbox.py`: Security model, trust scoring
- `/document_loader.py`: Five document categories and evolution

