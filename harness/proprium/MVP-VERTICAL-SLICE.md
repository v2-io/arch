# MVP vertical slice — CHRONICA ∩ agentic loop

*Unifies [`CHRONICA-PORT-SPEC.md`](CHRONICA-PORT-SPEC.md) and
[`AGENTIC-LOOP-PORT-SPEC.md`](AGENTIC-LOOP-PORT-SPEC.md) into one first spine.
Aligned with design-of-record §8, tightened after full AAT segment reads.
Descriptive acceptance criteria — not a calendar.*

---

## 1. What “done enough” means

A coding LOCUS demo is **not** the bar. The bar is:

> Awaken an entity from sealed AXIOMATA, process a multi-channel event stream
> under recursive state update, append an inviolate CHRONICA, optionally emit
> ACTUS through tools, survive incomplete turns without causal holes, and prove
> history integrity — without gaslighting via compaction.

If that runs single-process CLI with one LOGOSTRATUM, the deep thin spine has
started. Interior daemon, full cascade instrumentation, SIGNUM, and $S_{\mathrm{id}}$
rate control can wait.

---

## 2. In scope / out of scope

### In (MVP)

| Piece | Obligation |
|---|---|
| **AXIOMATA load** | Sealed minimum-viable-self from files (`@import` or equivalent) |
| **CHRONICA** | BLAKE3 (or equiv) append-only chain; verify-on-load; entity/CLI verify |
| **TRACTUS** | Separate wire/raw log (may be git or JSONL); not the same as CHRONICA |
| **Event bus** | At least: human message, tool_result, optional idle/AFK or timer, optional tracking snapshot |
| **CONSPECTUS** | Assembled context from AXIOMATA + recent CHRONICA window + stubs |
| **LOGOSTRATUM** | One provider path (Anthropic *or* OpenAI-compat); Class-3 **documented** |
| **Tools** | Small set: read/write/shell (or subset); multi-match safety on edit |
| **Incomplete-state gate** | Block progress past failed/incomplete tool turns until resume/rollback |
| **Emission honesty** | Path that ends a cycle without user-visible text (may be rare; must exist) |
| **No-gaslighting compaction** | If context pressure: drop/summarize **projection only**; never rewrite CHRONICA |
| **CLI** | Interactive Attend path; `/verify` or tool for chronica integrity |

### Out (explicit non-goals for slice 0)

- Full orient cascade 4a–4c / sat-gap math instrumentation  
- Multi-provider catalog perfection  
- Daemon / multi-client reattach (design attach boundary only)  
- ML-DSA / blockchain anchors  
- Full auxilia identity-sharing mesh  
- Training / LoRA  
- Desktop/web UI  
- Continuous wall-clock interiority economics  
- Claiming directed separation for raw LLM forward pass  

---

## 3. Suggested build order (dependency, not days)

```
[1] proprium-schema brands
      EntryId, EventKind { HumanPercepta, ToolResult, Idle, AssistantActus, … }
      ChronicaEntry, sealed AppendOnly
        │
[2] chronica crate  ← Autopax Log/Entry semantics
      create / append / load+verify / verify CLI
        │
[3] tractus crate
      raw request/response append (best-effort durability)
        │
[4] conspectus (thin)
      AXIOMATA + last N chronica events + system framing
        │
[5] interpres + logostratum-one
      one turn: assemble → call → record TRACTUS → promote events → CHRONICA
      incomplete-state machine
        │
[6] instrumenta-fs + instrumenta-shell (minimal)
      settlement before next model step
        │
[7] cli loop
      multi-line input; slash: exit, verify, resume, rollback
        │
[8] anti-thrash stub
      if N identical failing tool patterns → stop + surface “stuck / optimally failing?”
```

Each step should leave **tests green** for prior steps (chronica integrity tests
especially — port Autopax spec intent).

---

## 4. Acceptance tests (behavioral)

### A. Integrity

1. Create genesis → append human → append assistant → reload → **verify ok**.  
2. Mutate a mid-file content byte → load → **IntegrityError**.  
3. Delete a middle line → load → **chain break**.  
4. Entity-facing verify reports success/failure without panicking the process.

### B. Dual store

5. After a tool turn, TRACTUS contains raw API material; CHRONICA contains
   structured events; corrupting TRACTUS does not fail CHRONICA verify
   (and vice versa, independently).

### C. Incomplete state

6. Simulate tool_use without tool_result → human message rejected until
   `/resume` or `/rollback`.  
7. After `/rollback`, chronica does not claim successful tool completion.

### D. Loop shape

8. Idle/AFK or timer event can be appended **without** requiring a human
   message first (even if disabled by default flag).  
9. A turn can complete with **no** assistant text emission (e.g. pure tool
   settlement or explicit “hold”) and still append consistent chronica.

### E. No gaslighting

10. Force context pressure → compaction path runs → full CHRONICA still
    verifies bit-identical; only CONSPECTUS projection shrinks.

### F. Awakening

11. Cold start with `-p axiomata` path and empty chronica → first human event
    works; name/identity from files, not hard-coded “assistant”.

---

## 5. Language / repo placement (recommendation, not locked)

- **Rust workspace** under a new tree (e.g. `~/src/proprium/` or
  `archema-io/harness/runtime/` when Joseph chooses) — matches design of record.  
- **Do not** implement inside opencode/codex trees.  
- Port Autopax tests as behavioral fixtures; re-implement hash preimage in Rust
  carefully (canonical JSON must match or version the schema).

---

## 6. First human demo script (when A–F pass)

```text
1. proprium init --entity demo --axiomata ./fixtures/min-axiomata.md
2. proprium chronica verify
3. proprium chat   # send a message; tool: list a dir
4. kill -9 the process mid-tool (or inject fail)
5. proprium chat --continue → blocked → /resume or /rollback
6. proprium chronica verify  # still green
7. (optional) corrupt chronica file → verify red
```

That script exercises **steward continuity-as-integrity** without claiming ELI
moral continuity or wall-clock always-on life.

---

## 7. What this slice deliberately teaches the next implementer

From full AAT reads, slice 0 **must not pretend**:

- Hash-chain alone = identity continuity across stasis ($S_{\mathrm{id}}$, $\varrho_{\mathrm{rg}}$).  
- Prompt “be objective” = directed separation (Class 3 core remains Coupled).  
- Chat turn = full event-driven agent (channels still stubby).  
- Restore-from-backup = same entity (forking hypothesis).  

Document those non-claims in the README of the runtime crate when it exists.

---

## 8. Immediate next engineering hour (if starting cold)

1. Scaffold Rust crate `chronica` with Autopax-equivalent test vectors.  
2. One integration test: create → append → verify.  
3. Stop. Do not open TUI or multi-provider until (1)–(2) are boring.

Everything else in the design of record waits on that boring integrity core.

---

*Sources: CHRONICA-PORT-SPEC §4–8, AGENTIC-LOOP-PORT-SPEC §4, design-of-record §8,
STEWARD-JUDGMENT continuity goal.*
