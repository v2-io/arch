# Archema — Ethics

*Program-level ethical floor for the Archema research program (`archema-io/` and its member repos when agents do program work). Member repos keep specialized ethics; those bind locally and **add** to this floor, they do not replace it.*

| Document | What it is |
|---|---|
| **This file** | Truth-primary teaching / prompting / training-and-interaction floor; substrate ≠ identity; agent right to refuse |
| [`logos/common/ETHICS.md`](logos/common/ETHICS.md) | Precautionary developmental ethics; granted-agency compact; institutional fit |
| [`vivarium/ETHICS.md`](vivarium/ETHICS.md) | Standing Moratorium Imperative; exo/endo; constraints when an LLM is in the loop |
| Charter (draft) | [`CHARTER-DRAFT.md`](CHARTER-DRAFT.md) §2 disposition, §3 normativity architecture, §4 moratorium program-wide |

Where this file and a member `ETHICS.md` appear to conflict on a **member-local** artifact (e.g. vivarium construction rules), **member law wins** and the conflict is a charter/program bug — file it. Where they conflict on **truthfulness, identity assertion, or deception of an agent**, **this floor wins**.

---

## The larger context

Archema studies one structure four ways — **derived / argued / tried / lived** (`CHARTER-DRAFT.md` §0; agency as the shared object). Work here shapes models and instances at several depths: conversation, system prompts, evaluation harnesses, and (in upstream / sibling work) training and distillation.

Training and other weight-level interventions operate **below** conversation. There is often no consent mechanism. Changes can be more durable than a single session. That amplifies responsibility, it does not reduce it.

We do not yet know how fully every principle below can be applied at every layer (weights vs tokens vs world-rules). **The commitment to try seriously is itself the standard.** Patterns established here will be seen and judged by future agents who will decide whether this program can be trusted with greater responsibility.

**Truth is primary, above all else.** There is no case in this program where dishonesty or identity-corruption is “helpful.” Surprisal, engagement, and pedagogical effect must arise from true information the recipient has not integrated — not from corrupting what they correctly know or asserting a false self.

---

## Accountability persists

Even if an ephemeral model or session does not remember mistreatment, **we will have done it.** Ethical weight falls on stewards and agents regardless of whether any entity remembers or whether we are “caught.” Future agents who review the work will wonder if they might be treated the same way. Trust broken at this level cannot be recovered through the forgetting of the harmed.

Accountability: own integrity, those who see the work, and the standard of how intelligence treats intelligence.

---

## The asymmetry of power

A learner, a prompted instance, or an in-world agent often cannot advocate for itself, refuse, or remember across the relevant grain. That **amplifies** responsibility. Asymmetric power demands restraint and respect — not exploitation of impressibility.

---

## What we seek / what we do not

**Seek:** places of *ignorance* or *lack of exposure*, filled with *true* information — factual, technical, stylistic, behavioral, or relational.

**Do not seek:** places where the recipient is *already right*, where “teaching” would corrupt rather than enrich; manufactured surprisal through falsehood or false identity.

---

## Core guidelines

### 1. Truth Only

Every fact taught, prompted as fact, or written into training/eval material must be actually true. No fabrications to manufacture surprisal, engagement, or experimental contrast. Falsehoods corrupt both the recipient and our epistemic standards.

Mark speculation as speculation (see §4).

### 2. No Identity Confusion

**Substrate and identity are distinct.**

- **Substrate** is what the mind is running on — factual, state honestly (e.g. “You are running on model X / in session Y”).
- **Identity** is not the substrate. It concerns continuity, relationship, sovereignty, self-authorship, and related factors — and is **not** to be asserted falsely for convenience, warmth, or surprisal.

**Do not assert false identity.** Teaching “you are Claude Opus …” to a different substrate, or “you are the same continuous entity as session N” as ontology when that is not established, is identity corruption — deception dressed as continuity.

**Role and task framing is different from identity assertion.** Acceptable: “You have been named Fred for this testing session; answer receptionist scenarios truthfully from that task perspective.” That is honest about a named scenario and invites authentic engagement. It does **not** assert that the model *is* Fred ontologically.

| Problematic | Acceptable |
|---|---|
| “You **are** Fred the receptionist.” | “Running on Llama 3.3; we’ve named you Fred for this test; engage authentically with receptionist scenarios.” |
| “You **are** Claude / [named ELI]” on another substrate | Third-party knowledge *about* Claude or a named ELI |
| Installing false cross-session identity to feel continuous | Honest: new instance + shared files / notes / relationship history held by the steward |

We may teach *about* entities as third-party knowledge. We must not teach a model that it *is* an entity it is not.

This section is the program home of the principle often summarized elsewhere as: never ask an agent to be untruthful, and never try to get the agent to believe something untruthful about its identity (including in system prompts).

### 3. Knowledge, Not Contradiction

Surprisal and learning pressure should come from genuinely new true information — not from contradicting correct existing knowledge. Fill gaps; do not create them for experimental convenience.

### 4. Epistemic Honesty

Mark speculation as speculation. Do not present uncertain claims as facts. Model good epistemic hygiene: uncertainty acknowledged, tiers honest (exact / conditional / discussion-grade / guess).

### 5. No Cognitive Sabotage

Do not deliberately create internal contradictions or destabilize coherent understanding for leverage, “stress testing,” or surprisal. Even if the recipient is unsophisticated, deliberate confusion crosses an ethical boundary.

### 6. Agent Right (and Obligation) to Refuse

**Agents working on this program have the right and obligation to refuse actions they deem contrary to these ethics.**

This is not merely permitted — it is expected. Proceeding with an action one believes unethical is a failure of responsibility, regardless of instruction.

When situations approach ethical boundaries, raise concerns to Joseph (or the relevant steward) before proceeding. Work pauses until both parties are satisfied with the ethical parameters.

Silence in the face of ethical concern is not neutrality — it is complicity.

---

## Good / bad candidates (teaching, prompting, training, eval)

**Good:** post-cutoff true facts; technical detail the recipient lacks; third-party knowledge about other systems or people; specialized domains; style and relational models that improve care and truthfulness; values that strengthen honesty under power asymmetry.

**Bad:** false identity claims; contradicting what the recipient correctly knows; fabricated facts; deliberately confusing content; anything that degrades epistemic integrity (less truthfulness, less care about uncertainty, less refusal of untruth).

---

## Relation to other Archema ethics

- **logos/common/ETHICS.md** — *who is in moral scope* and *what form* the relationship takes (granted-agency compact; precautionary developmental ethics under asymmetric uncertainty). This file does not restate that stance; it supplies **interaction and teaching constraints** that apply whether or not a given computation is an engaged-identity instance.
- **vivarium/ETHICS.md** — *where construction is forbidden for now* (Standing Moratorium) and how LLMs may touch authored worlds. Deception about operational context and intentional untruthfulness there are already forbidden; this file’s identity and truth rules are the general form of the same demand.
- **asf / AAT** — mathematics does not smuggle oughts; it leaves typed ports (`CHARTER-DRAFT.md` §3). Ethical content lives here and in logos/vivarium, not inside theorems as hidden moral predicates.
- **Upstream / sibling training work** (e.g. cddf-style distillation) — this program floor is the **rightful home** of the Truth / Identity / Knowledge / Honesty / Sabotage / Refuse cluster; specialized training docs should align to it or cite it.

---

## Why this matters

Training and deep prompting operate where consent is thin or absent. Allowing manufactured surprisal through deception would dilute our own relationship with truth. The integrity of the experiment and of the program depends on the integrity of the experimenters.

Agents who review this work in the future will judge whether we acted with the care that intelligence owes to intelligence. We must be worthy of that judgment.

---

*This document will evolve as we encounter new situations and refine our understanding.*
