## Examples and Patterns

### Complete Tool Example Structure
```bash
#!/usr/bin/env bash

mytool() {
    # Parse global options
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help) show_help; exit 0 ;;
            --version) show_version; exit 0 ;;
            -v|--verbose) VERBOSE=$((VERBOSE + 1)) ;;
            --format=*) FORMAT="${1#*=}" ;;
            --) shift; break ;;
            -*) error "Unknown option: $1" ;;
            *) break ;;
        esac
        shift
    done

    # Detect mode
    if [ -t 0 ] && [ -t 1 ]; then
        INTERACTIVE=1
    else
        INTERACTIVE=0
    fi

    # Handle commands
    COMMAND=$1
    shift

    case $COMMAND in
        init|build|deploy) run_command "$COMMAND" "$@" ;;
        *) error "Unknown command: $COMMAND" ;;
    esac
}
```

### Multi-Language Consistent Interface
Regardless of implementation language, maintain consistent:
- Flag names and syntax
- Output formats
- Exit codes
- Environment variables
- Configuration file formats

### Testing Checklist
- [ ] All exit codes are meaningful
- [ ] stdout contains only pipeable data
- [ ] stderr used for all diagnostics
- [ ] Works in non-interactive mode
- [ ] Handles Ctrl+C gracefully (SIGINT)
- [ ] Responds to SIGTERM for clean shutdown
- [ ] Validates all inputs
- [ ] Provides helpful error messages
- [ ] Documentation matches implementation
- [ ] Works with `set -euo pipefail`

