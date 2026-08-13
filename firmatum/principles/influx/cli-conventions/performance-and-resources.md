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

