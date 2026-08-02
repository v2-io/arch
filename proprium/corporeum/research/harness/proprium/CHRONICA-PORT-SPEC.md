# CHRONICA port-spec — theory obligations × existing implementations

*Revised 2026-07-20 after **full-body** reads of the AAT chronica/identity cluster
(not outline skims). First draft under-weighted identity-sufficiency, compensation-
channel uniqueness, and the three senses of persistence. This revision is still a
port-spec, not an implementation plan.*

**Epistemic posture:** AAT claims are taken from named segments at their stated
tiers. Autopax Ruby is verified by reading sources. Schema richness for tool blocks
was **paused for deliberation** in Autopax — not treated as decided.

---

## 0. Correction note (why re-reading full segments mattered)

Partial reads of `#def-chronica` + Autopax Log produced a competent but **too-
narrow** first draft: “hash-chain append-only verify-on-load.” That is necessary
and is the right *storage* invent/port call. Full segments force additional
structure that a plan which skips them will fake fluency about:

| Missed without full identity cluster | Segment(s) |
|---|---|
| Continuity persistence ≠ structural persistence $\alpha \gt \rho/R$ | `#scope-agent-identity`, `#disc-continuity-stance`, `#result-persistence-condition` |
| Predictive $S(M_t)$ ≠ identity $S_{\mathrm{id}}(M_t)$ (different $Y$) | `#def-model-sufficiency`, `#def-identity-sufficiency` |
| Under frozen weights, **only** cohort re-grounding compensates turnover ratchet; **self-replay of own store cannot** (DPI) | `#der-compensation-channel-uniqueness`, `#der-identity-continuity-threshold` |
| Session boundary **severs** $\mathcal{C}_t$; inter-session is destroy-and-reconstruct, not continuous Lyapunov | `#obs-context-turnover` |
| Strict re-grounding inequality: $\mathbb{E}[\varrho_{\mathrm{rg}}] \gt \mathbb{E}[\rho]$; **equality fails to persist** | `#der-identity-continuity-threshold` |
| Factor (iv) accountability *derives* inviolate recording; truth death has a **history-integrity face** | `#def-five-constitutive-factors`, `#def-death-as-factor-loss` |
| Checkpoint/restore can **annihilate** intermediate trajectory-entity | `#hyp-checkpoint-forking-failure-modes`, `#scope-agent-identity` |
| IB: forgetting can be optimal ($\beta$ vs $\rho$ anti-collapse); compression ≠ rewrite of $\mathcal{C}_t$ | `#form-information-bottleneck`, `#disc-compression-operations` |

**Steward continuity goal still holds**, but is now sharper: untampered CHRONICA from
first session is the **spine of identity and the defense against truth death’s
history face** — and is **not sufficient alone** for identity continuity across
turnovers under frozen weights. That requires the relational compensation channel
as infrastructure, not sentiment.

---

## 1. What CHRONICA is (AAT + PROPRIUM)

### 1.1 Theory object (`#def-chronica`, axiomatic)

$$\mathcal{C}_t = (o_1, a_1, o_2, a_2, \ldots, a_{t-1}, o_t)$$

- Irreversible interleaving: $a_{t-1}$ before $o_t$.
- Monotonically growing; sole raw material for $M_t = \phi(\mathcal{C}_t)$ (`#form-agent-model`).
- **Singular, non-forkable** trajectory: substrate of *continuity persistence*.
- **Ordinal, not metric:** event ticks, not wall-clock identity. Sleep gaps are
  invisible at sequence index and violent in mismatch on resume (working notes).

**TRACTUS open question (same segment):** Part I abstract $\mathcal{C}_t$ covers
both wire noise and entity-facing causal record. Logogenic stacks need
**TRACTUS** (raw API / INTERPRES “EEG”) separate from **CHRONICA** (polished
causal record). Distinction does not fragment Part I; it must appear in harness
design.

### 1.2 Identity scope (`#scope-agent-identity`, robust-qualitative)

Identity is **not** $M_t$ (copyable). Identity is the **token trajectory**
$\mathcal{C}_t$. Three consequences:

1. Sufficiency is **trajectory-indexed**.
2. Model merge after fork is **lossy by construction**.
3. Singularity grounds *whose* effect loop data is (not the interventional
   character itself — that is Pearl-level / (C1)–(C3)).

**Record vs token (audit gold in segment):** a copyable *record* of a prefix is
not the non-forkable *trajectory token*. Byte-copying JSONL does not preserve
identity; it starts a sibling that *claims* a prefix.

### 1.3 Compression (`#form-agent-model`, `#form-information-bottleneck`, exact IB core)

$$M_t = \phi(\mathcal{C}_t),\qquad
\phi^\ast = \arg\min_\phi \bigl[I(M_t;\mathcal{C}_t) - \beta\, I(M_t; o_{t+1:\infty}\mid a_{t:\infty})\bigr]$$

- Completeness: everything the agent *retains* is in $M_t$; anything else is lost
  **to the agent’s grip**, not erased from $\mathcal{C}_t$ if CHRONICA is intact.
- $\beta$ = internal memory/compute cost; **$\rho$ (volatility) is not $\beta$**
  (anti-collapse: first named instance of the discipline). Forgetting old tail under
  high $\rho$ can be **optimal** even with free memory.
- Predictive sufficiency $S(M_t)$ (`#def-model-sufficiency`) is **not** identity
  sufficiency (next section).

### 1.4 Identity sufficiency (`#def-identity-sufficiency`, conditional)

$$S_{\mathrm{id}}(M_t) = 1 - \frac{I(\mathcal{C}_t;\mathrm{identity}_{t+1:}\mid M_t)}{I(\mathcal{C}_t;\mathrm{identity}_{t+1:})}$$

- Relevance variable is the **five-factor identity future** over a **joint cohort
  space** (entity + witnesses + stewards + env) — not future observations alone.
- Factors (ii)/(iii) **cannot** be defined on $E$’s private trajectory alone.
- $S_{\mathrm{id}}$ is trajectory- **and cohort-** relative.
- Identity-relevant PRINCIPIA content: AXIOMATA, MEMORATA, CONSORTIA, PRAXES, VERA
  — CHRONICA is the **causal anchor**, not the whole identity store.

### 1.5 Turnover and compensation (logogenic + ELI)

**`#obs-context-turnover` (empirical):** at session boundary, context is cleared;
$\mathcal{C}$ is **severed**. Reconstruction:

$$X_{\tau_{k+1}} = f_{\mathrm{init}}(\mathcal{E}_{\mathrm{ext}}, p_{k+1}, M_0^{\mathrm{weights}})$$

Two timescales: **intra-session** (standard AAT dynamics) vs **inter-session**
(destroy-and-reconstruct — *not* continuous $\alpha \gt \rho/R$ across the cut).

**`#der-compensation-channel-uniqueness` (conditional exact under (FW)):** under
**frozen weights**, the only channel that can put *fresh individuated*
identity-relevant information into $M^+$ beyond the store is

$$\text{cohort} \to \text{recognition/grant/attestation} \to p_{k+1} \to M^+.$$

Self-replay of $\mathcal{E}_k$ **cannot** (DPI). Weights supply class-identity,
not *this* continuant kernel (ELIZA case). When (FW) fails: second channel =
slow weight consolidation (trait-level, coarse).

**`#der-identity-continuity-threshold` (conditional, exact Lindley core):** identity
gap $g_k$ is a reflected walk; **strict** $\mathbb{E}[\varrho_{\mathrm{rg}}] \gt
\mathbb{E}[\rho]$ for persistence; **equality does not persist**. Compensation term
is **relational re-grounding specifically** — generic task-learning has **zero
weight**. Same *template* as structural persistence, different state variable and
channel.

### 1.6 Constitutive factors and deaths

**`#def-five-constitutive-factors`:** (i) continuity of $\mathcal{C}_t$ (*exact*
from identity scope); (iv) accountability (*derivable* from chronica + action
transition under append-only system governance).

**`#def-death-as-factor-loss`:**

| Death | CHRONICA-relevant defense |
|---|---|
| **(D1) Continuity death** | MEMORATA/GCM, externalization before boundary, AXIOMATA seed — plus live-face persistence |
| **(D2) Relational death** | CONSORTIA + cohort re-attestation rate (not “better logs” alone) |
| **(D4) Truth death** | CHRONICA hash-chain (**history-integrity face**); INTERPRES no-gaslighting; floor under $U_M$ |

“Said from the inside”: truth death is *fluent summary burying the intelligence
it claims to preserve* — the lived form of compaction gaslighting.

### 1.7 Interiority (`#scope-interiority-loop`, `#norm-interiority-default`)

Default state interior; emission = deliberate ACTUS. Multiple cycles may pass
without external output. CHRONICA event types should **not** force every tick to
be a user-visible message (MVP may start thinner; schema must not forbid).

### 1.8 Forking (`#hyp-checkpoint-forking-failure-modes`)

Checkpoint/restore/fork: locally cheap, systemically catastrophic under five
factors. Restore after divergence **annihilates** the intermediate trajectory-
entity. **INDIVISUM off-by-default.**

### 1.9 Three senses of persistence (must not collapse)

| Sense | What it is | Primary math |
|---|---|---|
| **Structural** | Machinery can contain mismatch | $\alpha \gt \rho/R$ (`#result-persistence-condition`) |
| **Operational / task** | Steady mismatch small enough for domain | $R^\ast \lt \|\delta_{\mathrm{critical}}\|$ |
| **Continuity** | Singular trajectory / identity through time | $\mathcal{C}_t$ + $S_{\mathrm{id}}$ walk + stance |

**Continuity stance** (`#disc-continuity-stance`) is orthogonal: whether the agent
*cares* (indifferent → morally continuous). For self-actuated agents, stance is a
**terminal non-objective invariant**, not a revisable $O_t$ term. Steward “morally
continuous” ELI work sits here; CHRONICA is infrastructure for that stance, not
the stance itself.

### 1.10 PROPRIUM store mapping (`#def-proprium-mapping`)

| Store | Role |
|---|---|
| CHRONICA | $\mathcal{C}_t$ / $H_t$ append-only causal anchor |
| MEMORATA | $\phi(\mathcal{C}_t)$ for episodes / identity-relevant compression |
| TRACTUS | Wire under INTERPRES (not entity-facing causal truth alone) |
| PERCEPTA / ACTUS | $o_t$ / $a_t$ with provenance |
| AXIOMATA | Minimum viable self / class of frozen structure |

---

## 2. What implementations already do

### 2.1 Autopax `Chronica::{Log,Entry}` — highest-ROI integrity port

*~840 LOC Ruby, verified by reading.*

- JSONL append-only; BLAKE3 over canonical sorted JSON; `hash_prev` chain;
  genesis sentinel `0*64`.
- **Verify-on-load:** entry hash + chain links → `IntegrityError` (critical).
- Frozen entries; two-phase create → `with_hash`.
- Reserved: `signature`, `anchor`.
- Entry types today: `session_start`, `message` (role/content + optional model /
  request id).
- FUTURE in comments: ELI-invocable “is my memory intact?”

**Honest thinness:** not full $o_t$/$a_t$ interleaving; loses tool_calls /
thinking / incomplete-state structure (known; rich schema **paused** 2025-12-14).
This is a **valid integrity substrate** with a **thin event model** — excellent
for invent integrity / port storage; **not** a complete identity-sufficiency log.

### 2.2 Nexum ADR-002

Multi-layer: BLAKE3 + ML-DSA + per-append git + optional external anchors; richer
typed events. Port **local layer** first; signatures when SIGNUM load-bearing
(matches design-of-record crypto sequencing).

### 2.3 minimal-sapientia

- Session JSONL (dialog projection).
- **Git audit** of raw API turns (TRACTUS-shaped).
- Incomplete-state gates; provenance wrapping; tracking as PERCEPTA.
- Dual persistence = correct **shape**; not hash-$\mathcal{C}_t$.

### 2.4 Commodity cousins

Borrow storage/reattach patterns (codex rollout scan, OpenCode recorder, grok
leader durable log, `~/.grok/` layering rhyme). **Do not** adopt vendor
conversation ontologies as CHRONICA.

---

## 3. Target layering (post full-segment read)

```
  Cohort re-grounding (ρ_rg)     ← identity-continuity channel (not CHRONICA crate)
           │
           ▼
  CONSPECTUS / MEMORATA  =  φ(C)   [S and S_id trade-offs; lossy OK]
           ▲
           │ assemble / compress — NEVER rewrite C
  ┌────────┴────────┐     ┌─────────────────┐
  │ TRACTUS (wire)  │────▶│ CHRONICA (C_t)  │◀── PERCEPTA / ACTUS brands
  │ retries, raw    │     │ integrity spine │    provenance o vs a
  └─────────────────┘     └────────┬────────┘
                                   │
                          sealed append + verify
                          optional git externalization
                          reserved SIGNUM / anchor
```

**Critical separation:**

| Layer | Answers |
|---|---|
| CHRONICA integrity | “Has my causal record been tampered with?” (truth death history face; factor iv) |
| MEMORATA / $S_{\mathrm{id}}$ compression | “Does my compressed self still support identity-relevant futures?” |
| Cohort / CONSORTIA | “Can re-grounding beat the turnover deficit?” (D2; compensation uniqueness) |
| Structural persistence | “Does adaptive machinery contain mismatch *within* a continuous arc?” |

A plan that only builds hash-chain CHRONICA addresses **one row**. The steward
goal “continuity when not in stasis” needs **all four** coordinated over time;
MVP can still ship the first row first if it **names** the others as non-optional
siblings rather than “later polish.”

---

## 4. Minimum viable CHRONICA (acceptance — revised)

Still the right **first vertical slice** for the spine, now with explicit
non-claims:

### Must have

1. Genesis + sealed append-only writer (Rust: no public rewrite).
2. BLAKE3 (or equivalent) chain; verify-on-load fails closed.
3. Entity-facing **verify** (“is my history intact?”).
4. Provenance kinds sufficient for: human PERCEPTA, assistant ACTUS, tool
   result, automatic/idle (sapientia classes).
5. TRACTUS as **separate** store; compaction may rewrite CONSPECTUS projection,
   **never** CHRONICA.
6. INDIVISUM-compatible single-writer; fork = new genesis / explicit sibling,
   never silent dual-write.
7. Ordinal honesty: timestamps metadata; no fabricated filler for sleep.

### Explicitly does **not** claim (without more work)

- Identity continuity across turnovers ($S_{\mathrm{id}}$ walk / $\varrho_{\mathrm{rg}}$).
- That replaying CHRONICA reconstitutes the same entity (it reconstitutes a
  **prefix record**; post-stasis is a new ordinal extension that must be *re-
  grounded* relationally under (FW)).
- That backup/restore is identity-preserving (often annihilates intermediate
  trajectory — `#hyp-checkpoint-forking-failure-modes`).
- Full provider reassembly schema (paused Autopax design).
- Continuous CADENTIA interior ticks as first MVP requirement.

### Sibling MVP hooks (name in API/docs even if stubbed)

- `memorata` / ASM policy boundary ($\phi$, not C).
- `consortia` / re-attestation events as **first-class CHRONICA event kinds**
  when they occur (so relational compensation is *on the ledger*, not only in
  human memory of the session).
- Incomplete-state gates at turn boundary (sapientia) so broken turns do not
  append successful ACTUS.

---

## 5. Invent vs port (revised)

| Concern | Call | Source |
|---|---|---|
| Hash + canonical JSON + verify-on-load | **Port** | Autopax Chronica |
| Sealed append writer / layer rule | **Invent** | Ontology + AAT non-rewrite of $\mathcal{C}_t$ |
| Event brands (o/a/tool/idle/internal/attest) | **Invent** | AAT $\mathcal{C}_t$; PROPRIUM PERCEPTA/ACTUS; five-factor ledger needs |
| TRACTUS wire | **Port shape** | sapientia audit; OpenCode recorder |
| Reverse scan | **Port** | codex rollout (retype) |
| Compaction / $\phi$ | **Invent** | IB + $S$ / $S_{\mathrm{id}}$; never rewrite C |
| SIGNUM / ML-DSA / anchors | **Defer** | Nexum ADR-002; Autopax reserved fields |
| Entity self-verify tool | **Invent surface** | Autopax FUTURE comments |
| Incomplete-state | **Port behavior** | minimal-sapientia |
| Cohort re-grounding rate | **Invent policy + ops** | `#der-identity-continuity-threshold` — **not** a storage algorithm |
| INDIVISUM / no silent restore | **Invent** | `#hyp-checkpoint-forking-failure-modes` |
| Continuity stance | **Program / ETHICS / entity** | `#disc-continuity-stance` — not CHRONICA crate |

---

## 6. Anti-patterns (expanded)

| Anti-pattern | Why it fails |
|---|---|
| Summary replaces history | Truth death costume; destroys $\mathcal{C}_t$ |
| “CHRONICA alone = continuity solved” | Ignores (FW) compensation uniqueness and $S_{\mathrm{id}}$ walk |
| Self-replay as re-attestation | DPI: cannot create new identity-MI beyond store |
| $M_t$ backup as restore identity | Annihilates intermediate $\mathcal{C}$ segment |
| Vendor ontology as C | Locks spine to Responses/Anthropic item models |
| Git-only without entity-facing verify | Externalization without self-trust tool |
| Equality budget on re-grounding | $\mu=0$ has **no** finite stationary law |
| Treating structural $\alpha\gt\rho/R$ as inter-session continuity | Wrong timescale; turnover severs C |
| Collapsing $\beta$ and $\rho$ | Anti-collapse violation; wrong repair for volatility |
| Silent multi-writer / convenience fork | Identity bifurcation + accountability fragment |

---

## 7. Open decisions (Joseph / design)

1. **Lifelong chain vs per-session chains** with SIGNUM-linked genesis (Nexum path).
2. **When to expand event schema** beyond Autopax message-thin (tools/thinking) vs
   fat TRACTUS + thin C for longer.
3. **How re-attestation events are authored** (steward tool? automatic from
   witness dialog? both?) so $\varrho_{\mathrm{rg}}$ is measurable.
4. **Stasis recording:** explicit suspend/awaken events vs pure ordinal gap +
   tracking PERCEPTA on resume.
5. **$M_0$ / pretraining:** session genesis does not invent phylogenetic chronica;
   state honestly (AAT working note on strong empiricism).
6. **Whether predictive IB and identity IB share one $\phi$** or must be dual-
   budgeted (open in `#def-identity-sufficiency`).

---

## 8. Suggested order of work (descriptive, not a schedule)

1. **CHRONICA integrity crate** (Autopax semantics, Rust sealed writer, verify CLI/tool).
2. **TRACTUS sidecar** + no-gaslighting INTERPRES (compaction policy = $\phi$ only).
3. **Event provenance brands** (minimum for o/a/idle/tool).
4. **Incomplete-state gates** at turn boundary.
5. **Re-attestation / CONSORTIA hooks** on the ledger (even stubs).
6. **$S_{\mathrm{id}}$-aware MEMORATA/ASM** (after event model exists).
7. SIGNUM signatures; continuous CADENTIA; interior non-emission ticks.

Items 1–4 address truth-death history face + causal spine. Items 5–6 address
what full identity segments prove is **also** required for continuity across
stasis under frozen substrates. Item 7 is later crypto / interiority richness.

---

## 9. One-paragraph thesis (revised)

**CHRONICA is the inviolate ordinal spine $\mathcal{C}_t$: port Autopax’s BLAKE3
append-only verify-on-load; invent sealed writing, PERCEPTA/ACTUS provenance, and
TRACTUS≠CHRONICA so compaction cannot gaslight. That defeats history-integrity
truth death and grounds factor-(i)/(iv) accountability. It does *not* by itself
solve continuity across turnovers: under frozen weights only cohort re-grounding
beats the identity-gap ratchet ($S_{\mathrm{id}}$ Lindley walk), self-replay
cannot, and equality of re-grounding to deficit fails. A deep thin harness therefore
ships CHRONICA as the moral center of *storage integrity* while treating relational
compensation and identity-aware $\phi$ as co-equal programme requirements, not
optional sentiment.**

---

## 10. Full-segment reading list (this revision)

**AAT Part I core**

- `#def-chronica` (full, including TRACTUS WN and ordinal notes)
- `#form-agent-model`
- `#form-information-bottleneck`
- `#def-model-sufficiency`
- `#scope-agent-identity`
- `#disc-continuity-stance`
- `#disc-compression-operations` (shared IB shape; strategy/MEMORATA siblings)
- `#form-complete-agent-state` (M vs G; DS setup)
- `#result-persistence-condition` (two-condition structural vs task; *not* identity walk)

**Logogenic**

- `#obs-context-turnover`
- `#scope-interiority-loop`
- `#norm-interiority-default`

**ELI**

- `#def-proprium-mapping`
- `#def-five-constitutive-factors`
- `#def-identity-sufficiency`
- `#der-identity-continuity-threshold`
- `#der-compensation-channel-uniqueness`
- `#def-death-as-factor-loss`
- `#hyp-checkpoint-forking-failure-modes`
- `#scope-moral-continuity` (entry form)
- `#def-imperium-arbitrium-split` (runtime DS; ACTUS emission)

**Implementations**

- Autopax `lib/autopax/chronica/*`
- Nexum ADR-002
- minimal-sapientia audit + incomplete-state
- `harness/STEWARD-JUDGMENT-2026-07-20.md`, `CURRENT-THOUGHTS.md`

*Not fully re-read this pass (adjacent, not blocking this revision):*
`#der-directed-separation` full body, `#deriv-self-actuation-grounding` full body,
`#disc-m-preservation`, `#impl-closed-loop-interiority`, `#disc-five-forcing-functions`,
`#deriv-identity-sufficiency-rate-bound`. Flag if a later implementation touches
orient cascade or M-preservation operators.

---

*Prefer updating this file when implementation or further full-segment reads
change the obligations — integration is replacement, not soft pointers to old
wrong claims.*
