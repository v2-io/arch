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

