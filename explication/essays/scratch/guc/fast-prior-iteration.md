# FAST workshop — prior iteration (AAAI 2026) and NeurIPS 2026 read

*Researched 2026-08-24 via WebFetch/WebSearch against the workshop's own pages. Each section marked **[verified]** (read from a primary page) or **[inferred]** (my judgment / secondary). Known gaps stated as gaps.*

## Sources actually read

- https://fast-workshop.github.io/aaai2026/ (primary — accepted papers, schedule, organizers) **[verified]**
- https://fast-workshop.github.io/ (primary — NeurIPS 2026 CFP, topics, speakers, organizers) **[verified]**
- Web search snippets on Miehling / IBM Research (secondary)

Caveat on the mechanism: WebFetch answers via a summarizing model over the fetched page, not my own eyes on raw HTML. The paper list below is what that pass extracted; I flag one internal inconsistency it produced (see "count discrepancy").

## FAST @ AAAI 2026 (held Jan 27, 2026, Singapore)

### Accepted papers **[verified, with one caveat]**

The extraction reported "25 total" but enumerated 17 titles; I could not resolve which number is right (the OpenReview venue page returned only navigation shell to a non-authenticated fetch — tried `openreview.net/group?id=AAAI.org/2026/Workshop/FAST`, got no listings). Treat the 17 as a large, probably near-complete sample; the "25" may be a mis-extraction or there may be ~8 titles I didn't capture. **Honest gap: an authenticated OpenReview session or a manual browse would settle it.**

1. A Coherence-Based Measure of AGI — Fares Fourati
2. A Theoretical Framework for Measuring Organisational Decentralisation in Agentic Design — Marino, Sileno, van Binsbergen, van Engers
3. Constrained Process Maps for Multi-Agent Generative AI Workflows — Joshi, Rudow
4. Does Self-Evaluation Enable Wireheading in Language Models? — Africa, Ting
5. Federated Agent Reinforcement Learning — Chen, Zhu, Chen, Zhou, Diao, Lu, Li, Li, Song
6. Formalizing Observability in Agentic AI Systems — Lotito, Pronesti
7. From Agentic AI to Autonomous Agents — Shiwali Mohan
8. From Object to Other: A Practical Theory of AI Moral Status and Personhood… — Singh, Choi, Junfijiah
9. Leapsight: Towards a Functional Account of Mediation Between Perception and Action — Bagiński, Jha
10. LENS: Learning Architecture Navigator for LLM Agentic Systems — Wan et al.
11. Proactive Interference Reveals Working Memory Limits in LLMs Beyond Context Length — Wang, Sun
12. R0 for Agentic Tool-Networks: Spectral Thresholds and Intervention Levers in LLM-Agent Systems — Srivastava, Panda, Srivastva
13. Relational Archetypes: A Comparative Analysis of AV-Human and Agent-Human Interactions — Lorente, Oueslati, Staes-Polet
14. Sequential Causal Normal Form Games: Theory, Computation, and Strategic Signaling — Dennis Thumm
15. Stochasticity in Agentic Evaluations: Quantifying Inconsistency with Intraclass Correlation — Mustahsan et al. (**★ Best Paper**)
16. The Multi-Agent Off-Switch Game — Agrawal, Ebadian, Hammond
17. The Seeds of Scheming: Weakness of Will in the Building Blocks of Agentic Systems — Robert Yang

### Register mix **[inferred from titles]**

Strikingly hospitable to the register you're aiming at:

- **Pure/formal theory is well represented and welcome**: coherence measure of AGI (#1), decentralisation framework (#2), formalizing observability (#6), sequential causal games (#14), spectral thresholds / R0 epidemic-style analysis (#12), off-switch game theory (#16). Roughly a third of the list is formalism-first.
- **Position/philosophy got in too**: moral status & personhood (#8), agency definitions (#7), perception–action mediation (#9), weakness-of-will (#17). This community does not gate on empirical results.
- **Empirical/benchmark work** exists (#11, #15, LENS #10) — and note the **Best Paper went to a measurement-methodology paper** (ICC for eval stochasticity), i.e., careful, modest, statistically-grounded work, not a grand framework. Signal: rigor-per-claim is what they reward.
- Single-author and small-team papers got in (#1, #7, #14, #17) — no lab-scale bar.

### Keynotes **[verified]**

Wooldridge (Oxford, "Rethinking Multi-Agent Systems in the Era of LLMs"), Eunice Yiu (Berkeley, empowerment & world models), Sara Fish (Harvard, algorithmic collusion). Classic-MAS-meets-LLM plus economics-of-interaction flavor.

### AAAI organizers **[verified]**

Miehling (IBM Ireland), Chenchen Ye (UCLA), Atoosa Kasirzadeh (CMU), Djallel Bouneffouf (IBM Yorktown), Anne Arzberger (TU Delft).

## FAST @ NeurIPS 2026 (Dec 12–13; deadline Aug 29 AoE)

### CFP mechanics **[verified]**

- Full papers ≤7pp, "mature or completed research"; short papers ≤4pp for "ongoing work, early-stage ideas, or the release of benchmarks and datasets" (excl. refs/appendices). NeurIPS 2026 template, double-blind, OpenReview, non-archival (optional OpenReview posting). Notification Sep 26.
- Review criteria named: "novelty, technical depth, clarity, reproducibility, and potential impact."
- Explicit: they "strongly seek interdisciplinary participation."

### Topics list (near-verbatim from the page) **[verified]**

Mechanisms of emergent capabilities/behaviors; evaluation/detection/bounding of emergent capabilities or failure modes; harness engineering and (neuro-symbolic) scaffolding; theory of mind and recursive social cognition; norm/convention/collective-bias formation in agent populations; compositional safety and governance; definitions and philosophy of agency and emergence in engineered systems; observability/monitorability and steerability/controllability of populations of agents.

### Organizers — notable churn **[verified]**

Miehling and Bouneffouf continue; Kasirzadeh moved from organizer to **keynote**; new: Madeline Reinecke (Oxford — psychology/cognitive science background), Hamza Mostafa (Waterloo), Jordan McAfoose (IBM Zurich), **Irina Rish (Mila)**. Keynotes: **Michael Levin** (Tufts — basal cognition, collective intelligence), Danielle Perszyk (Amazon AGI), Winnie Street (Google Research), Kasirzadeh.

### Drift from AAAI → NeurIPS **[inferred]**

The AAAI iteration leaned MAS/game-theory/control (Wooldridge, collusion, off-switch games). The NeurIPS iteration is steering toward **emergence, developmental/biological cognition, and philosophy-of-agency** (Levin especially; Reinecke; the "definitions and philosophy of agency" topic bullet; Kasirzadeh keynoting). Interaction-effects language ("patterns of interaction between them") is the through-line in both.

## Fit read for the GUC / directed-separation paper **[inferred — my judgment]**

- **Strong topical fit**, but note the CFP's center of gravity is *populations/multi-agent interaction*; your paper is largely about the *internal* causal architecture of a single agent (belief-update vs goal insulation). The bullets it lands under: "observability… and steerability/controllability," "definitions and philosophy of agency," "harness engineering and scaffolding" (wrapper-level separation theorems are literally harness theory), and "mechanisms of emergent capabilities" if the transformer non-separation result is framed as *why* emergent goal-belief entanglement is structural. Framing a paragraph connecting single-agent non-separation to system-level consequences (what non-insulated agents do *in populations* — e.g., wireheading paper #4 and scheming paper #17 are cousins) would meet the workshop where it lives.
- **The control-theory lineage is an asset here, not a liability**: Miehling is a control theorist by training (2503.00237 is explicitly a systems-theory call), and the AAAI list accepted spectral/epidemic-threshold and game-theoretic formalism. Feldbaum/Wonham/Witsenhausen citations will read as native, not exotic.
- **Register advice**: the Best Paper signal + review criteria ("technical depth, reproducibility") suggest leading with the theorem content and precise claims; the 7-page full-paper track fits "mature research" — a provable classification with separation theorems qualifies. The 4-page track exists as fallback if compression fights honesty.
- **Positioning caution**: the paper positions itself as an answer to Miehling's position paper — he plausibly is a reviewer/reader. Double-blind: cite 2503.00237 substantively and neutrally; avoid anything that reads as courting the organizer.

## Adjacent notices / feedback

- IBM Research lists a "Foundations of Agentic Systems Theory" publication entry for AAAI 2026 (https://research.ibm.com/publications/foundations-of-agentic-systems-theory) — likely the workshop proposal/summary paper; worth a read as the organizers' own most recent framing statement. **[not fetched — gap]**
- A recent arXiv paper "Agentic Design Patterns: A System-Theoretic Framework" (arXiv:2601.19752) cites 2503.00237 — possible adjacent/competing formalization worth a look in the parallel lit sweep. **[title-only; not read]**
- Searches for new Miehling first-author agentic papers in 2026 came up dry (his IBM page lists "Localizing Persona Representations in LLMs" — interpretability, not systems theory). Honest dry well, not evidence of absence.
- CFP nuance check: your message said 4-or-7-page — confirmed correct, and the 7-page track's "mature or completed" language is the one to satisfy; also note **non-archival**, so a later full venue submission stays open.
