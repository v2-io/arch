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

