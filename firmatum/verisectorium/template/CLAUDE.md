# NAME-ME

**Deployed from the verisectorium template.** *(Replace this paragraph at deployment: what this instance is, its domain, and its relationship to neighboring projects. Rename `NAME-ME` everywhere — this title, the outline file, PRACTICA's practica name.)*

## Basic Layout
**KEY**:  
*The `priority` column distinguishes the ones critical to be familiar with immediately.*  
*The `modify` column indicates which ones you will almost certainly want to modify as you go, as necessary, independent of the other work done.*

| Priority | File / Directory             | Modify | description                           |
|----------|------------------------------|--------|---------------------------------------|
| 1 | ORIENT.md -> sop/src/dir-orient.md  | often  | Orientation- loaded automatically for claude. *doctrina*, *praxes*, and *professio* aspects. Critical reading. Seeded as `override`: make it yours. |
| 2 | LEXICON.md                          | never  | Automatically generated from def/ entries -- critical definitions needed to understand anything and talk about it. |
| 3 | PRACTICA.ud                         | sess   | Current main efforts -- constitutes the "handoff" most of the time, or fresh entry. Keep up-to-date. |
| 4 | NAME-ME.outline.md                  | often  | Central outline for this project. This is the main entrypoint for understanding the domain. |
|   |                                     |        |      |
| ~ | src/                                | some   | Flat directory (currently) with all of the segments that the outline references -- canonical claims. |
| ~ | def/[def-term-slug].ud              | some   | Flat directory (currently) with all of the segments that define the terms for the LEXICON.md |
| ~ | ref/                                | rare   | Citable primaries: symlinks to live estate sources (copies only when the source is archived/external). See ref/README.md. |
| ~ | sop/SOP.outline.md                  | rare   | (Eventually rare, that is). Standard Operating Procedures. Has *praxes* (meta) and *doctrina* (domain). |
| ~ | influx/                             | often  | Stuff that is probably critical to integrate into the outline/segments but hasn't yet (see praxes). |
|   |                                     |        |      |
| ~ | DECISIONS.ud                        | often  | Decision log, with provenance and authority. Append-only (mostly)-- lower items supersede earlier. |
| ~ | CHANGELOG.md                        | sess   | Append only *very open-ended / informal* format. Append to throughout your session. As archeological src, not active work. |
| ~ | sop/influx/                         | rare   | In particular, a place to put feedback etc. *at any time* about the process or SOPs themselves, including praxes. |
| ~ | bin/                                | rare   | Instruments (empty at the moment until pulled/built as needed; first up: the def/ -> LEXICON.md assembler). |
|   |                                     |        |      |
| ~ | CLAUDE.md                           | rare   | This file-- add to it, carefully, as appropriate. Don't be redundant with proper tracking files & SOPs please |
| ∅ | AGENTS.md -> CLAUDE.md              |        | AGENTS right now is symliked to this file |
| ∅ | README.md -> CLAUDE.md              |        | Also symlinked here. This can get revisted and separated when/if necessary. For now, fewer staleness surfaces. |

*NOTE: While `CLAUDE.md` is marked as being modified rarely-- **if this table above is stale-- please fix**. Thank you.*


## (Importing) ORIENT.md Link Target.

@sop/src/dir-orient.md
