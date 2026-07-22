# Building Operata: A hierarchical task system for human-agent collaboration

**No existing CLI task tool fully supports the combination of arbitrary-depth hierarchy, speculative decomposition, graph structures, and multi-agent coordination you're designing.** Org-mode comes closest with unlimited depth and mid-work decomposition, but lacks graph relationships and multi-agent coordination. The design space you're exploring—particularly speculative decomposition and cross-cutting concerns—is genuinely novel in task management tooling. However, substantial prior art from HTN planning, CRDT-based collaboration, and LLM agent frameworks provides proven patterns for each component.

This report synthesizes research across eight domains to inform Operata's design: existing CLI task tools, HTN/GOAP planning systems, multi-agent coordination, identity schemes, graph models, LLM tool conventions, and Ruby implementation patterns.

---

## Existing CLI tools reveal a hierarchy gap

The landscape of hierarchical task CLI tools exposes a consistent limitation: **most tools are fundamentally flat**, relying on dependencies rather than true parent-child hierarchies.

**Taskwarrior** offers the most sophisticated dependency system (`depends:UUID`) but explicitly avoids subtasks—tasks are flat with `+BLOCKING`/`+BLOCKED` virtual tags. Its dual ID system (permanent UUID + ephemeral sequential ID) provides a model for CLI ergonomics, but dependency chains can't display as trees without external scripting. The project hierarchy via dot notation (`project:Work.Sprint.Feature`) enables unlimited categorization depth, but this doesn't extend to task structure.

**Org-mode** stands alone in supporting **arbitrary-depth hierarchy** through outline indentation. Mid-work "explosion" is trivial—demote a heading and add children. Progress cookies (`[1/3]`) auto-update, and `org-depend.el` enables limited cross-tree dependencies. However, org-mode lacks persistent IDs (tasks are identified by headline text + location) and has no multi-agent coordination.

**dstask** provides the best git-friendly model: **one YAML file per task** with UUID-based filenames, designed to avoid merge conflicts. Its sync approach (`dstask sync` = pull + push with auto-merge) demonstrates passwordstore.org-style distributed state management. However, it has no hierarchy or dependency support.

| Tool | Native Depth | Mid-Work Explosion | Graph Structure | Stable IDs |
|------|--------------|-------------------|-----------------|------------|
| org-mode | **Unlimited** | **Excellent** | Limited (via deps) | None by default |
| Taskwarrior | None | Poor | DAG via depends: | UUID + short ID |
| dstask | None | None | None | UUID (YAML files) |
| TaskLite | None | None | None | ULID (sortable) |
| Obsidian Tasks | Limited | Lossy in queries | None | None |

**Key insight**: No tool supports speculative/draft task decompositions. Workarounds involve `+draft` tags or separate planning files, but no native "try multiple approaches" capability exists.

---

## HTN planning offers the decomposition model

Hierarchical Task Network planning provides the conceptual framework for Operata's decomposition system. HTN distinguishes **compound tasks** (high-level, must be decomposed) from **primitive tasks** (atomic, executable). The critical concept is **methods**—alternative decomposition strategies selected based on context.

```
CompoundTask [DeployApplication]
  Method [environment == production]
    Subtasks [RunTests(), Build(), Stage(), Deploy(), Verify()]
  Method [environment == development]
    Subtasks [Build(), DeployLocal()]
  Method [is_hotfix]
    Subtasks [Build(), DeployDirect()]
```

**Handling unexpected complexity mid-execution** is precisely what HTN addresses through:

1. **Partial planning**: SHOP2's forward decomposition plans only a few steps ahead, knowing the current state at each point. This prevents over-commitment when complexity emerges.

2. **Recursion for emergent depth**: Tasks can recursively reference compound tasks (including themselves). When a primitive fails, the system can re-decompose from the current state with updated knowledge.

3. **Task insertion**: AAAI 2020 research on "Refining HTN Methods via Task Insertion" addresses incomplete methods by inserting additional tasks during execution—directly applicable to your "~10% completion explosion" scenario.

4. **Expected effects**: HTN tracks what execution *should* produce versus what it *actually* produces, triggering replanning on divergence.

**GOAP** (Goal-Oriented Action Planning, from game AI) complements this with short, dynamic plans. F.E.A.R.'s GOAP used A* search backward from goal state, producing plans of 1-4 actions that replan rapidly when world state changes. The key pattern: **don't plan too far ahead, and replan frequently**.

For Operata, the mapping is:

| HTN Concept | Operata Equivalent |
|-------------|-------------------|
| Compound task | High-level intent/goal |
| Primitive task | Atomic CLI operation or agent action |
| Method | Alternative decomposition strategy |
| World state | File existence, test status, environment |
| Expected effects | Completion criteria |

---

## Multi-agent coordination favors soft claiming

Research reveals a spectrum from hard locking to full eventual consistency, with **soft claiming + CRDTs** providing the best fit for "claiming available but not required."

**Blackboard architecture** provides the coordination model: agents communicate solely through a shared knowledge repository without direct contact. Modern implementations (bMAS for LLM agents, AWS Arbiter pattern) add conflict-resolver agents that mediate contradictions and dynamic agent selection based on blackboard state. This maps directly to a shared task file structure.

**CRDTs** (Conflict-Free Replicated Data Types) enable mathematical conflict resolution without coordination:
- Add-wins sets for task creation
- Sequence CRDTs (RGA) for ordered subtask lists
- Strong eventual consistency: replicas with same updates reach identical state

Real-world usage in Google Docs, Redis, Yjs, and Automerge demonstrates production viability.

**The recommended pattern for Operata combines three approaches**:

1. **Status-based soft claiming** (Kanban-style): Moving a task to "in_progress" signals intent without hard locking. Others can see and adapt or proceed anyway.

2. **Optimistic locking at commit**: Work independently, resolve conflicts at integration time—git's model applied to tasks.

3. **CRDT-style merging** for non-structural changes: Metadata updates (priority, tags, notes) merge automatically.

```yaml
task:
  id: "01863d24"
  status: in_progress
  claimed_by: agent-coder-1  # Soft signal, not a lock
  claimed_at: 2025-01-15T10:30:00Z
  allow_concurrent: true  # Explicit permission for overlap
```

**Contract Net Protocol** remains relevant for explicit task auction scenarios, but its communication overhead makes it better suited for high-value task assignment than routine coordination.

---

## Speculative decomposition needs invention

**No existing tool directly supports "draft multiple decompositions"**—this is genuinely novel. However, several patterns inform the design:

**Git's topic branch model** provides the closest analogy: create a branch for speculative work, evaluate, merge or discard. For Operata, this suggests:

```yaml
task:
  id: "01863d24"
  decompositions:
    - id: draft-1
      status: draft
      created_by: agent-planner
      subtasks: [...]
    - id: draft-2
      status: draft
      created_by: human
      subtasks: [...]
  committed_decomposition: null  # None yet committed
```

**ADaPT** (As-Needed Decomposition and Planning) from recent LLM research decomposes tasks only when the executor fails—adaptive depth based on complexity. This matches your "~10% completion mark" explosion trigger.

**Interactive speculative planning** uses Monte Carlo tree search to explore multiple action trajectories before committing, with human-in-the-loop visualization for steering. This suggests Operata could score draft decompositions before commitment.

---

## UUID7 with Base58 prefixes solves identity

The identity scheme should combine **UUID7 for stability and sortability** with **Base58 short prefixes for CLI ergonomics**.

**UUID7** (RFC 9562, 2024) provides:
- **Time-sortability**: 48-bit Unix timestamp prefix means newer IDs are lexicographically greater
- **Database-friendly**: Sequential inserts reduce B-tree fragmentation
- **Collision-resistant**: Random component ensures uniqueness at same millisecond

**Short prefix handling** follows git's model:
- Minimum 4 characters for `ops show sMHu`
- 4-char Base58 = ~11.3M unique values (ample for task systems)
- Collision detection: if multiple matches, require longer prefix or display candidates
- Auto-extend: `--abbrev` style automatic length increase for uniqueness

**Base58** (Bitcoin's alphabet: `123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`) excludes confusing characters (0/O, I/l), making it ideal for human-visible IDs.

```ruby
# Ruby implementation
require 'uuid7'
require 'base58'

def generate_task_id
  uuid = UUID7.generate  # "01863d24-6d1e-78ba-92ee-6e80c79c4e28"
  bytes = [uuid.delete('-')].pack('H*')
  Base58.binary_to_base58(bytes, :bitcoin)  # "2XFWEM4vDqKZg9RDT4jTQh"
end

def short_id(full_id, length = 4)
  full_id[0...length]  # "2XFW"
end
```

**Hybrid content-hash pattern**: Store both stable UUID (for references) and content hash (for integrity/deduplication):

```yaml
task:
  id: "01863d24-6d1e-78ba-92ee-6e80c79c4e28"
  content_hash: "sha256:abc123..."  # Changes when content changes
  version: 3
```

---

## Graph structure requires explicit cross-cutting links

Pure trees can't model cross-cutting concerns—the solution is a **DAG with explicit soft links**.

**What trees can't represent**:
- A "Fix security vulnerability" task that contributes to Security, Performance, and Compliance goals simultaneously
- Diamond dependencies where D needs both B and C, which both depend on A
- "Related to" relationships without hierarchical ownership

**Recommended model**: One primary parent (preserves tree traversal) plus typed soft links:

```yaml
task:
  id: security-fix
  parent: security-initiative  # Primary ownership
  contributes_to:
    - goal: q4-release
      weight: 0.3
    - goal: tech-debt-reduction
      weight: 0.1
  blocks:
    - release-1.5
  related:
    - performance-audit
  tags: [security, urgent]
```

**Storage implications for git-friendliness**:

| Approach | Git-Friendliness | Query Performance |
|----------|-----------------|-------------------|
| File-per-task YAML | **Excellent** (line-level diffs) | Slower (many files) |
| Monolithic YAML | Poor (merge conflicts) | Fast reads |
| SQLite | Bad (binary) | Best for queries |
| JSON Lines | Good (append-only) | Moderate |

**Recommended structure**:
```
.operata/
  tasks/
    01863d24.yml   # File per task, UUID-named
    01863d25.yml
  index.yml        # Regenerated fast-lookup index
  graph.yml        # Explicit dependency graph (optional cache)
```

With UUID7's timestamp prefix, date-based subdirectories (`2024/01/`) align naturally with ID ordering.

---

## LLM agent patterns reveal the plumbing/intelligence split

Claude Code's **TodoWrite/TodoRead** pattern demonstrates the exact architecture you're describing: a no-op tool that structures agent thinking without executing external logic.

**Key design principles from LLM agent systems**:

1. **Status state machine**: `pending → in_progress → completed` with explicit transitions. Claude Code: "Mark as in_progress BEFORE beginning work. Only have one in_progress at a time."

2. **Tool returns**: Consistent schema with `success: boolean`, `result/output`, `error/isError`.

3. **Git as checkpoint system**: Aider auto-commits with sensible messages; Claude Code modifies files directly but within git for rollback. Changes are atomic and reversible.

4. **Propose vs. commit separation**: Cursor shows diff preview; Aider auto-commits (git rollback available); this maps to Operata's speculative decomposition.

5. **Single-threaded execution**: "Ideally only have one todo as in_progress at a time" prevents context thrashing.

**Tool schema pattern for Operata**:

```json
{
  "name": "ops_decompose",
  "description": "Create draft decomposition for a task",
  "inputSchema": {
    "type": "object",
    "properties": {
      "task_id": { "type": "string" },
      "subtasks": { "type": "array", "items": { "type": "string" } },
      "draft": { "type": "boolean", "default": true }
    }
  },
  "outputSchema": {
    "type": "object",
    "properties": {
      "success": { "type": "boolean" },
      "decomposition_id": { "type": "string" },
      "subtask_ids": { "type": "array" }
    }
  }
}
```

**LLMCompiler's DAG pattern** is particularly relevant: the planner generates a DAG of tasks with dependencies, and a Task Fetching Unit schedules execution based on those dependencies. This separates "what to do" (agent intelligence) from "when to do it" (CLI/scheduler determinism).

---

## Ruby implementation: Dry-CLI with TTY for interaction

**Framework recommendation**: **Dry-CLI** for clean, testable command structure, with **GLI** as alternative for git-style nested subcommands.

Dry-CLI advantages:
- Modern API with command objects as plain Ruby classes
- Composable with dry-rb ecosystem
- Better separation of concerns than Thor
- Excellent for both human and programmatic use

**Command structure for `ops sMHu subitems '1. First; 2. Second'`**:

```ruby
module Operata
  module Commands
    class Subitems < Dry::CLI::Command
      desc "Add subitems to a task"
      argument :task_id, required: true, desc: "Task ID or prefix"
      argument :items, required: true, desc: "Semicolon-separated items"
      option :draft, type: :boolean, default: false
      
      def call(task_id:, items:, **opts)
        task = resolve_short_id(task_id)
        parsed = parse_subitems(items)
        # deterministic state change
      end
    end
  end
end
```

**Structured text parsing with Parslet**:

```ruby
class SubitemParser < Parslet::Parser
  rule(:number) { match('[0-9]').repeat(1).as(:num) }
  rule(:text) { match('[^;]').repeat(1).as(:text) }
  rule(:item) { number >> str('.') >> space? >> text }
  rule(:items) { item >> (str(';') >> space? >> item).repeat }
  root(:items)
end
```

**Agent-friendly output**: Follow gh CLI patterns with `--json` and `--format` flags:

```ruby
class_option :json, type: :boolean
class_option :format, type: :string, values: %w[json yaml text]

def output(data)
  case options[:format]
  when 'json' then puts JSON.pretty_generate(data)
  when 'yaml' then puts data.to_yaml
  else puts format_human(data)
  end
end
```

**UUID7 in Ruby**: Use the `uuid7` gem or `uuidx` (faster, ~1M iterations/sec):

```ruby
require 'uuidx'
id = Uuidx.v7  # "01863d24-6d1e-78ba-92ee-6e80c79c4e28"
```

---

## Conclusion

Operata's design requirements push beyond existing tools into genuinely novel territory—particularly speculative decomposition with multiple draft alternatives and graph-based cross-cutting concerns. The architecture that emerges from this research combines:

**From HTN**: Compound/primitive task distinction, method-based decomposition alternatives, partial planning with re-decomposition on complexity discovery, expected vs. actual effects for replanning triggers.

**From multi-agent coordination**: Blackboard architecture for shared state visibility, soft claiming via status fields, CRDT-inspired eventual consistency for metadata merging, git-style conflict resolution for structural changes.

**From LLM agent patterns**: CLI as deterministic plumbing layer (TodoWrite pattern), single in-progress task at a time, git as checkpoint/rollback system, propose-before-commit workflow for speculative decomposition.

**From identity research**: UUID7 for time-sortable stable IDs, Base58 encoding with 4-6 char prefixes for CLI ergonomics, collision detection with disambiguation prompts, hybrid UUID + content-hash for referential stability plus integrity verification.

**From storage patterns**: File-per-task YAML with UUID filenames for git-friendly merging, explicit DAG links (`contributes_to`, `blocks`, `related`) alongside primary parent for cross-cutting concerns, regenerated indexes for fast queries.

The novel contribution Operata makes is combining these into a coherent system where agents can "spend more thoughtful time on tasks" by exploring deeper decompositions speculatively—creating multiple draft breakdowns, evaluating them, and committing to one—all while collaborating with humans through an imperative CLI that maintains deterministic state management beneath the intelligence layer.