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

