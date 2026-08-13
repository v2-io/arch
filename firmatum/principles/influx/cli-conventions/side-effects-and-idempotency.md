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

