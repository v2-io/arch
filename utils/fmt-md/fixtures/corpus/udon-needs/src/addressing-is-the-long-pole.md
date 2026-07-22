---
slug: addressing-is-the-long-pole
type: demand
evidence: [T1, T2, T3]
status: cross-tier-convergent (demand); all syntax questions deliberately open
stage: drafted (bridge form, 2026-07-22 — absorbed tables now live in the promoted report)
consumers: both (udon-primary)
depends: [schema-guarded-mutation, freshness-and-atomicity]
opens: reports/addressing-exploration.md
handoff-routing: feeds the paths design probe (phase 3); this bridge + the exploration report are its brief-context  # auditor apparatus
sources:
  - ../reports/addressing-exploration.md  # the body report this bridge opens
  - ../reports/agent-utility-exploration.md  # §3 "Addressing is load-bearing"
  - ../../01-ideation/02-provenanced/syntheses/tier2-invivo-digest.md  # yq match() singleton
  - ../../../DECISIONS.md  # PATH-1, S14; ../../OPEN.md S3
---

# Addressing is the long pole

The last two chapters left the same debt twice. A guarded edit needs a way
to say *where* the guarantee applies; a freshness check needs a way to say
*what* must still be true at write time. Both assume something this
notation does not yet have: a stable way to **name a place in a
document**. This chapter is about that missing piece — why nearly
everything else waits on it, what the evidence already pins about its
shape, and what remains genuinely open.

**Why it is the long pole.** Count the mouths pulling on one design: the
edit tool needs targets; a refusal that says "did you mean one of these?"
needs paths it can print; a document skeleton is only useful if its lines
can be pasted back as addresses; queries need a way to ask for exactly-one
and for many; in-document references are addresses the *document* makes;
and templates turn out to want them too — when a template's data context
is itself a document, "insert the value of X" is a path expression. Two
independent examinations of this territory arrived at the same sentence:
everything bottoms out on addressing. The full consumer map, with the
collisions between them, is the first thing the
[addressing exploration](../reports/addressing-exploration.md) lays out.

**What the evidence already pins.** Four findings, each argued in full in
the exploration:

1. **Agents address relationally, not positionally.** A set of
   walkthroughs followed an agent through a full day of realistic
   document work — reading unfamiliar files, making guarded edits,
   coordinating with a second writer. Nearly every query in that day
   began "find the element with this key, at any depth" — the tree
   served as storage, not as the mental model. (One day's sample, and the exploration says so — but it
   inverts the XPath-style assumption that root-to-leaf navigation comes
   first.)
2. **Asking-for-one and asking-for-many are different questions.** A
   query that expects exactly one match should *fail loudly* when there
   are zero or two — a silently empty result leaves the agent unable to
   tell "not present" from "my path is wrong" from "the document changed
   under me." Query languages that return quiet empty sets teach exactly
   the habit an agent must not learn.
3. **Failures at an address need different names.** "Two elements matched
   my path," "an attribute holds two stacked values," and "a reference
   resolves to several definitions" are three different situations with
   three different repairs — and staleness (the previous chapter's
   subject) is a fourth. One error name covering any two of them routes a
   reader to the wrong fix.
4. **A path must be writable *inside* a document without breaking it.**
   Today a path in a document must be quoted, because bare `|` or `@` in
   value position already means something. Whether paths ever get a bare
   in-document form is exactly the kind of question that cannot be
   answered locally — it collides with value termination, arrays, and
   inline forms, and the exploration's unfinished stress table shows how
   sharp those collisions get.

One piece of shipping prior art deserves its own sentence: of the
fourteen harnesses examined, exactly one tool treats *position in the
source* as first-class queryable data (a match operator returning offset,
length, and captures) — the shape a span-precise edit substrate needs,
and otherwise absent from the ecosystem.

**What is already decided, and what is genuinely open.** Three boundary
decisions frame the design space. First, the question of where a path
stops. Picture the situation that raises it: an agent is working through
a document, follows a reference, and the reference points at material
that lives in *another file*. Does the path language itself express that
hop — or does the path stop at the file's edge, with "open the other
document, then path within it" left to the tool driving the session? The
walkthroughs of realistic agent workdays consistently took the second
view, and a path language could plausibly stop there forever. But that
boundary has been ruled out as a *permanent* assumption (the ruling is
recorded in the [[DECISIONS.md|design ledger]]): cross-document
addressing is in scope for the eventual design, so a tool that hard-codes
"paths never leave this file" into its workflows would be built on sand.
Second: UDON documents can already point at their own elements with a
small reference form, and the temptation is to grow it a feature at a
time toward a path language. That road is closed — each incremental field
would be a constraint the real path language later has to honor or break,
debt without a design — so the reference form stays frozen exactly as it
is until a whole path language replaces it (also a recorded
[[DECISIONS.md|ledger]] ruling). Third: an element can carry more than
one key, and how addressing should treat that is explicitly undecided —
an [[OPEN.md|open question]], not an oversight. Everything else — the syntax, the verbs, whether positional
access ever becomes language rather than tooling — is deliberately open:
this report maps the demand and declines to design.

**Where to go from here.** Read the
[addressing exploration](../reports/addressing-exploration.md) whole — it
is the deepest single treatment of this territory. Its nine provisional
boundary demands (§8) are the demand floor; its trap list (§9) records
the dead ends already found so the design work ahead does not rediscover
them as ideas; its open questions (§10) are the sharpest current
statement of what any future path language has to answer — starting with
the load-bearing one: *what is the smallest in-document reference form
that is still a true subset of the full path language?* The
[agent-utility exploration](../reports/agent-utility-exploration.md) §3
independently corroborates the dependency map from the tool side.

**Who reads this and when:** whoever takes up path-language design starts
here and reads the exploration whole before sketching syntax. The harness
reads findings 2–3 as requirements on any tool that reports locations to
agents, in any notation: two verbs, loud failure, distinct failure names.

## Honest edges

The relational-first finding rests on a one-day sample; the consumer map
is design-corpus convergence that largely shares an author. This is a
demand map, not a validated design, and the exploration's own unfinished
stress table means every claim about in-document embeddability is soft
until that table is forced.
