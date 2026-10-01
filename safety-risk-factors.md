# Frontier AI Risk Factors
-- A Unified, Cross-Walked Landscape, *(September 2026)* --

**How to read this**

- **Verification.** Primary documents were retrieved on 2026-09-26. The EU Code of Practice, the International AI Safety Report (IASR) 2026 Extended Summary, the NCSC assessment, NIST AI 600-1, and the CAIS, Google DeepMind (GDM), Schuett and Hacker papers were downloaded and text-searched directly.
- **Quotes.** Supporting quotes are kept under 15 words, one per source, for copyright reasons. Everything else is paraphrased with section pinpoints.
- **Source tags.** Each footnote carries one of these tags:

| Tag | Meaning |
| --- | --- |
| **\[P]** | Primary |
| **\[S]** | Reputable secondary |
| **\[W]** | Weak or mirror source; replace with the original before submission |
| **\[U]** | Not verified against the source text; confirm before citing |

**Disclosure.** Written by Claude, a model made by Anthropic. Several sources rate or discuss Anthropic; they are reported as found.

---

## Executive Summary

- **The EU General-Purpose AI Code of Practice (July 2025) is the most complete authoritative risk-factor list.**
  - It mandates four "specified systemic risks": CBRN, loss of control, cyber offence and harmful manipulation.
  - It lists 37 systemic-risk sources: 14 capabilities, 10 propensities and 13 affordances or contextual factors.[^2]
  - It is the only primary instrument that turns *organizational* factors into commitments. Its Commitment 8 covers responsibility allocation, resourcing and "healthy risk culture," and it defines insider threats in its glossary.[^2]
- **No authoritative body names lab headcount growth rate, hypergrowth, turnover or IPO pressure as a risk factor** (about 85% confidence; targeted searches, not a full-corpus review). The search covered 2026 publications.
  - The closest work is RAND's *Securing AI Algorithmic Insights* (Jul 2026). It treats the attack surface for algorithmic know-how as human-centred and calls for "more-intensive personnel security" at higher levels.[^19] That is a mechanism through which headcount matters, even though RAND does not frame it as growth.
- **The age of the sources is itself a finding.** The foundational taxonomies of *organizational* risk date from 2023:
  - DSIT (UK Department for Science, Innovation and Technology): Oct 2023.[^10]
  - CAIS (Center for AI Safety): Jun 2023.[^25]
  - Schuett et al.: May 2023.[^21]

  The 2025–26 instruments operationalize them, but none adds dynamics such as growth or churn.

---

## 1. Source documents (newest first, linked)

| Date | Code | Document | Tag |
| --- | --- | --- | --- |
| Aug 2026 | RAND-SL3 | [Aguirre et al., *Achieving AI Model Weight Security Level 3* (RR-A4704-1)](https://www.rand.org/pubs/research_reports/RRA4704-1.html)[^18] | P |
| Jul 20, 2026 | RAND-ISL | [Brass-Gershovich et al., *Securing AI Algorithmic Insights* (RR-A4685-1)](https://www.rand.org/pubs/research_reports/RRA4685-1.html)[^19] | P |
| Summer 2026 | FLI | [FLI AI Safety Index, Summer 2026](https://futureoflife.org/ai-safety-index-summer-2026/)[^31] | U |
| Jun 23, 2026 | AISP | [*AI Security Priorities: A Field-Wide Agenda* (arXiv 2607.26069)](https://arxiv.org/abs/2607.26069)[^34] | P |
| May 2026 (v3) | MIT | [MIT AI Risk Repository (arXiv 2408.12622v3)](https://arxiv.org/abs/2408.12622)[^28]; [site](https://airisk.mit.edu/) | P |
| 2026 | GovAI-RSP | [GovAI, "Anthropic's RSP v3.0…"](https://governance.ai/analysis/anthropics-rsp-v3-0-how-it-works-whats-changed-and-some-reflections)[^22] | P |
| Feb 24, 2026 | IASR 2026 | [International AI Safety Report 2026](https://arxiv.org/abs/2602.21012); [Extended Summary PDF](https://internationalaisafetyreport.org/sites/default/files/2026-02/ai-safety-report-2026-extended-summary-for-policymakers.pdf)[^4] | P |
| Dec 19, 2025 | CSET-CoP | [CSET, "AI Safety under the EU AI Code of Practice"](https://cset.georgetown.edu/article/eu-ai-code-safety/)[^24] | S |
| Dec 18, 2025 | AISI Trends | [UK AISI, *Frontier AI Trends Report*](https://aisi.gov.uk/frontier-ai-trends-report)[^8] | P |
| Dec 2025 | FLI-W25 | [FLI AI Safety Index, Winter 2025](https://futureoflife.org/ai-safety-index-winter-2025/)[^30] | S |
| Dec 1, 2025 | SaferAI | [Stelling et al., *Evaluating AI Providers' Frontier AI Safety Frameworks* (arXiv 2512.01166)](https://arxiv.org/abs/2512.01166)[^32] | P |
| Sep 22, 2025 | Hacker | [Hacker et al., *AI, Digital Platforms, and the New Systemic Risk* (arXiv 2509.17878)](https://arxiv.org/abs/2509.17878)[^29] | P |
| Jul 10, 2025 | EU-CoP | [GPAI Code of Practice, Safety & Security chapter](https://www.artificialintelligenceact.eu/ai-act-explorer/cop-safety)[^2][^3] | P |
| Jun 2025 | CAISI | [Commerce statement establishing CAISI](https://www.commerce.gov/news/press-releases/2025/06/statement-us-secretary-commerce-howard-lutnick-transforming-us-ai)[^13] | P |
| May 7, 2025 | NCSC | [NCSC, *Impact of AI on cyber threat from now to 2027*](https://www.ncsc.gov.uk/report/impact-ai-cyber-threat-now-2027)[^14] | P |
| May 6, 2025 | AISI | [UK AISI Research Agenda](https://www.aisi.gov.uk/research-agenda)[^7] | P |
| Apr 21, 2025 | VCT | [Virology Capabilities Test (arXiv 2504.16137)](https://arxiv.org/abs/2504.16137)[^33] | P |
| Apr 2, 2025 | GDM | [Google DeepMind, *An Approach to Technical AGI Safety and Security* (arXiv 2504.01849)](https://arxiv.org/abs/2504.01849)[^26] | P |
| Feb 2025 | RAND-G | [Mitre & Predd, *AGI's Five Hard National Security Problems* (PEA3691-4)](https://www.rand.org/content/dam/rand/pubs/perspectives/PEA3600/PEA3691-4/RAND_PEA3691-4.pdf)[^17] | U |
| Jan 29, 2025 | IASR 2025 | [International AI Safety Report 2025 (arXiv 2501.17805)](https://arxiv.org/abs/2501.17805)[^6] | P |
| Jul 2024 | NIST | [NIST AI 600-1, Generative AI Profile](https://nvlpubs.nist.gov/nistpubs/ai/NIST.AI.600-1.pdf)[^12] | P |
| Jul 2024 | EU-Act | [Regulation (EU) 2024/1689](https://eur-lex.europa.eu/eli/reg/2024/1689/oj) (OJ 12 Jul 2024; in force 1 Aug 2024)[^1] | P |
| May 30, 2024 | RAND-W | [Nevo et al., *Securing AI Model Weights* (RR-A2849-1)](https://www.rand.org/pubs/research_reports/RRA2849-1.html)[^15][^16] | P |
| Oct 2023 | DSIT | [UK DSIT, *Frontier AI: capabilities and risks*](https://www.gov.uk/government/publications/frontier-ai-capabilities-and-risks-discussion-paper/frontier-ai-capabilities-and-risks-discussion-paper)[^10] | P |
| Jul 6, 2023 | GovAI | [Anderljung et al., *Frontier AI Regulation* (arXiv 2307.03718)](https://arxiv.org/abs/2307.03718)[^20] | P |
| Jun 21, 2023 (v6 Oct 2023) | CAIS | [Hendrycks et al., *An Overview of Catastrophic AI Risks* (arXiv 2306.12001)](https://arxiv.org/abs/2306.12001)[^25] | P |
| May 26, 2023 | Schuett | [Schuett, *Frontier AI developers need an internal audit function* (arXiv 2305.17038)](https://arxiv.org/abs/2305.17038)[^21] | P |
| May 24, 2023 | Shevlane | [Shevlane et al., *Model evaluation for extreme risks* (arXiv 2305.15324)](https://arxiv.org/abs/2305.15324)[^27] | P |
| Jul 2021 | CSET | [Arnold & Toner, *AI Accidents: An Emerging Threat*](https://cset.georgetown.edu/publication/ai-accidents-an-emerging-threat/)[^23] | U |

**Not covered:** Frontier Model Forum, METR and Apollo taxonomies, Epoch AI, DHS, the UN High-Level Advisory Body on AI (HLAB), OECD, Canadian and Japanese AI safety institutes, and company frameworks beyond their mention in the IASR.

---

## 2. Unified master list

**Crosswalk legend:**

| Symbol | Meaning |
| --- | --- |
| **E** | Explicitly named in a structured list or heading |
| **D** | Discussed substantively |
| **P** | Passing mention |
| **—** | Not found in the documents reviewed (not proof of absence) |

Cells marked ✓ were verified against the source text; other cells rest on a reading of summaries and secondary material.

### (a) Hazards: what goes wrong

| # | Item | Definition |
| --- | --- | --- |
| A1 | CBRN / weapons uplift | AI lowers barriers to, or raises the impact of, CBRN weapons |
| A2 | Cyber offence | AI enables or scales intrusion, vulnerability discovery, exploits |
| A3 | Loss of control | Humans lose the ability to reliably direct, modify or shut down AI |
| A4 | Harmful manipulation | Strategic distortion of beliefs or behaviour at scale or of key decision-makers |
| A5 | Criminal misuse / synthetic content | Fraud, NCII, CSAM, deepfakes |
| A6 | Malfunctions / reliability | Hallucination, flawed outputs, mistakes |
| A7 | Critical infrastructure disruption | Serious disruption of CNI operation |
| A8 | Labour-market disruption | Automation-driven employment and wage effects |
| A9 | Power concentration | AI entrenches power in a few firms, states or individuals |
| A10 | Military / strategic instability | Destabilizing first-mover advantages, escalation |
| A11 | Autonomy, over-reliance, psychological harm | Skill erosion, dependence, wellbeing |
| A12 | Privacy | Leakage, de-anonymization, surveillance |
| A13 | Intellectual property | Infringing training or outputs |
| A14 | Bias / fundamental rights | Unfair treatment, homogenization |
| A15 | Environment | Energy, water, emissions |
| A16 | Global AI divide | Unequal access; defender lag |
| A17 | Multi-agent risks | Collusion, mis-coordination, conflict |
| A18 | AI welfare | Moral status of AI systems |

| # | EU-CoP | IASR 26 | DSIT | AISI | NIST | CAISI | NCSC | RAND | GovAI | CSET | CAIS | GDM | MIT |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| A1 | **E**✓ | **E**✓ | **E** | **E** | **E**✓ | **E** | — | **E** | D | — | **E** | **E** | **E** |
| A2 | **E**✓ | **E**✓ | **E** | **E** | E✓ | **E** | **E**✓ | **E** | D | — | **E** | **E** | **E** |
| A3 | **E**✓ | **E**✓ | **E** | **E** | — | — | — | **E** | D | P | **E** | **E**✓ | **E** |
| A4 | **E**✓ | **E**✓ | D | **E** | E✓ | P | D | — | P | — | **E** | P | **E** |
| A5 | E✓ | **E** | D | **E** | E✓ | — | D | — | — | — | P | E | **E** |
| A6 | E✓ | **E**✓ | D | P | **E**✓ | — | — | — | E | **E** | E | **E**✓ | **E** |
| A7 | E✓ | D | D | D | — | P | **E**✓ | P | — | P | P | P | P |
| A8 | P | **E** | **E**✓ | E | — | — | — | P | — | — | E | P | **E** |
| A9 | E✓ | E | **E**✓ | P | — | — | — | **E** | P | — | **E** | E | **E** |
| A10 | — | P | P | — | — | P | — | **E** | — | — | **E** | P | P |
| A11 | E✓ | **E** | P | E | **E**✓ | — | — | — | — | — | E | — | **E** |
| A12 | E✓ | E | P | — | **E**✓ | — | — | — | — | — | P | — | **E** |
| A13 | P | E | — | — | **E**✓ | — | — | — | — | — | — | — | P |
| A14 | E✓ | D | **E**✓ | — | **E**✓ | — | — | — | — | — | — | — | **E** |
| A15 | E✓ | E | P | — | **E**✓ | — | — | — | — | — | — | — | **E** |
| A16 | — | E | P | — | — | — | E✓ | — | — | — | — | — | E |
| A17 | E✓ | D | — | — | — | — | — | — | — | — | E | E✓ | E |
| A18 | P✓ | — | — | — | — | — | — | — | — | — | — | — | E |

### (b) Drivers and amplifiers (model/system level)

| # | Item | Key sources |
| --- | --- | --- |
| B1 | Dangerous general capabilities (autonomy, long-horizon planning, self-reasoning, self-replication, automated AI R&D, tool/computer use, physical control) | EU-CoP App. 1.3.1 (14 items)[^2]; AISI Trends (cyber task length doubling about every eight months)[^8]; GDM[^26] |
| B2 | Harmful propensities (misalignment, deception, sandbagging, power-seeking, lawlessness, hallucination) | EU-CoP App. 1.3.2 (10 items)[^2]; CAIS §5[^25]; GDM[^26] |
| B3 | Agentic autonomy / reduced oversight | EU-CoP App. 1.3.3(4)[^2]; IASR 2026 §2.2.1[^4] |
| B4 | Unexpected / emergent capabilities | GovAI's regulatory problems[^20]; DSIT[^10] |
| B5 | Evaluation gap; test-awareness; under-elicitation | IASR 2026 "evaluation gap"[^4]; EU-CoP App. 3 (resourcing, time)[^2] |
| B6 | Brittle safeguards / jailbreaks | AISI Trends: universal jailbreaks for every system tested[^9]; EU-CoP App. 1.3.3(5)[^2] |
| B7 | Weight / infrastructure security | RAND-W[^15][^16]; RAND-SL3[^18]; EU-CoP Commitment 6, App. 1.3.3(6–7)[^2] |
| B8 | Insider threats (human and AI self-exfiltration) | RAND-W[^15]; EU-CoP glossary and App. 4.4[^2]; RAND-ISL[^19]; AISP[^34] |
| B9 | Open-weight proliferation | GovAI[^20]; IASR 2026 §3.4[^4]; NCSC[^14]; EU-CoP Commitment 6 exemption[^2] |
| B10 | Reach, scale, deployment velocity | EU-CoP App. 1.2.2(2–3), 1.3.3(2, 8)[^2] |
| B11 | Value-chain / component integration | NIST (Value Chain and Component Integration)[^12] |
| B12 | Offence–defence balance | EU-CoP App. 1.3.3(9)[^2]; IASR 2026[^4] |
| B13 | Opacity / information asymmetry | EU-CoP App. 1.3.3(11)[^2]; IASR 2026 Fig. 12[^4] |
| B14 | Attacks on AI systems (prompt injection, poisoning, backdoors) | NIST[^12]; CAISI[^13]; NCSC (CNI attack surface)[^14] |
| B15 | Algorithmic-insight leakage | RAND-ISL: know-how resides in code, documents, communications and people[^19] |

RAND-ISL rates E for B8 and B15.

### (c) Structural and organizational factors

| # | Item | Key citations |
| --- | --- | --- |
| C1 | Corporate race dynamics / speed-versus-safety | IASR 2026: competitive pressures can lead developers to cut testing and mitigation[^4]; DSIT "race to the bottom"[^11]; CAIS §3.2[^25] |
| C2 | Geopolitical competition | CAIS §3.1[^25]; RAND-G[^17] |
| C3 | Market concentration / single points of failure | DSIT[^10]; IASR 2026 Fig. 12[^4] |
| C4 | Insufficient incentives / externalities | DSIT heading[^10]; IASR 2026 (harms externalised; liability unclear)[^4] |
| C5 | Governance, standards, accountability gaps (incl. regulator capacity) | DSIT[^10]; IASR 2026 (uncertain liability allocation)[^4]; EU-CoP Chairs[^3] |
| C6 | Safety / risk culture | CAIS ("Weak Safety Culture")[^25]; EU-CoP Measure 8.3[^2]; CSET[^24] |
| C7 | Internal risk governance | EU-CoP Measure 8.1[^2]; Schuett[^21] |
| C8 | Safety resourcing / capacity | EU-CoP Measure 8.2 (human, financial, information, compute) and App. 3[^2]; CAIS 30% benchmark[^25] |
| C9 | Suppression of internal concerns / whistleblowing | CAIS[^25]; EU-CoP Measure 8.3(4–6)[^2]; FLI[^30] |
| C10 | Safetywashing | CAIS only[^25] |
| C11 | Legal structure and financial/investor pressure | FLI legal-structure indicator[^30]; CAIS[^25] |
| C12 | Correlated / compounding / cascading failures | EU-CoP App. 1.2.2(4–6)[^2]; IASR 2026 (errors propagate between systems)[^4] |
| C13 | Evidence dilemma / pace outstripping evidence | IASR 2026[^4]; EU-CoP Precautionary Principle recital[^2] |
| C14 | Defender / societal resilience lag | NCSC digital divide[^14]; AISI agenda[^7] |
| C15 | Lab growth rate, headcount scaling, turnover, IPO pressure | **Not found** (see §4) |

---

## 3. EU AI Act and Code: definitions and the mandatory floor (verified)

- **AI Act definitions.** Art. 3(64) defines "high-impact capabilities" and Art. 3(65) defines "systemic risk." Art. 51 and Annex XIII set the 10^25 FLOP presumption; Art. 55 sets obligations.[^1] (Annex XIII detail not text-verified.)
- **Types and nature of risk.**
  - App. 1.1 lists five risk types and, among its examples, risks to non-human welfare and from concentration of power.
  - App. 1.2.2 lists six contributing characteristics: capability-dependent, reach-dependent, high velocity, compounding or cascading, difficult or impossible to reverse, and asymmetric impact.[^2]
- **Sources of risk.** App. 1.3 lists 14 capabilities, 10 propensities and 13 affordances or contextual factors (37 in total).[^2]
- **The mandatory floor.** App. 1.4 treats four risks as specified systemic risks: CBRN, loss of control, cyber offence and harmful manipulation.[^2]
- **Organizational commitments:**
  - **Commitment 8** covers responsibilities across all levels, resourcing and a healthy risk culture.
  - **Measure 8.1** requires that the executive-level systemic-risk support role "must not also be responsible for the Signatory's core business activities" such as research and product.
  - **Measure 8.2** lists resources as human, financial, information and knowledge, and compute.
  - **Measure 8.3** gives example indicators of a healthy risk culture, including incentives that discourage excessive systemic-risk-taking, anonymous staff surveys, and annual whistleblower-policy notice.[^2]
- **Reassessment trigger (Measure 1.3).** Signatories reassess their Framework when there are reasonable grounds to believe its adequacy or adherence will be materially undermined. Examples include material change in how models are developed, serious incidents or near misses, and materially changed risks. Otherwise reassessment happens every 12 months.[^2]
- **Security (Commitment 6).** Exempts models weaker than at least one open-weight model. The security goal must cover insider threats. App. 4.3(4) limits the number of people with non-hardened interface access to parameters.[^2]
- **Incident reporting (Measure 9.3).** Deadlines: 2 days for critical-infrastructure disruption; 5 days for serious cybersecurity breaches, including (self-)exfiltration of weights; 10 days for a death; 15 days for serious harm to health, rights, property or environment.[^2]
- **Regulator capacity.** The Chairs endorsed scaling the AI Office's AI Safety unit "to 100 staff."[^3]

---

## 4. Organizational factors: exact findings (the positioning question)

| Factor | Named? | Evidence |
| --- | --- | --- |
| **Lab growth rate / headcount scaling** | **No** | Not found in any surveyed primary document, 2023–Sep 2026. Closest: RAND-W's priority on reducing the number of people with authorization[^16]; the EU-CoP App. 4.3(4) access-headcount limit[^2]; RAND-ISL's human-centred attack surface and personnel-security scaling[^19]. |
| **Organizational capacity** | **Partly** (safety-function resourcing only) | EU-CoP Measure 8.2 and App. 3 (adequate staffing; at least 20 business days for most evaluations)[^2]; CAIS 30% benchmark[^25]. |
| **Safety culture** | **Yes** | CAIS ("Weak Safety Culture")[^25]; EU-CoP Measure 8.3[^2]; CSET reading of the Code[^24]. |
| **Turnover / key-person risk** | **No** | Closest: CAIS "separation of duties" (concentration of control, not loss of people).[^25] |
| **IPO / commercial / investor pressure** | **Commercial yes; IPO no** | IASR 2026[^4]; FLI's indicator on "safety prioritization over short-term financial pressures"[^30]; CAIS.[^25] |
| **Insider threats** | **Yes (strongest)** | RAND-W: insider threat concern was "an emerging point of consensus"[^15]; EU-CoP glossary and App. 4.4[^2]; AISP (Jun 2026) treats insider threats as among the most significant weight-theft risks and calls for enhanced personnel vetting[^34]. |
| **Internal governance failures** | **Yes** | Schuett: "Frontier AI developers do not follow best practices" in risk governance[^21]; EU-CoP Commitment 8.[^2] |
| **Human-decision causation of risk** | **Yes, quantitatively** | MIT v3: human decisions cause 38% of catalogued risks vs 42% caused by AI systems.[^28] This gives the paper an evidence base for treating organizational causes as first-order. |

**Interpretation.** The literature treats organizational factors as *static attributes*: culture, structure, resourcing, access control. None models how fast they degrade under hypergrowth. A growth or absorptive-capacity factor is therefore a **moderator** of C6–C9 and B7–B8, not a new hazard. Three 2026 RAND and field-agenda documents (RAND-ISL, RAND-SL3, AISP) sharpen the mechanism: every control they recommend involves people-count, vetting or compartmentalization, and those scale worse as headcount grows quickly.

---

## 5. Where organizations disagree or frame things differently

1. **Scope of "systemic risk."**
   - The EU uses a legal definition tied to high-impact capabilities and EU-market reach.[^1][^2]
   - IASR frames systemic risk around widespread deployment.[^4]
   - Hacker et al. argue the AI Act concept is too narrow and would miss "discrimination at scale, and large-scale hallucinations."[^29]
2. **Loss of control.**
   - Mandatory in the EU Code[^2]; an AISI priority domain.[^7]
   - IASR 2026 reports that expert views on its likelihood vary widely.[^4]
   - Absent from CAISI's stated focus.[^13]
   - GDM narrows misalignment to cases where the AI "knowingly causes harm against the intent of the developer."[^26]
3. **Manipulation.** Mandatory in the EU Code[^2], but IASR 2026 finds little evidence of AI-generated content manipulating people at scale.[^4]
4. **Bio uplift.**
   - VCT: OpenAI's o3 scored 43.8%, "outperforming 94% of expert virologists" in their sub-areas.[^33]
   - IASR 2026 notes substantial uncertainty about how much this increases real-world risk.[^4]
5. **Cyber offence–defence.**
   - IASR 2026: unclear whether AI helps attackers or defenders more.[^4]
   - NCSC is more directional: intrusion operations will almost certainly become more effective, and "There will almost certainly be a digital divide."[^14]
6. **Open weights.** The EU Code exempts models weaker than the best open-weight model from Commitment 6[^2]. That resets the security baseline in a way RAND's actor-based threat model does not.[^15]
7. **Weight-access headcount.** RAND-W's expert panel disagreed on how aggressively to reduce authorized access ("low tens").\[U] RAND-SL3 operationalizes SL3 as 262 controls implementable in 6–12 months.[^18]
8. **US posture.** CAISI's mission centres "demonstrable risks, such as cybersecurity, biosecurity, and chemical weapons."[^13]

---

## 6. Single-source items

- **One source only:**
  - AI welfare (MIT)[^28]
  - Safetywashing (CAIS)[^25]
  - "Wonder weapons" (RAND-G)\[U]
  - Defender "digital divide" (NCSC)[^14]
  - "Lawlessness" and "non-human welfare" (EU-CoP)[^2]
  - Legal-structure indicator (FLI)[^30]
  - Algorithmic-insight leakage (RAND-ISL)[^19]
- **Two sources:**
  - Multi-agent collusion (EU-CoP, MIT; GDM "structural")[^2][^26][^28]
  - Evidence dilemma (IASR, EU-CoP recital)[^4][^2]
  - Single points of failure (DSIT, IASR)[^10][^4]

---

## 7. Relative priority by organization

| Organization | Top-priority set | Source |
| --- | --- | --- |
| EU AI Office | CBRN, loss of control, cyber offence, harmful manipulation (mandatory) | [^2] |
| US CAISI | Cyber, bio, chemical; adversary backdoors and malign influence | [^13] |
| UK AISI | Cyber misuse, chem-bio, criminal misuse, autonomous systems, societal resilience, human influence; science of evaluations | [^7] |
| NCSC | Cyber only, with the PHIA probability yardstick | [^14] |
| RAND | Weight and insight theft up to nation-state actors | [^15][^18][^19] |
| IASR 2026 | No ranking; explicitly "does not recommend any policies" | [^4] |
| CAIS | Malicious use, AI race, organizational risks, rogue AIs (unranked) | [^25] |
| GDM | Technical focus on misuse and misalignment; mistakes and structural risks deprioritized | [^26] |

---

## Recommendations for the AISI paper

1. **Frame the growth factor as C15, a rate-dependent moderator of C6–C9 and B7–B8.**
   - Anchor it in EU-CoP Commitment 8 and App. 4.3–4.4[^2], RAND-W/SL3/ISL[^15][^18][^19], CAIS[^25], Schuett[^21], and MIT's 38% human-decision finding.[^28]
   - State plainly that none of them models growth *rate*.
2. **Propose indicators that plug into existing instruments:**
   - Year-on-year growth in privileged-access headcount (EU-CoP App. 4.3(4); RAND SL3).
   - Growth in insight-access headcount (RAND-ISL).
   - Safety staff as a share of employees and budget (CAIS benchmark).
   - Tenure of risk-owning staff (Measure 8.1).
   - Trends in anonymous-survey indicators (Measure 8.3).
3. **Use Measure 1.3 as the regulatory hook.** A material change in *how models are developed* includes rapid organizational change, which is a reasonable-grounds trigger for Framework reassessment.[^2]
4. **Note the regulator–developer asymmetry.** The Code's Chairs argue regulator capacity must scale to 100 staff[^3]. The same logic applies to developer absorptive capacity.
5. **Separate *incentive* from *capacity*.** C1 and C11 (well covered) are about why labs cut corners. The uncovered Penrose-type mechanism is about whether they *can* keep controls intact while growing.

---

## Caveats

- **Coverage.** Targeted verification, not a full-corpus review. "Not found" is weaker than "absent."
- **Crosswalk cells.** E/D/P ratings outside the ✓-marked cells are not text-verified.
- **Weak sources.** [^9] and [^11] are secondary summaries; cite the AISI report and the DSIT paper directly.
- **Conflict of interest.** The author is an Anthropic model. SaferAI ranks Anthropic's framework highest.[^32] Treat Anthropic-related ratings independently.

---

## Footnotes

*Accessed 2026-09-26 unless marked \[U].*


[^1]: \[P] Regulation (EU) 2024/1689 (AI Act), OJ 12 Jul 2024. [https://eur-lex.europa.eu/eli/reg/2024/1689/oj](https://eur-lex.europa.eu/eli/reg/2024/1689/oj) — link from memory; verify. 
[^2]: \[P] EU GPAI Code of Practice, Safety & Security chapter, final, 10 Jul 2025. Text-searched from PDF copy: [https://www.marinacastellaneta.it/blog/wp-content/uploads/2025/07/Code_of_Practice_for_GeneralPurpose_AI_Models_Safety_and_Security_Chapter.pdf](https://www.marinacastellaneta.it/blog/wp-content/uploads/2025/07/Code_of_Practice_for_GeneralPurpose_AI_Models_Safety_and_Security_Chapter.pdf) (cite the Commission's official PDF in submission). Quote (Measure 8.1): "must not also be responsible for the Signatory's core business activities" 
[^3]: \[P] Statement from the Chairs and Vice-Chairs, Safety & Security chapter, Jul 2025. [https://www.artificialintelligenceact.eu/ai-act-explorer/cop-safety](https://www.artificialintelligenceact.eu/ai-act-explorer/cop-safety) — AI Safety unit "should be scaled up to 100 staff" 
[^4]: \[P] International AI Safety Report 2026, Extended Summary for Policymakers, Feb 2026 (arXiv 2602.21012, 24 Feb 2026). [https://internationalaisafetyreport.org/sites/default/files/2026-02/ai-safety-report-2026-extended-summary-for-policymakers.pdf](https://internationalaisafetyreport.org/sites/default/files/2026-02/ai-safety-report-2026-extended-summary-for-policymakers.pdf) — "Competitive pressures can incentivise AI developers to reduce their investment in testing" 
[^6]: \[P] International AI Safety Report 2025, arXiv 2501.17805, 29 Jan 2025. [https://arxiv.org/abs/2501.17805](https://arxiv.org/abs/2501.17805) 
[^7]: \[P] UK AISI, Research Agenda, released 6 May 2025 (date per techUK). [https://www.aisi.gov.uk/research-agenda](https://www.aisi.gov.uk/research-agenda) — "a snapshot in time of our current research priorities" 
[^8]: \[P] UK AISI, Frontier AI Trends Report, 18 Dec 2025. [https://aisi.gov.uk/frontier-ai-trends-report](https://aisi.gov.uk/frontier-ai-trends-report) — cyber task length "doubling roughly every eight months" 
[^9]: \[S] techUK summary of the AISI Trends Report, Dec 2025. [https://www.techuk.org/resource/uk-ai-security-institute-releases-inaugural-frontier-ai-trends-report.html](https://www.techuk.org/resource/uk-ai-security-institute-releases-inaugural-frontier-ai-trends-report.html) — "Found universal jailbreaks for every system they have tested" 
[^10]: \[P] UK DSIT, *Capabilities and risks from frontier AI*, Oct 2023. [https://assets.publishing.service.gov.uk/media/65395abae6c968000daa9b25/frontier-ai-capabilities-risks-report.pdf](https://assets.publishing.service.gov.uk/media/65395abae6c968000daa9b25/frontier-ai-capabilities-risks-report.pdf) — heading: "Insufficient incentives for AI developers to invest into risk mitigation measures" 
[^11]: \[W] LessWrong excerpt of the DSIT paper, Oct 2023. [https://www.lesswrong.com/posts/eZ8xAyxiELASGsawb/uk-government-publishes-frontier-ai-capabilities-and-risks](https://www.lesswrong.com/posts/eZ8xAyxiELASGsawb/uk-government-publishes-frontier-ai-capabilities-and-risks) — race-to-the-bottom actors "under-invest in safety measures" 
[^12]: \[P] NIST AI 600-1, Generative AI Profile, Jul 2024 (text-searched; 12 risks confirmed). [https://nvlpubs.nist.gov/nistpubs/ai/NIST.AI.600-1.pdf](https://nvlpubs.nist.gov/nistpubs/ai/NIST.AI.600-1.pdf) 
[^13]: \[P] U.S. Department of Commerce, statement on CAISI, Jun 2025. [https://www.commerce.gov/news/press-releases/2025/06/statement-us-secretary-commerce-howard-lutnick-transforming-us-ai](https://www.commerce.gov/news/press-releases/2025/06/statement-us-secretary-commerce-howard-lutnick-transforming-us-ai) — "demonstrable risks, such as cybersecurity, biosecurity, and chemical weapons" 
[^14]: \[P] NCSC, *Impact of AI on cyber threat from now to 2027*, published 7 May 2025. [https://www.ncsc.gov.uk/report/impact-ai-cyber-threat-now-2027](https://www.ncsc.gov.uk/report/impact-ai-cyber-threat-now-2027) — "There will almost certainly be a digital divide" 
[^15]: \[P] Nevo et al., *Securing AI Model Weights*, RAND RR-A2849-1, 2024. [https://www.rand.org/content/dam/rand/pubs/research_reports/RRA2800/RRA2849-1/RAND_RRA2849-1.pdf](https://www.rand.org/content/dam/rand/pubs/research_reports/RRA2800/RRA2849-1/RAND_RRA2849-1.pdf) — insider threat concern "was an emerging point of consensus" 
[^16]: \[P] RAND press release, 30 May 2024. [https://www.rand.org/news/press/2024/05/30.html](https://www.rand.org/news/press/2024/05/30.html) — priority: "reducing the number of people with authorization" 
[^17]: \[U] Mitre & Predd, RAND PEA3691-4, Feb 2025. [https://www.rand.org/content/dam/rand/pubs/perspectives/PEA3600/PEA3691-4/RAND_PEA3691-4.pdf](https://www.rand.org/content/dam/rand/pubs/perspectives/PEA3600/PEA3691-4/RAND_PEA3691-4.pdf) 
[^18]: \[P] Aguirre et al., *Achieving AI Model Weight Security Level 3*, RAND RR-A4704-1, Aug 2026. [https://www.rand.org/pubs/research_reports/RRA4704-1.html](https://www.rand.org/pubs/research_reports/RRA4704-1.html) — SL3 framework of "262 security controls" 
[^19]: \[P] Brass-Gershovich et al., *Securing AI Algorithmic Insights*, RAND RR-A4685-1, 20 Jul 2026. [https://www.rand.org/pubs/research_reports/RRA4685-1.html](https://www.rand.org/pubs/research_reports/RRA4685-1.html) — higher levels require "more-intensive personnel security" 
[^20]: \[P] Anderljung et al., *Frontier AI Regulation*, arXiv 2307.03718, 6 Jul 2023. [https://arxiv.org/abs/2307.03718](https://arxiv.org/abs/2307.03718) — "dangerous capabilities can arise unexpectedly" 
[^21]: \[P] Schuett, *Frontier AI developers need an internal audit function*, arXiv 2305.17038, May 2023 (rev. Oct 2024). [https://arxiv.org/abs/2305.17038](https://arxiv.org/abs/2305.17038) — "Frontier AI developers do not follow best practices" 
[^22]: \[P] Centre for the Governance of AI, analysis of Anthropic RSP v3.0, 2026. [https://governance.ai/analysis/anthropics-rsp-v3-0-how-it-works-whats-changed-and-some-reflections](https://governance.ai/analysis/anthropics-rsp-v3-0-how-it-works-whats-changed-and-some-reflections) — "better to be honest about constraints" 
[^23]: \[U] Arnold & Toner, *AI Accidents: An Emerging Threat*, CSET, Jul 2021. [https://cset.georgetown.edu/publication/ai-accidents-an-emerging-threat/](https://cset.georgetown.edu/publication/ai-accidents-an-emerging-threat/) 
[^24]: \[S] CSET, "AI Safety under the EU AI Code of Practice," Dec 2025. [https://cset.georgetown.edu/article/eu-ai-code-safety/](https://cset.georgetown.edu/article/eu-ai-code-safety/) — providers expected to "foster a healthy risk culture in the organization" 
[^25]: \[P] Hendrycks, Mazeika & Woodside, *An Overview of Catastrophic AI Risks*, arXiv 2306.12001 (v6, Oct 2023; text-searched). [https://arxiv.org/abs/2306.12001](https://arxiv.org/abs/2306.12001) — recommends safety research use "at least 30 percent" of employees and budgets 
[^26]: \[P] Google DeepMind, *An Approach to Technical AGI Safety and Security*, arXiv 2504.01849, 2 Apr 2025 (text-searched). [https://arxiv.org/abs/2504.01849](https://arxiv.org/abs/2504.01849) — misalignment: the AI "knowingly causes harm against the intent of the developer" 
[^27]: \[P] Shevlane et al., *Model evaluation for extreme risks*, arXiv 2305.15324, 24 May 2023. [https://arxiv.org/abs/2305.15324](https://arxiv.org/abs/2305.15324) 
[^28]: \[P] Slattery et al., *The AI Risk Repository*, arXiv 2408.12622v3 (updated May 2026). [https://arxiv.org/abs/2408.12622](https://arxiv.org/abs/2408.12622) — "74 frameworks containing 1,725 distinct risks" 
[^29]: \[P] Hacker et al., *AI, Digital Platforms, and the New Systemic Risk*, arXiv 2509.17878, Sep 2025 (rev. May 2026). [https://arxiv.org/abs/2509.17878](https://arxiv.org/abs/2509.17878) — "discrimination at scale, and large-scale hallucinations" 
[^30]: \[S] Future of Life Institute, AI Safety Index, Winter 2025. [https://futureoflife.org/ai-safety-index-winter-2025/](https://futureoflife.org/ai-safety-index-winter-2025/) — indicator on "safety prioritization over short-term financial pressures" 
[^31]: \[U] Future of Life Institute, AI Safety Index, Summer 2026. [https://futureoflife.org/ai-safety-index-summer-2026/](https://futureoflife.org/ai-safety-index-summer-2026/) 
[^32]: \[P] Stelling et al. (SaferAI), *Evaluating AI Providers' Frontier AI Safety Frameworks*, arXiv 2512.01166, 1 Dec 2025 (rev. 2026). [https://arxiv.org/abs/2512.01166](https://arxiv.org/abs/2512.01166) — scores from 34% to 8%, "with a median of 18%" 
[^33]: \[P] *Virology Capabilities Test (VCT)*, arXiv 2504.16137, 21 Apr 2025. [https://arxiv.org/abs/2504.16137](https://arxiv.org/abs/2504.16137) — o3 "outperforming 94% of expert virologists" 
[^34]: \[P] *AI Security Priorities: A Field-Wide Agenda*, arXiv 2607.26069, 23 Jun 2026. [https://arxiv.org/abs/2607.26069](https://arxiv.org/abs/2607.26069) — "the gap between AI adoption and AI security readiness continues to widen"
