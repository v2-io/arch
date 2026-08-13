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

