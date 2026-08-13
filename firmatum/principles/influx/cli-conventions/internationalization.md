## Internationalization

### Locale Detection
```bash
# Environment variable precedence
$LC_ALL > $LC_MESSAGES > $LANG

# Command-line override
--locale=en_US.UTF-8
--language=es
--encoding=UTF-8
```

### Message Catalogs
```bash
# Standard locations
/usr/share/locale/{locale}/LC_MESSAGES/mytool.mo
~/.local/share/locale/{locale}/LC_MESSAGES/mytool.mo

# Catalog selection
--message-catalog=/path/to/catalog
--fallback-locale=en_US
```

### Localization Features
```bash
# Date/time formatting
--date-format=iso8601|locale|custom
--timezone=UTC|local|America/New_York

# Number formatting
--decimal-separator=.
--thousands-separator=,

# Currency
--currency=USD|EUR|locale
```

### Right-to-Left Support
```bash
# RTL language handling
--text-direction=auto|ltr|rtl
--bidi-mode=on|off

# Terminal width calculation
--terminal-encoding=utf8|utf16
```

### Translation Management
```bash
# Extract translatable strings
mytool i18n extract --output=messages.pot

# Update translations
mytool i18n update --locale=es

# Compile message catalogs
mytool i18n compile --check
```

