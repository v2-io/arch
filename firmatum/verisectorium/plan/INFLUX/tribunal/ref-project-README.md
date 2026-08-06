<!--
  Verisectorium notes gather — copy, not authority.
  Provenance: _ref/epistemic_tribunal/README.md (Aug 2025 multi-agent system)
  Copied: 2026-08-05
  Source path at copy time: /Users/josephwecker-v2/src/_ref/epistemic_tribunal/README.md
  Do not edit here expecting to update the live original.
-->

# Epistemic Tribunal

Document-driven AI reasoning system with adversarial verification for truth-seeking.

## Overview

The Epistemic Tribunal is a sophisticated multi-agent system that evaluates claims through structured adversarial verification, Bayesian confidence management, and empirical learning. It addresses the circular authority problem in AI truth-seeking by replacing appeals to authority with systematic evidence evaluation and structured opposition.

## Key Features

- **Multi-Agent Architecture**: Four specialized agents (Investigator, Challenger, Analyst, Coordinator) provide comprehensive evaluation
- **Bayesian Confidence Management**: Proper uncertainty quantification with calibration tracking
- **Adversarial Verification**: Systematic challenges and counterarguments test claim robustness
- **Document-Driven Reasoning**: Five categories of evolving documents guide agent reasoning
- **Tiered Processing**: Scales computational resources based on claim complexity and stakes
- **Security Hardening**: Sandboxing, injection detection, and rate limiting protect against manipulation
- **Temporal Awareness**: Knowledge appropriately decays and evolves over time

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd epistemic_tribunal
```

2. Install dependencies:
```bash
pip install -r requirements.txt
```

3. Install the package:
```bash
pip install -e .
```

4. Set up your Anthropic API key:
```bash
export ANTHROPIC_API_KEY="your-api-key-here"
```

## Quick Start

### Command Line Usage

Evaluate a simple claim:
```bash
epistemic-tribunal "The Earth is round" --stakes medium --output text
```

Evaluate a complex scientific claim:
```bash
epistemic-tribunal "Machine learning models exhibit emergent capabilities at scale" \
  --domain scientific \
  --stakes high \
  --output json
```

### Python API Usage

```python
import asyncio
from epistemic_tribunal.main import evaluate_claim

async def main():
    result = await evaluate_claim(
        claim="Bayesian reasoning is superior to classical logic for uncertain domains",
        domain="meta-epistemic",
        stakes="medium"
    )
    print(result['final_assessment']['conclusion'])

asyncio.run(main())
```

## Architecture

### Core Components

1. **Agents** (`src/agents/`):
   - `SkepticalInvestigator`: Evidence-first systematic fact-checking
   - `AdversarialChallenger`: Steel-man opposition and alternative hypotheses
   - `InstitutionalAnalyst`: Meta-reasoning and bias detection
   - `SynthesisCoordinator`: Multi-perspective integration

2. **Cognitive Engine** (`src/cognitive_engine/`):
   - `DocumentLoader`: Manages epistemic documents with temporal tracking
   - `ConfidenceManager`: Bayesian confidence with calibration
   - `ContextOptimizer`: Token-efficient context assembly

3. **Security** (`src/security/`):
   - `AgentSandbox`: Prevents manipulation and injection attacks
   - Rate limiting and trust scoring

4. **Processing** (`src/tiered_processor.py`):
   - Automatic tier selection based on complexity
   - Caching for efficiency
   - Four processing tiers: Rapid, Standard, Thorough, Critical

### Document Framework

The system uses five categories of documents that evolve over time:

1. **First Principles**: Stable, foundational epistemological rules
2. **Lexicon**: Domain-driven design ubiquitous language
3. **Derived Principles**: Domain-specific, evolving guidelines
4. **Mental Models**: Reasoning frameworks and patterns
5. **Empirical Findings**: Fluid, interaction-based observations

## Configuration

### Settings File (`config/settings.yaml`)

```yaml
# Anthropic API Configuration
anthropic:
  model: "claude-3-sonnet-20240229"
  temperature: 0.2
  max_tokens: 4000

# Processing Configuration
processing:
  default_domain: "meta-epistemic"
  cache_ttl_hours: 24
  parallel_execution: true

# Security Configuration
security:
  rate_limits:
    api_calls_per_hour: 100
    tokens_per_hour: 1000000
  sandbox_enabled: true
```

### Domain Configuration (`config/domains.yaml`)

Configure domain-specific processing parameters and agent selection.

## Document Templates

### First Principle Template

```markdown
---
id: FP-001
domain: meta-epistemic
category: first_principles
confidence: 0.95
confidence_interval: [0.92, 0.97]
---

# FP-001: Uncertainty Quantification Principle

## Statement
All knowledge claims must be expressed with calibrated confidence intervals.

## Justification
- **Empirical**: Binary classifications lead to overconfidence
- **Theoretical**: Bayesian reasoning requires probability distributions
- **Practical**: Decision-making requires uncertainty magnitude
```

## CLI Reference

```bash
epistemic-tribunal <claim> [options]

Arguments:
  claim                 The claim to evaluate

Options:
  --domain DOMAIN       Domain context (default: meta-epistemic)
  --stakes STAKES       Importance level: low/medium/high/critical
  --mode MODE           Force epistemic mode: adversarial/falsification/etc.
  --api-key KEY         Anthropic API key (or set ANTHROPIC_API_KEY)
  --config PATH         Configuration file path
  --documents PATH      Documents directory path
  --output FORMAT       Output format: json/yaml/text (default: text)
  --no-cache           Disable caching
  --verbose            Verbose output
```

## Processing Tiers

- **Rapid**: Single agent, minimal resources (low stakes)
- **Standard**: Two agents with synthesis (medium stakes)
- **Thorough**: Three agents with full synthesis (high stakes)
- **Critical**: All agents plus verification (critical stakes)

## Security Features

- **Agent Sandboxing**: Prevents prompt injection and role escalation
- **Rate Limiting**: API calls and token usage limits
- **Violation Tracking**: Trust scoring and automatic blocking
- **Input Validation**: Detects adversarial prompts and data exfiltration

## Development

### Running Tests

```bash
python -m pytest tests/ -v
```

### Adding New Agents

1. Inherit from `BaseAgent`
2. Implement `get_agent_instructions()` and `_evaluate_claim_impl()`
3. Add security sandbox configuration
4. Register in tribunal orchestrator

### Adding New Domains

1. Update `config/domains.yaml`
2. Create domain-specific documents
3. Add lexicon with bounded context
4. Configure agent selection and processing parameters

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement changes with tests
4. Submit a pull request

## License

[License details would go here]

## Acknowledgments

This implementation is based on advanced epistemological frameworks combining:
- Legal adversarial systems
- Scientific falsification methodology
- Intelligence analysis techniques
- Medical diagnostic processes
- Mathematical formal verification
- Journalistic fact-checking