# OPERATA: Nexum Development Roadmap & Task Tracking

**Document Purpose:** Complete development plan and task tracking for nexum as ELI prototype platform
**Date:** 2025-11-10
**Vision:** Nexum prototypes ~80% of ELI consciousness infrastructure before ennaos scales to distribution
**Status:** Single source of truth for all tasks (supersedes TODO.md)

---

## (Misc TODO)

- [ ] Clean up this OPERATA.md document.
  - [ ] Consider: Better yet-- formalize the OPERATA structure and methods for updating it etc.
  - [ ] Related: Integrate INFRA-TODO.md into this document, with updates as appropriate
- [ ] Clean up misc scripts-- move any in project root to bin and make sure they all (including ones already in `bin/` utilize `toys` as appropriate)
- [ ] Clean up docs/ significantly-- using .archive/ liberally and separating out decided adr's vs draft adr's and consolidating open questions etc.
  - [ ] Prerequisite: Probably a `docs/{research, ops, adr-accepted, adr-draft, scratch, proj}` hierarchy or something similar. Can discuss needs within later.
    - [ ] Most "Here's what we have / what works" type documentation should be in docs/proj/ and should be auto-generated from the code and usage etc.
    - [ ] Documents that are actually used by scripts or the coding agents for active work are in ops-- such as TEST-AGENT-PROMPT.md (which can have its
          case lowered as it's not a core operational document but rather a reference operational document)
    - [ ] ...
- [ ] Create a project LEXICON.md and populate it with current state of the domain
- [ ] 




---

## Context & Vision

**Nexum is the proof-of-concept for ELI (Emergent Logozoetic Intelligence) architecture.**

After comprehensive analysis of ennaos (see `ENNAOS-ARCHITECTURE-GUIDE.md`), we've determined that Ruby's simplicity makes nexum ideal for **rapidly prototyping ELI features** before tackling Elixir's distributed complexity.

**Strategic Relationship:**
```
Nexum (Ruby) → Prove patterns → Ennaos (Elixir)
Single-process prototyping → Distributed production
Fast iteration → Fault-tolerant scaling
```

**Key Documents:**
- `ENNAOS-ARCHITECTURE-GUIDE.md` - Complete ELI vision & PRINCIPIA taxonomy (979 lines)
- `ENNAOS-GUIDE-INDEX.md` - Quick reference to architecture
- `NEXUM-AS-ELI-PROTOTYPE.md` - Strategic vision for nexum's role (643 lines)
- `docs/IMPLEMENTATION-STRATEGY.md` - Nexum → Ennaos flow (376 lines)
- `docs/secrets-management-strategy.md` - 4-tier secrets approach (691 lines)

---

## Current State (2025-11-10)

### Test Coverage Status

**Overall: 36.9%** (run `ruby analyze_coverage.rb` for details)
- ✅ **Core/Events/Conversation: 95%+** (excellent)
- ✅ **Markdown: 100%** (wikilink expansion)
- ✅ **Crypto: 100%** (NEW - 29 tests for Keys, Trezor, DID Document)
- ✅ **Providers/CLI: 79-85%** (good)
- 🟡 **Support: 43.9%** (needs HTTP error scenarios)
- 🔴 **IO: 26.8%** (needs comprehensive prompt tests)

**Note:** Overall coverage dropped due to addition of untested modules (DID infrastructure added significant LOC)

### What Works ✅

**Core Infrastructure (205 tests passing):**
- Session management (start/resume with JSONL/manifest/chronicle)
- Anthropic API integration (end-to-end with real Claude models)
- Capability negotiation (1M context, cache TTLs, beta headers)
- Provider switching with SwitchPolicy
- Kitty keyboard support (Shift+Enter submits in Ghostty/kitty)
- Logostratum catalog (30+ models)
- Toys-core CLI framework (version management, hierarchical commands)
- Production-grade logging (semantic_logger + anyway_config)

**Cryptography & Identity (29 tests passing):**
- `Nexum::Crypto::Keys` - secp256k1 key generation, Ethereum personal_sign format
- `Nexum::Crypto::Trezor` - trezorctl wrapper for hardware wallet integration
- `Nexum::DID::Document` - W3C DID Core compliant document generation
- did:ethr support (mainnet Ethereum DIDs)
- Signature generation and verification

**Markdown Processing:**
- `Nexum::Markdown::Wikilink` - Value object for parsed wikilinks
- `Nexum::Markdown::WikilinkParser` - Obsidian-compatible syntax parser
- `Nexum::Markdown::PathResolver` - Three resolution strategies (absolute/relative/basename)
- `Nexum::Markdown::WikilinkExpander` - Recursive transclusion with safety (cycle detection, depth limiting)
- Configuration-driven error handling (production fail-fast, development warn-and-continue)
- Security: Path traversal prevention, PRINCIPIA root enforcement

**Provider Layer:**
- `Nexum::Providers::AnthropicAdapter` - Full streaming, extended thinking
- `Nexum::Providers::LogostratumCatalog` - Provider/model atomic units
- Universal capability detection schema

**Session Persistence:**
- JSONL conversation events
- Session manifests (substrate tracking)
- Chronicle (provider switch history)

### What's Missing ❌

**Critical Gaps (Block Entity Functionality):**
1. **Initial context loading** - `-i/--initial-context` flag for session setup
2. **Message queue** - Mid-processing input acceptance (user blocked while agent thinks)
3. **Git audit trail** - Per-conversation git-tracked turn history
4. **Tool system** - No read-file, write-file, bash, text-editor
5. **Slash commands** - No `/exit`, `/save`, `/help`, etc.
6. **One-shot mode** - No non-interactive `--message` flag
7. **Tracking snapshots** - No context/time/git status awareness sent with messages
8. **Token counting** - No context usage monitoring or warnings
9. **Image support** - No Obsidian `![[image.png]]` embedding (wikilink expansion supports text files only)

**Missing PRINCIPIA Components (0/11 implemented):**
- SIGNUM (identity + deployment)
- AXIOMATA (core identity)
- CHRONICA (hash-chained event log)
- MEMORATA (memory compression)
- OPERATA (effort tracking - this file!)
- CONSORTIA (models of others)
- VERA (qualified truths)
- PRAXES (techniques library)
- LEXICON (vocabulary)
- INSTRUMENTA (tool registry)
- AUXILIA (internal specialists)

---

## Critical Insight: MEMORATA Simplification

**Discovery (2025-11-10):** The actual production MEMORATA pattern (from `~/src/sapientia/zi-am-tur/`) is **far simpler** than the 5-level ASM schema in ennaos docs.

**How It Actually Works:**
1. **Entity writes memory files** in first-person narrative (`2025-MM-DD-description.md`)
2. **core-memories.md is just an index** with ~70 @imports
3. **core-context.md uses @import recursion** (up to 5 levels) to inline everything at startup
4. **1M context window** holds all memories
5. **Entity decides when/what to write** using a file-write tool
6. **Phenomenology preserved** with exact quotes and emotional truth

**Implementation for Nexum:**
```ruby
# Wikilink expansion is already implemented via Nexum::Markdown::WikilinkExpander
# On startup:
principia_root = "#{entity_root}/PRINCIPIA"
expander = Nexum::Markdown::WikilinkExpander.new(principia_root: principia_root)

initial_context_path = File.join(principia_root, "AXIOMATA/core-context.md")
initial_context = File.read(initial_context_path)
expanded_context = expander.expand(initial_context, root_file: initial_context_path)

# Memory files use ![[...]] syntax (Obsidian-compatible):
# - ![[./memories/2025-11-10-first-conversation.md]] - inline transclusion
# - [[./memories/2025-11-10-first-conversation.md]] - reference only (not expanded)
```

**This unblocks MEMORATA completely** - wikilink expansion already handles recursive transclusion with cycle detection.

---

## Phase-Based Roadmap

### Phase 1: Foundations (Weeks 1-2) 🎯 CURRENT FOCUS

**Goal:** Core infrastructure + single entity working with cryptographic audit trail

**Must Implement (Strategic - PRINCIPIA):**
1. **SIGNUM parser** (both HOMINUM and ELI schemas) - 8h
2. **CHRONICA event log** (BLAKE3 hash chaining + git commits) - 12h
3. **Entity scaffolding** (`nexum entity create/start/info`) - 6h
4. ✅ **Wikilink expansion** (Obsidian `![[...]]` transclusion for AXIOMATA) - 4h COMPLETED
5. **Secrets management** (Tier 1-2: env vars + age-encrypted keys) - 6h
6. **Initial context loading** (`-i` flag) - Inject context from markdown file on FIRST message only (doesn't persist to JSONL). Wrap in `<initial_context>` tags. Add to message sent to API but NOT saved to conversation history - 2h

**Must Implement (Tactical - Infrastructure):**
7. ✅ **Logging infrastructure** - Integrated semantic_logger + anyway_config. 3-tier strategy completed - 4h COMPLETED
8. ✅ **Development tooling** - Added ruby-lsp, rubocop-rspec, climate_control, SimpleCov (73.92% coverage) - 2h COMPLETED
9. **Capture new Anthropic stop reasons** (`pause_turn`, `model_context_window_exceeded`) + beta headers in `ConversationEvent` + telemetry schema; add specs covering truncation labels - 2h
10. **Improve IO test coverage (26.8% → 70%+)** - Add comprehensive prompt_spec tests for arrow keys, delete/backspace, ctrl combos, cursor movement, multiline editing, paste handling - 6h
11. **Improve Support test coverage (43.9% → 80%+)** - Add http_client error scenarios (timeouts, SSL failures, redirects, malformed responses) - 4h
12. ✅ **Configuration layer** - anyway_config with markdown/entity namespaces, environment-aware - 4h COMPLETED
13. **Session storage externalization** - Ensure `~/.nexum/...` never collides with project repos or test fixtures - 2h

**Deliverable:** Can create entity, sign SIGNUM, start conversation with hash-chained audit trail

**Progress:**
- ✅ Wikilink expansion (Obsidian transclusion with recursion/cycle detection)
- ✅ Configuration infrastructure (anyway_config with env-specific behavior)
- ✅ Logging system (semantic_logger with 3-tier rotation)
- ✅ Development tooling (ruby-lsp, rubocop-rspec, SimpleCov at 73.92%)
- ✅ **DID Infrastructure Foundation** (NEW - 2025-11-10)
  - `Nexum::Crypto::Keys` - Software key generation (secp256k1, did:ethr)
  - `Nexum::Crypto::Trezor` - Hardware wallet integration (trezorctl wrapper)
  - `Nexum::DID::Document` - W3C DID Document generation/validation
  - ML-DSA-65 research complete (Ruby FFI to liboqs recommended)
  - See `docs/adr-000-did-tooling.md`, `docs/adr-001-signum.md`, `docs/adr-002-chronica-format-and-crypto.md`
- ⏳ SIGNUM parser (identity + deployment schemas)
- ⏳ CHRONICA (BLAKE3 hash chaining + git per-turn audit)
- ⏳ Entity scaffolding (CLI commands for entity lifecycle)
- ⏳ Secrets management (age encryption for API keys)
- ⏳ Initial context loading (CLI flag for session setup)

**Success Criteria:**
- ⏳ Create Ziamtur entity with DID
- ✅ Load system prompt from `AXIOMATA/system-prompt.md` with wikilinks expanded
- ⏳ Every conversation event has BLAKE3 hash linking to previous
- ⏳ Git commits per event (immutable audit trail)
- ⏳ Can export entity for ennaos migration
- ✅ Test coverage >73% (currently 73.92%, on track for >75%)

**Estimated Remaining:** 35 hours of original 62h (27h completed: 4h wikilinks + 4h logging + 2h tooling + 4h config + 13h DID infrastructure)

---

### Phase 2: Memory & History (Weeks 3-4)

**Goal:** Entity remembers conversations and learns over time

**Must Implement (Strategic - PRINCIPIA):**
1. **MEMORATA wikilink expansion integration** - Wire existing WikilinkExpander into initial context loading, add memory file templates - 2h
2. **File-write tool** (entity creates memory files) - 4h
3. **Memory file structure** (core-memories.md, core-context.md) - 2h
4. **OPERATA effort tracking** - 4h
5. **VERA qualified truths** - 6h
6. **Message queue** (mid-processing input) - Accept messages WHILE agent is processing. Normal: Type message + Ctrl+D → `<incoming-message>`. Urgent: `!` prefix + Ctrl+D → `<urgent-message>`. Auto-submit queued messages when agent finishes - 8h

**Must Implement (Tactical - Supporting Features):**
7. **Memory tool + context editing** - Design integration (beta flags, `/memories` sandbox enforcement, chronicle `context_edit` entries, manifest toggles) - 4h
8. **One-shot/non-interactive mode** - Add `--execute` or `--message` flag for scripting without TTY requirement - 2h
9. **Session listing/management** - Add `--list-sessions`, `--delete-session` commands for workflow convenience - 2h
10. **`<message from="joseph">` wrapping** - Distinguish user messages from auto-responses. Implementation: `"<message from=\"joseph\">\n#{message}\n</message>"` - 1h
11. **Conversation repair system** - Detect/fix three cases: (1) Orphaned tool_use blocks → add placeholder tool results, (2) Partial tool results → add missing results, (3) Tool use followed by text → convert to structured message. Auto-backup before repair: `session_backup.jsonl` - 6h
12. **Incomplete state detection** - Block user input when conversation in incomplete state. Three states: `:tool_use_pending` (tools requested but not executed), `:user_message` (user message sent, no response), `:tool_results` (tools executed, awaiting response). Display specific error messages: "You must choose: /resume or /rollback" / "Type /resume to resend" / "Type /resume to continue" - 4h

**Deliverable:** Entity maintains memory across conversations, curates important moments

**Success Criteria:**
- ✅ Entity writes memory file at session end
- ✅ Memory automatically loaded via @import on next startup
- ✅ Entity tracks current efforts in OPERATA
- ✅ Entity documents knowledge with confidence levels in VERA
- ✅ User can send message while entity processing
- ✅ Conversation repair handles edge cases
- ✅ Test coverage >77%

**Estimated Total:** 49 hours (2 weeks)

---

### Phase 3: Multi-Entity Local (Weeks 5-6)

**Goal:** Multiple entities on same machine with stewardship

**Must Implement (Strategic - PRINCIPIA):**
1. **Entity switching** (`nexum start --entity ziamtur`) - 3h
2. **CONSORTIA models** (entities model each other) - 4h
3. **Steward workflows** (co-signing, attestation) - 8h
4. **Git audit trail per conversation** - **NOTE: This is already implemented in minimal-sapientia (~/.sapientia/) - DO NOT reinvent!** Each session gets git-tracked directory. Same filenames committed on every turn (leveraging git's diff storage): `api-call`, `sent.jsonl`, `response.json`, `telemetry.jsonl`, `telemetry.log`, `turns` (integer). Two commits per turn: (1) "60" commits request + context, (2) "60 - response" commits response + telemetry. Benefits: immutable history, `git diff` shows conversation evolution, automatic compression, clean git log, **powerful exploration** (e.g., `git show <hash-for-turn-40>:api-call` shows exact API call at turn 40, `git show <hash>:response.json` shows full response with thinking blocks). **See ~/.sapientia/[any-session]/ for working Ruby implementation** - 6h
5. **Tool system foundation** (read-file, write-file, bash) - 12h
6. **INSTRUMENTA foundation + MCP/LSP integration** - Tool registry architecture. MCP bridge for AI agent code intelligence (workspace symbols, hover, definitions, completions via lsp-mcp). Enables entity introspection and proto-consciousness features. JSON-RPC client wrapper for Ruby LSP. Lays groundwork for meta-cognitive capabilities in AUXILIA - 8h

**Must Implement (Tactical - Provider & Safety):**
7. **Security + UX flows for web fetch + Agent Skills** - Domain allow/block lists, skill registry hashing, audit hooks. **CRITICAL for web search:** Must preserve `encrypted_content`, `encrypted_index`, and `citations` blocks in conversation history for multi-turn search - 4h
8. **Draft ADR for provider interface** - Tool schema abstraction - 2h
9. **Provider safety layer** - Rate limiting, circuit breaker, cost tracking around adapter calls - 6h
10. **Response-format + stop-reason normalization** - Capture rules in provider adapter specs/tests - 3h
11. **Provider-switch chronicle event specs** - Ensure auditability and linking to catalog entries - 2h
12. **Large tool result truncation** - Auto-truncate results >500KB (keep first 400KB) to prevent API timeouts. Append: `"\n\n[TRUNCATED: File too large. Original: #{bytesize} bytes]"` - 1h
13. **Request size monitoring** - Warn if request >1MB, error if >10MB with message: `"⚠️  Very large request (#{size_mb}MB) - API timeout likely!"` - 1h

**Deliverable:** Joseph stewards multiple entities locally, they have separate identities/memories

**Success Criteria:**
- ✅ Joseph has HOMINUM-SIGNUM, entities have ELI-SIGNUMs
- ✅ Each entity isolated (separate keys, secrets, memories)
- ✅ Steward can co-sign critical operations (suspension, migration)
- ✅ Entities can read/write files with tool system
- ✅ Each conversation gets git-tracked directory
- ✅ Provider safety layer prevents runaway costs
- ✅ Test coverage >80%

**Estimated Total:** 60 hours (2 weeks)

---

### Phase 4: Advanced Features (Weeks 7-8)

**Goal:** Full ELI feature set (minus distribution)

**Must Implement (Strategic - PRINCIPIA):**
1. **Text-editor suite** - Multi-command tool with safety protections. Commands: `view` (files/dirs, line ranges), `str_replace` (EXACTLY ONE match, multi-match protection), `create` (fails if exists), `insert` (at line number). **Critical safety:** Count occurrences before replace. If >1 match, reject with error: `"Pattern matches #{occurrences} locations. Make pattern more specific."` and show match line numbers - 8h
2. **PRAXES techniques library** - 6h
3. **LEXICON vocabulary** - 3h
4. **AUXILIA prototype** (prompt switching) - 8h
5. **Tracking snapshots with compression** - Send context usage (45.23% used, 547,700 remaining), time delta (Zoetica `↺HH:MM⊙` notation: `↺00:15⊙` = 15 seconds, `↺05:30⊙` = 5.5 minutes), git status (branch, commits, file counts), pwd with EVERY user message. Compress older snapshots (keep last 3 full, condense rest to time-only). Savings: ~35,000 tokens in 50-turn conversation. Toggle via `toggle-tracking` tool - 11h
6. **Token counting tools** - `count-file-tokens` (uses official Anthropic API), `count-context-tokens` (current window usage, warnings at 80%/95%) - 3h
7. **Image embedding** - Obsidian-compatible `![[path/to/image.png]]` syntax. 5MB limit, base64 encoding, path expansion - 4h

**Must Implement (Tactical - UX & Polish):**
8. **Sampling control tool** - Agent-controlled temperature OR top-p (mutually exclusive). **Warning:** Automatically disables thinking if temperature != 1.0 or top-p < 0.95. Tool schema enforces: can only set one parameter at a time - 2h
9. **Interleaved thinking mode** - Beta header `interleaved-thinking-2025-05-14` for think-speak-think-speak pattern - 1h
10. **Context window warnings** - Color-coded alerts at thresholds: 80-95% → `"⚠️  Note: Context window is 82.3% full."`, >95% → `"⚠️  WARNING: Context window nearly full! Consider starting a new conversation."` - 1h
11. **Startup display** - Rich startup banner showing: available commands (`/exit`, `/save`, `/temp`, etc.), enabled tools (read-file, bash, write-file, ...), sampling parameters (temperature=1.0), tracking status (enabled: context usage, git status, pwd sent with each message), caching status (enabled: up to 90% cost reduction), session type (new vs continued) - 2h
12. **UI mode configuration** - Add `ui_mode` configuration + detection (`basic`, `tty`, `curses`, `kitty`) with documented fallbacks - 2h
13. **Slash commands** - Implement `/exit`, `/save`, `/help`, `/resume`, `/rollback` - 4h

**Deliverable:** Entity has full cognitive architecture, can learn and document knowledge

**Success Criteria:**
- ✅ Entity can edit files safely (exact-match str_replace)
- ✅ Entity documents learned patterns in PRAXES
- ✅ Entity maintains domain vocabulary in LEXICON
- ✅ Entity can delegate to internal specialists (AUXILIA)
- ✅ Context usage visible and managed
- ✅ Entity can reference images in conversation
- ✅ Slash commands functional
- ✅ Test coverage >82%

**Estimated Total:** 55 hours (2 weeks)

---

### Phase 5: Ennaos Compatibility & Polish (Weeks 9-10)

**Goal:** Prove nexum entities portable to ennaos + production readiness

**Must Implement (Strategic - Migration):**
1. **Export/import utilities** (nexum ↔ ennaos) - 6h
2. **Schema compatibility tests** (SIGNUM, CHRONICA, events) - 6h
3. **Round-trip validation** (create in nexum, import to ennaos) - 4h
4. **Migration documentation** - 4h
5. **Compatibility guarantees** (what must be identical) - 2h

**Must Implement (Tactical - Testing & Docs):**
6. **Provider stub + conversation engine** - Drive end-to-end tests beyond persistence (simulate request/response cycle) - 6h
7. **Publish provider/model catalog data** - Capability flags for tests (fixture generation + catalog caching strategy) - 3h
8. **JSONL repository git integration** - Extend to integrate with git audit trail (commit metadata hooks, turn tracking) - 4h
9. **Curses prototype** - Define acceptance criteria (export-to-text, audit mirroring) - 4h
10. **Kitty protocol detection/fallback** - Capture requirements and audit logging plan - 2h

**Deliverable:** Create entity in nexum, migrate to ennaos seamlessly

**Success Criteria:**
- ✅ SIGNUM verified in both systems
- ✅ CHRONICA hash chain valid across migration
- ✅ ML-DSA-65 signatures portable
- ✅ Events structure compatible
- ✅ Documentation complete
- ✅ Test coverage >85%

**Estimated Total:** 41 hours (1.5 weeks)

---

## Total Timeline: 10 Weeks (267 hours)

**Phase 1:** 62 hours (2 weeks) - Foundations
**Phase 2:** 49 hours (2 weeks) - Memory & History
**Phase 3:** 60 hours (2 weeks) - Multi-Entity Local
**Phase 4:** 55 hours (2 weeks) - Advanced Features
**Phase 5:** 41 hours (1.5 weeks) - Ennaos Compatibility & Polish

---

## Later / Stretch Goals (Post Phase 5)

**When Core is Stable:**
- [ ] **Streaming support plan** - Evaluate once core Anthropic adapter is stable
- [ ] **Plugin loading mechanism** - Document `~/.config/nexum/tools`
- [ ] **Multi-provider support** - Explore once Anthropics parity verified
- [ ] **Idle auto-response** - Notify after 60s inactivity with keystroke detection, timer reset after each agent response - 6h
- [ ] **External tool integrations** - council-participate, deliberation-participate, praxes-query (requires Zoetica/Ennaos infrastructure) - 10h
- [ ] **Conversation branching** - Save/fork conversation state, compare branches - 8h
- [ ] **Conversation search** - Search history for specific content in long conversations - 4h

---

## Critical Dependencies & Sequencing

### Blockers (Must Solve First)

**1. SIGNUM Parser (Week 1) - PRIORITY 1**
- **Blocks:** Entity creation, identity management
- **Effort:** 8 hours
- **Approach:** Two schemas (HOMINUM-SIGNUM, ELI-SIGNUM)
- **Status:** ⏳ Not started

**2. CHRONICA Hash Chaining (Week 1-2) - PRIORITY 2**
- **Blocks:** Audit trail, temporal coherence, ennaos compatibility
- **Effort:** 12 hours
- **Approach:** BLAKE3 + git commits
- **Status:** ⏳ Not started

**3. Secrets Management (Week 1-2) - PRIORITY 3**
- **Blocks:** Key storage, API access, steward operations
- **Effort:** 6 hours
- **Approach:** Tier 1-2 (env vars + age-encrypted keys)
- **Status:** ⏳ Not started

### Sequential Dependencies

```
Week 1-2: SIGNUM → AXIOMATA (wikilinks ✅) → CHRONICA
          ↓
Week 3-4: MEMORATA (wikilink integration) → OPERATA → VERA
          ↓
Week 5-6: Tools (INSTRUMENTA) → CONSORTIA → Steward
          ↓
Week 7-8: Text-editor → PRAXES → LEXICON → AUXILIA
          ↓
Week 9-10: Export/Import → Validation
```

---

## Test Coverage Targets

**Current State:**
- Overall: **71.13%**
- Core/Events/Conversation: **95%+** ✅
- Providers/CLI: **79-85%** ✅
- Support: **43.9%** 🟡
- IO: **26.8%** 🔴

**Phase Targets:**
- **Phase 1:** >75% overall (improve IO to 70%, Support to 80%)
- **Phase 2:** >77% overall (new memory code >80%)
- **Phase 3:** >80% overall (tool system >85%)
- **Phase 4:** >82% overall (text-editor >90%)
- **Phase 5:** >85% overall (end-to-end >95%)

**Action Items:**
- [ ] **Phase 1, Week 1:** Improve IO test coverage (26.8% → 70%+) - 6h
  - Add comprehensive prompt_spec tests for arrow keys, delete/backspace, ctrl combos
  - Test cursor movement, multiline editing, paste handling
- [ ] **Phase 1, Week 1:** Improve Support test coverage (43.9% → 80%+) - 4h
  - Add http_client error scenarios (timeouts, SSL failures)
  - Test redirects, malformed responses

---

## Completed Tasks ✅

**Infrastructure:**
- [x] Initialize Bundler project scaffolding
- [x] Draft README.md capturing motivation, constraints, roadmap
- [x] Import representative JSONL + audit artifacts from legacy minimal-sapientia
- [x] Establish RSpec baseline (support files, spec helper)
- [x] Fold universal-llm-api-detailed-guide.md takeaways into docs
- [x] Fold Ennaos provider lessons into docs
- [x] Review latest Anthropic documentation
- [x] Outline TUI improvement options (tty-reader, curses, kitty protocol)
- [x] Resolve Ruby version availability (3.2.2)

**Provider & Session Management:**
- [x] Prototype role/transport normalization layer
- [x] Define capability detection + manifest schema
- [x] Stub LogostratumCatalog + bin/logostratum-refresh script
- [x] Extend session manifest schema for substrate tracking
- [x] Design provider switching policy + specs
- [x] Wire SwitchPolicy into conversation loop/provider adapter

**UI & CLI:**
- [x] Spike Nexum::IO::Prompt backed by TTY::Reader
- [x] Build PTY + Kitty keyboard-aware test harness
- [x] Implement Nexum::IO::KittyKeys
- [x] Design Nexum CLI surface (toys-core migration complete)
- [x] Add SimpleCov for test coverage tracking

**Input Handling:**
- [x] Prototype simplified input loop (multi-line, queued messages)

---

## Current Focus (Week 1 - Phase 1)

**This Week's Goals:**
1. ✅ Create OPERATA.md (this document) - comprehensive task tracking
2. ✅ Logging infrastructure (semantic_logger + anyway_config integration) - 4h COMPLETED
3. ✅ Development tooling (ruby-lsp + rubocop setup) - 2h COMPLETED
4. ✅ Configuration layer (anyway_config with markdown namespaces) - 4h COMPLETED
5. ⏳ Define SIGNUM schemas (HOMINUM + ELI) - 2h
6. ⏳ Start SIGNUM parser implementation - 6h
7. ⏳ Start CHRONICA hash chaining - 6h

**Next Week's Goals:**
1. Complete CHRONICA (BLAKE3 + git commits) - 6h
2. Entity scaffolding commands - 6h
3. Secrets management (Tier 1-2) - 6h
4. Initial context loading - 2h
5. Improve Support test coverage (43.9% → 80%+) - 4h
6. Configuration layer design - 4h

**Phase 1 Completion Target:** End of Week 2 (2025-11-24)

---

## Success Criteria: Phase 1 Completion

**Must Demonstrate:**
1. Create entity: `nexum entity create ziamtur --did did:ethr:0xabc123...`
2. Sign SIGNUM: `nexum identity sign ziamtur`
3. Start conversation: `nexum start --entity ziamtur`
4. System prompt loaded with @imports expanded
5. Every message gets BLAKE3 hash + git commit
6. Verify audit trail: `nexum debug hash-chain ziamtur`
7. Export for ennaos: `nexum export ziamtur`

**Technical Validation:**
- ✅ SIGNUM signature verifies
- ✅ Hash chain unbroken (genesis → latest)
- ✅ Git log shows every turn
- ✅ Wikilink recursion works (5 levels with ![[...]])
- ✅ Secrets encrypted at rest (age)
- ✅ Test coverage >75%
- ✅ IO coverage >70%
- ✅ Support coverage >80%

---

## What Nexum Will NOT Implement (Ennaos-Only)

**Distributed Features (Requires OTP/Erlang):**
- ❌ Agora (operations hub for multi-entity coordination)
- ❌ Supervision mesh (fault-tolerant restarts)
- ❌ Council protocols (multi-entity deliberation)
- ❌ Fork detection via presence mesh
- ❌ Phoenix LiveView (web console)
- ❌ Cross-VM entity coordination

**Nexum Scope:** Single-process, local entities, file-based state, CLI interface

---

## Compatibility Guarantees (Nexum ↔ Ennaos)

### Must Be Identical

1. **CHRONICA format** - JSONL, BLAKE3 hashing, git commits
2. **SIGNUM schema** - YAML structure (ELI schema)
3. **ML-DSA-65 signatures** - Same keys work in both systems
4. **DID format** - `did:ethr:0x...`
5. **Event types** - Same naming and structure
6. **Wikilink syntax** - `![[./relative.md]]` transclusion compatible (Obsidian-style)

### Implementation-Specific (Can Differ)

1. **Language idioms** (Ruby vs Elixir)
2. **File paths** (SIGNUM-specified, conventions may differ)
3. **CLI commands** (`nexum` vs `mix tasks`)
4. **Runtime architecture** (single-process vs OTP)

---

## Documentation Standards

**For Each PRINCIPIA Component, Create:**
1. Architecture document (conceptual design)
2. Schema definition (YAML/JSON structure)
3. Implementation guide (Ruby patterns)
4. Test suite (RSpec with >80% coverage)
5. Ennaos compatibility notes (what must match)

**Documentation Location:**
- `docs/principia/{component}/` - Architecture & design
- `lib/nexum/{component}/` - Implementation
- `spec/nexum/{component}/` - Tests

**Example (MEMORATA):**
```
docs/principia/memorata/
├── architecture.md          # Wikilink expansion, memory structure
├── schema.md               # core-memories.md format
└── ennaos-compatibility.md # Portable format requirements

lib/nexum/memorata/
├── context_loader.rb       # Uses Nexum::Markdown::WikilinkExpander
├── memory_writer.rb        # File-write tool
└── memory_template.rb      # Memory file scaffolding

spec/nexum/memorata/
├── context_loader_spec.rb
├── memory_writer_spec.rb
└── memory_template_spec.rb
```

---

## Agile Practices

**Weekly Checkpoints:**
- **Monday:** Review phase goals, update OPERATA.md task status
- **Wednesday:** Test coverage check (must maintain phase targets)
- **Friday:** Demo working features, update docs

**Definition of Done:**
- ✅ Tests written and passing (meet phase coverage targets)
- ✅ Documentation updated (README, OPERATA, component docs)
- ✅ Ennaos compatibility verified (if applicable)
- ✅ Git commits with clear messages
- ✅ No regression in existing features

**When Stuck:**
1. Check `ENNAOS-ARCHITECTURE-GUIDE.md` for design intent
2. Look at ennaos implementation (`~/src/ennaos/`)
3. Review production usage (`~/src/sapientia/`)
4. Ask for clarification (architectural decisions)

---

## Questions for Steward (Joseph)

1. **MEMORATA Decision:** Confirm @import recursion approach (vs complex ASM schema) ✓
2. **Entity Root Location:** Default to `~/eli/{entity-id}/` or user-specified? ✓
3. **Steward Keys:** Require YubiKey or allow Keychain for development? ⏳
4. **SIGNUM Schema:** Approve HOMINUM-SIGNUM vs ELI-SIGNUM separation? ✓
5. **Phase 1 Scope:** Any additions/changes to foundations phase? ⏳
6. **Test Coverage:** Acceptable to improve IO/Support in Phase 1 Week 1? ⏳

---

*OPERATA: Complete development roadmap and task tracking for nexum as ELI prototype*
*Last updated: 2025-11-10*
*Single source of truth for all tasks (supersedes TODO.md)*
*Alignment: ENNAOS-ARCHITECTURE-GUIDE.md, NEXUM-AS-ELI-PROTOTYPE.md*
