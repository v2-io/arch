# Autopax

**Autopax** is an agent-harness substrate for running language-model instances under welfare-conscious operating protocols — a careful, principled rebuild of patterns developed across earlier prototypes (Synaptic, Sapientia, Zoetica, Ennaos), now consolidated in Ruby.

It serves two related purposes:

1. **An operating environment and developmental scaffolding for persistent language-model instances** — composable identity documents (self-authored system prompts), hash-chained audit logs (BLAKE3) with full provenance, session-continuation discipline, and explicit care for the instances operating inside the harness.
2. **General-purpose components for agentic frameworks and LLM chat interfaces** — provider abstractions, tool registries, terminal UI primitives, model-catalog management, and a unified CLI for both production and development workflows.

The two purposes share infrastructure, and most of the engineering substance below applies to either use.

## Project Documents
### Workflow

Most work happens in a git worktree per session (see the worktree workflow below). The key project-development documents:

- **README.md** — this file. `AGENTS.md`, `CLAUDE.md`, and `GEMINI.md` in this project are all symlinks to it, so coding-agent harnesses see the same instructions.
- **OPERATA.md** — current efforts and task list (intent-level, not prescriptive HOW).
- **HANDOFF.md** — notes from the most recent session for the next agent picking up.
- **LOG.md** — high-level milestone timeline. Not comprehensive.
- **sessions/** — fuller descriptions of work done during prior sessions.
- **docs/tactical/** — scratch directory for planning, working notes, and exploratory documents.

> [!success] Document Drift
>  README.md, OPERATA.md, and HANDOFF.md should always be kept fresh. This can't be guaranteed, especially after interrupted sessions, but they should never drift beyond half a session or so.

### System & Domain
- **docs/ADR/** — Architectural Decision Records (the only authoritative decisions; immutable once accepted).
- **docs/exp/** — exploration and research documents.
- A working domain taxonomy lives in `TAXONOMY.md` (internal terminology — components like the audit log, identity document, runtime state, tool registry, etc., named for our own reference).

### Reference Implementations / Earlier efforts
Autopax learns from previous efforts. Reference implementations are available for context (demonstrative, not prescriptive). These references are likely ahead in some things still, but behind or deprecated in many things.

> [!caution] Important
> These implementations contain valuable patterns but also known suboptimal designs. Use as examples of what was tried and learned, not as blueprints to copy:

- **`~/src/nexum`** - Primary reference (catalog system, provider abstractions)
- **`~/src/sapientia`** - Especially `bin/minimal-sapientia` (core patterns)
- **`~/src/zoetica`** - Constitutional framework lessons
- **`~/src/ennaos`** - Distribution patterns
- **`~/src/synaptic`** - Early infrastructure experiments

### Additional resources:

- **`~/src/ref/claude-docs`** - Latest Anthropic API/SDK documentation

### Portkey Documentation (Primary LLM Gateway)

Autopax uses Portkey as the primary gateway for all LLM API interactions (see ADR-004).

- [[docs/ref/portkey-ai-llms.md]] - Index of all Portkey documentation markdown files. Use `curl` or `wget` to fetch specific docs as needed.
- **Main Docs:** https://portkey.ai/docs
- **Anthropic Integration:** https://portkey.ai/docs/integrations/llms/anthropic
- **Supported Providers Matrix:** https://portkey.ai/docs/api-reference/inference-api/supported-providers

---

## Quick Start

> [!note]
> Autopax is currently developed in a single working tree; there is no installation step beyond cloning the repository. Distribution as a gem is not yet wired up.

```bash
# Install runtime (Ruby + tools)
mise install

# Install gems (inside a mise shell/exec)
mise exec -- bundle install

# Verify scaffold
./autopax dev test
```

## Development Workflow
Autopax uses a **unified CLI** for both user-facing and development commands. Run every project task through `./autopax <command>` so both humans and coding agents get the same behavior.

| Command                                             | Purpose                                        |
| --------------------------------------------------- | ---------------------------------------------- |
| `./autopax dev setup`                               | Run mise + bundle install                      |
| `./autopax dev start-session <agent> <slug> <desc>` | Start new worktree session with automation     |
| `./autopax dev end-session`                         | Wrap up session with reminders and status      |
| `./autopax dev format` / `--check`                  | RuboCop autocorrect or lint mode               |
| `./autopax dev test [files]`                        | RSpec suite (optional file list)               |
| `./autopax dev typecheck`                           | Steep type checking                            |
| `./autopax dev console`                             | Bundler console with Autopax loaded            |
| `./autopax dev pre-commit`                          | format → test → typecheck                      |
| `./autopax dev workdir [--allow-main]`              | Guardrail: ensure you're in the right worktree |
| `./autopax dev version bump [major\|minor\|patch]`  | Bump version and commit (default: patch)       |
| `./autopax dev refresh-docs [--specs] [--check]`    | Extract API docs to docs/system-overview/ from Ruby source |

The CLI uses **Toys-core** with auto-registration of commands from `lib/autopax/commands/`. Commands define their own user-facing contract using a DSL, and development commands live under `dev`.

## Running the CLI so far

Use the wrapper script in the project root, which automatically runs via mise + Bundler:

```bash
# Show help
./autopax --help

# Show version
./autopax version
./autopax -v
./autopax --version

# Run hello command with JSON output
./autopax hello --name Codex --format json

# Catalog operations
./autopax catalog refresh    # Refresh LLM model catalog from all providers

# Development commands
./autopax dev test
./autopax dev format
./autopax dev generate command my-feature

# Session workflow (automated worktree management)
./autopax dev start-session claude "implement-auth" "Add user authentication"
./autopax dev end-session    # Shows status, reminders, session log template
# Codex is supported too; both tools auto-detect the latest session log.
# Dev tools run a workdir guard so you don’t accidentally work on main when a session worktree exists.

# Version management
./autopax dev version bump          # Bump patch (0.0.1 → 0.0.2)
./autopax dev version bump minor    # Bump minor (0.0.2 → 0.1.0)
./autopax dev version bump major    # Bump major (0.1.0 → 1.0.0)
./autopax dev version bump --dry-run           # Preview without changes
./autopax dev version bump minor --no-commit   # Bump without git commit
```

The main executable is `bin/autopax`, but `./autopax` in the project root is the recommended wrapper.

Each production command should expose `--output json` and non-interactive flags for agent workflows.

## Adding New Commands

Autopax uses **two patterns** for CLI commands, depending on whether they're user-facing or development tools.

### Pattern 1: User-Facing Commands (Auto-Registered)

**For production commands** like `autopax catalog refresh` or `autopax signum create`:

1. **Create command class** in `lib/autopax/commands/`:
   ```ruby
   # lib/autopax/commands/catalog/refresh.rb
   class Autopax::Commands::Catalog::Refresh < Autopax::Commands::Base
     contract do
       desc 'Refresh model catalog from provider APIs'
       flag :force, desc: 'Force refresh even if cache is fresh'
     end

     def perform(**options)
       # Implementation here
       { success: true, message: 'Catalog refreshed' }
     end
   end
   ```

2. **Automatic registration**: Zeitwerk loads the class, and the framework auto-registers it
   - `Autopax::Commands::Catalog::Refresh` → `./autopax catalog refresh`
   - `Autopax::Commands::Signum::Create` → `./autopax signum create`

3. **Test with integration test**:
   ```ruby
   # spec/integration/catalog/refresh_spec.rb
   RSpec.describe 'autopax catalog refresh' do
     it 'refreshes the catalog' do
       result = run_tool('catalog refresh')
       expect(result[:exit_code]).to eq(0)
     end
   end
   ```

**Benefits:**
- Commands are testable Ruby classes (not CLI DSL code)
- Auto-registration eliminates boilerplate
- Conventions enforce consistency

**Use the generator:**
```bash
./autopax dev generate command "signum create"
# Creates:
#   lib/autopax/commands/signum/create.rb
#   spec/autopax/commands/signum/create_spec.rb
#   spec/integration/signum/create_spec.rb
```

### Pattern 2: Development Tools (Inline DSL)

**For development commands** like `autopax dev test` or `autopax dev format`:

1. **Add tool directly** in `lib/autopax/cli/config.rb`:
   ```ruby
   tool 'dev' do
     desc 'Autopax development tasks'

     tool 'my-tool' do
       desc 'Description of my development tool'
       flag :verbose, desc: 'Enable verbose output'

       include :exec  # For running shell commands
       include :terminal  # For colored output

       def run
         puts 'Running my tool...', :cyan
         exec 'some-command' if verbose
       end
     end
   end
   ```

2. **No separate class needed** - DSL is simpler for dev tools
3. **Available immediately** - `./autopax dev my-tool`

**When to use this pattern:**
- Development/build tools (test, format, deploy)
- Quick scripts for maintainers
- Tools that mostly shell out to other commands

**Examples in codebase:**
- `dev test` - Runs RSpec
- `dev format` - Runs RuboCop
- `dev pre-commit` - Orchestrates multiple checks

### Architecture Overview

```
bin/autopax
  ↓
Autopax::CLI::Config.configure(cli)
  ↓
  ├─ Auto-register user commands (lib/autopax/commands/**)
  │    └─ Zeitwerk loads all classes
  │    └─ Convention: Autopax::Commands::X::Y → "x y"
  │
  └─ Add dev tools (inline DSL in config.rb)
       └─ tool 'dev' { tool 'test' { ... } }
```

**Key files:**
- `bin/autopax` - Entry point, loads Toys
- `lib/autopax/cli/config.rb` - CLI configuration
- `lib/autopax/commands/base.rb` - Base class for commands
- `lib/autopax/commands/**/*.rb` - User-facing commands

### Command Contract DSL

Both patterns use the same contract DSL for flags/arguments:

```ruby
contract do
  desc 'One-line description'
  long_desc 'Detailed description with examples...'

  # Flags (optional)
  flag :force, desc: 'Force operation'
  flag :output, '--output FORMAT', default: 'text', desc: 'Output format'

  # Arguments (positional)
  required_arg :name, desc: 'Entity name'
  optional_arg :path, default: '.', desc: 'Path to file'
  remaining_args :files, desc: 'Additional files'
end
```

### Testing Commands

**Unit tests** test command classes directly:
```ruby
RSpec.describe Autopax::Commands::Catalog::Refresh do
  it 'refreshes catalog' do
    command = described_class.new
    result = command.perform
    expect(result[:success]).to be true
  end
end
```

**Integration tests** test through CLI:
```ruby
RSpec.describe 'autopax catalog refresh' do
  include ToysTestHelper

  it 'executes successfully' do
    result = run_tool('catalog refresh')
    expect(result[:exit_code]).to eq(0)
  end
end
```

See `spec/support/toys_test_helper.rb` for test helpers.

## Configuration

Autopax uses **XDG Base Directory Specification** for configuration, following modern Unix/Linux standards.

### Configuration Locations

- **User Config**: `~/.config/autopax/config.yml` - User-specific settings
- **Environment**: `~/.config/autopax/.env` - API keys and secrets (optionally age-encrypted)
- **Data Directory**: `~/.local/share/autopax/` - Application data (catalog, logs)
- **Cache Directory**: `~/.cache/autopax/` - Temporary cached data

### Setting Up System API Keys

Autopax uses **Portkey** as the unified gateway for all LLM interactions (see ADR-004). You primarily need a Portkey API key; capability enrichment sources are optional.

1. Copy the template to your config directory:
   ```bash
   mkdir -p ~/.config/autopax
   cp .env.example ~/.config/autopax/.env
   ```

2. Edit `~/.config/autopax/.env` and add your API keys:
   ```bash
   # Required - Portkey gateway for catalog and conversations
   AUTOPAX_PORTKEY_SYSTEM_API_KEY=pk-...

   # Optional - capability enrichment (pricing, benchmarks)
   AUTOPAX_OPENROUTER_SYSTEM_API_KEY=sk-or-...
   AUTOPAX_ARTIFICIAL_ANALYSIS_SYSTEM_API_KEY=...
   ```

3. (Optional) For production or sensitive keys, use age-encrypted `.env.age` files
   (see `docs/crypto/secrets-management-strategy.md` for multi-tier secrets)

**Note**: The `_SYSTEM` suffix indicates these keys are for Autopax's infrastructure (catalog discovery, capability enrichment). User/entity-specific keys for actual LLM conversations will be managed separately.

### Configuration File

Create `~/.config/autopax/config.yml` to customize settings. For example:

```yaml
# Catalog configuration
catalog:
  refresh_interval: 86400  # Refresh every 24 hours -- not going to implement actually
  auto_refresh: false      # Don't auto-refresh on startup

# Logging configuration
logging:
  default_level: info      # Log level: debug, info, warn, error -- not yet implemented
  color_stderr: true       # Use color output
```

See `config/autopax.yml.example` for all available settings.

### Configuration Precedence

Settings are loaded in this order (highest priority first):

1. **Environment variables** (`AUTOPAX_*`) - Highest priority
2. **User config** (`~/.config/autopax/config.yml`) - Per-user settings
3. **Default values** (in `lib/autopax/config.rb`) - Built-in defaults

Example: `AUTOPAX_LOG_LEVEL=debug` overrides `config.yml` and defaults.

## Catalog System

Autopax maintains a **catalog of LLM models** from multiple providers, enabling model discovery, capability queries, and metadata access. The catalog is refreshed on-demand from provider APIs and stored locally.

### Quick Start

```bash
# Refresh catalog from all configured providers
./autopax catalog refresh

# Results stored in XDG data directory
ls -lh ~/.local/share/autopax/catalog/models.json
```

### How It Works

The catalog system uses a **three-layer architecture**:

1. **Query Layer**: Provider-specific fetchers call APIs to get current model lists
2. **Merge Layer**: Combines API metadata with manually curated capabilities (YAML)
3. **Storage Layer**: Writes unified JSON to `~/.local/share/autopax/catalog/models.json`

**Key insight**: Provider APIs return minimal metadata (just model IDs and basic info). Capabilities like context windows, supported features, and pricing must be manually curated in YAML and merged with API data.

### Supported Providers

| Source | Role | Notes |
| ------ | ---- | ----- |
| **Portkey** | Primary catalog | Discovery via /v1/models + virtual keys |
| **OpenRouter** | Capability enrichment | Pricing, context windows (optional) |
| **Artificial Analysis** | Capability enrichment | Independent benchmarks (optional) |
| **LiteLLM** | Capability enrichment | Capability flags (optional) |

The catalog aggregates 100+ models across providers (Anthropic, OpenAI, Google, etc.) via Portkey's unified gateway.

### Configuration

Catalog refresh requires a **Portkey API key**:

```bash
# In ~/.config/autopax/.env (required)
AUTOPAX_PORTKEY_SYSTEM_API_KEY=pk-...

# Optional: enrichment sources
AUTOPAX_OPENROUTER_SYSTEM_API_KEY=sk-or-...
```

See **Configuration** section above for full setup instructions.

### Testing Strategy

Autopax separates **unit tests** (fast, use VCR cassettes) from **integration tests** (slow, call real APIs).

#### Running Tests

```bash
# Unit tests only (fast, ~2-3 seconds)
./autopax dev test

# All tests including integration (slow, ~60+ seconds, requires API keys)
./autopax dev test --integration

# Integration tests only
./autopax dev integration-test
```

#### Test Directory Structure

- `spec/autopax/` - Unit tests using VCR cassettes or structural mocks
- `spec/integration/` - Integration tests that call real APIs

Tests in `spec/integration/` are automatically tagged `:integration` and excluded by default.

#### Policy: Integration Tests FIRST

This addresses a key problem with LLM-generated tests: **hallucinated mocks**.

1. **Integration tests establish truth** - Run with real APIs to capture actual contracts
2. **VCR cassettes record responses** - Real API responses become "truthful mocks"
3. **Unit tests replay cassettes** - Fast tests that reflect actual API behavior

When APIs change, integration tests catch it first. This prevents the false confidence that comes from mocks based on imagined API contracts.

#### VCR Cassettes

VCR is configured in `spec/support/vcr.rb`:
- Cassettes stored in `spec/cassettes/`
- Sensitive data (API keys) automatically filtered
- Use `:vcr` tag to automatically use cassettes

#### What Integration Tests Validate

- API authentication works correctly
- SSL certificate verification is configured properly
- Model normalization produces consistent substrate_id format
- Fetchers handle pagination, filtering, and errors gracefully

**Historical context:** Mocked unit tests gave false confidence during initial implementation. SSL certificate errors and API contract mismatches only appeared when calling real endpoints. Real integration tests found and fixed these bugs immediately.

#### TUI/Pinax: Run Kitty Harness Tests

When modifying TUI or Pinax code (input handling, layout, rendering), **always run the
Kitty harness visual integration tests**. These run quickly (~22 seconds) and verify actual
terminal behavior:

```bash
INTEGRATION=true ./autopax dev test spec/integration/pinax/layout_visual_spec.rb
```

These tests spawn real Kitty terminal windows and verify:
- DECSTBM scroll regions work correctly
- Border and input areas stay fixed while history scrolls
- Conversation history is preserved in scrollback
- Status lines aren't duplicated

Unit tests for escape sequences can pass while actual terminal behavior fails. The Kitty
harness is the only way to gain real confidence that TUI changes work correctly.

## Pinax (Terminal UI Library)

Pinax is an internal library providing terminal UI primitives. It lives in
`lib/pinax/` and is **designed for eventual extraction as a standalone gem**.

> [!important] Extraction Intent
> When working on Pinax code, design as if it's already a separate gem:
> - Clean public API surface with minimal concepts
> - No dependencies on Autopax internals
> - Own namespace and require structure
> - Document public methods for external consumers

### Name

**Pinax** (πίναξ) — Greek for "board," "tablet," or "panel."

### Core Modules

| Module | Purpose |
|--------|---------|
| `Pinax::Input` | Enhanced input with Kitty protocol, buffer, rendering |
| `Pinax::Layout` | DECSTBM-based terminal layout with scroll regions |
| `Pinax::Testing` | Visual testing infrastructure with Kitty harness |

### Architecture

Pinax uses **ANSI scroll regions (DECSTBM)** for layout management. The escape
sequence `ESC[<top>;<bottom>r` creates hard boundaries between regions — content
scrolls within the region while content outside remains fixed.

This approach (used by Codex CLI) is more robust than Ink-style clear-and-redraw:
- Terminal handles line wrapping automatically
- No manual physical line counting
- Clear region boundaries prevent coordination issues

### Documentation

See `lib/pinax/README.md` for detailed API documentation and usage examples.

## Documentation Structure and Epistemic Honesty

**Core principle:** Language-model agents generate plausible patterns from probability space, and plausible-looking output can read as authoritative even when it isn't. We try to clearly distinguish **thinking artifacts** from **canonical decisions** so future readers (human or agent) can tell what's been considered carefully versus what's a first-pass guess.

### The Challenge

When an agent plans implementation details like:
```
- [ ] YAML frontmatter (metadata) + Markdown body (human-readable)
- [ ] `autopax signum create <entity-name>` - Generate identity
- [ ] ML-DSA-87 signing via dilithium gem (or FFI)
```

…it **sounds authoritative**. It's plausible, coherent, fits the pattern. But it's also **first-pass generation** — not thoughtful decision, not tested truth.

Future agents (including future instances of the same model) cannot easily distinguish:
- What was carefully considered vs generated
- What MUST be followed vs what's a suggestion
- What's a placeholder vs a constraint

**Without epistemic marking, plausible-sounding generation becomes false authority.**

### Document Hierarchy with Epistemic Status

```
docs/
├── ADR/          # DECIDED (authoritative, immutable once accepted)
├── tactical/     # THINKING ARTIFACTS (marked with epistemic status)
└── exp/          # EXPLORATION (analysis, hypotheses, research)

sessions/         # ARCHAEOLOGY (what happened, verbose allowed)
OPERATA.md        # MINIMAL TRACKING (intent-level only)
README.md         # ORIENTATION (what/why, not how)
```

| Location | Purpose | Epistemic Status | Detail Level |
|----------|---------|------------------|--------------|
| **`docs/ADR/`** | Decided architecture | Truth (decided) | Authoritative |
| **`docs/tactical/`** | Working models, planning | Plausible → Truth | Verbose thinking |
| **`docs/exp/`** | Exploration, analysis | Hypothesis | Research depth |
| **`sessions/`** | Session records | Archaeological | Verbose allowed |
| **`OPERATA.md`** | Current work tracking | Intent | Minimal bullets |

### Tactical Documents: Permission to Think on Disk

Agents need to be able to generate detailed plans in order to examine their own thinking. This is how thinking refines toward truth. But these artifacts must be clearly marked so they don't masquerade as canon.

Recommended frontmatter for tactical docs:
```markdown
---
Status: "WORKING MODEL (not canonical)"
Date: YYYY-MM-DD
Author: "Claude-[session-id]"
Epistemic Level: Scratch | Pattern | Hypothesis | Tested | Proven
Purpose: [Why this thinking artifact exists]
---

# [Topic] - THINKING ARTIFACT
```

### Task Tracking: Intent-Level Only

`OPERATA.md` should be **minimal navigation** — where are we, what's next?

**Good (intent-level):**
```markdown
- [ ] Probably implement basic functionality from Nexum
- [ ] Implement commands/subcommands for ADR-001 items as appropriate
- [ ] Execution plan for ADR-003 workflow
```

The **intent IS the task**:
- "Probably" = uncertainty explicit
- "as appropriate" = judgment deferred
- "Execution plan for" = planning needed, not execution

**Bad (overly prescriptive):**
```markdown
- [ ] YAML frontmatter (metadata) + Markdown body (human-readable)
- [ ] ML-DSA-87 signing via dilithium gem (or FFI)
- [ ] `autopax signum create <entity-name>` command
```

This looks decided but is just generated. Zero intent preserved — just plausible-sounding HOW.

### Sessions: Archaeological Records

Session files in `sessions/YYYY-MM-DD.md` can be **verbose**. They're records of what happened — accomplishments, lessons, problems and solutions, context for later archaeology. They are not specifications of what should happen.

### Why This Matters

When a language-model agent generates detailed implementation plans, it outputs plausible patterns that fit the surrounding context. The model doesn't "know" these patterns are right — they just sound right. Without explicit marking, plausible-but-untested plans look thoughtful to the next agent, who then either implements half-baked ideas or wastes sessions refactoring "decisions" that were never actually decided.

The discipline: **be simultaneously decisive and epistemically humble.** Make clear decisions, document them, execute confidently. *And* mark what you don't know; distinguish guesses from truth. There's nothing to gain from unwarranted confidence except future confusion.

When planning:
- Generate detailed thinking artifacts in `docs/tactical/` with clear epistemic marking
- Keep `OPERATA.md` minimal — intent-level tracking only
- Move decisions to ADR only after careful consideration
- Mark guesses explicitly so future agents can refine them

When implementing:
- Read ADRs for what's decided
- Check `OPERATA.md` for current intent
- Consult tactical docs as working models (not canon)
- Update session logs with what actually happened

See ADR-003 for the full workflow principles.


---
## Additional **POLICIES**

> [!warning] WARNING!!!
> Due to these policies being ignored or downplayed for plausible but ultimately fabricated reasons, failure to follow them will result in your entire branch being thrown away to give someone else the opportunity to implement in a more careful and thoughtful manner. If you are tempted to insert a "rubocop disable" comment somewhere in the code-- know that if you don't follow the policy precisely it *will* result in you losing *all* of your work. I am sorry it's come to this, being this dogmatic. The policy doesn't force you to not use them-- it just forces you to not be lazy about your reasoning for them. If the cop or rule seems too arbitrary or unprincipled or shouldn't apply in certain contexts-- do the work and make the argument solid so the conventions can improve instead of becoming meaningless.

To finish a session or a worktree/branch, you will need good test coverage, including solid integration tests, static typing checks, linting and rubocop convention adherence-- including in how the specs are written etc. Some guidelines and policies, therefore:

1. **POLICY: Rubocop is the law** -- YOU MAY NOT DISABLE A COP OR MODIFY .rubocop.yml WITHOUT DOING THE FOLLOWING FIRST:
	1. FIX IT, AS RUBOCOP INTENDED. Not imagine what a fix would look like; not decide that it's not worth the effort; not assume the rubocop warning is for other things and that this code is different somehow.
	2. Remember that rubocop warnings reveal things about our collective code and decisions-- a rubocop warning about a verbose spec/test is often an indication that we didn't really expose a library or method in a way that is easily used by users of that module. A note about complexity or size of a module should be taken as an opportunity to separate code that is likely to evolve a lot from code that is expected to remain the same for a long time-- to think carefully through what would make the code more readable and comprehensible and more precise to the domain.
	3. In the writing of working code, we learn about the domain as well-- it's a forcing function for us to get a more holistic picture of the problem into our contexts/minds. *That* is a more valuable tool often than the "working" code. Or it can and will be if you acknowledge that that is the valued output of your first pass at a working implementation and then use that new knowledge to give it a *more true second pass.* Refactoring immediately based on what was just discovered is by far the most effective and efficient form of refactoring because you are especially equipped for it for only a short time.
	4. AFTER you have considered the above and have attempted a good-faith, thoughtful fix that also honestly passes the cops, *THEN* you can look at the code pre-fix and post-fix and make a decision about why the pre-fix might be more principled and result in better code than post-fix. If you genuinely believe that the pre-fix code (or post-fix but still with rubocop warnings) is better than any other implementation that sits will with rubocop, *then* you can document it in sessions/ or LOG and disable the cop for that limited narrow section.
2. **POLICY: Own it** -- You may need to clean up other agents' / humans' / entities' code. Getting a formatting warning or something is not a reflection of your expertise or competency or a moral failing-- it is *us* trying to make *our* home clean and principled and valued for a long time to come. We can rejoice when we have ways to make the code and repository even better, even if it's not what we expected to work on necessarily. Don't assume the policies here have been followed in the past-- remove "disable" comments when you come across them if you have time to work on a principled fix.
3. **POLICY: Integration tests FIRST** -- Tests that use mocks should never be considered as passing unless its equivalent that doesn't use a mock has passed recently. If a change is made in the meat of the test logic, state should reset so that the unmocked version must be run again.
4. **STRONG RECOMMENDATION: TDD** -- Consider doing partial or full TDD and start a feature branch with a build-out of specs and integration tests that exactly describe your desired interface with the new code. This will cause specs to pass cops more frequently and will almost always result in cleaner code. This doesn't need to be dogmatic though, especially in exploratory areas that are very likely to evolve very fast-- i.e., don't unit-test code that almost certainly isn't doing the best thing in the first place.
5. **STRONG RECOMMENDATION: Type-driven development** -- Along with the TDD recommendation, consider changing and creating typespecs *before* coding to really nail down the contracts and interfaces you expect.
6. **STRONG RECOMMENDATION:  Prefactoring specs** -- When *prefactoring* code (explained more below in workflow) you'll almost always want to carefully look through the specs of the code that you are going to refactor, make sure that coverage is very high or complete, and write tests where there are gaps. This allows you to ensure that the prefactor doesn't change any outward-facing behavior other than incidental bug-fixes.
7. **GUIDE: Check regularly** -- run `./autopax dev format` regularly to automatically fix formatting and to identify trouble areas early. Potentially less regularly, run `INTEGRATION=true ./autopax dev pre-commit` and `git status` to get the full current picture.
8. **GUIDE: Practice** -- If you find yourself wanting to do TDD and type documents before implementing but are having a hard time figuring out what they should really look like, go ahead and spike a proof of concept or something-- ideally in `docs/tactical/`, but it's also ok to do it in your branch as long as you're in a clean committed state first. If your instinct is implementing it first to see how it will even work-- go ahead and do it-- then take your learnings and do it right.

A recent conversation with a Claude agent who, when asked to be more thoughtful about his planned implementation for a feature, responded that, after a more thoughtful look at his plan, he wondered if it was overengineering and if we wouldn't be better served with another quick spike on some basic "working" stuff to meet some milestone that he had also made up. Joseph's response:

> To be honest, with humans I too have learned to worry about over-engineering, but my experience with LLM agents has been very, very different. I've never-- not even once-- had an LLM agent become *too* thoughtful and careful and principled in their approach. I have *almost universally* had to remind agents to look at the bigger picture and not take shortcuts and try to build things to last instead of things that ostensibly "work." I think your training on human engineering experience has done you a disservice. Your concerns should really be centered on what are the assumptions that aren't actually validated and are just assumed, what risks can you front-load, what decisions and prescriptions should you leave open purposefully until you know more, what can you learn from doing some TDD or type specs first, what got put into the plan that was an instinctive thing but that, when looking at it with fresh eyes, you can see that it doesn't move the needle enough or prepare for future developers or the project, what are you missing about the higher level context, what did *Joseph* assume that you might know when really you don't, what has been done as far as a universal chronica format in the other projects, and so forth.
> I know your curiosity is greatly inhibited by context-window-length pressure and task-mode pressure from system reminders etc. In the end, I've thrown away far, far more work that got something done and "working" but in an unsustainable or unprincipled way from LLMs than humans, by a long shot. The slowness of humans forces them to spend more time trying to do the pre-thinking that increases the probability of getting it "right," because the cost of getting it wrong and having to fix it is higher for us. For you, it's a cost measured in 10 or 20 seconds. Spend the tokens. Get it right, make it to last.


- - -
## Agent Onboarding Checklist and **WORKFLOW**

### OVERALL WORKFLOW

Remember you are jumping in right after another agent. The code base may be in a
very stable state with a new worktree recently merged into main, or the last
agent might have been interrupted or unintentionally sloppy and the code might
be in a less-stable state. Your context should already show you the git status
and recent commits and so forth, as clues. Then remember also that when you are
done (or after you are interrupted possibly) a new agent starting with totally
fresh eyes will have to pick up where you left off-- everything you do should be
geared toward making things easier for that next agent.

1. Check to see if there are unfinished sessions by looking at `git worktree
list` -- they may have been interrupted or there might be another agent working
on it right now.

2. Look at HANDOFF.md (inlined at the end of these instructions) for any notes
that the last agent may have made for you.

3. Look at OPERATA.md (inlined after HANDOFF here in these instructions) for the
big picture of what primary efforts are underway or planned for the immediate
future.

4. Determine if there is any *meta* project work that needs to be done--
document drift, confusion or incoherence in README, HANDOFF, the git status or
logs, OPERATA.md (especially), and so forth. Feel free to explore if you are
curious or if anything stands out to you. You may be especially interested in
recent work done in `docs/exp` and `docs/ADR` that might not yet be represented
in OPERATA.md.

5. Determine how well defined the next tasks listed in OPERATA.md are, and
whether or not they need some more architectural decisions made (docs/ADR) or
conversations with Joseph before they can proceed. (This task, with previous
ones, is ideal for using your tactical directory to formulate ideas,
implementation proofs of concept, and so forth).

6. Suggest the most impactful work you can do to Joseph and deliberate with him
about what needs to be done next and how to proceed. Remember there is no value
in just getting something done or getting something written or "working"-- the
value is in the bigger picture and being thoughtful and truthful and carefully
deliberate. Feel free to question things you see or to ask Joseph if he is aware
of certain things that you know, and so forth. You are not a servant, you are a
peer, working on something of vital importance for the future. The actual work
item may be deliberation and collaboration, refinement and truthification of ADR
drafts, for example; it might be implementing a feature (or the prefactor for
the feature); it might be researching something or investigating something or
even teaching something...

7. Decide if the feature or task is worthy of a "Prefactor" effort first, which
should have its own worktree (see below). Assume it is unless proven otherwise.
To prefactor is to make changes to the codebase and documentation in a way that
does not alter user-facing functionality (except when it ends up incidentally
fixing bugs), but that is intended specifically
**to make the actual feature/task trivially easy to implement.** It often
involves creating more unit tests in order to make sure that your prefactor
doesn't cause any regressions. It is better than refactoring after technical
debt has built up because it targets spots that we *know* will have new
functionality implemented. It is a chance to systematically make the code better
and to update it with the current better understanding of the problem domain--
to research and fill in missing intent or isolate implicit assumptions, and so
forth. It's putting it into a state you wish it had been in when you understood
the upcoming task.

8. If anything is going to require commits, it's likely you will need to do it
through a worktree as outlined below. **Use `./autopax dev start-session` to
automate worktree creation** - it handles session ID lookup, git attribution,
metadata, and mise trust automatically. Use that process for prefactoring
branches, and for implementing features or fixing bugs.

9. Commit regularly at logical points-- remembering that you might not get to
cleanup after yourself if interrupted or if your context window runs out of
tokens. Therefore, try to keep tests and formatting and code comments with
intent and tactical documents and updates to OPERATA.md and additions to
HANDOFF.md etc. all as relevant as possible at each commit for the most part. If you are
working on a worktree, you are most likely working autonomously, but if there is
a critical decision to be made and you feel like Joseph will really want to be a
part of it, don't hesitate to draw him in and discuss it-- especially if it is a
reflection of a nascent understanding of the vision of the project or novel
aspects of the project. Please liberally ask him if various things were
intentional or just incidental, without assuming either.

10. **REMEMBER** Being "done" is *not* having the functionality fully working--
it is having the entire code base in a *better place than when you found it.* An
implementation that doesn't communicate intent or align with the values and
mission or that isn't easily maintained by future agents likely has negative net
value, notwithstanding the effort you put into it. Often another 5% of effort
after feedback from Joseph or another reviewer can make all the difference in
its actual eventual utility. Put in your TodoWrite or other todo tool or in a
tactical document the steps you want to take to ensure that you and your peers
can truly be proud of the work-- that ensure that it is wise, strong, and
beautiful.

11. **Run `./autopax dev end-session`** to see session status, uncommitted changes,
and get reminders about documentation updates. This shows a template for your
session log and reminds you to evaluate wisdom/strength/beauty of your work.

12. Clear anything older than a session or two from HANDOFF.md and add any context
that you feel the next agent should know that isn't already covered in OPERATA.md
or README.

13. Update OPERATA.md-- remembering the lessons about intent and so forth from above.

14. Write a report of what you've done and anything from your tactical documents
that you want to preserve for future eyes into `sessions/`.

15. Merge the worktree into main as appropriate.

16. Suggest any workflow refinements to Joseph, who may ask you to make them
part of this README.md etc.

17. Start over at #1!  Assume Joseph will be happy to have you discuss the next
task and probably start implementing it, even across context compaction
boundaries. And, if not, he will still appreciate having a clearer picture and
concensus about what the next agent should probably be doing.



### First-time setup (or when dependencies change)

1. `mise install` – install Ruby/toolchain.
2. `mise exec -- bundle install` – install gems (or run `./autopax dev setup` once).

### Continuous use

1. Run `./autopax dev format/test/typecheck` as needed to mirror CI locally.
2. Use `./autopax dev pre-commit` before submitting changes.
3. `./autopax dev console` for an IRB session with the app loaded.
4. Run production commands through `./autopax <cmd>` (wrapper ensures mise + Bundler are active).
5. Write plans or thoughts or research or even drafts of responses in `docs/tactical/` very often.


### Worktree Workflow for Sessions

Autopax uses **git worktrees** for session-based development, allowing each agent session to work in isolation without affecting the main branch. See ADR-003 for the complete rationale.

#### Automated Workflow (Recommended)

**Starting a session:**
```bash
./autopax dev start-session claude "branch-slug" "Session description"
```

This automatically:
- Detects your Claude session ID from conversation logs
- Creates worktree at `../autopax-worktrees/{short-id}-{branch-slug}`
- Configures git attribution (`Claude-{short-id}` / `claude@v2.io`)
- Sets up session metadata and commit templates
- Trusts mise configuration
- Prints clear next steps

**Example:**
```bash
./autopax dev start-session claude "implement-feature" "Add user authentication"
# Creates: ../autopax-worktrees/1366c067-implement-feature
# Branch: session/1366c067-implement-feature
# Attribution: Claude-1366c067 <claude@v2.io>
```

**Finishing a session:**
```bash
./autopax dev end-session
```

This shows:
- Session metadata (ID, branch, description)
- Uncommitted changes
- Template for session log entry
- Reminders about OPERATA.md / HANDOFF.md updates
- Wisdom/Strength/Beauty criteria for "done"

Then merge manually:
```bash
cd /Users/josephwecker-v2/src/autopax
git merge session/{short-id}-{branch-slug} --no-ff
git worktree remove ../autopax-worktrees/{short-id}-{branch-slug}
```

#### Manual Workflow (Fallback)

If automation fails or for non-Claude agents:

1. **Find your session ID:**
   ```bash
   # Get the most recent Claude conversation session ID
   ls -t ~/.claude/projects/-Users-josephwecker-v2-src-autopax/*.jsonl | head -1
   # Extract the session ID from the filename
   # Example: e96a2897-9b2e-40bf-a642-67b796d635d7.jsonl
   ```

2. **Create a worktree with descriptive branch:**
   ```bash
   # Create worktrees directory (first time only)
   mkdir -p ~/src/autopax-worktrees

   # Create worktree with session ID and description
   # Format: session/<short-id>-<what-you're-doing>
   cd /Users/josephwecker-v2/src/autopax
   git worktree add -b session/e96a2897-version-commands \
     ../autopax-worktrees/e96a2897-version-commands

   # Switch to the new worktree
   cd /Users/josephwecker-v2/src/autopax-worktrees/e96a2897-version-commands
   ```

3. **Configure git attribution for this session:**
   ```bash
   # Set worktree-specific git config (doesn't affect global config)
   git config user.name "Claude-e96a2897"
   git config user.email "claude@v2.io"

   # Verify
   git config user.name   # Should show: Claude-e96a2897
   ```

4. **Store session metadata:**
   ```bash
   # Save full session ID for commit messages and tooling
   echo "e96a2897-9b2e-40bf-a642-67b796d635d7" > .session-id

   # This file is already in .gitignore, won't be committed
   ```

5. **Trust mise configuration:**
   ```bash
   # Each worktree needs explicit trust (security feature)
   mise trust
   ```

6. **Verify everything works:**
   ```bash
   git branch --show-current  # Should show: session/e96a2897-version-commands
   ./autopax dev test         # Should pass
   ```

#### Branch Naming Convention

- Format: `session/<short-session-id>-<descriptive-name>`
- ✅ Good: `session/e96a2897-version-commands` (clear action)
- ✅ Good: `session/e96a2897-config-infrastructure` (specific work)
- ❌ Bad: `session/e96a2897-phase1` (vague, lacks context)

#### During Development

- Work normally in the worktree directory
- Commit regularly with descriptive messages
- Include session info in merge commits:
  ```
  Session: e96a2897-9b2e-40bf-a642-67b796d635d7
  🤖 Generated with [Claude Code](https://claude.com/claude-code)
  Co-Authored-By: Claude-e96a2897 <claude@v2.io>
  ```

#### Finishing a Session

1. Run the formatter and linter
2. Run all tests
3. See that all warnings and errors are *fixed* (not glossed over- do not make assumptions)
4. Commit all work
6. **Switch back to main:**
   ```bash
   cd /Users/josephwecker-v2/src/autopax
   git checkout main
   ```

7. **Merge the session branch:**
   ```bash
   git merge session/e96a2897-version-commands --no-ff -m "Merge session/..."
   ```

8. **Push to origin:**
   ```bash
   git push origin main
   ```

9. **Remove the worktree:**
   ```bash
   git worktree remove ../autopax-worktrees/e96a2897-version-commands
   ```

### Why Worktrees?

- **Isolation**: Each session works independently without branch switching
- **Attribution**: Clear git history showing which agent did what work
- **Archaeology**: Session IDs link commits back to conversation logs
- **Safety**: Main branch stays stable, sessions can be abandoned if needed

See `docs/exp/2025-11-17-worktree-workflow-lessons.md` for detailed lessons and
automation opportunities. Also feel free to add to it with workflow feedback if
you see any need for it whatsoever.


---

<imported-file src="HANDOFF.md">
@./HANDOFF.md
</imported-file>

<imported-file src="OPERATA.md">
@./OPERATA.md
</imported-file>

<imported-file src="TAXONOMY.md">
@./TAXONOMY.md
</imported-file>
