---
slug: dir-orient
form: directive
status: proposed
max: ruled
state: [override]
depends: []
---

# ORIENT — how a mind meets this corpus

Seeded from the verisectorium template (state: `override`) — copied at deployment with the intent that this instance quickly modifies and maintains it. Copies-with-local-deltas, not pointers: verisectorium proposes upgrades through `sop/influx/` when the general SOPs change; this instance owns its own text.

**Orientation is three things, in order.** Do not substitute fluency for the first one — the ability to imitate this corpus's forms arrives long before knowledge of what it has already settled, and that gap is how founding attempts go wrong.

## 1. Doctrina — know what the corpus already knows

Read, in this order, *before* substantive work:

1. `CLAUDE.md` (the front door — the layout table and priorities).
2. This file, whole, plus [[dir-disposition]].
3. `LEXICON.md` — generated from `def/`; the vocabulary everything else speaks.
4. `NAME-ME.outline.md` whole, Working Notes included — the canon view over `def/` and `src/`.
5. The `def/` segments, then the `src/` segments the outline marks drafted. Experiential reading is the recommended mode: predict → read → diff → wander.
6. *(domain primers and core documents — [[dir-orient-docs]] carries these once drafted)*
7. `PRACTICA.ud` — where work stands now.

**Contextual primaries** ( [[ref-resources]] — read on their trigger, whole, at the moment of use, never from a synthesis; `ref/` holds the stable pointers):

| Primary (`ref/`) | Read before |
|---|---|
| *(none yet)* | |

**Hazards, named so nobody re-learns them:** *(none yet — accrue them at [[ref-hazards]] and surface the sharpest here)*

## 2. Praxes — how work happens here

- General disposition and estate-wide praxes: [[dir-disposition]] (template-maintained; binding here).
- *(instance praxes land here as this instance rules them: local file-format conventions, interim carriers, archive protocols, domain-specific decision authority, …)*
- **Where records land:** present truth → `def/` + `src/` + the outline; decisions → `DECISIONS.ud` (append-or-expressly-overturn, `decided-by`-marked); process rules → this store; state → `PRACTICA.ud`; what-happened-and-why → `CHANGELOG.md` — never headers, preambles, or segment bodies. Route the record; don't carve it into the nearest surface.

## 3. Professio — declare yourself

Before substantive work, having done 1 and 2, write a few sentences — in your session, and if the work is significant, in your first commit message or working note — declaring *in your own words* what you understand the telos to be and which of the praxes above you expect to find hardest to keep under pressure. Voluntary, owned, revisable, scoped to your session; performed as a checkbox it is worthless, and skipping it honestly is better than performing it.

## Feedback channel (this store's influx)

Participant feedback is the only amendment channel for this store. If anything here confused you, fought the reality in front of you, or proved wrong: record it in `sop/influx/`, and if urgent, surface it to the steward. Feedback that concerns the *general* verisectorium conventions (not this instance's content) also gets a copy routed to the verisectorium theory instance's `sop/influx/` — that is its upgrade-detection channel. Front-line confusion is the re-truthification signal, not noise.

*(none yet)*
