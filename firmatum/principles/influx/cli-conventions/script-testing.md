## Script Testing

### Testing Frameworks

#### BATS (Bash Automated Testing System)
```bash
# test/script.bats
@test "processes valid input" {
    run ./script.sh valid.txt
    [ "$status" -eq 0 ]
    [[ "$output" =~ "Success" ]]
}

@test "fails on invalid input" {
    run ./script.sh invalid.txt
    [ "$status" -ne 0 ]
    [[ "$output" =~ "Error" ]]
}

# Run tests
bats test/*.bats
```

#### ShellCheck Integration
```bash
# Static analysis
shellcheck script.sh
shellcheck -S error script.sh  # Only errors
shellcheck -e SC2086 script.sh # Exclude specific

# In CI/CD
find . -name "*.sh" -exec shellcheck {} \;
```

### Test Patterns

#### Table-Driven Tests
```bash
# Define test cases
test_cases="
input1.txt:expected1.txt:0
input2.txt:expected2.txt:0
error.txt::1
"

# Run test cases
while IFS=: read -r input expected exit_code; do
    actual=$(./script.sh "$input" 2>&1)
    status=$?
    assert_equals "$status" "$exit_code"
    if [ -n "$expected" ]; then
        assert_equals "$actual" "$(cat "$expected")"
    fi
done <<< "$test_cases"
```

#### Mock External Dependencies
```bash
# Create mock commands
PATH="$PWD/mocks:$PATH"

# mocks/curl
#!/bin/bash
echo '{"mocked": true}'
exit 0

# Test with mocks
test_with_mock_api() {
    export PATH="$PWD/mocks:$PATH"
    ./script.sh --api-call
    # Verify mock was called
    assert_equals "$(cat mocks/curl.log)" "called"
}
```

### Coverage and Quality

#### Code Coverage
```bash
# Using kcov for shell scripts
kcov coverage/ ./script.sh test-input
kcov coverage/ bats test.bats

# Using bashcov (Ruby gem)
bashcov ./run-tests.sh
```

#### Test Quality Metrics
```bash
# Test completeness checklist
- [ ] Happy path tested
- [ ] Error conditions tested
- [ ] Edge cases covered
- [ ] Signal handling tested
- [ ] Cleanup tested
- [ ] Idempotency verified
- [ ] Performance benchmarked
- [ ] Security validated
```

