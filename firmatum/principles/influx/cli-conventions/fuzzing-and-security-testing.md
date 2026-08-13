## Fuzzing and Security Testing

### Fuzzing Support

#### Built-in Fuzz Testing
```bash
# Fuzz mode
mytool fuzz
mytool fuzz --input-dir=corpus/
mytool fuzz --max-runs=10000
mytool fuzz --timeout=1h

# Fuzz targets
mytool fuzz parse
mytool fuzz deserialize
mytool fuzz validate
```

#### AFL/LibFuzzer Integration
```bash
# AFL mode
mytool --afl-mode < test_input

# LibFuzzer harness
extern "C" int LLVMFuzzerTestOneInput(data, size) {
    mytool_parse(data, size);
    return 0;
}

# Corpus management
mytool fuzz-corpus add input.txt
mytool fuzz-corpus minimize
mytool fuzz-corpus merge corpus1/ corpus2/
```

### Security Testing Features

#### Input Validation Testing
```bash
# Test input boundaries
mytool security-test --check=overflow
mytool security-test --check=injection
mytool security-test --check=path-traversal

# Payload testing
mytool test --payloads=security-payloads.txt
mytool test --fuzz-strings
```

#### Vulnerability Scanning
```bash
# Self-scan for vulnerabilities
mytool security-scan
mytool security-scan --verbose
mytool security-scan --report=sarif

# Dependency scanning
mytool scan-dependencies
mytool check-cves
```

### Sanitizer Support

#### Address Sanitizer (ASAN)
```bash
# Build with ASAN
./configure --enable-asan
make CFLAGS="-fsanitize=address"

# Runtime options
ASAN_OPTIONS=detect_leaks=1 mytool
```

#### Other Sanitizers
```bash
# Thread Sanitizer
--enable-tsan
TSAN_OPTIONS=halt_on_error=1

# Undefined Behavior Sanitizer
--enable-ubsan
UBSAN_OPTIONS=print_stacktrace=1

# Memory Sanitizer
--enable-msan
```

