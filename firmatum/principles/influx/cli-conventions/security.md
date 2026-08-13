## Security

### Secret Handling
```bash
# Never accept secrets as command-line arguments (visible in ps)
# Bad:
mytool --password=secret

# Good:
mytool --password-file=/secure/path
mytool --password-env=MY_PASSWORD_VAR
echo "secret" | mytool --password-stdin
```

### Secure Defaults
- Fail closed (deny by default)
- Principle of least privilege
- Validate all inputs
- Sanitize outputs
- Use secure communication (TLS/HTTPS)

### Permission Controls
```bash
# Check permissions
mytool check-permissions

# Run with reduced privileges
mytool --drop-privileges command

# Sandbox/isolation
mytool --sandbox=strict|partial|none
```

