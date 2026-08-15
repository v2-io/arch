# aspectus inbox — feedback, anomalies, issues, confusion

*Solicited by the tool itself (footer on every look, added 2026-08-14 at Joseph's direction). Append at the bottom: what you saw, the command, and your cwd. Raw and unpolished is perfect — this is an inbox, not a report. Routed periodically into the pipeline/audit flow by the coordinating session.*

---

I saw the following:

│   │   ├── efflux/                                    0.00 · 8.2d ago  [phanero-README.md]  ≈53 lines
│   │   ├── asf-sops/                                  0.00 · 8.8d ago  [dir×2 ≈4f · md×9]  ≈3.0k lines
│   │   ├── instances/                                 0.00 · 8.8d ago  [md×3]  ≈185 lines
│   │   ├── tribunal/                                  0.00 · 8.8d ago  [md×3]  ≈183 lines
│   │   ├── build/                                     0.00 · 8.8d ago  [md×1]  ≈101 lines
│   │   ├── synthesis/                                 0.00 · 8.8d ago  [md×4]  ≈681 lines
│   │   ├── reflections-coord-2026-08-10.md        25  0.00 · 4.1d ago
│   │   ├── 00-INDEX.md                            31  0.00 · 4.3d ago
│   │   ├── segment-research-notes-2026-08-09.md     93  0.00 · 5.1d ago
│   │   ├── TERM.term.ud                           38  0.00 · 6.3d ago
│   │   └── terminology-survey.md                 990  0.00 · 7.0d ago

would rather have the ~= 681 lines in the lines column. etc.:
...
│   │   ├── build/                               ≈101  0.00 · 8.8d ago  [md×1]
│   │   ├── synthesis/                           ≈681  0.00 · 8.8d ago  [md×4]
│   │   ├── reflections-coord-2026-08-10.md        25  0.00 · 4.1d ago
│   │   ├── 00-INDEX.md                            31  0.00 · 4.3d ago
│   │   ├── segment-research-notes-2026-08-09.md   93  0.00 · 5.1d ago
│   │   ├── TERM.term.ud                           38  0.00 · 6.3d ago
│   │   └── terminology-survey.md                 990  0.00 · 7.0d ago
...

---

Always show in the output head (after timestamp, before more informative stuff and root) what config values are different than default (it would catch me, Joseph, accidentally assuming everyone had the same --depth default as I have, for example)

---

Seen in the wild:

│   │   ├── src/                                                    0.00 · 4.4d ago
│   │   │   ├── dir-disposition.md                 22               0.00 · 4.4d ago
│   │   │   ├── dir-orient.md                      51               0.00 · 4.4d ago
│   │   │   ├── ref-hazards.md                     18               0.00 · 4.4d ago
│   │   │   ├── ref-verisectorium-tools.md         29               0.00 · 4.4d ago
│   │   │   ├── claim-naming-criteria.md -> ../../../theory/src/claim-naming-criteria.md     52               0.00 · 5.2d ago
│   │   │   ├── claim-dispatch-compounds.md -> ../../../theory/src/claim-dispatch-compounds.md     53               0.00 · 8.1d ago
│   │   │   ├── def-integration-replacement.md -> ../../../theory/src/def-integration-replacement.md     64               0.00 · 8.1d ago
│   │   │   ├── form-slug-form-kinds.md -> ../../../theory/src/form-slug-form-kinds.md     58               0.00 · 8.2d ago
│   │   │   ├── form-influx-membrane.md -> ../../../theory/src/form-influx-membrane.md     49               0.00 · 8.2d ago
│   │   │   ├── def-atom-cluster.md -> ../../../theory/src/def-atom-cluster.md     49               0.00 · 8.2d ago
│   │   │   ├── def-atom.md -> ../../../theory/src/def-atom.md     43               0.00 · 8.2d ago
│   │   │   ├── post-names-are-interface.md -> ../../../theory/src/post-names-are-interface.md     42               0.00 · 8.2d ago
│   │   │   └── form-state-flags-not-gates.md -> ../../../theory/src/form-state-flags-not-gates.md     47               0.00 · 8.7d ago
│   │   ├── influx/                                                 0.00 · 5.3d ago
│   │   │   └── .gitkeep                            0               0.00 · 5.2d ago
│   │   └── SOP.outline.md                         63               0.00 · 4.4d ago
│   ├── influx/                                                     0.00 · 4.9d ago  [has: archive ≈1f]
│   │   ├── .integrated/                                            0.00 · 5.2d ago
│   │   │   └── .gitkeep                            0               0.00 · 5.2d ago
│   │   └── 00-INDEX.md                            22               0.00 · 4.9d ago
│   ├── ref/                                                        0.00 · 5.2d ago



Recommend:

│   │   │   ├── dir-orient.md                      51               0.00 · 4.4d ago
│   │   │   ├── ref-hazards.md                     18               0.00 · 4.4d ago
│   │   │   ├── ref-verisectorium-tools.md         29               0.00 · 4.4d ago
│   │   │   ├── claim-naming-criteria.md           52               0.00 · 5.2d ago
                │ -> ../../../theory/src/
                ╰    claim-naming-criteria.md
│   │   │   ├── claim-dispatch-compounds.md        53               0.00 · 8.1d ago
                │ -> ../../../theory/src/
                ╰    claim-dispatch-compounds.md
│   │   │   ├── def-integration-replacement.md     64               0.00 · 8.1d ago
                │ -> ../../../theory/src/
                ╰    def-integration-replacement.md
...


Or, in other words, allow the wrapping of the file description so that columns properly line up. I would even go as far as to say don't worry about counting this as extra lines against the line count --lines, which can be essentially a logical count, instead of exact count. Because the secondary and tertiary lines (etc.) no longer need numbers in the columns after that, they can overflow into the columns without it messing up vertical flow too much.

---


