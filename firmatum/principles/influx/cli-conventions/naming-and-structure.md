## Naming and Structure

### Tool Naming
- **Lowercase, hyphenated**: `my-tool`, `data-processor`
- **Avoid**: Underscores (`data_processor`), CamelCase (`MyTool`)
- **Be descriptive but concise**: 2-15 characters ideal

### Command Structure
```bash
# Simple tools
tool [options] [arguments]

# Complex tools with subcommands
tool subcommand [options] [arguments]

# Verb-noun pattern
git clone
docker run
aws s3 cp
```

