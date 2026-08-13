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

