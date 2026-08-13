# CLI Tool Conventions and Best Practices

## Table of Contents
1. [Core Design Philosophy](#core-design-philosophy)
2. [Naming and Structure](#naming-and-structure)
3. [Command-Line Interface](#command-line-interface)
4. [Input/Output Handling](#inputoutput-handling)
5. [Configuration Management](#configuration-management)
6. [Error Handling](#error-handling)
7. [AI Agent Considerations](#ai-agent-considerations)
8. [MCP and Advanced AI Tool Usage](#mcp-and-advanced-ai-tool-usage)
9. [Specialized Aliases and Mode Conventions](#specialized-aliases-and-mode-conventions)
10. [Signal Handling](#signal-handling)
11. [Side Effects and Idempotency](#side-effects-and-idempotency)
12. [One-off Scripts and Ad-hoc Tools](#one-off-scripts-and-ad-hoc-tools)
13. [Script Testing](#script-testing)
14. [Profiling and Performance Analysis](#profiling-and-performance-analysis)
15. [Fuzzing and Security Testing](#fuzzing-and-security-testing)
16. [Load Testing](#load-testing)
17. [Logging](#logging)
18. [Versioning and Updates](#versioning-and-updates)
19. [Security](#security)
20. [Performance and Resources](#performance-and-resources)
21. [Testing and Validation](#testing-and-validation)
22. [Internationalization](#internationalization)
23. [Multi-tenancy and Profiles](#multi-tenancy-and-profiles)
24. [Backup and Recovery](#backup-and-recovery)
25. [Distributed Operations](#distributed-operations)
26. [Data Migration](#data-migration)
27. [Observability and Hooks](#observability-and-hooks)
28. [Resilience Patterns](#resilience-patterns)
29. [Batch Processing](#batch-processing)

## Core Design Philosophy

### Unix Philosophy Foundations
- **Do one thing well** - Each utility should have a single, clear purpose
- **Composability** - Design for chaining with other tools via pipes
- **Text streams as universal interface** - With structured output options for machines
- **Silence is golden** - No output on success unless explicitly requested
- **Fail fast and explicitly** - Clear, immediate errors with proper exit codes
- **Idempotency** - Operations should be idempotent where possible

### AI Agent Design Principles
- **Predictable, deterministic behavior** - Same inputs always produce same outputs
- **Structured output modes** - JSON/TSV/CSV options via flags
- **Machine-readable errors** - Parseable error formats, not just human prose
- **Explicit verbosity control** - Clear separation of operational output vs diagnostic info
- **No interactive prompts in non-interactive mode** - Fail fast instead

## Naming and Structure

### Tool Naming
- **Lowercase, hyphenated**: `my-tool`, `data-processor`
- **Avoid**: Underscores (`data_processor`), CamelCase (`MyTool`)
- **Be descriptive but concise**: 2-15 characters ideal

### Command Structure
```bash
# Simple tools
tool [options] [arguments]

# Complex tools with subcommands
tool subcommand [options] [arguments]

# Verb-noun pattern
git clone
docker run
aws s3 cp
```

## Command-Line Interface

### Universal Flags
Every tool should support:
```bash
-h, --help                    # Show help
-v, --verbose                 # Increase verbosity (stackable: -vvv)
-q, --quiet                   # Suppress non-error output
--version                     # Show version and exit
--format=FORMAT               # Output format (json|text|csv|tsv|yaml)
--no-color                    # Disable colored output
--color=auto|always|never     # Color output control
--dry-run                     # Preview what would be done
--debug                       # Maximum verbosity for debugging
```

### Flag Conventions
```bash
# Short flags (single dash, single letter)
-v              # Boolean flag
-f filename     # With argument (space)
-ffilename      # With argument (no space)
-abc            # Combined boolean flags (equals -a -b -c)

# Long flags (double dash, descriptive)
--verbose                     # Boolean flag
--file=filename              # With argument (equals)
--file filename              # With argument (space)

# Special conventions
--                          # Stop processing flags
-                           # Stdin/stdout placeholder
@filename                   # Read arguments from file
```

### Exit Codes
```bash
0     Success
1     General errors
2     Misuse of shell command (invalid options, missing arguments)
64    Command line usage error (EX_USAGE)
65    Data format error (EX_DATAERR)
66    Cannot open input (EX_NOINPUT)
67    Addressee unknown (EX_NOUSER)
68    Host name unknown (EX_NOHOST)
69    Service unavailable (EX_UNAVAILABLE)
70    Internal software error (EX_SOFTWARE)
71    System error (EX_OSERR)
72    Critical OS file missing (EX_OSFILE)
73    Can't create output file (EX_CANTCREAT)
74    I/O error (EX_IOERR)
75    Temporary failure (EX_TEMPFAIL)
76    Remote error in protocol (EX_PROTOCOL)
77    Permission denied (EX_NOPERM)
78    Configuration error (EX_CONFIG)
126   Command found but not executable
127   Command not found
128+n Fatal signal n (e.g., 130 = 128+2 = SIGINT)
```

## Input/Output Handling

### Stream Usage
- **stdin**: Primary input data (when no file specified)
- **stdout**: Primary output, pipeable data only
- **stderr**: Errors, warnings, progress indicators, diagnostics

### Core Principle
`stdout` should be immediately pipeable - never mix status messages with data output.

### Stream Behavior Examples
```bash
# Good - clean separation
$ mytool process data.txt > output.txt
Processing 1000 records...    # to stderr
[progress bar]                # to stderr
Done!                         # to stderr
# output.txt contains only data

# Bad - mixed output
$ mytool process data.txt > output.txt
Processing 1000 records...    # to stdout (contaminating)
{"data": "actual output"}     # to stdout
Done!                         # to stdout (contaminating)
```

### Handling Merged Streams (2>&1)

#### Detection
```python
import os
import sys

streams_merged = os.fstat(sys.stdout.fileno()) == os.fstat(sys.stderr.fileno())
```

#### Adaptation Strategies

1. **Automatic Mode Switching**
   ```bash
   # When streams merged detected:
   - Suppress all progress/status to stderr
   - Switch to structured output if available
   - Use line prefixes for critical messages
   ```

2. **Prefixed Output Pattern**
   ```
   ERROR: Failed to process record 5
   WARNING: Using deprecated format
   DATA: {"actual": "data", "here": true}
   PROGRESS: 50/100 records processed
   ```

3. **Structured Output Mode**
   ```json
   {"type":"progress","current":0,"total":1000}
   {"type":"data","content":{"id":1,"value":"foo"}}
   {"type":"warning","message":"Deprecated field 'bar'"}
   {"type":"data","content":{"id":2,"value":"baz"}}
   {"type":"result","status":"success","records":1000}
   ```

### Interactive vs Non-Interactive

```bash
# Detection
if [ -t 0 ] && [ -t 1 ]; then
    # Interactive: colors, prompts, progress bars OK
else
    # Non-interactive: plain output, no prompts
fi

# Override flags
--interactive     # Force interactive mode
--batch          # Force non-interactive mode
--no-tty         # Assume no terminal
```

### Pipeline Safety
```bash
# Guarantee clean stdout for pipelines
--pipe           # Equivalent to: --quiet --format=text --no-progress

# Usage
mytool process --pipe < input.txt | next-tool
```

## Configuration Management

### Precedence Order (highest to lowest)
1. Command-line flags
2. Environment variables (`MYTOOL_*` prefix)
3. Local config file (`./.mytoolrc` or `./mytool.{json,yaml,toml}`)
4. User config file (`~/.config/mytool/config`)
5. System config (`/etc/mytool/config`)
6. Built-in defaults

### Configuration File Locations
```bash
# Standard locations (XDG Base Directory Specification)
~/.config/mytool/           # User config directory
~/.local/share/mytool/      # User data directory
~/.cache/mytool/            # User cache directory
/etc/mytool/                # System config directory

# Legacy support (if needed)
~/.mytool                   # Old-style dotfile
~/.mytool.conf              # Old-style config
```

### Environment Variables
```bash
# Naming convention
MYTOOL_CONFIG_FILE=/path/to/config
MYTOOL_LOG_LEVEL=debug
MYTOOL_FORMAT=json
MYTOOL_NO_COLOR=1

# Special variables
MYTOOL_HOME              # Override base directory
MYTOOL_DISABLE_UPDATE    # Disable auto-update checks
MYTOOL_AGENT_MODE=1      # Force agent mode
```

### Working Directory Behavior
1. Explicit paths in arguments are relative to CWD
2. Config files search order:
   - Relative to CWD first (project-specific)
   - Relative to script location (bundled configs)
   - Standard system locations
3. Use `--config=PATH` to override
4. Document this behavior clearly

## Error Handling

### Error Message Format

#### Human-Readable (default)
```
Error: Failed to parse configuration file
  File: /home/user/.config/mytool/config.yaml
  Line: 42
  Reason: Unexpected token ':'
  
Try 'mytool --help' for more information.
```

#### Machine-Readable (--format=json)
```json
{
  "error": {
    "code": "CONFIG_PARSE_ERROR",
    "message": "Failed to parse configuration file",
    "details": {
      "file": "/home/user/.config/mytool/config.yaml",
      "line": 42,
      "column": 15,
      "token": ":"
    },
    "help": "https://docs.example.com/errors/CONFIG_PARSE_ERROR"
  }
}
```

### Error Categories
- **Usage Errors**: Invalid flags, missing arguments
- **Input Errors**: Malformed data, missing files
- **Runtime Errors**: Network failures, resource exhaustion
- **Configuration Errors**: Invalid config, missing required settings
- **Permission Errors**: Insufficient privileges

## AI Agent Considerations

### Auto-Detection of Agent Mode
Trigger agent mode when:
- Non-interactive terminal (`!isatty()`)
- CI environment variable set
- Streams are merged (stdout==stderr)
- `MYTOOL_AGENT_MODE=1` environment variable
- `--format=json` or other structured format requested

### Agent Mode Behavior
- No progress indicators or spinners
- No colors or text formatting
- Structured output preferred
- No interactive prompts (fail instead)
- Deterministic output ordering
- Include metadata in structured output

### Recommended Agent Invocation
```bash
# Explicit agent-friendly invocation
mytool [command] \
  --format=json \
  --no-progress \
  --no-color \
  --batch

# Or via environment
export MYTOOL_AGENT_MODE=1
mytool [command]
```

### Help for Agents
```bash
# Machine-readable help
mytool --help --format=json

# List available commands/flags
mytool --list-commands
mytool --list-flags
mytool subcommand --list-flags

# Generate shell completions
mytool --generate-completion bash|zsh|fish
```

## MCP and Advanced AI Tool Usage

### MCP (Model Context Protocol) Integration

MCP enables AI assistants like Claude to interact with external tools and data sources through a standardized protocol.

#### MCP Server Implementation
```bash
# MCP server mode
mytool mcp-server
mytool --mcp-mode
mytool --mcp-port=8080
mytool --mcp-socket=/tmp/mytool.sock

# MCP configuration
--mcp-manifest=/etc/mytool/mcp-manifest.json
--mcp-capabilities="read,write,execute"
--mcp-rate-limit=100/min
```

#### MCP Tool Manifest
```json
{
  "name": "mytool",
  "version": "1.0.0",
  "description": "Tool description for AI agents",
  "tools": [
    {
      "name": "process_data",
      "description": "Process data with specified options",
      "inputSchema": {
        "type": "object",
        "properties": {
          "input": {"type": "string"},
          "format": {"type": "string", "enum": ["json", "csv"]},
          "verbose": {"type": "boolean"}
        },
        "required": ["input"]
      },
      "outputSchema": {
        "type": "object",
        "properties": {
          "result": {"type": "string"},
          "metadata": {"type": "object"}
        }
      }
    }
  ],
  "prompts": [
    {
      "name": "analyze",
      "description": "Analyze data and provide insights",
      "arguments": ["data_type", "depth"]
    }
  ]
}
```

### Claude-Specific Optimizations

#### Structured Communication
```bash
# Claude-friendly output modes
--output-style=claude-optimized
--claude-mode=on
--ai-assistant-mode=claude

# Features for Claude's capabilities
--markdown-output         # Claude handles markdown well
--code-fence-output       # Wrap code in proper fences
--structured-errors       # JSON errors Claude can parse
--semantic-sections       # Clear section boundaries
```

#### Context Window Management
```bash
# Help Claude manage context efficiently
--summary-mode            # Provide summaries for long outputs
--chunk-size=4000        # Break output into Claude-friendly chunks
--include-metadata       # Add helpful context metadata
--context-markers        # Add clear start/end markers

# Progressive disclosure
--detail-level=overview|standard|detailed
--expand-on-request      # Provide expansion commands
```

#### Error Context for Claude
```json
{
  "error": {
    "code": "PARSE_ERROR",
    "message": "Failed to parse input",
    "context": {
      "line": 42,
      "column": 15,
      "suggestion": "Check for missing closing bracket",
      "documentation": "https://docs.example.com/parse-errors",
      "examples": ["Valid: {\"key\": \"value\"}", "Invalid: {\"key\": \"value\""]
    }
  }
}
```

### AI Agent Interaction Patterns

#### Conversational Mode
```bash
# Support multi-turn interactions
mytool chat
mytool --interactive-ai-mode
--conversation-id=abc123
--maintain-context
--context-file=/tmp/context.json

# State management for conversations
--save-state=/tmp/state.json
--restore-state=/tmp/state.json
--stateless  # Explicit stateless mode
```

#### Explanation Mode
```bash
# Help AI understand what happened
--explain             # Explain what the tool did
--explain-level=basic|technical|implementation
--show-reasoning      # Show decision logic
--trace-decisions     # Output decision trace

# Example output with explanation
{
  "result": "processed",
  "explanation": "File was parsed as CSV due to .csv extension",
  "steps": ["Detected format", "Validated structure", "Processed rows"],
  "decisions": [
    {"point": "format_detection", "choice": "csv", "reason": "file_extension"}
  ]
}
```

#### Learning Support
```bash
# Help AI learn from interactions
--provide-examples
--show-patterns
--include-best-practices
--suggest-alternatives

# Feedback incorporation
--feedback-mode
--accept-corrections
--learn-from-errors
```

### Tool Chaining for AI Agents

#### Pipeline Metadata
```bash
# Pass context between tools for AI coordination
--pipeline-metadata='{"step": 1, "total": 3, "previous": "parse"}'
--chain-context=/tmp/chain-context.json
--emit-chain-events

# Tool coordination
--wait-for-ready
--signal-completion
--pass-through-metadata
```

#### Capability Discovery
```bash
# Help AI discover what the tool can do
mytool capabilities --format=json
mytool capabilities --category=data-processing
mytool capabilities --required-inputs
mytool capabilities --example-usage

# Output format
{
  "capabilities": [
    {
      "name": "csv_processing",
      "description": "Process CSV files",
      "required_inputs": ["file"],
      "optional_inputs": ["delimiter", "headers"],
      "example": "mytool process --format=csv input.csv"
    }
  ]
}
```

### AI Safety Features

#### Confirmation Requirements
```bash
# Require confirmation for destructive operations
--ai-requires-confirmation
--confirmation-token=abc123
--dry-run-first  # Always dry-run before actual execution

# Sandbox mode for AI
--ai-sandbox=strict
--readonly-mode
--simulation-mode
```

#### Audit Trail for AI Actions
```bash
# Enhanced logging for AI interactions
--ai-audit-log=/var/log/ai-actions.log
--log-ai-context
--log-decision-path

# Audit log format
{
  "timestamp": "2024-01-01T12:00:00Z",
  "ai_agent": "claude-3",
  "action": "delete_file",
  "dry_run": false,
  "confirmation_token": "abc123",
  "context": {"conversation_id": "xyz", "user": "alice"}
}
```

## Specialized Aliases and Mode Conventions

### Purpose-Specific Binary Names

Rather than requiring users (especially AI agents) to remember complex flag combinations, provide purpose-specific binary names that act as pre-configured aliases.

#### AI/Agent Mode Aliases
```bash
# Primary tool with agent-optimized defaults
mytool-ai           # Alias for: mytool --format=json --no-progress --no-color --batch
mytool-agent        # Same as above
mytool-mcp          # MCP server mode: mytool --mcp-mode --format=json

# Claude-specific optimization
mytool-claude       # mytool --format=json --chunk-size=4000 --markdown --explain

# API mode for programmatic usage
mytool-api          # mytool --format=json --no-interactive --strict-errors
```

#### Environment-Specific Aliases
```bash
# Development vs Production
mytool-dev          # mytool --verbose --debug --unsafe --no-cache
mytool-prod         # mytool --quiet --safe --cached --audit-log
mytool-staging      # mytool --verbose --safe --metrics

# Safety levels
mytool-safe         # mytool --dry-run --confirm --backup-first
mytool-unsafe       # mytool --force --no-confirm --no-backup
mytool-readonly     # mytool --read-only --no-side-effects
```

#### Use-Case Aliases
```bash
# Common scenarios
mytool-quick        # mytool --fast --no-validation --parallel=max
mytool-careful      # mytool --validate --check-twice --sequential
mytool-batch        # mytool --batch --no-interactive --log-file
mytool-interactive  # mytool --interactive --color --progress

# Output-specific
mytool-json         # mytool --format=json --no-headers
mytool-csv          # mytool --format=csv --headers
mytool-pretty       # mytool --format=table --color --borders
```

### Naming Convention Patterns

#### Suffix-Based Pattern
```bash
# Format: {tool}-{mode}
mytool-ai           # AI/agent mode
mytool-cli          # Explicit CLI mode
mytool-gui          # GUI mode (if applicable)
mytool-web          # Web interface
mytool-daemon       # Daemon/service mode
```

#### Prefix-Based Pattern
```bash
# Format: {mode}-{tool}
ai-mytool           # AI-optimized version
safe-mytool         # Safety-first version
fast-mytool         # Performance-optimized
debug-mytool        # Debug-enabled version
```

#### Abbreviation Pattern
```bash
# Short versions for common use
mt                  # mytool (base)
mta                 # mytool-ai
mtd                 # mytool-debug
mtp                 # mytool-prod
```

### Implementation Strategies

#### Symlink-Based Aliases
```bash
#!/usr/bin/env bash
# Detect how we were called
BINARY_NAME="$(basename "$0")"

case "$BINARY_NAME" in
    mytool-ai|mytool-agent)
        exec mytool --format=json --no-progress --no-color --batch "$@"
        ;;
    mytool-safe)
        exec mytool --dry-run --confirm --backup-first "$@"
        ;;
    mytool-dev)
        exec mytool --verbose --debug --unsafe "$@"
        ;;
    *)
        # Normal execution
        ;;
esac

# Installation
ln -s mytool /usr/local/bin/mytool-ai
ln -s mytool /usr/local/bin/mytool-safe
```

#### Built-in Alias Detection
```python
import sys
import os

def detect_mode():
    binary_name = os.path.basename(sys.argv[0])
    
    MODE_FLAGS = {
        'mytool-ai': ['--format=json', '--no-progress', '--no-color', '--batch'],
        'mytool-agent': ['--format=json', '--no-progress', '--no-color', '--batch'],
        'mytool-dev': ['--verbose', '--debug', '--unsafe'],
        'mytool-prod': ['--quiet', '--safe', '--audit-log'],
    }
    
    if binary_name in MODE_FLAGS:
        # Prepend mode-specific flags
        sys.argv[1:1] = MODE_FLAGS[binary_name]
    
    return binary_name.replace('mytool-', '')
```

#### Wrapper Script Pattern
```bash
# /usr/local/bin/mytool-ai
#!/usr/bin/env bash
exec mytool \
    --format=json \
    --no-progress \
    --no-color \
    --batch \
    --error-format=structured \
    --no-interactive \
    "$@"
```

### Discovery and Documentation

#### Listing Available Aliases
```bash
# Show all available aliases
mytool --list-aliases
mytool aliases

# Output format
Available command aliases:
  mytool-ai      : AI/agent-optimized mode (json, no-progress, batch)
  mytool-safe    : Safe mode with confirmations and dry-run
  mytool-dev     : Development mode with debug output
  mytool-prod    : Production mode with audit logging
  
To install aliases: mytool install-aliases [--system|--user]
```

#### Help Integration
```bash
# Alias-aware help
mytool-ai --help
# Shows: "Running in AI mode with: --format=json --no-progress --batch"

# Mode-specific help
mytool --help-mode=ai
mytool --help-mode=production
```

#### Man Page Sections
```man
.SH COMMAND ALIASES
The following specialized commands are available as optimized aliases:

.TP
.B mytool-ai, mytool-agent
Optimized for AI agents and automation. Equivalent to:
mytool --format=json --no-progress --no-color --batch

.TP
.B mytool-safe
Safety-first mode with confirmations. Equivalent to:
mytool --dry-run --confirm --backup-first
```

### Configuration File Support

#### Alias Definitions in Config
```yaml
# ~/.config/mytool/aliases.yaml
aliases:
  ai:
    description: "AI/agent-optimized mode"
    flags:
      - --format=json
      - --no-progress
      - --no-color
      - --batch
      - --structured-errors
  
  dev:
    description: "Development mode"
    flags:
      - --verbose
      - --debug
      - --hot-reload
    env:
      MYTOOL_ENV: development
      
  prod:
    description: "Production mode"
    flags:
      - --quiet
      - --safe
    env:
      MYTOOL_ENV: production
```

#### Dynamic Alias Creation
```bash
# Create custom alias
mytool alias create my-custom \
    --flags="--format=json,--verbose" \
    --description="My custom configuration"

# Use custom alias
mytool --alias=my-custom process data.txt
mytool-my-custom process data.txt  # After installation
```

### Standard Alias Sets

#### Minimal Set (Required)
```bash
mytool           # Standard invocation
mytool-ai        # AI/agent mode
mytool-help      # Extended help system
```

#### Common Set (Recommended)
```bash
mytool           # Standard
mytool-ai        # AI/agent mode
mytool-safe      # Safe mode
mytool-dev       # Development
mytool-prod      # Production
mytool-batch     # Batch processing
```

#### Extended Set (Optional)
```bash
mytool-debug     # Debug mode
mytool-quiet     # Quiet mode
mytool-verbose   # Verbose mode
mytool-json      # JSON output
mytool-csv       # CSV output
mytool-readonly  # Read-only mode
mytool-offline   # Offline mode
mytool-local     # Local-only mode
```

### Platform-Specific Considerations

#### Windows Aliases
```batch
REM mytool-ai.cmd
@echo off
mytool --format=json --no-progress --no-color --batch %*

REM PowerShell alias
function mytool-ai { mytool --format=json --no-progress --no-color --batch $args }
```

#### Shell Function Aliases
```bash
# In .bashrc/.zshrc
mytool-ai() {
    mytool --format=json --no-progress --no-color --batch "$@"
}

# Completion support
complete -F _mytool mytool-ai
```

### Alias Behavior Rules

#### Flag Precedence
```bash
# Alias flags are defaults, can be overridden
mytool-ai --format=xml  # Overrides json format
mytool-safe --force     # Overrides dry-run

# Some flags should be immutable in aliases
mytool-prod --debug     # Warning: debug flag ignored in production mode
```

#### Environment Detection
```bash
# Auto-select alias based on environment
if [ -n "$CI" ]; then
    exec mytool-ai "$@"  # CI environment
elif [ -n "$PRODUCTION" ]; then
    exec mytool-prod "$@"  # Production
else
    exec mytool "$@"     # Default
fi
```

#### Composition Rules
```bash
# Allow alias chaining
mytool-ai-safe       # Combines AI and safe modes
mytool-dev-verbose   # Dev mode with extra verbosity

# Conflict resolution
mytool-safe-unsafe   # Error: Conflicting modes
```

### Testing Aliases

#### Alias-Specific Tests
```bash
# Test each alias
for alias in mytool mytool-ai mytool-safe mytool-dev; do
    echo "Testing $alias..."
    $alias test-command
    assert_exit_code 0
done

# Verify flag application
output=$(mytool-ai status 2>&1)
assert_json "$output"
assert_no_color "$output"
```

#### Equivalence Testing
```bash
# Ensure alias equals flags
diff <(mytool-ai process) \
     <(mytool --format=json --no-progress --no-color --batch process)
```

### Migration and Deprecation

#### Introducing New Aliases
```bash
# Gradual rollout
mytool experimental-alias  # Phase 1: Experimental
mytool-new --preview       # Phase 2: Preview
mytool-new                 # Phase 3: Stable

# Announcement in help
"New: mytool-ai alias now available for agent usage"
```

#### Deprecating Old Aliases
```bash
# Deprecation warnings
mytool-old
# WARNING: 'mytool-old' is deprecated and will be removed in v3.0
# Please use 'mytool-new' instead
```

### Documentation Requirements

#### README Section
```markdown
## Quick Start

### For Humans
```bash
mytool process file.txt
```

### For AI Agents / Automation
```bash
mytool-ai process file.txt
```

### For Development
```bash
mytool-dev --hot-reload watch src/
```

### For Production
```bash
mytool-prod --audit-log process data/
```
```

#### Dedicated Alias Guide
```markdown
# Alias Usage Guide

## When to Use Each Alias

| Alias | Use Case | Key Features |
|-------|----------|--------------|
| `mytool` | Interactive human use | Colors, progress bars, prompts |
| `mytool-ai` | AI agents, CI/CD | JSON output, no interaction |
| `mytool-safe` | Critical operations | Dry-run, confirmations, backups |
| `mytool-dev` | Development work | Debug output, hot-reload |
| `mytool-prod` | Production systems | Audit logs, metrics, quiet |
```

### Best Practices

1. **Keep aliases discoverable** - List in --help, provide --list-aliases
2. **Make AI mode obvious** - Use `-ai` or `-agent` suffix consistently
3. **Document flag combinations** - Show exact flags each alias applies
4. **Allow override** - Let users override alias defaults when needed
5. **Version aliases** - Maintain backward compatibility
6. **Test all aliases** - Include in test suites
7. **Provide installation helper** - `mytool install-aliases`
8. **Consider shell completion** - Extend completion to aliases
9. **Monitor alias usage** - Track which aliases are most used
10. **Keep aliases focused** - Each alias should have a clear, single purpose

## Signal Handling

### Standard Signal Behavior

#### Core Signals
```bash
SIGINT (2)    # Ctrl-C: Graceful interruption
SIGTERM (15)  # Graceful termination request
SIGQUIT (3)   # Ctrl-\: Quit with core dump
SIGHUP (1)    # Hangup: Reload configuration
SIGUSR1 (10)  # User-defined: Often toggle debug
SIGUSR2 (12)  # User-defined: Often rotate logs
SIGPIPE (13)  # Broken pipe: Handle gracefully
SIGALRM (14)  # Timer expired
SIGCHLD (17)  # Child process status change
```

#### SIGINT (Ctrl-C) Handling
```bash
# Graceful interruption pattern
handle_sigint() {
    echo "Interrupt received, cleaning up..." >&2
    cleanup_temp_files
    save_progress
    exit 130  # 128 + 2 (SIGINT)
}
trap handle_sigint INT

# Progressive interruption
First Ctrl-C:  "Gracefully stopping... (press again to force)"
Second Ctrl-C: "Force stopping..."
Third Ctrl-C:  Immediate termination
```

#### EOF (Ctrl-D) Handling
```bash
# Ctrl-D sends EOF, not a signal
# Proper handling in interactive mode:
while IFS= read -r line; do
    process_line "$line"
done
# Ctrl-D here ends input gracefully

# Detection and response
if [ -t 0 ]; then  # Interactive terminal
    echo "Press Ctrl-D or type 'exit' to quit"
fi
```

#### SIGHUP Configuration Reload
```bash
# Standard SIGHUP behavior
handle_sighup() {
    echo "Reloading configuration..." >&2
    reload_config
    reopen_log_files  # Important for log rotation
    reset_connections
}
trap handle_sighup HUP

# Usage
kill -HUP $(pidof mytool)  # Reload config
```

### Signal Safety

#### Critical Section Protection
```bash
# Prevent interruption during critical operations
critical_section_start() {
    trap '' INT TERM  # Ignore signals
    CRITICAL=1
}

critical_section_end() {
    CRITICAL=0
    trap handle_sigint INT
    trap handle_sigterm TERM
    # Process any pending signals
    if [ -n "$PENDING_SIGNAL" ]; then
        kill -s "$PENDING_SIGNAL" $$
    fi
}
```

#### Cleanup Guarantees
```bash
# Ensure cleanup happens
cleanup() {
    local exit_code=$?
    set +e  # Don't exit on errors during cleanup
    
    # Remove temp files
    rm -f "$TMPFILE"
    
    # Release locks
    flock -u 9 2>/dev/null
    
    # Restore terminal settings
    stty "$SAVED_STTY" 2>/dev/null
    
    # Kill child processes
    jobs -p | xargs kill 2>/dev/null
    
    exit $exit_code
}
trap cleanup EXIT INT TERM
```

### Signal Propagation

#### Child Process Management
```bash
# Propagate signals to children
handle_signal() {
    local signal=$1
    # Send signal to process group
    kill -$signal 0  # 0 means current process group
    wait  # Wait for children to terminate
}

# Start processes in same group
set -m  # Enable job control
mytool &
child_pid=$!
```

#### Background Job Handling
```bash
# Proper daemon signal handling
--daemon
--pidfile=/var/run/mytool.pid
--signal-forward  # Forward signals to workers

# Signal commands
mytool signal reload    # Send SIGHUP
mytool signal stop      # Send SIGTERM
mytool signal kill      # Send SIGKILL
mytool signal status    # Check if responding
```

## Side Effects and Idempotency

### Idempotent Operations

#### Design Principles
```bash
# Idempotent by default
mytool apply config.yaml    # Safe to run multiple times
mytool ensure state         # Ensures desired state
mytool sync                 # Brings to desired state

# Explicit non-idempotent operations
mytool append --force       # Clearly non-idempotent
mytool increment counter    # Obviously stateful
```

#### Idempotency Patterns
```bash
# CREATE OR UPDATE (upsert)
mytool resource create-or-update
mytool resource apply  # Kubernetes-style

# CHECK THEN ACT
if ! mytool check condition; then
    mytool apply fix
fi

# DECLARATIVE over IMPERATIVE
# Good: Declare desired state
mytool set state=ready
# Bad: Imperative action
mytool make-ready
```

### Side Effect Management

#### Side Effect Declaration
```bash
# Explicit side effect documentation
--side-effects=none|local|network|system
--affects="filesystem,database"
--modifies="user_data,cache"

# Query for side effects
mytool describe-effects command
mytool --dry-run --show-effects
```

#### Transaction Boundaries
```bash
# Atomic operations
mytool transaction begin
mytool operation1
mytool operation2
mytool transaction commit|rollback

# Automatic transactions
--transactional  # Wrap in transaction
--atomic        # All or nothing
--isolation=read-committed|serializable
```

### Pure Functions vs Side Effects

#### Pure Operations
```bash
# Pure functions - no side effects
mytool validate input.json      # Only checks
mytool parse --stdout           # Output only
mytool calculate hash          # Deterministic
mytool transform --no-write    # In-memory only

# Markers for pure operations
--read-only
--no-side-effects
--pure
--immutable
```

#### Controlled Side Effects
```bash
# Explicit side effect boundaries
mytool --effects-log=/tmp/effects.log
mytool --track-changes
mytool --change-summary

# Effect isolation
--sandbox-writes=/tmp/sandbox
--virtual-filesystem
--mock-network
```

### State Management

#### Stateless Design
```bash
# Stateless operations
--no-state
--no-cache
--no-history
--ephemeral

# All state in arguments
mytool process --state-file=/tmp/state.json
mytool --context=/tmp/context
```

#### State Versioning
```bash
# Version state for rollback
--state-version=3
--state-backup
--state-history=10

# State migration
mytool state upgrade
mytool state rollback --to-version=2
mytool state validate
```

## One-off Scripts and Ad-hoc Tools

### Rapid Development Patterns

#### Script Header Template
```bash
#!/usr/bin/env bash
# Description: Quick script to process data
# Usage: ./script.sh [options] input output
# Author: Your Name
# Date: 2024-01-01
# Requirements: jq, curl, gnu-sed

set -euo pipefail  # Strict mode
IFS=$'\n\t'        # Safe Internal Field Separator

# Configuration
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly SCRIPT_NAME="$(basename "$0")"
readonly VERSION="0.1.0-adhoc"

# Default values
DRY_RUN=${DRY_RUN:-0}
VERBOSE=${VERBOSE:-0}
DEBUG=${DEBUG:-0}
```

#### Self-contained Scripts
```bash
# Embed dependencies
embed_jq() {
    # Embedded minimal jq implementation
    python3 -c "import json, sys; ..."
}

# Check/install dependencies
ensure_deps() {
    command -v jq &>/dev/null || {
        echo "Installing jq..." >&2
        curl -sL https://... | sudo tar -xz -C /usr/local/bin
    }
}

# Bootstrap function
bootstrap() {
    ensure_deps
    setup_environment
    validate_inputs "$@"
}
```

### Ad-hoc Tool Conventions

#### Naming for Temporary Tools
```bash
# Clear temporary nature
./tmp-analyze-logs.sh
./adhoc-data-migration.py
./one-off-cleanup.rb
./quick-fix-dates.awk

# Version in name for one-offs
./migrate-data-20240101.sh
./emergency-patch-v2.py
```

#### Progressive Enhancement
```bash
# Start simple
grep "ERROR" *.log | wc -l

# Evolve to script
#!/bin/bash
# count-errors.sh
grep "ERROR" "$@" | wc -l

# Add features gradually
#!/bin/bash
# count-errors.sh v2
pattern=${1:-ERROR}
shift
grep -c "$pattern" "$@"

# Know when to rewrite properly
# If script exceeds 100 lines, consider proper tool
```

### Quick Validation

#### Built-in Tests
```bash
# Self-test function
self_test() {
    echo "Running self-test..." >&2
    test_function_1 || return 1
    test_function_2 || return 1
    echo "Self-test passed" >&2
}

# Run if --test flag
if [[ "${1:-}" == "--test" ]]; then
    self_test
    exit $?
fi
```

#### Inline Assertions
```bash
# Simple assertions
assert() {
    if ! "$@"; then
        echo "Assertion failed: $*" >&2
        exit 1
    fi
}

assert [ -f "$input_file" ]
assert [ -n "$required_var" ]
assert command -v required_tool
```

## Script Testing

### Testing Frameworks

#### BATS (Bash Automated Testing System)
```bash
# test/script.bats
@test "processes valid input" {
    run ./script.sh valid.txt
    [ "$status" -eq 0 ]
    [[ "$output" =~ "Success" ]]
}

@test "fails on invalid input" {
    run ./script.sh invalid.txt
    [ "$status" -ne 0 ]
    [[ "$output" =~ "Error" ]]
}

# Run tests
bats test/*.bats
```

#### ShellCheck Integration
```bash
# Static analysis
shellcheck script.sh
shellcheck -S error script.sh  # Only errors
shellcheck -e SC2086 script.sh # Exclude specific

# In CI/CD
find . -name "*.sh" -exec shellcheck {} \;
```

### Test Patterns

#### Table-Driven Tests
```bash
# Define test cases
test_cases="
input1.txt:expected1.txt:0
input2.txt:expected2.txt:0
error.txt::1
"

# Run test cases
while IFS=: read -r input expected exit_code; do
    actual=$(./script.sh "$input" 2>&1)
    status=$?
    assert_equals "$status" "$exit_code"
    if [ -n "$expected" ]; then
        assert_equals "$actual" "$(cat "$expected")"
    fi
done <<< "$test_cases"
```

#### Mock External Dependencies
```bash
# Create mock commands
PATH="$PWD/mocks:$PATH"

# mocks/curl
#!/bin/bash
echo '{"mocked": true}'
exit 0

# Test with mocks
test_with_mock_api() {
    export PATH="$PWD/mocks:$PATH"
    ./script.sh --api-call
    # Verify mock was called
    assert_equals "$(cat mocks/curl.log)" "called"
}
```

### Coverage and Quality

#### Code Coverage
```bash
# Using kcov for shell scripts
kcov coverage/ ./script.sh test-input
kcov coverage/ bats test.bats

# Using bashcov (Ruby gem)
bashcov ./run-tests.sh
```

#### Test Quality Metrics
```bash
# Test completeness checklist
- [ ] Happy path tested
- [ ] Error conditions tested
- [ ] Edge cases covered
- [ ] Signal handling tested
- [ ] Cleanup tested
- [ ] Idempotency verified
- [ ] Performance benchmarked
- [ ] Security validated
```

## Profiling and Performance Analysis

### Profiling Tools

#### Time-based Profiling
```bash
# Built-in time command
time mytool process large-file
/usr/bin/time -v mytool process  # Verbose output

# Custom timing
--profile-time
--time-report
--show-duration

# Output format
{
  "total_time": "10.5s",
  "phases": {
    "parsing": "2.1s",
    "processing": "7.2s",
    "output": "1.2s"
  }
}
```

#### Memory Profiling
```bash
# Memory tracking
--profile-memory
--memory-report
--heap-profile=/tmp/heap.prof

# Using valgrind
valgrind --tool=massif mytool process
ms_print massif.out.*

# Built-in tracking
mytool --track-allocations
mytool --show-memory-stats
```

### Performance Benchmarking

#### Benchmark Mode
```bash
# Benchmark operations
mytool benchmark
mytool benchmark --iterations=1000
mytool benchmark --duration=60s
mytool benchmark --compare=baseline.json

# Benchmark output
{
  "operations_per_second": 1523,
  "average_latency_ms": 0.65,
  "p50_ms": 0.5,
  "p95_ms": 1.2,
  "p99_ms": 2.3
}
```

#### Micro-benchmarks
```bash
# Component benchmarking
mytool bench parse
mytool bench serialize
mytool bench hash

# Comparative benchmarks
mytool bench --compare algorithm1,algorithm2
mytool bench --format=csv > results.csv
```

### Optimization Helpers

#### Profiling Integration
```bash
# CPU profiling
--cpu-profile=/tmp/cpu.prof
--profile-format=pprof|callgrind|json

# Sampling profiler
--sample-rate=100  # Hz
--profile-symbols
```

#### Bottleneck Detection
```bash
# Identify slow operations
--trace-slow-ops=100ms
--warn-slow-queries
--debug-performance

# Automatic optimization suggestions
mytool analyze-performance trace.json
mytool suggest-optimizations
```

## Fuzzing and Security Testing

### Fuzzing Support

#### Built-in Fuzz Testing
```bash
# Fuzz mode
mytool fuzz
mytool fuzz --input-dir=corpus/
mytool fuzz --max-runs=10000
mytool fuzz --timeout=1h

# Fuzz targets
mytool fuzz parse
mytool fuzz deserialize
mytool fuzz validate
```

#### AFL/LibFuzzer Integration
```bash
# AFL mode
mytool --afl-mode < test_input

# LibFuzzer harness
extern "C" int LLVMFuzzerTestOneInput(data, size) {
    mytool_parse(data, size);
    return 0;
}

# Corpus management
mytool fuzz-corpus add input.txt
mytool fuzz-corpus minimize
mytool fuzz-corpus merge corpus1/ corpus2/
```

### Security Testing Features

#### Input Validation Testing
```bash
# Test input boundaries
mytool security-test --check=overflow
mytool security-test --check=injection
mytool security-test --check=path-traversal

# Payload testing
mytool test --payloads=security-payloads.txt
mytool test --fuzz-strings
```

#### Vulnerability Scanning
```bash
# Self-scan for vulnerabilities
mytool security-scan
mytool security-scan --verbose
mytool security-scan --report=sarif

# Dependency scanning
mytool scan-dependencies
mytool check-cves
```

### Sanitizer Support

#### Address Sanitizer (ASAN)
```bash
# Build with ASAN
./configure --enable-asan
make CFLAGS="-fsanitize=address"

# Runtime options
ASAN_OPTIONS=detect_leaks=1 mytool
```

#### Other Sanitizers
```bash
# Thread Sanitizer
--enable-tsan
TSAN_OPTIONS=halt_on_error=1

# Undefined Behavior Sanitizer
--enable-ubsan
UBSAN_OPTIONS=print_stacktrace=1

# Memory Sanitizer
--enable-msan
```

## Load Testing

### Load Generation

#### Built-in Load Testing
```bash
# Load test mode
mytool load-test
mytool load-test --rate=100/s
mytool load-test --concurrent=50
mytool load-test --duration=5m
mytool load-test --ramp-up=30s

# Load patterns
--load-pattern=constant|ramp|spike|wave
--spike-multiplier=10
--wave-period=60s
```

#### Scenario-based Testing
```bash
# Load scenarios
mytool load-test --scenario=normal-day
mytool load-test --scenario=black-friday
mytool load-test --scenario=ddos-simulation

# Scenario definition
{
  "name": "black-friday",
  "phases": [
    {"duration": "5m", "rate": "100/s"},
    {"duration": "10m", "rate": "1000/s"},
    {"duration": "5m", "rate": "500/s"}
  ]
}
```

### Stress Testing

#### Resource Stress
```bash
# Stress different resources
mytool stress --cpu=80%
mytool stress --memory=4G
mytool stress --io=1000iops
mytool stress --network=100mbps

# Chaos engineering
mytool chaos --kill-random-process
mytool chaos --network-partition
mytool chaos --disk-full
```

#### Breaking Point Detection
```bash
# Find limits
mytool find-limits
mytool find-limits --resource=connections
mytool find-limits --increase-rate=10%

# Output
{
  "breaking_point": {
    "connections": 10234,
    "requests_per_second": 5621,
    "memory_gb": 7.8
  }
}
```

### Load Test Reporting

#### Real-time Metrics
```bash
# Live dashboard
mytool load-test --live-dashboard
mytool load-test --metrics-port=8080

# Stream metrics
mytool load-test --stream-metrics | \
    mytool-dashboard
```

#### Report Generation
```bash
# Generate reports
mytool load-test --report=html
mytool load-test --report=json > results.json

# Report contents
- Response time distribution
- Throughput over time
- Error rate analysis
- Resource utilization
- Bottleneck identification
```

### Performance Regression Detection

#### Baseline Comparison
```bash
# Establish baseline
mytool perf-baseline create
mytool perf-baseline save --name=v2.0

# Compare against baseline
mytool perf-test --compare-baseline=v2.0
mytool perf-test --fail-on-regression=5%
```

#### CI/CD Integration
```bash
# Performance gates
mytool perf-test --gate-p95=100ms
mytool perf-test --gate-throughput=1000rps
mytool perf-test --export=junit.xml

# Trend analysis
mytool perf-trend --days=30
mytool perf-trend --plot=performance.png
```

## Logging

### Log Levels
Following syslog severity levels:
- `TRACE` (0) - Detailed trace information
- `DEBUG` (1) - Debug-level messages
- `INFO` (2) - Informational messages
- `NOTICE` (3) - Normal but significant events
- `WARNING` (4) - Warning conditions
- `ERROR` (5) - Error conditions
- `CRITICAL/FATAL` (6) - Critical conditions

### Logging Configuration
```bash
# Command-line flags
--log-level=debug|info|warn|error|critical
--log-file=/path/to/logfile
--log-format=text|json|logfmt
--log-timestamp=rfc3339|unix|iso8601
--log-max-size=100M
--log-max-files=10
--log-compress

# Environment variables
MYTOOL_LOG_LEVEL=debug
MYTOOL_LOG_FILE=/var/log/mytool.log
MYTOOL_LOG_FORMAT=json
```

### Structured Log Format
```json
{
  "timestamp": "2024-01-01T12:00:00.000Z",
  "level": "ERROR",
  "component": "parser",
  "correlation_id": "abc-123-def-456",
  "message": "Failed to parse input file",
  "context": {
    "file": "input.txt",
    "line": 42,
    "error_code": "PARSE_ERROR",
    "user": "alice",
    "duration_ms": 1234
  },
  "stack_trace": "..."
}
```

### Log Rotation Support
- Respond to `SIGHUP` for external rotation
- Built-in rotation with size/age limits
- Compress rotated logs
- Clean up old logs based on count or age

### Audit Logging
Separate audit logs from operational logs:
```bash
--audit-log=/var/log/mytool-audit.log
--audit-format=json
--audit-level=all|reads|writes|admin
```

## Versioning and Updates

### Version Format
Follow Semantic Versioning (SemVer):
```
MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]

Examples:
1.0.0
2.3.1
3.0.0-beta.1
4.1.2-rc.2+build.123
```

### Version Display
```bash
$ mytool --version
mytool version 2.3.1
Built: 2024-01-01T12:00:00Z
Commit: abc123def
Go version: go1.21.5
OS/Arch: linux/amd64
```

### Self-Update Mechanism
```bash
# Check for updates
mytool self-update --check

# Update to latest
mytool self-update

# Update to specific version
mytool self-update --version=2.3.1

# Update channels
mytool self-update --channel=stable|beta|nightly

# Disable auto-update checks
export MYTOOL_DISABLE_UPDATE_CHECK=1
```

### Backward Compatibility
```bash
# Version requirements
--require-version=">=2.0.0"
--require-version="~2.3.0"  # 2.3.x

# Format version negotiation
--input-format-version=1
--output-format-version=2

# Deprecation warnings
WARNING: Flag --old-flag is deprecated and will be removed in v3.0.0
         Use --new-flag instead.
```

### Command Wrapping/Proxying
When wrapping existing commands:
```bash
# Preserve original interface
wrapper [wrapper-options] -- [original-command-options]

# Support version selection
MYTOOL_EXEC=/usr/local/bin/mytool-v2 wrapper command

# Pass through unknown flags
wrapper --wrapper-opt command --unknown-flag
```

## Security

### Secret Handling
```bash
# Never accept secrets as command-line arguments (visible in ps)
# Bad:
mytool --password=secret

# Good:
mytool --password-file=/secure/path
mytool --password-env=MY_PASSWORD_VAR
echo "secret" | mytool --password-stdin
```

### Secure Defaults
- Fail closed (deny by default)
- Principle of least privilege
- Validate all inputs
- Sanitize outputs
- Use secure communication (TLS/HTTPS)

### Permission Controls
```bash
# Check permissions
mytool check-permissions

# Run with reduced privileges
mytool --drop-privileges command

# Sandbox/isolation
mytool --sandbox=strict|partial|none
```

## Performance and Resources

### Resource Limits
```bash
# Timeouts
--timeout=30s              # Overall timeout
--connect-timeout=5s       # Connection timeout
--read-timeout=10s         # Read timeout

# Memory limits
--max-memory=1G
--max-file-size=100M

# Concurrency
--parallel=4               # Worker threads/processes
--rate-limit=100/min      # Rate limiting
--max-connections=10      # Connection pool size

# CPU
--max-cpu-percent=80
--nice=10                 # Process priority
```

### Caching
```bash
# Cache control
--cache-dir=~/.cache/mytool
--cache-ttl=1h
--no-cache
--clear-cache

# Cache headers support (for HTTP)
--cache-control="max-age=3600"
--if-modified-since="..."
```

### Progress Reporting
```bash
# Progress indicators
--progress=auto|always|never|json
--progress-format=bar|percent|dots|json

# JSON progress format
{"type":"progress","operation":"download","current":50,"total":100,"percent":50}
```

## Testing and Validation

### Built-in Testing
```bash
# Self-test
mytool self-test
mytool self-test --verbose

# Validate configuration
mytool validate-config /path/to/config

# Validate input
mytool validate input.txt

# Health check
mytool health
mytool health --format=json
```

### Dry Run Levels
```bash
--dry-run              # Basic dry run
--dry-run=parse        # Parse and validate only
--dry-run=validate     # Parse and deep validation
--dry-run=plan         # Show execution plan
--dry-run=execute      # Execute but don't commit
```

### Debug Support
```bash
# Debug output
--debug                # Enable debug output
--trace               # Enable trace output
--debug-file=debug.log

# Profiling
--profile=cpu|memory|trace
--profile-output=/tmp/profile

# Metrics
--metrics             # Show execution metrics
--metrics-format=json|prometheus
```

## Internationalization

### Locale Detection
```bash
# Environment variable precedence
$LC_ALL > $LC_MESSAGES > $LANG

# Command-line override
--locale=en_US.UTF-8
--language=es
--encoding=UTF-8
```

### Message Catalogs
```bash
# Standard locations
/usr/share/locale/{locale}/LC_MESSAGES/mytool.mo
~/.local/share/locale/{locale}/LC_MESSAGES/mytool.mo

# Catalog selection
--message-catalog=/path/to/catalog
--fallback-locale=en_US
```

### Localization Features
```bash
# Date/time formatting
--date-format=iso8601|locale|custom
--timezone=UTC|local|America/New_York

# Number formatting
--decimal-separator=.
--thousands-separator=,

# Currency
--currency=USD|EUR|locale
```

### Right-to-Left Support
```bash
# RTL language handling
--text-direction=auto|ltr|rtl
--bidi-mode=on|off

# Terminal width calculation
--terminal-encoding=utf8|utf16
```

### Translation Management
```bash
# Extract translatable strings
mytool i18n extract --output=messages.pot

# Update translations
mytool i18n update --locale=es

# Compile message catalogs
mytool i18n compile --check
```

## Multi-tenancy and Profiles

### Profile Management
```bash
# Profile selection
--profile=production|staging|dev|custom
--context=team-a|team-b|personal

# Environment variables
MYTOOL_PROFILE=production
MYTOOL_CONTEXT=team-a

# Configuration files per profile
~/.config/mytool/profiles/production.yaml
~/.config/mytool/profiles/staging.yaml
```

### Profile Commands
```bash
# List profiles
mytool profile list
mytool profile show production

# Create/edit profiles
mytool profile create new-profile
mytool profile edit staging
mytool profile clone production production-copy

# Switch default profile
mytool profile use staging
mytool profile set-default production

# Delete profile
mytool profile delete old-profile
```

### Multi-account Support
```bash
# Account/tenant selection
--account=12345
--tenant=customer-a
--organization=acme-corp

# Credential management per account
--credentials-profile=account-prod
--assume-role=arn:aws:iam::12345:role/admin
```

### Workspace Isolation
```bash
# Workspace directories
--workspace=/path/to/workspace
--workspace-name=project-a

# Isolated configurations
~/.config/mytool/workspaces/project-a/
~/.cache/mytool/workspaces/project-a/
~/.local/share/mytool/workspaces/project-a/
```

## Backup and Recovery

### Backup Operations
```bash
# Create backups
mytool backup create
mytool backup create --incremental
mytool backup create --full
mytool backup create --name="before-upgrade"

# Backup options
--backup-dir=/path/to/backups
--compression=gzip|bzip2|xz|none
--encryption=aes256|none
--retention-days=30
--backup-format=tar|zip|custom
```

### Restore Operations
```bash
# List available backups
mytool backup list
mytool backup list --verbose

# Restore from backup
mytool backup restore --from=backup-id
mytool backup restore --from-file=/path/to/backup.tar.gz
mytool backup restore --point-in-time="2024-01-01T12:00:00Z"

# Restore options
--restore-dir=/path/to/restore
--verify-integrity
--dry-run
--force
```

### Automatic Backups
```bash
# Before dangerous operations
--backup-before-change
--auto-backup=on|off
--backup-on-upgrade

# Scheduled backups
mytool backup schedule --cron="0 2 * * *"
mytool backup schedule list
mytool backup schedule delete backup-job-1
```

### Backup Verification
```bash
# Verify backup integrity
mytool backup verify backup-id
mytool backup verify --all

# Test restore
mytool backup test-restore backup-id --target=/tmp/test
```

## Distributed Operations

### Node Management
```bash
# Target specific nodes
--node=node1.example.com
--nodes=node1,node2,node3
--node-group=web-servers

# Broadcast operations
--broadcast              # Run on all nodes
--parallel-nodes=5       # Concurrent node operations
```

### Cluster Operations
```bash
# Cluster awareness
--cluster=production
--cluster-config=/etc/mytool/cluster.yaml

# Leader election
--require-leader        # Only run on leader node
--leader-election-timeout=30s

# Quorum requirements
--quorum=3              # Minimum nodes required
--quorum-percentage=51  # Percentage of nodes required
```

### Distributed Coordination
```bash
# Locking
--distributed-lock=operation-name
--lock-timeout=60s
--lock-wait            # Wait for lock availability

# Consensus
--consensus-timeout=30s
--consistency-level=strong|eventual|quorum
```

### Health and Discovery
```bash
# Service discovery
--discovery-service=consul|etcd|zookeeper
--discovery-endpoint=http://consul:8500
--service-name=mytool-worker

# Health checks
mytool cluster health
mytool cluster health --format=json
mytool node status node1
```

### Distributed Tracing
```bash
# Trace context propagation
--trace-id=abc123def456
--parent-span-id=span789
--baggage="user=alice,tenant=acme"

# Correlation
--correlation-id=request-123
--causation-id=parent-operation-456
```

## Data Migration

### Migration Commands
```bash
# Run migrations
mytool migrate up              # Run all pending migrations
mytool migrate up --to=v3      # Migrate to specific version
mytool migrate down            # Rollback last migration
mytool migrate down --to=v1    # Rollback to specific version

# Migration options
--dry-run                      # Preview changes
--force                        # Skip confirmation
--timeout=5m                   # Migration timeout
--batch-size=1000              # For data migrations
```

### Migration Management
```bash
# List migrations
mytool migrate list            # Show all migrations
mytool migrate status          # Show current status
mytool migrate pending         # Show pending migrations
mytool migrate history         # Show applied migrations

# Create new migration
mytool migrate create "add_user_table"
mytool migrate generate        # Auto-generate from schema diff
```

### Schema Versioning
```bash
# Version tracking
--schema-version=3
--target-version=5
--compatibility-mode=backward|forward|full

# Version info storage
~/.local/share/mytool/schema_version
/var/lib/mytool/migrations/
```

### Data Transformation
```bash
# Transform during migration
--transform-script=/path/to/transform.js
--validation-script=/path/to/validate.js

# Rollback support
--reversible                  # Ensure migration is reversible
--checkpoint-interval=1000    # Checkpoint every N records
--resume-from-checkpoint      # Resume failed migration
```

### Migration Safety
```bash
# Pre-migration checks
mytool migrate validate
mytool migrate check-compatibility

# Backup before migration
--backup-before-migration
--verify-backup

# Post-migration validation
--validate-after
--integrity-check
--compare-checksums
```

## Observability and Hooks

### Lifecycle Hooks
```bash
# Hook execution points
--pre-init-hook=/path/to/script
--post-init-hook=/path/to/script
--pre-execute-hook=/path/to/script
--post-execute-hook=/path/to/script
--error-hook=/path/to/script
--finally-hook=/path/to/script

# Hook configuration
--hook-timeout=30s
--hook-env="KEY=value,FOO=bar"
--hook-dir=/etc/mytool/hooks.d/
```

### Hook Context
```bash
# Environment variables passed to hooks
MYTOOL_COMMAND="process"
MYTOOL_ARGS="file1.txt file2.txt"
MYTOOL_EXIT_CODE="0"
MYTOOL_DURATION_MS="1234"
MYTOOL_USER="alice"
MYTOOL_TIMESTAMP="2024-01-01T12:00:00Z"
MYTOOL_CORRELATION_ID="abc-123"
```

### Event Streaming
```bash
# Event emission
--emit-events
--event-stream=/tmp/mytool-events
--event-format=json|cloudevents

# Event types
{"event":"command.start","command":"process","timestamp":"..."}
{"event":"progress","percent":50,"timestamp":"..."}
{"event":"command.complete","duration_ms":1234,"timestamp":"..."}
```

### Monitoring Integration
```bash
# Metrics emission
--statsd-host=localhost:8125
--prometheus-port=9090
--openmetrics-endpoint=/metrics

# Custom metrics
--custom-metric="operations.count:1|c"
--custom-tag="environment:production"
```

### Notification Hooks
```bash
# Notification channels
--notify-on-success="webhook:https://..."
--notify-on-failure="email:ops@example.com"
--notify-on-start="slack:#deployments"

# Notification configuration
--notification-template=/path/to/template
--notification-timeout=10s
--notification-retry=3
```

## Resilience Patterns

### Retry Mechanisms
```bash
# Retry configuration
--retry=3                      # Number of retries
--retry-delay=1s              # Initial delay
--retry-max-delay=30s         # Maximum delay
--retry-multiplier=2          # Exponential backoff multiplier
--retry-jitter=0.1            # Randomization factor

# Retry strategies
--retry-strategy=exponential|linear|fixed|fibonacci
--retry-on-errors="TIMEOUT,UNAVAILABLE"
--no-retry-on="PERMISSION_DENIED,INVALID_ARGUMENT"
```

### Circuit Breaker
```bash
# Circuit breaker configuration
--circuit-breaker=on
--circuit-breaker-threshold=5     # Failures before opening
--circuit-breaker-timeout=60s     # Time before half-open
--circuit-breaker-success-threshold=2  # Successes to close

# Circuit breaker states
mytool circuit-breaker status
mytool circuit-breaker reset
mytool circuit-breaker force-open
```

### Bulkhead Pattern
```bash
# Resource isolation
--max-concurrent-operations=10
--operation-timeout=30s
--queue-size=100
--reject-on-full-queue

# Pool management
--connection-pool-size=20
--connection-pool-timeout=5s
--connection-idle-timeout=300s
```

### Timeout Management
```bash
# Hierarchical timeouts
--global-timeout=5m           # Overall operation timeout
--connect-timeout=10s         # Connection establishment
--request-timeout=30s         # Individual request timeout
--idle-timeout=60s           # Idle connection timeout

# Timeout policies
--timeout-policy=fail|retry|fallback
--fallback-result="default"
```

### Graceful Degradation
```bash
# Fallback options
--fallback-mode=cache|default|simplified
--use-cache-on-error
--cache-stale-timeout=1h

# Feature flags for degradation
--disable-features="analytics,recommendations"
--essential-only
--degraded-mode
```

### Health Checks
```bash
# Health check configuration
--health-check-interval=30s
--health-check-timeout=5s
--health-check-retries=3

# Startup/liveness/readiness
mytool health startup
mytool health liveness
mytool health readiness

# Dependencies health
mytool health dependencies
mytool health dependency database
```

## Batch Processing

### Batch Configuration
```bash
# Batch size control
--batch-size=100              # Records per batch
--batch-timeout=30s          # Max time per batch
--batch-memory-limit=100M    # Memory limit per batch

# Batch strategies
--batch-strategy=size|time|memory|adaptive
--adaptive-batch-min=10
--adaptive-batch-max=1000
```

### Checkpoint and Resume
```bash
# Checkpointing
--checkpoint=on
--checkpoint-interval=1000    # Records between checkpoints
--checkpoint-file=/tmp/checkpoint.json
--checkpoint-backend=file|redis|database

# Resume from failure
--resume                      # Resume from last checkpoint
--resume-from=checkpoint-id   # Resume from specific checkpoint
--skip-processed             # Skip already processed records
```

### Parallel Batch Processing
```bash
# Parallelization
--parallel-batches=4          # Concurrent batch processing
--batch-queue-size=10         # Queue size for batches
--preserve-order             # Maintain input order

# Work distribution
--partition-key=user_id       # Partition data for processing
--partition-count=10          # Number of partitions
--partition-strategy=hash|range|round-robin
```

### Batch Error Handling
```bash
# Error strategies
--batch-error-strategy=fail|continue|isolate
--max-batch-errors=10         # Failures before stopping
--error-file=/tmp/errors.json # Log failed records

# Dead letter queue
--dead-letter-file=/tmp/dlq.json
--retry-dead-letters
--dead-letter-max-retries=3
```

### Progress Tracking
```bash
# Progress reporting
--progress-interval=100       # Report every N records
--progress-format=bar|percent|json|detailed
--progress-file=/tmp/progress.json

# Estimation
--estimate-completion        # Show estimated time remaining
--show-rate                 # Show processing rate
--show-statistics           # Show batch statistics
```

### Batch Transaction Support
```bash
# Transaction control
--batch-transaction=on       # Wrap batch in transaction
--transaction-size=1000      # Records per transaction
--two-phase-commit          # For distributed transactions

# Rollback handling
--rollback-on-error         # Rollback batch on any error
--savepoint-interval=100    # Create savepoints
```

### Memory Management
```bash
# Memory optimization
--stream-mode               # Process without loading all data
--buffer-size=10M          # I/O buffer size
--gc-interval=1000         # Force garbage collection
--memory-profile           # Track memory usage

# Spill to disk
--spill-threshold=1G       # Memory limit before spilling
--spill-directory=/tmp/spill
--compress-spill          # Compress spilled data
```

## Plugin/Extension System

### Plugin Management
```bash
# Plugin locations
~/.config/mytool/plugins/     # User plugins
/usr/share/mytool/plugins/    # System plugins

# Plugin commands
mytool plugin list
mytool plugin install <name>
mytool plugin remove <name>
mytool plugin update [name]
mytool plugin info <name>
```

### Plugin Interface
```yaml
# plugin.yaml
name: my-plugin
version: 1.0.0
api_version: 2
description: "My awesome plugin"
author: "Developer Name"
commands:
  - name: new-command
    description: "Adds new-command to mytool"
    executable: bin/new-command
hooks:
  - pre-execute
  - post-execute
```

## Telemetry and Metrics

### Metrics Collection
```bash
# Opt-in telemetry
--telemetry=on|off
--telemetry-endpoint=https://...

# Metrics output
--metrics
--metrics-format=json|prometheus|statsd
--metrics-output=/path/to/file
```

### OpenTelemetry Support
```bash
# Tracing
--trace
--trace-endpoint=http://jaeger:14268
--trace-sample-rate=0.1

# Standard OTEL environment variables
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector:4317
OTEL_SERVICE_NAME=mytool
```

## Documentation Standards

### Inline Help Structure
```
Usage: mytool [options] command [arguments]

Brief description of what mytool does.

Commands:
  init        Initialize a new project
  build       Build the project
  deploy      Deploy to production
  help        Show help for a command

Options:
  -h, --help              Show this help message
  -v, --verbose          Increase output verbosity
  --version              Show version information

Examples:
  mytool init myproject
  mytool build --parallel=4
  mytool deploy --environment=staging

For more help on a command, use: mytool help <command>
```

### Man Page Sections
```
NAME
SYNOPSIS
DESCRIPTION
OPTIONS
COMMANDS
EXIT STATUS
ENVIRONMENT
FILES
EXAMPLES
DIAGNOSTICS
BUGS
AUTHOR
SEE ALSO
```

## Examples and Patterns

### Complete Tool Example Structure
```bash
#!/usr/bin/env bash

mytool() {
    # Parse global options
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help) show_help; exit 0 ;;
            --version) show_version; exit 0 ;;
            -v|--verbose) VERBOSE=$((VERBOSE + 1)) ;;
            --format=*) FORMAT="${1#*=}" ;;
            --) shift; break ;;
            -*) error "Unknown option: $1" ;;
            *) break ;;
        esac
        shift
    done

    # Detect mode
    if [ -t 0 ] && [ -t 1 ]; then
        INTERACTIVE=1
    else
        INTERACTIVE=0
    fi

    # Handle commands
    COMMAND=$1
    shift

    case $COMMAND in
        init|build|deploy) run_command "$COMMAND" "$@" ;;
        *) error "Unknown command: $COMMAND" ;;
    esac
}
```

### Multi-Language Consistent Interface
Regardless of implementation language, maintain consistent:
- Flag names and syntax
- Output formats
- Exit codes
- Environment variables
- Configuration file formats

### Testing Checklist
- [ ] All exit codes are meaningful
- [ ] stdout contains only pipeable data
- [ ] stderr used for all diagnostics
- [ ] Works in non-interactive mode
- [ ] Handles Ctrl+C gracefully (SIGINT)
- [ ] Responds to SIGTERM for clean shutdown
- [ ] Validates all inputs
- [ ] Provides helpful error messages
- [ ] Documentation matches implementation
- [ ] Works with `set -euo pipefail`

## Summary

These conventions prioritize:
1. **Predictability** - Consistent behavior across tools
2. **Composability** - Tools work well together
3. **Discoverability** - Easy to learn and explore
4. **Agent-Friendliness** - Works well with automation
5. **Human-Friendliness** - Pleasant interactive experience
6. **Robustness** - Handles errors gracefully
7. **Security** - Secure by default
8. **Performance** - Efficient resource usage
9. **Maintainability** - Easy to debug and extend
