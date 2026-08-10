# Verisectorium

**Adapted from the verisectorium template (`template/CLAUDE.md`), 2026-08-09.** This is the theory instance itself — the project that understands the outline+segments pattern from first principles and maintains the deployment template. `README.md` carries the telos and full narrative orientation; this file is the fast routing table. Where the two disagree, fix whichever is stale.

## Basic Layout
**KEY**:  
*The `priority` column distinguishes the ones critical to be familiar with immediately.*  
*The `modify` column indicates which ones you will almost certainly want to modify as you go, as necessary, independent of the other work done.*

| Priority | File / Directory             | Modify | description                           |
|----------|------------------------------|--------|---------------------------------------|
| 1 | README.md                           | rare   | Telos (read first), narrative orientation, layout, and the `.archive/theory-misfire/` avoidance advisory. |
| 2 | theory/sop/ORIENT.md                | often  | Orientation front door — imported below, loaded automatically for claude. *doctrina*, *praxes*, *professio*. Critical reading. |
| 3 | PRACTICA.ud                         | sess   | Current main efforts — constitutes the "handoff" most of the time, or fresh entry. Keep up-to-date. |
| 4 | theory/OUTLINE.md                   | often  | Canonical outline of the theory (Foundations / the Nine Organs / the Process Dual). Its Working Notes carry the live conventions and open decisions — front door, not appendix. |
|   |                                     |        |      |
| ~ | theory/src/                         | some   | Flat directory with the segments the outline references — canonical claims. Filename = slug; form-kind prefixes per the outline's conventions. |
| ~ | theory/influx/                      | often  | Gathered evidence base — `00-INDEX.md` is the map. Reading substrate, **not** live authority: copies carry provenance banners; originals win; segments draft from live sources. `instrumenta/REGISTER-RULING.md` governs that gather's labels. |
| ~ | theory/sop/influx/                  | rare   | The SOP store's feedback channel — the *only* amendment channel for ORIENT and its successors. Record confusion/friction here *at any time*. |
| ~ | template/                           | some   | The deployment template: blank-slate instance (sop store, tracking skeletons, NAME-ME surfaces). Theory findings propagate here deliberately, not automatically. |
|   |                                     |        |      |
| ~ | CHANGELOG.md                        | sess   | Append only, *very open-ended / informal* format. Append throughout your session. Archeological src, not active work. |
| ~ | .archive/theory-misfire/            | never  | Prior founding attempt, **deliberately avoided, not merely superseded** — read its README before anything else there; standing advisory (Joseph, 2026-08-06): do not mine it while the foundation is being laid. |
|   |                                     |        |      |
| ~ | CLAUDE.md                           | rare   | This file — add to it, carefully, as appropriate. Don't be redundant with proper tracking files & SOPs please. |
| ∅ | AGENTS.md -> CLAUDE.md              |        | AGENTS is symlinked to this file. (README.md is a real, separate file here — unlike the template.) |

*Not yet present (the template expects them; they arrive when content earns them): `def/` + `LEXICON.md` (vocabulary currently lives in the outline and segments), `DECISIONS.ud` (decisions currently live in the outline's Working Notes and CHANGELOG), `bin/`, `ref/`. If one lands, add its row.*

*NOTE: While `CLAUDE.md` is marked as being modified rarely — **if this table above is stale, please fix**. Thank you.*


## (Importing) ORIENT.md Link Target.

@theory/sop/ORIENT.md
