<!--
  Verisectorium theory INFLUX — steward brainstorm, verbatim payload + coord notes.
  Provenance: Joseph, in-session (coord = Fable instance), 2026-08-06, explicitly incomplete
  ("pause this a little bit" is part of the content, not a defect).
  Register: pre-validation brainstorm. Coord notes follow the verbatim block, marked [C].
  Destination: feeds [[form-addressing]] / [[form-store-triplet]] drafting, and — per the
  steward's own closing move — routes outward as DEMAND to the udon paths/references work.
-->

# Steward brainstorm: instance naming, address paths, and the udon-URI convergence

Verbatim from chat, 2026-08-06:

> Let's say that every verisectorium is named essentially after its directory relative to its project (with config or something allowing mappings to override, e.g., asf/aat/ -> asf/01-aat-core/..., or, in our case we might say `vera/` -> `arch/firmatum/verisectorium/theory/` ... or something, config sitting in the project root? of course then we have to worry about when this becomes its own submodule... which it probably already should for parallelism's sake... We could also have a `example/` -> `arch/firmatum/verisectorium/template/example-outline/` or something)
>
> Alternately the mapping could (maybe should) actually be mapping to the canonical outline or any of the outlines... maybe the `abc/xyz/....` is really `[project-slug]/[outline-slug]/[segment-within-outline]` with one outline-slug standing in as default ('theory' in verisectorium's case...)
>
> Third, lets say that the primary triplet is something like this:
>
> ```text
> vera/  (points to the main corpus)
> vera/theory/ (same)
> vera/theory/in/[influx-slug = 'flux' by default]
> vera/in/flux/....
> vera/def/[term-slug]
> vera/sop/[index-slug, default main]
> vera/sop/.../...
>
> vera/example/def/foobar
> vera/template/sop/praxes/oob/upgrades
> ```
>
> *(fence added by coord for line preservation only — content verbatim)*
>
> You know... this is becoming very URI-ish. Rather than reinventing URIs, I wonder if it's now time to pause this a little bit and really nail down udon paths / references, based on our intuition known here now and with NORM -- both of which haven't really made their way into the path discussion in udon yet...

---

## Coord notes [C]

**1. The pause instinct is right, and it has a name in this corpus.** Designing a bespoke address scheme here, now, is the instrument-becomes-the-project hazard ( [[disc-launch-problem]]) applied to addressing. The honest shape of this brainstorm is *demand*: verisectorium is the richest demand-driver yet for the udon paths/references spec, exactly the udon-needs pattern — capture the demand precisely, route it to the owning project, don't half-build the mechanism locally.

**2. What the examples reveal: two address kinds are being interleaved, and keeping them distinct is the load-bearing move.** The path fragments mix a *store selector* (`in/`, `def/`, `sop/`), an *outline selector* (`theory/`), and an item slug. These belong to two different address spaces the theory already distinguishes:

- **Identity addresses** — `[instance]/[store]/[slug]` — point into the *population*, are view-free, and should be the citable, rot-proof form ( [[claim-identity-move-proofness]]; the trio synthesis's `#asf/aat/der-directed-separation` is this form with volume-as-store).
- **View addresses** — `[instance]/[outline-slug]/…` — are *view-relative* (ordering, next/prev, quiz position, "chapter 3 of the canon view") and legitimately rot when views are re-authored, because a view is cheap and re-authorable by design ( [[claim-outline-as-view]]).

The `[project]/[outline]/[segment]` alternative quietly makes every address view-relative; the store-selector form quietly makes every address population-direct. Recommendation carried forward as a question, not a resolution: the spec probably wants *both*, visibly distinct, with identity addresses as the default citable form and view addresses as an explicit qualifier — the addressing twin of the selection/projection and authored/generated splits.

**3. The default-collapse chains are the designator ladder.** `vera/` ≡ `vera/theory/`; `in` defaulting to `flux`; `sop` defaulting to `main` — each is a resolution step, and [[form-addressing]]'s ladder (auto when unique → visible escalation, never silent best-guess) is where the semantics already live. A bare local slug resolving instance-locally first is the same ladder's bottom rung. The mapping/override config (`asf/aat/` → `asf/01-aat-core/`) is the *registry* layer of the same ladder — and note it exists because of the identity-vs-layout law: the address is stable, the mapping absorbs the physical moves.

**4. The store prefixes are form-kind-adjacent but not identical — flag before it fuses.** `vera/def/[term-slug]` reads `def/` as *the lexicon store*, but `def-` is also a segment form-kind prefix in the canon store ( [[form-slug-form-kinds]]). Those are different axes (which store vs what shape of atom), and a scheme that fuses them will collide the day a canon segment `def-atom` and a lexicon term `atom` both want addresses. The trio scheme's `#asf/term/<slug>` (store-qualified, not form-qualified) is the existing precedent that keeps them apart.

**5. Fragments are already specified in spirit.** Below the atom, the theory's addressing form is slug + section + word-anchor spans (quiz units, provenance spans, comproprium's checker). A URI-shaped scheme gets these as fragments — meaning the *whole* stack (instance registry / store / slug / section-span fragment, view-qualified variant) is one spec, and the pieces all exist in NORMS (BASENAME manifestation-independence = address opacity over physical layout), the trio cross-member scheme, form-addressing, and this brainstorm. That's four partial specifications of one thing — the strongest possible signal that the udon paths work is ripe and that verisectorium should feed it rather than fork it. **Correction, same evening:** it is *six* — DISCUSSION-THOUGHTS.udon O13/O13a (read after this note was written) carry the richest decomposition of all: perspective rungs incl. host/protocol, stem shapes, aspirational-schema designators, `#`-fragments (`@element[key]:attribute`), and the match-vs-walk dimension that types wikilinks. See the crossmap note beside this file.

**6. Submodule/parallelism.** Instance-as-submodule for write-parallelism has the relata precedent (externalized data, globally-addressed); it strengthens the registry argument — a mapping that lives *with* the instance survives the submodule move, while project-root config would need the same alias machinery slugs already get. `example/` and `template/` as addressable pseudo-instances is quietly excellent: kit exemplars ( [[form-exemplar-seeds]]) become citable in the same scheme as live corpora.

**Routing:** this file stays live until (a) its content lands in [[form-addressing]] / [[form-store-triplet]] drafting, and (b) the demand is delivered to the udon paths/references effort in whatever form that project's influx wants. PRACTICA carries the thread.
