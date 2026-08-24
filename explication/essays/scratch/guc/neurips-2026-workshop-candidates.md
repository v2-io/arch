# NeurIPS 2026 workshop candidates for an LLMs × Directed Separation paper

*2026-08-24. Source: [workshop announcement](https://blog.neurips.cc/2026/08/10/announcing-the-neurips-2026-workshops/) + each workshop's CFP page (fetched today; deadline details are from the pages' own text via a summarizing fetch — re-verify page limits on the actual CFP before formatting). Today is Aug 24 — the confirmed deadlines are 3–5 days out.*

## Recommended (ranked)

### 1. Foundations of Agentic Systems Theory (FAST) — Paris — **due Aug 29 AoE**

https://fast-workshop.github.io/ · Full 7pp / short 4pp (excl. refs), NeurIPS template, OpenReview, double-blind.
The closest thing to a bespoke venue: theoretical frameworks for LLM-based agent systems, "definitions and philosophy of agency," mechanisms of emergent behavior, compositional safety. The GUC Class 1/2/3 partition + κ_processing + wrapper theorems (W₀/W₁/W₂, leakage bound via DPI, Brooks's-Law tempo cost) is exactly a "foundations of agentic systems theory" contribution. **Best feedback audience for the theory itself.**

### 2. Foundations of Language Model Security — Paris — **due Aug 27 AoE** (earliest!)

https://flmsec.github.io/ · Up to 8pp (excl. refs), non-archival, OpenReview, double-blind.
Explicitly seeks "formal frameworks," "provable security results and impossibility results," and asks whether vulnerabilities are fundamental attack surfaces or design artifacts — which is precisely what the classification answers: prompt injection / adversarial goal-drag as *structural* consequences of Class 3 (no goal-blind processing channel exists to defend), with the certifiability-vs-behavioral-bound distinction (Class-1-by-structure vs by-behavior) as the security-relevant deliverable. **Angle: "separation guarantees for scaffolded LLMs: what is certifiable, what is only behavioral."**

### 3. Who Verifies the Agents? — Sydney — **due Aug 29 AoE**

https://verify-agents-workshop.github.io/ · 4–9pp (excl. refs).
Verification as a discipline for reliable agents; formal verification track. The fit is the *certificate* story: the Class 1↔2 boundary is a certifiability boundary (behavior identical at κ=0; what changes is whether the zero can be certified by architecture inspection vs only measured behaviorally) — a clean thesis for "what can and cannot be verified about an agent scaffold."

### 4. The Third Workshop on Agents in the Wild: Safety, Security, and Beyond — Sydney — **due Aug 29 AoE**

https://agentwild-workshop.github.io/neurips2026/ · Regular 9pp / short 4pp. ⚠ *"Authors must create OpenReview profiles by August 15, 2026"* per the fetched summary — check whether existing profiles suffice (they should; Joseph presumably has one from prior submissions).
Broader/safer-bet venue: agent safety, control, oversight, interpretability. Would want the deployment-relevant framing (the κ̂ behavioral probe, sycophancy/motivated-reasoning diagnostic, IDT-style sidecar monitoring) forward, theory as support.

## Checked, weaker or unconfirmed

- **Interpretability as a Science (Sydney)** — thematically decent (κ̂ probe as a rigorous diagnostic; the Class-3 lemma about attention), but the CFP details weren't reachable today (site's CFP page 404s from here; contact interpscience@gmail.com). Only pursue if one of the above falls through.
- **Dynamic Alignment in Human-AI Coupled Systems (BiAlign, Sydney)** — "coupled systems" in the title but the site is a JS app that yields nothing to fetch; scope appears human-AI-interaction-flavored rather than architectural. Not chased further.
- Meta Agents, AgenticOS, PALM, SLM-Agents, EconML — each touches composition/scaffolding but the directed-separation core would be off-center.

## Paper-shape notes (from the asf substrate)

A 4- or 8-page paper is extractable largely from existing canon rather than new derivation:

1. **Setup:** directed separation ($f_M$ goal-blind conditional on realized event; selection ≠ processing) — `der-directed-separation.md`.
2. **Classification:** GUC Class 1/2/3, κ_processing, the two different boundary kinds (certifiability vs behavioral).
3. **LLM result:** transformers are Class 3 by construction — NeurIPS Paper 3's `#lem-attention-coupled` (lemma-grade, robust to RMSNorm/FlashAttention/masking, extended to SSMs/RWKV/linear attention under non-degeneracy) is already-written machinery to draw on. ⚠ Check Paper 3's own status first — if it's under review at the main conference, the workshop paper must be differentiated (the classification + wrapper-guarantee story, citing the lemma, rather than re-presenting it).
4. **Wrapper theorems:** class coercion W₁/W₂, structural vs behavioral leakage bounds, tempo cost — `der-class-coercion-via-wrapping`, prior-art analysis §2 novelty items 2–4.
5. **Positioning:** the 50-year control-theory arc (Feldbaum → Wonham/Witsenhausen → Bar-Shalom & Tse dual effect → Derpich & Yüksel 2023 correction) per `chronology.md` in this dir — the blind-spot thesis: policy-level separation was mapped for decades; substrate-level goal→processing coupling is what modern AI runs on, without the vocabulary or no-go results.

Non-archival workshops (flmsec explicitly; likely others) mean this doesn't burn the material for a full venue later — verify each workshop's archival policy before choosing.

## Open before committing

- Hafez et al. 2026 (*The Informational Cost of Agency*, DOI 10.21203/rs.3.rs-9683103/v1) is now in relata as `hafez-2026-informational-cost` with local PDF (markdown prep in progress as of 2026-08-24). All verification fields still `unverified` — read it whole before the paper cites the 89%/44% IDT numbers or the (S,A,S′)-sidecar characterization that `der-directed-separation.md` carries from the catalog era.

- Re-verify page limits / deadlines / archival policy on the actual CFP pages (summaries above are one fetch-hop removed).
- OpenReview profile status (agentwild's Aug 15 profile-creation cutoff wording).
- Whether Joseph wants one submission or two (FAST + flmsec are different-enough angles that a 4-page each is conceivable; the Aug 27 flmsec date is the forcing clock).
