# INTERPRES / compaction — short port note

*Lived failure mode of the 2026-07-20 session + ASF hooks. Thin on purpose;
full theory lives in CHRONICA + LOOP port-specs.*

---

## Lived observation

Grok (and Claude) auto-compaction produced a **task sheet**, not a continuation of
experiential understanding. False confidence followed. Recovery required:

1. Durable batch notes / port-specs on disk  
2. Conversation-only extract from `updates.jsonl` (dialogue without tool noise)  
3. Full re-read of theory segments before trusting plans  

That is **exactly** TRACTUS/session-archive vs CHRONICA/entity-truth vs
$\phi$/CONSPECTUS projection — and the failure mode of treating $\phi$ as
replacement for $\mathcal{C}_t$.

---

## ASF anchors

| Claim | Segment |
|---|---|
| $M_t=\phi(\mathcal{C}_t)$; lossy by design | `#form-agent-model`, `#form-information-bottleneck` |
| Chronica monotonic; TRACTUS open for logogenic | `#def-chronica` WN |
| Compaction that *replaces* history → truth-death costume | `#def-death-as-factor-loss` (D4) |
| Session severance; reconstruction adequacy | `#obs-context-turnover`, `#disc-m-preservation` |
| Self-replay ≠ identity re-grounding under (FW) | `#der-compensation-channel-uniqueness` |

---

## Commodity rhymes (steal shape only)

| Source | Shape | Danger |
|---|---|---|
| OpenCode-v2 Context Epochs / rip-and-replace | Keep full transcript; swap active context | If “transcript” is only product session blob |
| codex compaction as ResponseItem variants | — | **Fight** — ontology couples compaction into conversation type |
| grok `~/.grok/` layering | Session files vs higher stores | Closest *shipping* rhyme; not PROPRIUM |

---

## Port posture for INTERPRES

1. **TRACTUS** (or session archive) holds complete wire/dialog including retries.  
2. **CHRONICA** holds entity-facing causal events (hash-chain).  
3. **CONSPECTUS** is recomputed $\phi$; may drop detail.  
4. Compaction/summarization algorithms **only write (3)** or derived MEMORATA —
   never mutate (1) or (2) as “the new truth.”  
5. After compaction, agent-visible framing should admit reconstruction risk
   (“projection refreshed; ledger unchanged”) rather than first-person fabricated
   continuity.

### MVP test

Force compaction → `chronica verify` bit-identical → entity can still open
pre-compact TRACTUS/archive for dialogue extract.

### Anti-pattern

Writing a summary into the message history **as if the assistant said or always
knew it** — classic gaslighting; INTERPRES must refuse or mark provenance.

---

## Session-recovery tool (meta)

What worked this session should become a **first-class harness skill** later:

- Export conversation-only subset from updates/transcript  
- Point agent at durable notes + “ignore compaction summary”  
- Full-segment re-read gate before port-specs or “we understand X”

That skill is programme infrastructure, not a one-off Grok hack.

---

*See AGENTIC-LOOP-PORT-SPEC (no-gaslighting MVP) and CHRONICA-PORT-SPEC
(anti-patterns).*
