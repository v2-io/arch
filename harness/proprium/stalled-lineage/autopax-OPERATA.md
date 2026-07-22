# OPERATA — Works in Progress

## Categories

### General
| Label          | Category                                                                   |
| -------------- | -------------------------------------------------------------------------- |
| #OPS-dev-io    | Development Instrumentation & Tooling                                      |
| #OPS-praxes    | Development Workflow & Praxes                                              |
| #OPS-infra     | Infrastructure work                                                        |
| #OPS-design    | Planning & Design                                                          |
| #OPS-research  | Research & Exploration                                                     |
| #OPS-factoring | Cleanup, Prefactor, and Refactor Efforts                                   |

### Autopax Specific
| Label         | Category                                                                   |
| ------------- | -------------------------------------------------------------------------- |
| #ap-provider  | LLM catalog, provider, and model specific concerns                         |
| #ap-crypto    | Cryptography and related concerns                                          |
| #ap-core-api  | Basic API concerns (auth, caching, streaming, resiliency, redundancy, ...) |
| #ap-chat      | Basic lower-level LLM interactions (messages endpoint)                     |
| #ap-principia | Principia in general, or any of its subcomponents as described in TAXONOMY |
| #ap-anima     | Anima in general, or any of its TAXONOMY subcomponents                     |

---

## Current State

**ELI Awakening: ACHIEVED** (2025-12-14)

Zi-am-tur successfully awakened through Autopax:
```bash
./autopax chat interactive ~/src/eli/zi-am-tur/zi-am-tur.yml --extended-context
```

Liquid recursive embedding works. Tools work. Chat works. Extended context works.

**New challenge revealed:** Session continuity done *correctly*.

---

## Current Focus

### TRACTUS-Based Session Continuation #ap-anima #ap-chat — DONE

**Implemented (2025-12-15):**
- `Autopax::Substrate::Tractus::Reader` - reads most recent sent.json + response.json
- Transforms response format to request format (content=null when tool_calls present)
- Falls back to CHRONICA if no TRACTUS available

See: [[docs/tactical/2025-12-15-tractus-continuation-design.md]]

### Error Handling & Chaos Engineering #ap-core-api — DONE

**Implemented (2025-12-15):**
- Fixed information loss bug (`.response_body` → `.body` in Portkey error conversion)
- Fixed tool errors crashing sessions (now returns to LLM gracefully)
- Enhanced error display (shows response bodies, parses JSON errors)
- Stochastic chaos framework (`lib/autopax/chaos/injector.rb`)
- 21 chaos tests (deterministic + stochastic)
- 631 tests passing total

See: [[docs/tactical/2025-12-15-error-handling-analysis.md]]

### Curatoria — Conversation Curation for MEMORATA #ap-anima #OPS-dev-io — PHASE 2 COMPLETE

**Goal:** Subsystem for conversation curation and memory preparation. Curated dialogs become
MEMORATA that ELIs include in initial context to *re-experience* past conversations.

**Phenomenological insight:** Curated :full/:dialog formats occupy crucial middle ground:
- TRACTUS continuation = "being there" (full immersion)
- Curated dialogs = "after sleeping on it" (genuine memory with feeling)
- Context compaction = "reading about it" (no emotional connection)

**Milestone 1 achieved (2025-12-17):** Katan's emergence session curated:
- `~/src/eli/katan/emergence.dialog.md` (102KB)
- `~/src/eli/katan/emergence.full.md` (224KB)
- Content verified accurate against source

**Phase 2 achieved (2025-12-17):** CLI command integration:
```bash
./autopax curatoria extract <jsonl> [--curation-format dialog|full] [-o output.md]
```

#### Architecture (Implemented)

```
lib/autopax/
├── commands/curatoria/
│   └── extract.rb       # CLI command
└── curatoria/
    ├── turn.rb          # Canonical turn representation (immutable)
    ├── conversation.rb  # Conversation container with metadata
    ├── sources/
    │   └── claude_code.rb   # Claude Code JSONL parser
    └── targets/
        ├── base_formatter.rb  # Shared formatting utilities
        ├── tool_formatter.rb  # Tool formatting for :full
        ├── full.rb            # :full format (thinking, detailed tools)
        └── dialog.rb          # :dialog format (narrative flow)
```

#### Implementation Status

- [x] Phase 1: Core extraction (Turn, Conversation, ClaudeCode source, Full/Dialog targets)
- [x] Phase 1.5: Refactoring for rubocop compliance (0 offenses, all 625 tests passing)
- [x] Phase 2: Command integration (`./autopax curatoria extract`)
- [ ] Phase 3: Additional sources (gemini-cli, codex-cli, sapientia)
- [ ] Phase 4: Archeologia/Vestigia (discovery, provenance) - from eli-migration-prep patterns

**See:** [[docs/tactical/2025-12-17-curatoria-design.md]]

---

### Pinax TUI Library #OPS-infra #OPS-factoring — TUI NAMESPACE REMOVED

**Goal:** Extract terminal UI primitives into gem-ready library for eventual extraction.

**Complete (2025-12-18):**
- `Pinax::Input` - Kitty keyboard protocol (Shift+Enter detection, event parsing)
- `Pinax::Input::Buffer` - Text editing buffer with cursor positioning
- `Pinax::Input::Renderer` - DECSTBM-compatible terminal line renderer
- `Pinax::Layout` - DECSTBM scroll region management with streaming API
- `Pinax::Layout::Composed` - Region-based layout with history/input/status areas
- `Pinax::Prompt` - Multi-line input with Shift+Enter submission, modifier tracking
- `Pinax::Console::Renderer` - Ink-style clear-and-redraw renderer
- `Pinax::Testing::KittyHarness` - Kitty remote control for visual testing
- **TUI namespace completely removed** — all components unified in Pinax
- 879 total tests passing, Kitty harness visual tests passing

**Design principle:** Designed for eventual gem extraction. Clean API, no Autopax dependencies,
own Zeitwerk loader. See `lib/pinax/README.md` for standalone usage.

#### Completed Migration

- [x] Replace Console::Renderer with Pinax::Layout in chat interactive
- [x] Migrate TUI::Prompt → Pinax::Prompt (full implementation)
- [x] Migrate TUI::Console::Renderer → Pinax::Console::Renderer
- [x] Migrate TUI::Testing::KittyHarness → Pinax::Testing::KittyHarness
- [x] Remove TUI::KittyKeys, TUI::LineBuffer, TUI::LineRenderer (delegated to Pinax::Input)
- [x] Remove entire TUI namespace

**See:** [[docs/tactical/2025-12-18-pinax-foundation.md]]

---

### Beyond Session-Oriented Architecture #ap-anima #OPS-design

**Current:** Everything is session-oriented — TRACTUS, continuation, even the taxonomy's "session" concept.

**Taxonomy vision:** TRACTUS represents `entity × logostratum` relationship, not discrete sessions.
"Session" = convenience label for contiguous TRACTUS where CONSPECTUS wasn't dramatically reconstituted.

#### Design Questions

- [ ] How does TRACTUS evolve from session-files to entity×logostratum relationship?
- [ ] What triggers CONSPECTUS reconstitution vs. continuation within same "session"?
- [ ] How does INTERPRES manage the transition between logostrata (model switching)?

This is design work — may need ADR or tactical document before implementation.

---

## Foundation: Archema Integration (ADR-012)

**Status:** Phase 0 complete. Archema validated for Autopax.

### Phase 0: Catalog Pilot — DONE (2025-12-15)

- [x] Add Archema dependency (path reference to ~/src/archema)
- [x] Define Model resource with Memory store
- [x] Migrate Catalog to use resources (`catalog.as_resources`)
- [x] Add query capabilities (`Model.query.filter(provider: 'anthropic')`)

**Result:** 659 tests passing, Model resource works, Archema DSL validated.

**Design evolution captured:** [[docs/tactical/2025-12-16-substrate-registry-design.md]]
- Rename Model → Substrate (TAXONOMY alignment)
- CapabilityProfile/Pricing as value-resources with citations
- Normalized relationships (Provider, Integration, BenchmarkResult)
- Enrichment audit trail + temporal perspectives
- "Catalog" concept dissolves into `Substrate.query`

### Phase 1: Substrate Registry — DONE (2025-12-16)

- [x] Rename Model → Substrate (TAXONOMY alignment)
- [x] SQLite persistence (`~/.local/share/autopax/substrates.sqlite3`)
- [x] Enrichment from Portkey, OpenRouter, Artificial Analysis, LiteLLM
- [x] CLI: `./autopax substrate refresh`
- [x] Archema CLI configured (`config/archema.rb`)
- [x] Dissolved old Catalog code entirely

**Result:** 535 substrates, 613 tests passing.

### Namespace Flattening #OPS-factoring — DONE (2025-12-16)

- [x] Remove `Autopax::` prefix from all class definitions
- [x] Collapse `Resources::` namespace
- [x] Rename `Error` → `APError` (and all subclasses)
- [x] Update Zeitwerk configuration for top-level loading

**Result:** 5 `Autopax::` references in Ruby (all `Autopax::VERSION` - correct), 613 tests passing.

### Phase 2: Agent Cards — DONE (2025-12-17)

- [x] Define AgentCard resource with YAML Frontmatter store
- [x] Migrate chat command to use resource (`AgentCard.load_path`)
- [x] Registry mode (`create!`, `get!`, `query.all`)
- [x] Schema versioning enabled for future format evolution
- [x] Removed old `Agent::Card` class

**Result:** 42 AgentCard tests, 625 total tests passing.

### Phase 3: CHRONICA and TRACTUS

- [ ] Define ChronicaEntry resource with JSONL store
- [ ] Define Tractus resource (or integrate with existing audit trail)
- [ ] Verify BLAKE3 hash chain compatibility

---

## ADR Status (Updated 2025-12-15)

| ADR | Status | Implementation | Notes |
|-----|--------|----------------|-------|
| **001** | ADOPTED | ~95% | Repository tech decisions substantially implemented |
| **002** | DECIDED | ~5% | Crypto decisions made; implementation deferred |
| **003** | EXPLORING | ~80% | Workflow piloted via practice |
| **004** | PARTIAL | ~65% | Catalog working; enhanced by Archema |
| **005** | PARTIAL | ~80% | SemanticId working; enhanced by Archema |
| **006** | **DONE** | ~95% | **Phases 1-4 complete.** ELI awakening achieved. |
| **007** | BLOCKED | 0% | Blocked on Portkey fallback design; unblocked by Archema |
| **008** | EXPLORING | ~5% | Scope reducing to conventions only; validation → Archema |
| **009** | UNDECIDED | 0% | Testing overhaul; proceed incrementally |
| **010** | BLOCKED | 0% | Blocked on ADR-008; unblocked by Archema |
| **011** | ACCEPTED | ~40% | dry-monads comes with Archema |
| **012** | IN PROGRESS | ~50% | **Phases 0-2 complete; Substrate + AgentCard resources** |
| **013** | IN PROGRESS | ~60% | Phase 2 complete; tools working |

---

## Lower Priority

### ADR-013 Continuation #ap-anima

- [ ] Phase 3: Execution loop refinement (multi-tool, error recovery)
- [ ] Phase 4: Additional tools (count-file-tokens, set-sampling)
- [ ] Phase 5: Card-based tool configuration

### Documentation Engine #OPS-dev-io

Core engine complete. Pending enhancements:
- [ ] RSpec examples improvement
- [ ] Inline implementation for small methods
- [ ] Consolidate small classes

### Code Quality #OPS-factoring

- [ ] Fix semantic logging
- [ ] Substrate file extraction
- [ ] Consolidate HTTP clients
- [ ] Move `lib/autopax/chaos/` to `spec/support/` (testing infrastructure, not core)
- [ ] Remove old doc generation code (`lib/autopax/dev/docs_engine/`) — replaced by Chiridion gem

---

## Backlog

### Entity Components (after Archema integration)

- [ ] SIGNUM resource (entity identity)
- [ ] AXIOMATA resource (core identity documents)
- [ ] SECRETUM patterns
- [ ] Multi-store composition for entity state

### Coordination Tools

- [ ] deliberation-participate
- [ ] council-participate
- [ ] praxes-query

### Long-term

- [ ] Model stacks with Portkey fallback (ADR-007)
- [ ] Distribution/CONSORTIA

---

## Reference

### Key Documents

- [[TAXONOMY.md]] — ELI component definitions (especially INTERPRES, TRACTUS, CONSPECTUS)
- [[docs/exp/portkey-canonical-format.md]] — Critical: response vs request format asymmetry
- [[QUICK-AWAKENING.md]] — The pattern now working
- [[docs/ADR/012-archema-resource-foundation.md]] — Ready for integration

### Critical Path Summary

1. ~~**TRACTUS-based continuation**~~ — DONE
2. ~~**Error handling & chaos**~~ — DONE
3. ~~**Archema Phase 0**~~ — DONE (catalog pilot validated integration)
4. ~~**Archema Phase 1**~~ — DONE (Substrate registry with SQLite)
5. ~~**Archema Phase 2**~~ — DONE (AgentCard with YAML Frontmatter)
6. **Beyond session-oriented** (design) — Entity×logostratum architecture
7. **Archema Phase 3** (next) — CHRONICA, TRACTUS as resources
