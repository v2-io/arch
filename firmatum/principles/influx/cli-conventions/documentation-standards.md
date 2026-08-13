## Documentation Standards

### Inline Help Structure
```
Usage: mytool [options] command [arguments]

Brief description of what mytool does.

Commands:
  init        Initialize a new project
  build       Build the project
  deploy      Deploy to production
  help        Show help for a command

Options:
  -h, --help              Show this help message
  -v, --verbose          Increase output verbosity
  --version              Show version information

Examples:
  mytool init myproject
  mytool build --parallel=4
  mytool deploy --environment=staging

For more help on a command, use: mytool help <command>
```

### Man Page Sections
```
NAME
SYNOPSIS
DESCRIPTION
OPTIONS
COMMANDS
EXIT STATUS
ENVIRONMENT
FILES
EXAMPLES
DIAGNOSTICS
BUGS
AUTHOR
SEE ALSO
```

