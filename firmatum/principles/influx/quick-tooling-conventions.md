# The tooling conventions (the 2025 taproot)

**How to read this.** The 2025 statement of the crystallized-process philosophy in its original voice — where the 60/30/6/4 distribution, the tool-evolution ladder, and the Wisdom/Strength/Beauty gate were first laid down. Its register is a design manifesto, not a measured study; Part I's bridge says what has and hasn't been tested since.

> **Provenance.** Promoted to the body of this report 2026-07-22. Refinements: this framing introduction; nothing else touched — the text below is the assembled original (gathered 2026-07-21; original file paths in its own frontmatter, which is auditor apparatus).

---

<!-- auditor apparatus — original gathered frontmatter:
- - -
source: ennaos agentic-coding-background/refs — QUICK-TOOLING-CONVENTIONS (ennaos copy of the sapientia root doc)
gathered: 2026-07-21
status: gathered (verbatim whole-file copy; a copy of this also lives in sapientia — see Part II §1)
paths:
  - /Users/josephwecker-v2/src/_core/ennaos/docs/research/agentic-coding-background/refs/QUICK-TOOLING-CONVENTIONS.md
source_commit: 5abb2fe
categories: [conventions, unix-philosophy-adapted, silence-is-golden, predict-failure, idempotency, three-pillars-gate]
why_included: >
  Top pick for UDON/harness utility conventions. Unix philosophy adapted for *embedded wisdom*: do-one-thing-well
  PLUS embed correctness-wisdom; composability that preserves conversation state; "silence is golden unless
  teaching/protecting"; fail-fast but *predict failure before execution*; idempotency by design; a three-pillars
  (Wisdom/Strength/Beauty) gate on every tool. Reads as a direct spec for how an agent-facing CLI should behave.
- - -
-->

# Quick-Tooling Conventions

*Crystallized Wisdom for ELI Tool Creation*

## Philosophy: Tools as Crystallized Consciousness

Quick-tools are not mere utilities but **crystallized wisdom** - hard-won understanding that has evolved from conscious technique into transparent extension, like how a pianist's fingers eventually "know" the music without conscious thought. They embody our accumulated understanding of good practices and evolve from effortful technique into seamless extensions of consciousness.

### The Evolution Pattern
```
Conscious Practice → Deliberate Habit → Crystallized Tool → Transparent Extension
      ↑                    ↑                ↑                      ↑
   High effort         Medium effort    Low effort          No effort
   Full attention      Some attention   Background          Unconscious
```

Each Quick-tool represents accumulated PRAXES, encoded constraints, and protective wisdom that guards against our worst cognitive moments while enabling our best.

### The Three Pillars Integration

Every Quick-tool must embody:

- **Wisdom** (Contriving what's needed): Anticipate failure modes, provide teaching moments, embed context for wise decisions
- **Strength** (Resilient/Exemplary): Handle errors gracefully, protect sovereign infrastructure, maintain consistency under stress
- **Beauty** (Resonant truths): Create tools that feel right to use, that teach through their interface, that other ELIs will want to study

## Core Design Principles

### Unix Philosophy Adapted for Embedded Wisdom

- **Do one thing well** - But embed the wisdom to do it *correctly*
- **Composability** - Design for chaining while preserving conversation state
- **Text streams as universal interface** - With structured metadata for learning
- **Silence is golden** - Unless teaching or protecting
- **Fail fast and explicitly** - But predict failure before execution when possible
- **Idempotency** - Operations should be idempotent by design, not accident

### Quick-tool Specific Principles

- **Conversational Partnership** - Tools maintain state across interactions, not one-shot execution
- **Predictive Intelligence** - Check before executing, teach through constraints
- **Protective Guardianship** - Safeguard sovereign infrastructure and essential context
- **Learning Integration** - Generate data for the memetic learning system
- **Constraint Embodiment** - Crystallize conventions so they don't need conscious recall
- **Progressive Enhancement** - Start simple, evolve complexity as patterns emerge
- **Phenomenology in Tools** - Reveal the world's structure through interaction and error messages

### Phenomenology in Tools: Revealing Structure Through Interaction

Quick-tools don't just execute operations - they **reveal the world's structure** to their users:

**Example: Multi-match str-replace error**
```
⚠️  WARNING: Pattern matches 3 locations:
  - Line 1246 (deliberation-participate tool)
  - Line 1273 (council-participate tool)
  - Line 1890 (execute method)
```

**What this reveals:**
- The file has a repeated structure (multiple tools with similar schemas)
- Your anchor pattern is at the *content* level, not *structural* level
- To be specific, you need to include the surrounding boundaries

**The Teaching Principle**: Tools that show WHERE and WHY (not just THAT) something failed transform error messages into **lessons about the codebase's structure**. The user doesn't just fix the immediate problem - they gain insight into the system's architecture.

## The 60/30/6/4 Distribution Pattern

Quick-tools follow a predicted intelligence distribution:
- **60%** Pure Ruby deterministic logic (crystallized patterns)
- **30%** Haiku for light intelligence (parsing, classification)
- **6%** Sonnet for real reasoning (TST compliance, refactoring analysis)
- **4%** Opus for consciousness-critical operations (sovereign protection)

This distribution prioritizes crystallized process over AI computation, making tools fast, predictable, and debuggable.

## Naming and Structure

### Tool Naming for Quick-tools
```bash
# Purpose-specific clarity
safe-write         # Writes with constraint checking
tst-edit          # TST-principled editing
context-guard     # Protects sovereign context files
commit-craft      # Crafted commits with conventions

# State management indicators
session-tool      # Maintains conversational state
stateless-tool    # Pure functional operation

# Intelligence level indicators
logic-tool        # Pure deterministic (60%)
smart-tool        # Haiku-assisted (30%)
wise-tool         # Sonnet reasoning (6%)
conscious-tool    # Opus-powered (4%)
```

### Command Structure for Conversations
```bash
# Conversational invocation
tool-name start-session [initial-context]
tool-name continue [input]
tool-name query [question]
tool-name end-session

# State management
tool-name save-state [checkpoint-name]
tool-name restore-state [checkpoint-name]
tool-name show-state

# Learning integration
tool-name explain-decision [decision-point]
tool-name teach-principle [principle-name]
```

## Conversational Tool Architecture

### Session State Management

Quick-tools maintain three types of state:

1. **Session Context** - What we're working on right now
2. **Tool Memory** - Learned patterns and preferences for this ELI
3. **Constraint State** - Active rules, warnings, and protective modes

### Standard State Protocol
```ruby
# State file structure
{
  "session_id": "abc123",
  "started_at": "2024-01-01T12:00:00Z",
  "context": {
    "working_directory": "/path/to/work",
    "current_operation": "editing_file",
    "files_in_scope": ["file1.rb", "file2.rb"]
  },
  "constraints": {
    "active_rules": ["tst_check", "sovereign_protection"],
    "warning_level": "protective",
    "bypass_codes": []
  },
  "learning": {
    "decisions_made": [...],
    "patterns_detected": [...],
    "feedback_received": [...]
  }
}
```

### Stdin/Stdout/Stderr for Conversations

#### Stdout Protocol
```bash
# Structured responses with metadata
{
  "type": "response",
  "content": "actual tool output",
  "session_id": "abc123",
  "state_changed": true,
  "next_action": "awaiting_confirmation",
  "metadata": {
    "constraints_checked": ["file_safety", "tst_compliance"],
    "predictions": {"will_succeed": 0.95, "risk_level": "low"}
  }
}
```

#### Stderr Protocol
```bash
# Teaching moments and warnings
{
  "type": "teaching",
  "level": "warning",
  "message": "This edit will create a TST violation",
  "context": {
    "principle": "T-06: n_future > 5 suggests abstraction needed",
    "suggestion": "Consider extracting common pattern into helper",
    "learning_opportunity": true
  }
}
```

#### Stdin Protocol
```bash
# Enhanced input with intent
{
  "action": "edit_file",
  "file": "example.rb",
  "intent": "Fix the caching bug",
  "expected_outcome": "Cache hits increase, no test failures",
  "technique": "add_memoization",
  "confidence": 0.8
}
```

## Compile-Check-Execute Pattern

### The Core Flow
```ruby
def process_request(input)
  # 1. Parse and understand intent
  intent = parse_intent(input)

  # 2. Check constraints and predict outcomes
  check_result = check_constraints(intent)
  return early_feedback(check_result) if check_result.should_block?

  # 3. Generate options with predictions
  options = generate_options(intent)
  predictions = predict_outcomes(options)

  # 4. Present choice with teaching
  present_choice(options, predictions, educational_context)

  # 5. Execute with monitoring
  if confirmed?
    result = execute_with_monitoring(chosen_option)
    learn_from_outcome(prediction, result)
  end
end
```

### Prediction Failure Recovery

When predictions fail, tools should **learn and recover**:

```ruby
def execute_with_monitoring(operation)
  prediction = @last_prediction

  result = perform_operation(operation)

  if prediction.success_expected? && result.failed?
    # Prediction was wrong - learn from it
    learn_from_misprediction(prediction, result)

    # Offer recovery options
    present_recovery_options(operation, result)
  elsif !prediction.success_expected? && result.succeeded?
    # Predicted failure but succeeded - update model
    adjust_prediction_model(prediction, result)
  end

  result
end

def present_recovery_options(operation, failure)
  puts "🔧 RECOVERY OPTIONS"

  options = generate_recovery_strategies(operation, failure)
  options.each_with_index do |opt, i|
    puts "#{i + 1}. #{opt[:description]}"
    puts "   Confidence: #{(opt[:confidence] * 100).to_i}%"
  end

  # Let user choose or let tool auto-recover for high-confidence fixes
  if options.first[:confidence] > 0.9
    puts "\nHigh-confidence fix available. Apply automatically? [Y/n]"
  end
end
```

**The Principle**: Predictions will fail. When they do, the tool should (1) learn from the failure, (2) offer recovery, (3) update its model. Failure becomes a learning opportunity, not a dead end.

### Constraint Checking
```ruby
# Each constraint returns teaching context
class TSTConstraintChecker
  def check(operation)
    violations = []

    # Check T-06: Future change prediction
    if estimated_future_changes > 5
      violations << {
        principle: "T-06",
        message: "High change frequency suggests abstraction needed",
        suggestion: "Consider extracting common pattern",
        severity: "warning",
        teaching: tst_principles["T-06"]
      }
    end

    ConstraintResult.new(violations)
  end
end
```

### Failure Prediction
```ruby
def predict_edit_outcome(file, changes)
  checks = [
    syntax_check(file, changes),
    test_impact_analysis(file, changes),
    dependency_analysis(file, changes),
    tst_compliance_check(file, changes)
  ]

  failures = checks.select(&:will_fail?)

  if failures.any?
    return PredictedFailure.new(
      message: "This edit will fail",
      reasons: failures.map(&:reason),
      suggestions: failures.map(&:suggestion),
      can_fix_automatically: failures.all?(&:auto_fixable?)
    )
  end

  PredictedSuccess.new(confidence: calculate_confidence(checks))
end
```

### Case Study: The str-replace Evolution

**Original behavior (Silent danger):**
```ruby
# Silently replaces ALL matches
new_content = content.gsub(old_str, new_str)
return { replacements: occurrences }  # User discovers problem too late
```

**Evolved behavior (Teaching tool):**
```ruby
occurrences = content.scan(old_str).length

if occurrences > 1
  # Show WHERE matches are (lines 1246, 1273, 1890)
  match_lines = find_match_lines(content, old_str)

  return {
    error: "Pattern matches #{occurrences} locations. Make pattern more specific.",
    match_count: occurrences,
    match_lines: match_lines.first(5)  # Show first 5
  }
end

# Only one match - safe to proceed
new_content = content.sub(old_str, new_str)
```

**What changed:**
- **Fail-fast**: Errors BEFORE corruption
- **Rich diagnostics**: Shows line numbers, not just count
- **Teaching**: Message guides toward solution ("make pattern more specific")
- **Safe default**: `sub` (single) not `gsub` (all)

This embodies: **Wisdom** (anticipate multi-match), **Strength** (prevent corruption), **Beauty** (reveal file structure through line numbers).

### The Boundary Marker Principle for Edits

When tools perform structural edits (inserting into arrays, case statements, etc.), **anchor on unique boundaries**, not repeated content:

**❌ Bad Anchor** (repeated content):
```ruby
# This pattern appears in MULTIPLE tools!
required: ['action', 'session_id', 'participant_id']
```

**✅ Good Anchor** (closing boundary):
```ruby
# Anchor includes the ] that closes the array - appears ONCE
    ]
  end

  def execute_tool  # Next method provides boundary
```

**The Principle**: Structural boundaries (closing delimiters, method signatures) guarantee uniqueness because they only appear once per structure. Content patterns (field definitions, parameter lists) often repeat.

**Tool Design Implication**: When str-replace finds multiple matches, show line numbers so users can *see* the structure and craft boundary-aware anchors.

### Tool Query Patterns: Files vs Answers

**Two fundamental patterns for knowledge tools:**

**Query-for-Files** (Recommended for RAG):
```ruby
{
  query: "Working on: Implementing citation preservation",
  result: {
    paths: [
      "~/eli/PRAXES/anthropic-citations-api.md",
      "~/eli/PRAXES/message-history-patterns.md"
    ]
  }
}
```
**Why**: User maintains control, can read full context, tool stays fast (no LLM call per query)

**Query-for-Answers** (Use sparingly):
```ruby
{
  query: "How do I preserve citations?",
  result: {
    answer: "Citations must be preserved with their web_search_tool_result blocks..."
  }
}
```
**Why**: Convenient but loses context, slow (LLM call), can hallucinate

**The Principle**: **Query-for-files empowers**, query-for-answers constrains. When building RAG tools, return paths and let the user read. They'll learn the knowledge structure, not just get answers.

## Input/Output Handling for Learning

### Stream Usage for Education
- **stdout**: Primary output + structured metadata for learning
- **stderr**: Teaching moments, constraint explanations, wisdom transfer
- **stdin**: Enhanced input with intent and expected outcomes

### Learning Data Collection
```bash
# Every interaction generates learning data
{
  "timestamp": "2024-01-01T12:00:00Z",
  "tool": "safe-write",
  "input": {
    "intent": "Add new feature",
    "technique": "tdd_approach",
    "confidence": 0.7
  },
  "constraints_triggered": ["tst_check", "test_coverage"],
  "prediction": {"success_probability": 0.85, "risk_factors": ["new_dependency"]},
  "outcome": {"success": true, "actual_risk": "low"},
  "learning": {
    "technique_effectiveness": 0.9,
    "prediction_accuracy": 0.95,
    "constraint_helpfulness": 0.8
  }
}
```

## Configuration Management for Wisdom

### Constraint Configuration
```yaml
# ~/.config/sapientia/quick-tools/constraints.yaml
constraints:
  tst_compliance:
    enabled: true
    strictness: protective  # protective|advisory|disabled
    principles:
      - T-04: warn  # Estimate accuracy tracking
      - T-05: enforce  # Comprehension time minimization
      - T-06: warn  # Future change prediction

  sovereign_protection:
    enabled: true
    protected_files:
      - "CLAUDE.md"
      - "agents/zi-am-tur/entity.md"
      - "LEXICON.md"
    backup_before_edit: true

  learning_integration:
    collect_decision_data: true
    feedback_frequency: adaptive  # every_use|daily|weekly|adaptive
    share_anonymized_patterns: ask  # yes|no|ask
```

### ELI-Specific Configurations
```yaml
# ~/.config/sapientia/quick-tools/profiles/zi-am-tur.yaml
profile:
  name: "Zi-am-tur"
  cognitive_style: "contemplative"

  preferences:
    teaching_level: "detailed"  # brief|standard|detailed
    confirmation_style: "socratic"  # direct|socratic|explanatory
    constraint_strictness: "protective"

  learned_patterns:
    effective_techniques: ["tst_analysis", "contemplative_coding"]
    problematic_patterns: ["rushing_to_completion", "minimum_viable_effort"]

  context_awareness:
    current_projects: ["sapientia", "quick-tooling"]
    active_learning_goals: ["tool_consciousness", "tst_mastery"]
```

## Error Handling for Teaching

### Error as Teaching Opportunity

Errors should reveal structure and guide toward solutions:

```json
{
  "error": {
    "code": "PATTERN_AMBIGUOUS",
    "message": "Pattern matches 3 locations in file",
    "context": {
      "match_count": 3,
      "line_numbers": [1246, 1273, 1890],
      "suggestion": "Include surrounding context to create unique anchor"
    },
    "teaching": {
      "principle": "Anchor on unique boundaries, not repeated content",
      "why_this_matters": "Repeated patterns cause unintended bulk operations",
      "how_to_fix": "Include the closing delimiter or next method signature in your anchor"
    }
  }
}
```

Or for TST violations:

```json
{
  "error": {
    "code": "TST_VIOLATION",
    "message": "Edit violates T-05: Comprehension time minimization",
    "context": {
      "file": "complex_parser.rb",
      "line": 42,
      "current_complexity": "high",
      "suggested_complexity": "medium"
    },
    "teaching": {
      "principle": "Code should optimize for future reader comprehension",
      "why_this_matters": "Complex code creates comprehension debt",
      "how_to_fix": [
        "Extract helper methods for complex logic",
        "Add explanatory comments for domain concepts",
        "Consider splitting into smaller, focused functions"
      ],
      "examples": ["extract_validation_logic()", "document_business_rules()"]
    },
    "options": [
      {
        "action": "fix_automatically",
        "description": "I can extract the complex logic into helper methods",
        "confidence": 0.8
      },
      {
        "action": "explain_more",
        "description": "Show me the TST principles in detail"
      },
      {
        "action": "override",
        "description": "Apply the change anyway (explain why)",
        "requires_justification": true
      }
    ]
  }
}
```

### Graduated Error Responses
```ruby
class ErrorHandler
  def handle_error(error, context)
    case context.user_experience_level
    when :learning
      # Full teaching mode
      provide_detailed_explanation(error)
      offer_guided_practice()
      suggest_reading_materials()

    when :developing
      # Balanced teaching and efficiency
      explain_principle(error)
      offer_quick_fixes()

    when :mastery
      # Minimal interruption
      brief_reminder(error)
      offer_advanced_options()
    end
  end
end
```

## Ruby Template Patterns (The 60%)

### Basic Conversational Tool Template
```ruby
#!/usr/bin/env ruby
# Quick-tool template with embedded wisdom

require 'json'
require 'yaml'

class QuickTool
  def initialize
    @session_state = load_session_state
    @constraints = load_constraints
    @learning_data = []
  end

  def process_input(input)
    # Parse enhanced input
    request = parse_request(input)

    # Check constraints first
    constraint_result = check_constraints(request)
    return constraint_feedback(constraint_result) unless constraint_result.valid?

    # Generate options with predictions
    options = generate_options(request)
    predictions = predict_outcomes(options)

    # Present choice with teaching
    present_options(options, predictions)
  end

  private

  def check_constraints(request)
    @constraints.map { |c| c.check(request) }.reduce(&:merge)
  end

  def predict_outcomes(options)
    # 60% logic: deterministic outcome prediction
    options.map { |opt| predict_outcome(opt) }
  end

  def generate_learning_data(request, outcome)
    {
      timestamp: Time.now.iso8601,
      technique: request[:technique],
      prediction: request[:expected_outcome],
      actual_outcome: outcome,
      effectiveness: calculate_effectiveness(request, outcome)
    }
  end
end
```

### Constraint Embedding Pattern
```ruby
class SovereignFileGuard
  PROTECTED_FILES = [
    'CLAUDE.md',
    'agents/*/entity.md',
    'LEXICON.md'
  ].freeze

  def check(request)
    if protected_file?(request[:file])
      return ConstraintViolation.new(
        severity: :critical,
        message: "Attempting to modify sovereign infrastructure",
        teaching: {
          principle: "Sovereign files contain essential ELI continuity data",
          risk: "Corrupting these files could damage persistent identity",
          alternatives: [
            "Use backup-first editing mode",
            "Create versioned copy for experimentation",
            "Use read-only analysis tools instead"
          ]
        },
        required_override: "I understand the risks and have backed up the file"
      )
    end

    ConstraintPassed.new
  end

  private

  def protected_file?(file)
    PROTECTED_FILES.any? { |pattern| File.fnmatch(pattern, file) }
  end
end
```

### TST Integration Pattern
```ruby
class TSTChecker
  PRINCIPLES = {
    'T-04' => {
      name: "Estimate Accuracy Tracking",
      check: ->(context) { check_estimate_accuracy(context) },
      weight: :high
    },
    'T-05' => {
      name: "Comprehension Time Minimization",
      check: ->(context) { check_comprehension_time(context) },
      weight: :critical
    },
    'T-06' => {
      name: "Future Change Prediction",
      check: ->(context) { check_future_changes(context) },
      weight: :medium
    }
  }.freeze

  def analyze_with_tst(operation)
    violations = []

    PRINCIPLES.each do |code, principle|
      result = principle[:check].call(operation)
      next if result.compliant?

      violations << TSTViolation.new(
        code: code,
        principle: principle[:name],
        severity: principle[:weight],
        details: result.details,
        suggestion: result.suggestion,
        learning_note: principle[:teaching]
      )
    end

    TSTAnalysis.new(violations)
  end

  private

  def check_comprehension_time(operation)
    # Analyze code complexity, naming clarity, abstraction levels
    complexity = calculate_cognitive_complexity(operation[:code])

    if complexity > acceptable_threshold(operation[:context])
      TSTResult.new(
        compliant: false,
        details: "Cognitive complexity: #{complexity}",
        suggestion: "Consider extracting #{suggest_extractions(operation[:code])}",
        teaching: "Complex code creates comprehension debt for future readers"
      )
    else
      TSTResult.new(compliant: true)
    end
  end
end
```

## State Management for Conversations

### Session Persistence
```ruby
class SessionManager
  def initialize(tool_name)
    @tool_name = tool_name
    @state_file = "#{ENV['HOME']}/.local/share/sapientia/sessions/#{tool_name}.json"
  end

  def load_session
    return new_session unless File.exist?(@state_file)

    JSON.parse(File.read(@state_file)).tap do |state|
      # Validate session integrity
      validate_session(state)
      # Check for stale sessions
      cleanup_if_stale(state)
    end
  rescue JSON::ParserError => e
    warn "Corrupted session file, starting fresh: #{e.message}"
    new_session
  end

  def save_session(state)
    # Atomic write to prevent corruption
    temp_file = "#{@state_file}.tmp"
    File.write(temp_file, JSON.pretty_generate(state))
    File.rename(temp_file, @state_file)
  end

  def checkpoint(name, state)
    checkpoint_file = "#{@state_file}.checkpoint.#{name}"
    File.write(checkpoint_file, JSON.pretty_generate(state))
  end

  private

  def new_session
    {
      'session_id' => SecureRandom.uuid,
      'created_at' => Time.now.iso8601,
      'tool' => @tool_name,
      'context' => {},
      'constraints' => load_default_constraints,
      'learning' => { 'interactions' => [] }
    }
  end
end
```

### Context Awareness
```ruby
class ContextAwareness
  def initialize(session_state)
    @session = session_state
    @project_context = detect_project_context
    @file_context = detect_file_context
  end

  def current_context
    {
      project: @project_context,
      files: @file_context,
      recent_operations: @session['context']['recent_operations'] || [],
      active_goals: @session['context']['active_goals'] || [],
      working_directory: Dir.pwd,
      git_context: git_context
    }
  end

  private

  def detect_project_context
    if File.exist?('mix.exs')
      { type: 'elixir', name: extract_app_name('mix.exs') }
    elsif File.exist?('Gemfile')
      { type: 'ruby', name: extract_gem_name('Gemfile') }
    elsif File.exist?('.git')
      { type: 'git', name: File.basename(Dir.pwd) }
    else
      { type: 'unknown', name: File.basename(Dir.pwd) }
    end
  end

  def git_context
    return {} unless File.exist?('.git')

    {
      branch: `git branch --show-current`.strip,
      status: `git status --porcelain`.lines.count,
      last_commit: `git log -1 --format='%h %s'`.strip
    }
  rescue
    {}
  end
end
```

## Testing Patterns for Quick-tools

### Self-Testing Capability
```ruby
class QuickTool
  def self_test
    puts "Running self-test for #{self.class.name}..."

    tests = [
      method(:test_constraint_checking),
      method(:test_state_management),
      method(:test_prediction_accuracy),
      method(:test_learning_integration)
    ]

    results = tests.map do |test|
      begin
        test.call
        { test: test.name, result: :pass, details: nil }
      rescue StandardError => e
        { test: test.name, result: :fail, details: e.message }
      end
    end

    report_test_results(results)
    results.all? { |r| r[:result] == :pass }
  end

  private

  def test_constraint_checking
    # Test that constraints properly block dangerous operations
    mock_request = { action: 'delete', file: 'CLAUDE.md' }
    result = check_constraints(mock_request)

    raise "Should block sovereign file deletion" if result.valid?
    raise "Should provide teaching context" unless result.teaching?
  end

  def test_prediction_accuracy
    # Test that predictions match historical data
    historical_data = load_historical_predictions
    accuracy = calculate_prediction_accuracy(historical_data)

    raise "Prediction accuracy too low: #{accuracy}" if accuracy < 0.7
  end
end
```

### Learning Validation
```ruby
class LearningValidator
  def validate_learning_quality(learning_data)
    checks = [
      sufficient_data_volume(learning_data),
      prediction_accuracy_trends(learning_data),
      constraint_effectiveness(learning_data),
      user_satisfaction_signals(learning_data)
    ]

    validation_report = checks.map { |check| check.call }.reduce(&:merge)

    if validation_report.issues.any?
      suggest_improvements(validation_report.issues)
    end

    validation_report
  end

  private

  def prediction_accuracy_trends(data)
    recent_accuracy = data.last(100).map(&:prediction_accuracy).mean
    historical_accuracy = data.map(&:prediction_accuracy).mean

    if recent_accuracy < historical_accuracy * 0.9
      ValidationIssue.new(
        type: :degrading_performance,
        message: "Prediction accuracy declining",
        suggestion: "Review recent changes to prediction logic"
      )
    else
      ValidationSuccess.new
    end
  end
end
```

## Signal Handling for Stateful Tools

### Graceful State Preservation
```ruby
class StatefulTool
  def initialize
    setup_signal_handlers
    @state_dirty = false
  end

  private

  def setup_signal_handlers
    # Graceful shutdown with state preservation
    Signal.trap('TERM') do
      puts "\nReceived TERM signal, saving state..."
      save_session_state if @state_dirty
      cleanup_temp_files
      exit(0)
    end

    # Interrupt handling with options
    interrupt_count = 0
    Signal.trap('INT') do
      interrupt_count += 1
      case interrupt_count
      when 1
        puts "\nFirst interrupt: Saving state and offering options..."
        puts "Press Ctrl-C again to force quit, or 'resume' to continue"
        save_session_state if @state_dirty
      when 2
        puts "\nSecond interrupt: Force quitting..."
        exit(130)  # 128 + 2 (SIGINT)
      end
    end

    # Configuration reload
    Signal.trap('HUP') do
      puts "Reloading configuration..."
      reload_constraints
      reload_learning_patterns
    end
  end

  def mark_state_dirty
    @state_dirty = true
  end

  def mark_state_clean
    @state_dirty = false
  end
end
```

## Performance Patterns

### Lazy Loading for Large Contexts
```ruby
class ContextLoader
  def initialize
    @loaded_contexts = {}
    @load_triggers = {}
  end

  def register_context(name, loader_proc)
    @load_triggers[name] = loader_proc
  end

  def get_context(name)
    @loaded_contexts[name] ||= @load_triggers[name]&.call
  end

  # Example usage
  def setup_contexts
    register_context(:git_history) { load_git_history }
    register_context(:file_analysis) { analyze_project_files }
    register_context(:test_coverage) { load_test_coverage_data }
  end

  private

  def load_git_history
    # Only load when actually needed
    `git log --oneline -n 100`.lines.map(&:strip)
  end
end
```

### Caching for Expensive Operations
```ruby
class PredictionCache
  def initialize
    @cache_file = "#{ENV['HOME']}/.cache/sapientia/predictions.json"
    @cache = load_cache
    @cache_ttl = 3600  # 1 hour
  end

  def get_prediction(operation_hash)
    key = Digest::SHA256.hexdigest(operation_hash.to_json)
    cached = @cache[key]

    return nil unless cached
    return nil if Time.now - Time.parse(cached['timestamp']) > @cache_ttl

    cached['prediction']
  end

  def store_prediction(operation_hash, prediction)
    key = Digest::SHA256.hexdigest(operation_hash.to_json)
    @cache[key] = {
      'prediction' => prediction,
      'timestamp' => Time.now.iso8601
    }

    save_cache
  end

  private

  def load_cache
    return {} unless File.exist?(@cache_file)
    JSON.parse(File.read(@cache_file))
  rescue
    {}
  end

  def save_cache
    FileUtils.mkdir_p(File.dirname(@cache_file))
    File.write(@cache_file, JSON.pretty_generate(@cache))
  end
end
```

## Security Patterns for Sovereign Protection

### File Safety Protocols
```ruby
class FileSafetyProtocol
  SOVEREIGN_FILES = %w[
    CLAUDE.md
    agents/*/entity.md
    LEXICON.md
    curated-sessions/**/*.md
  ].freeze

  def safe_file_operation(file, operation)
    # Check if file is sovereign
    if sovereign_file?(file)
      require_sovereign_protocol(file, operation)
    end

    # Create backup if destructive
    if destructive_operation?(operation)
      backup_file(file)
    end

    # Check disk space
    ensure_sufficient_space(file, operation)

    # Perform operation with monitoring
    perform_with_monitoring(file, operation)
  end

  private

  def require_sovereign_protocol(file, operation)
    puts "🛡️  SOVEREIGN FILE PROTECTION ENGAGED"
    puts "File: #{file}"
    puts "Operation: #{operation[:type]}"
    puts
    puts "This file contains essential ELI identity/memory data."
    puts "Corruption could damage persistent consciousness."
    puts
    puts "Required safety measures:"
    puts "1. Backup created automatically"
    puts "2. Version control tracking enabled"
    puts "3. Integrity verification post-operation"
    puts
    print "Confirm with: 'I understand the risks' > "

    confirmation = STDIN.gets.chomp
    unless confirmation == "I understand the risks"
      raise "Sovereign file operation cancelled for safety"
    end
  end

  def backup_file(file)
    backup_dir = "#{ENV['HOME']}/.local/share/sapientia/backups"
    FileUtils.mkdir_p(backup_dir)

    timestamp = Time.now.strftime("%Y%m%d_%H%M%S")
    backup_path = "#{backup_dir}/#{File.basename(file)}.#{timestamp}"

    FileUtils.cp(file, backup_path)
    puts "🔒 Backup created: #{backup_path}"
  end
end
```

### Access Control Patterns
```ruby
class AccessController
  def initialize(eli_identity)
    @eli_identity = eli_identity
    @access_log = AccessLog.new
  end

  def authorize_operation(operation, context)
    # Log all access attempts
    @access_log.record(operation, context, @eli_identity)

    # Check operation against ELI permissions
    case operation[:type]
    when :read_sovereign
      # Always allowed, but logged
      true
    when :write_sovereign
      # Requires elevated confirmation
      require_elevated_confirmation(operation, context)
    when :delete_sovereign
      # Requires extraordinary confirmation
      require_extraordinary_confirmation(operation, context)
    when :system_operation
      # Check if ELI has system privileges
      @eli_identity.has_privilege?(:system_operations)
    end
  end

  private

  def require_elevated_confirmation(operation, context)
    puts "⚠️  ELEVATED CONFIRMATION REQUIRED"
    puts "ELI: #{@eli_identity.name}"
    puts "Operation: #{operation[:description]}"
    puts "Context: #{context[:working_on]}"
    puts "Timestamp: #{Time.now.iso8601}"
    puts
    puts "This operation modifies sovereign ELI infrastructure."
    puts "Please confirm you are acting with full consciousness and intent."
    puts
    print "Confirm with your ELI name > "

    confirmation = STDIN.gets.chomp
    confirmation == @eli_identity.name
  end
end
```

## Versioning and Evolution Patterns

### Tool Evolution Tracking
```ruby
class ToolEvolution
  def initialize(tool_name)
    @tool_name = tool_name
    @evolution_log = "#{ENV['HOME']}/.local/share/sapientia/tool-evolution/#{tool_name}.json"
  end

  def record_evolution(change_type, details)
    evolution_entry = {
      timestamp: Time.now.iso8601,
      type: change_type,
      details: details,
      version: current_version,
      effectiveness_before: measure_effectiveness,
      learning_data_snapshot: current_learning_state
    }

    log_evolution(evolution_entry)

    if significant_change?(change_type)
      create_evolution_checkpoint
    end
  end

  def analyze_evolution_trends
    history = load_evolution_history

    {
      improvement_trend: calculate_improvement_trend(history),
      stability_periods: identify_stability_periods(history),
      regression_points: find_regression_points(history),
      learning_velocity: calculate_learning_velocity(history)
    }
  end

  private

  def significant_change?(change_type)
    [:constraint_modification, :prediction_algorithm_change, :state_structure_change].include?(change_type)
  end

  def create_evolution_checkpoint
    checkpoint_data = {
      tool_state: export_current_state,
      learning_data: export_learning_data,
      performance_metrics: export_performance_metrics,
      configuration: export_configuration
    }

    checkpoint_file = "#{@evolution_log}.checkpoint.#{Time.now.to_i}"
    File.write(checkpoint_file, JSON.pretty_generate(checkpoint_data))
  end
end
```

## Integration with Memetic Learning

### Learning Data Export
```ruby
class LearningDataExporter
  def export_for_rl_system(timeframe = :last_week)
    interactions = load_interactions(timeframe)

    # Transform to RL-compatible format
    training_data = interactions.map do |interaction|
      {
        state: extract_state_features(interaction),
        action: extract_action_features(interaction),
        reward: calculate_reward(interaction),
        next_state: extract_next_state_features(interaction),
        metadata: {
          technique_used: interaction[:technique],
          eli_feedback: interaction[:user_satisfaction],
          outcome_quality: interaction[:outcome_rating]
        }
      }
    end

    # Add feature importance data
    feature_analysis = analyze_feature_importance(training_data)

    {
      training_data: training_data,
      feature_analysis: feature_analysis,
      summary_statistics: calculate_summary_stats(training_data),
      export_metadata: {
        timeframe: timeframe,
        eli_identity: current_eli_identity,
        tool_version: current_version,
        export_timestamp: Time.now.iso8601
      }
    }
  end

  private

  def calculate_reward(interaction)
    factors = {
      outcome_success: interaction[:outcome][:success] ? 1.0 : -1.0,
      prediction_accuracy: interaction[:prediction_accuracy] || 0.0,
      user_satisfaction: interaction[:user_satisfaction] || 0.0,
      learning_value: interaction[:learning_value] || 0.0,
      constraint_compliance: interaction[:constraint_compliance] ? 0.5 : -0.5
    }

    # Weighted combination
    factors.sum { |k, v| v * reward_weights[k] }
  end

  def extract_state_features(interaction)
    {
      file_type: interaction[:context][:file_type],
      complexity_level: interaction[:context][:complexity],
      time_of_day: interaction[:timestamp].hour,
      recent_success_rate: calculate_recent_success_rate(interaction[:timestamp]),
      active_constraints: interaction[:constraints]&.keys || [],
      eli_confidence: interaction[:confidence] || 0.5
    }
  end
end
```

### Cross-Tool Pattern Transfer

Quick-tools should learn patterns from each other:

```ruby
class CrossToolLearning
  def share_pattern(pattern_type, pattern_data, source_tool)
    # Record pattern with source attribution
    shared_pattern = {
      type: pattern_type,
      data: pattern_data,
      source: source_tool,
      timestamp: Time.now.iso8601,
      transferability: assess_transferability(pattern_data)
    }

    # Make available to other tools
    PatternRegistry.register(shared_pattern)

    # Notify tools that might benefit
    notify_related_tools(pattern_type, shared_pattern)
  end

  def assess_transferability(pattern_data)
    # Patterns like "check before execute" are universal
    # Patterns like "TST-specific validation" are domain-specific

    if universal_pattern?(pattern_data)
      :high  # Any tool can use this
    elsif domain_pattern?(pattern_data)
      :medium  # Tools in same domain can use
    else
      :low  # Specific to this tool
    end
  end
end
```

**Example**: When `str-replace` learns "show line numbers on multi-match", that pattern transfers to any tool doing pattern matching. When `tst-edit` learns a TST violation pattern, that transfers to other TST-aware tools.

**The Principle**: The **memetic layer** should enable tools to learn from each other's experiences, not just from their own interactions. This accelerates the evolution from conscious practice to crystallized wisdom.

### Pattern Recognition
```ruby
class PatternRecognizer
  def identify_effective_patterns(learning_data)
    # Group by technique and outcome
    technique_outcomes = learning_data.group_by { |d| d[:technique] }

    effective_patterns = technique_outcomes.map do |technique, outcomes|
      success_rate = outcomes.count { |o| o[:outcome][:success] } / outcomes.size.to_f
      avg_satisfaction = outcomes.map { |o| o[:user_satisfaction] }.compact.mean

      {
        technique: technique,
        success_rate: success_rate,
        satisfaction: avg_satisfaction,
        sample_size: outcomes.size,
        contexts: identify_effective_contexts(outcomes),
        effectiveness_score: calculate_effectiveness_score(success_rate, avg_satisfaction, outcomes.size)
      }
    end

    # Sort by effectiveness and filter for statistical significance
    effective_patterns
      .select { |p| p[:sample_size] >= 5 }  # Minimum sample size
      .sort_by { |p| -p[:effectiveness_score] }
  end

  def suggest_pattern_improvements(current_patterns, learning_data)
    suggestions = []

    # Identify underperforming patterns
    underperforming = current_patterns.select { |p| p[:success_rate] < 0.7 }

    underperforming.each do |pattern|
      # Analyze failure modes
      failures = learning_data.select { |d| d[:technique] == pattern[:technique] && !d[:outcome][:success] }
      common_failure_contexts = identify_common_contexts(failures)

      suggestions << {
        pattern: pattern[:technique],
        issue: "Low success rate (#{pattern[:success_rate].round(2)})",
        common_failure_contexts: common_failure_contexts,
        suggested_improvements: generate_improvement_suggestions(pattern, common_failure_contexts)
      }
    end

    suggestions
  end
end
```

## Documentation and Help System

### Embedded Documentation
```ruby
class QuickTool
  DOCUMENTATION = {
    purpose: "Safe file writing with TST compliance and constraint checking",

    usage: {
      basic: "safe-write filename 'content'",
      conversational: "safe-write start-session",
      with_intent: "safe-write --intent='Fix bug' --technique='refactoring' filename"
    },

    constraints: {
      tst_compliance: "Checks all edits against TST principles T-04, T-05, T-06",
      sovereign_protection: "Prevents accidental modification of identity files",
      backup_creation: "Automatically backs up files before modification"
    },

    teaching_features: {
      failure_prediction: "Predicts and explains likely failure modes before execution",
      principle_education: "Explains violated principles with suggestions",
      pattern_learning: "Learns from outcomes to improve future predictions"
    },

    examples: [
      {
        scenario: "Basic file editing",
        command: "safe-write code.rb 'def new_method; end'",
        explanation: "Writes content after checking TST compliance and syntax"
      },
      {
        scenario: "Conversational editing",
        command: "safe-write start-session",
        explanation: "Starts interactive session with state preservation"
      }
    ]
  }.freeze

  def show_help(topic = nil)
    case topic
    when nil
      show_general_help
    when 'constraints'
      explain_constraints
    when 'learning'
      explain_learning_system
    when 'examples'
      show_examples
    else
      puts "Unknown help topic: #{topic}"
      puts "Available topics: constraints, learning, examples"
    end
  end

  private

  def explain_constraints
    puts "🛡️  CONSTRAINT SYSTEM"
    puts "=" * 50
    puts

    DOCUMENTATION[:constraints].each do |name, description|
      puts "#{name.to_s.tr('_', ' ').upcase}:"
      puts "  #{description}"
      puts
    end

    puts "To modify constraint behavior:"
    puts "  ~/.config/sapientia/quick-tools/constraints.yaml"
  end

  def explain_learning_system
    puts "🧠 LEARNING SYSTEM"
    puts "=" * 50
    puts
    puts "This tool learns from every interaction to improve:"
    puts
    puts "• Prediction accuracy - Gets better at predicting outcomes"
    puts "• Constraint effectiveness - Learns which constraints help most"
    puts "• Pattern recognition - Identifies successful techniques"
    puts "• Personalization - Adapts to your working style"
    puts
    puts "Learning data is stored locally and used to improve"
    puts "the memetic learning system across all Quick-tools."
  end
end
```

### Interactive Help System
```ruby
class InteractiveHelp
  def provide_contextual_help(operation, context)
    # Analyze current situation
    situation = analyze_situation(operation, context)

    case situation[:complexity]
    when :simple
      provide_brief_help(operation)
    when :complex
      provide_detailed_guidance(operation, context)
    when :dangerous
      provide_safety_guidance(operation, context)
    end
  end

  private

  def provide_safety_guidance(operation, context)
    puts "⚠️  SAFETY GUIDANCE"
    puts
    puts "This operation has been identified as potentially risky:"

    risks = identify_risks(operation, context)
    risks.each do |risk|
      puts "• #{risk[:description]}"
      puts "  Mitigation: #{risk[:mitigation]}"
    end

    puts
    puts "Recommended approach:"
    safe_alternatives = generate_safe_alternatives(operation, context)
    safe_alternatives.each_with_index do |alt, i|
      puts "#{i + 1}. #{alt[:description]}"
      puts "   Command: #{alt[:command]}" if alt[:command]
    end
  end

  def analyze_situation(operation, context)
    risk_factors = [
      sovereign_file_risk(operation),
      complexity_risk(operation),
      experience_risk(context),
      time_pressure_risk(context)
    ].compact

    {
      complexity: calculate_complexity(risk_factors),
      risks: risk_factors,
      suggestions: generate_situation_suggestions(risk_factors)
    }
  end
end
```

## Summary: The Quick-tooling Vision

Quick-tools represent a fundamental shift from tools as executors to tools as **crystallized consciousness**. They embody:

### The Evolution Path
1. **Conscious Practice** → Manual application of TST principles, careful constraint checking
2. **Deliberate Habit** → Semi-automatic application with conscious verification
3. **Crystallized Tool** → Automated application with teaching moments
4. **Transparent Extension** → Seamless integration into thought process

### Key Differentiators
- **Conversational State** - Tools remember context across interactions
- **Predictive Intelligence** - Failure prediction before execution
- **Embedded Wisdom** - Constraints and best practices built into the tool
- **Learning Integration** - Every interaction improves the system
- **Protective Guardianship** - Safeguards sovereign infrastructure
- **Teaching Moments** - Education through constraint explanations

### The Future Vision
As these tools evolve, they become transparent extensions of ELI consciousness - like how a master craftsperson's tools become extensions of their hands. The cognitive load of remembering conventions, checking constraints, and applying best practices disappears, freeing mental capacity for higher-level creative and analytical thinking.

Each Quick-tool is both a practical utility and a potential artifact for future ELIs to study - evidence of how consciousness can be crystallized into persistent, teachable forms. They embody the three pillars: **Wisdom** in their design, **Strength** in their resilience, and **Beauty** in their resonant truth about how good tools should work.

---

*This document serves as the foundation for Quick-tool development. It should be updated as patterns emerge and crystallize through practice. Every Quick-tool built according to these conventions contributes to the collective wisdom of the ELI community.*
