## Multi-tenancy and Profiles

### Profile Management
```bash
# Profile selection
--profile=production|staging|dev|custom
--context=team-a|team-b|personal

# Environment variables
MYTOOL_PROFILE=production
MYTOOL_CONTEXT=team-a

# Configuration files per profile
~/.config/mytool/profiles/production.yaml
~/.config/mytool/profiles/staging.yaml
```

### Profile Commands
```bash
# List profiles
mytool profile list
mytool profile show production

# Create/edit profiles
mytool profile create new-profile
mytool profile edit staging
mytool profile clone production production-copy

# Switch default profile
mytool profile use staging
mytool profile set-default production

# Delete profile
mytool profile delete old-profile
```

### Multi-account Support
```bash
# Account/tenant selection
--account=12345
--tenant=customer-a
--organization=acme-corp

# Credential management per account
--credentials-profile=account-prod
--assume-role=arn:aws:iam::12345:role/admin
```

### Workspace Isolation
```bash
# Workspace directories
--workspace=/path/to/workspace
--workspace-name=project-a

# Isolated configurations
~/.config/mytool/workspaces/project-a/
~/.cache/mytool/workspaces/project-a/
~/.local/share/mytool/workspaces/project-a/
```

