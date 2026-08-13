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
