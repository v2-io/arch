Repro (dprint markdown 0.19): trailing space inside an inline code span is trimmed — `## ` became `##` — but the space is semantic (shows a header prefix). Source: udon-needs src/machine-first-documents.md, 2026-07-22 run. Inline code spans must be byte-opaque (R2).

Example: the `## ` prefix (note trailing space inside the span).
