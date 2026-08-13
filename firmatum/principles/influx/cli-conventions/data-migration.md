## Data Migration

### Migration Commands
```bash
# Run migrations
mytool migrate up              # Run all pending migrations
mytool migrate up --to=v3      # Migrate to specific version
mytool migrate down            # Rollback last migration
mytool migrate down --to=v1    # Rollback to specific version

# Migration options
--dry-run                      # Preview changes
--force                        # Skip confirmation
--timeout=5m                   # Migration timeout
--batch-size=1000              # For data migrations
```

### Migration Management
```bash
# List migrations
mytool migrate list            # Show all migrations
mytool migrate status          # Show current status
mytool migrate pending         # Show pending migrations
mytool migrate history         # Show applied migrations

# Create new migration
mytool migrate create "add_user_table"
mytool migrate generate        # Auto-generate from schema diff
```

### Schema Versioning
```bash
# Version tracking
--schema-version=3
--target-version=5
--compatibility-mode=backward|forward|full

# Version info storage
~/.local/share/mytool/schema_version
/var/lib/mytool/migrations/
```

### Data Transformation
```bash
# Transform during migration
--transform-script=/path/to/transform.js
--validation-script=/path/to/validate.js

# Rollback support
--reversible                  # Ensure migration is reversible
--checkpoint-interval=1000    # Checkpoint every N records
--resume-from-checkpoint      # Resume failed migration
```

### Migration Safety
```bash
# Pre-migration checks
mytool migrate validate
mytool migrate check-compatibility

# Backup before migration
--backup-before-migration
--verify-backup

# Post-migration validation
--validate-after
--integrity-check
--compare-checksums
```

