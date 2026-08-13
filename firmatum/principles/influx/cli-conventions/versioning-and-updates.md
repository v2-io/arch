## Versioning and Updates

### Version Format
Follow Semantic Versioning (SemVer):
```
MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]

Examples:
1.0.0
2.3.1
3.0.0-beta.1
4.1.2-rc.2+build.123
```

### Version Display
```bash
$ mytool --version
mytool version 2.3.1
Built: 2024-01-01T12:00:00Z
Commit: abc123def
Go version: go1.21.5
OS/Arch: linux/amd64
```

### Self-Update Mechanism
```bash
# Check for updates
mytool self-update --check

# Update to latest
mytool self-update

# Update to specific version
mytool self-update --version=2.3.1

# Update channels
mytool self-update --channel=stable|beta|nightly

# Disable auto-update checks
export MYTOOL_DISABLE_UPDATE_CHECK=1
```

### Backward Compatibility
```bash
# Version requirements
--require-version=">=2.0.0"
--require-version="~2.3.0"  # 2.3.x

# Format version negotiation
--input-format-version=1
--output-format-version=2

# Deprecation warnings
WARNING: Flag --old-flag is deprecated and will be removed in v3.0.0
         Use --new-flag instead.
```

### Command Wrapping/Proxying
When wrapping existing commands:
```bash
# Preserve original interface
wrapper [wrapper-options] -- [original-command-options]

# Support version selection
MYTOOL_EXEC=/usr/local/bin/mytool-v2 wrapper command

# Pass through unknown flags
wrapper --wrapper-opt command --unknown-flag
```

