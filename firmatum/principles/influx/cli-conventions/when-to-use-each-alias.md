## When to Use Each Alias

| Alias | Use Case | Key Features |
|-------|----------|--------------|
| `mytool` | Interactive human use | Colors, progress bars, prompts |
| `mytool-ai` | AI agents, CI/CD | JSON output, no interaction |
| `mytool-safe` | Critical operations | Dry-run, confirmations, backups |
| `mytool-dev` | Development work | Debug output, hot-reload |
| `mytool-prod` | Production systems | Audit logs, metrics, quiet |
```

### Best Practices

1. **Keep aliases discoverable** - List in --help, provide --list-aliases
2. **Make AI mode obvious** - Use `-ai` or `-agent` suffix consistently
3. **Document flag combinations** - Show exact flags each alias applies
4. **Allow override** - Let users override alias defaults when needed
5. **Version aliases** - Maintain backward compatibility
6. **Test all aliases** - Include in test suites
7. **Provide installation helper** - `mytool install-aliases`
8. **Consider shell completion** - Extend completion to aliases
9. **Monitor alias usage** - Track which aliases are most used
10. **Keep aliases focused** - Each alias should have a clear, single purpose

