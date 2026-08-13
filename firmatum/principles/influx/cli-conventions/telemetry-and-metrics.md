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

