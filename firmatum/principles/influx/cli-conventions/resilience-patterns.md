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

