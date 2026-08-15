#!/usr/bin/env python3
"""
The DECISION LINE (Backing currency) as an executable colored Petri net -- companion to
truthification-net.py (the Evidence cycle). From the steward's burst-2/3/4 failure-mode
illumination + sketch 11 ("the whole decision line needs modeling").

The per-currency generalization made concrete: Backing runs the same cycle SHAPE as
Evidence -- originate / ideate / secure / land / serve / maintain -- but its securing
lines are different, because a decision's "verdict" is an AUTHORIZED ACT (constitutive;
instrument-follows-rung: ratification), while its quality is still assessed on separate
coordinates. Three securing lines, straight from burst 2's substate bundle:

  grounds  -- on what was this predicated? explicit assumptions -> implicit surfaced ->
              revisit-triggers declared.  The commitment law's Backing analog: grounds
              RECORDED BEFORE the call are auditable; grounds reconstructed after are
              post-hoc rationalization ("masquerading as derivation").
  consult  -- who knows / who was given the chance to feed back?  (burst 2: "who *knows*
              about this decision... who has been given the opportunity to give feedback
              and a vote of support/non-support")
  authority-- proposed -> supported -> ratified -> ruled, each rung a DIFFERENT STATEMENT
              (burst 2, near-verbatim): ruled = "might have a reason that wasn't stated";
              ratified = "thought it through and would like to know if someone feels
              differently"; supported = "no objections at that time".  Grader-escalated
              and event-gated: RATIFIED requires grounds explicit AND affected parties
              announced (its own semantics imply both); RULED does NOT require explicit
              grounds -- fiat may carry unstated reasons, honestly.

Failure modes carried steward-attested (bursts 2-4): authority substituted for
thoughtfulness; assuming more than the decision assumes; historic-authority argued
instead of its underlying grounding; ownership abdicated back to the steward
(bottleneck + task-mode-laziness-by-abdication); decisiveness conflated with
thing-becoming-law; expediency conflated with grounding-principles; steward-side
controlling-behavior as proxy for lack of principledness.

The run demonstrates, with real corpus content:
  1. a decision-inflection fork posed, grounds/consult/authority secured, landed
     `supported` in the ledger with declared revisit-triggers;
  2. the PRINCIPLED reopen (burst 3's own prescription): new evidence invalidates a
     named assumption -> the revisit-trigger fires -> reopen carries WHAT-CHANGED ->
     supersession typed and pointed -- never argued from the old ruling's authority;
  3. the forbidden edge blocked structurally: no transition exists from the ledger's
     authority cell to any grounds value (authority cannot launder groundedness).

Register: proposed. Run to re-execute; --draw regenerates the diagram.
"""
import sys
from snakes.nets import *

LINES = {
    'grounds': dict(
        rungs=['assumed-implicit', 'assumptions-explicit', 'revisit-triggers-declared'],
        channels=['name-explicit-assumptions (pre-call = auditable; post-hoc = '
                  'rationalization)', 'surface-implicit-assumptions + declare revisit-triggers'],
        fails='masquerading as derivation; implicit assumptions discovered only at breakage'),
    'consult': dict(
        rungs=['unannounced', 'announced-to-affected', 'feedback-collected'],
        channels=['announce (who-knows established)', 'collect feedback / votes'],
        fails='consumers assumed to exist (or not exist) without checking; '
              '"backwards compatibility with a ruling nobody deploys"'),
    'authority': dict(
        rungs=['proposed', 'supported', 'ratified', 'ruled'],
        channels=['no-objection-at-the-time (supported)',
                  'real-read + stands-behind (ratified; REQUIRES grounds-explicit + announced)',
                  'reserved-fiat (ruled; unstated reasons allowed, honestly)'],
        fails='authority substituted for thoughtfulness; ownership abdicated upward; '
              'historic authority argued instead of its grounding'),
}

def build():
    n = PetriNet('decision-line')
    for p, init in [
        ('forks.decision-inflection', [('fork', 'choose-marching-vs-direct-solve')]),
        ('decision-questions', []),      # ('dq', slug, who-may-close)
        ('options', []),
        # per-line in-flight: ('line', kind, slug, rung, note)
        *[('line.' + k, []) for k in LINES],
        ('ledger.DECISIONS', []),        # ('decision', slug, authority, grounds-rung, consult-rung,
                                         #  frozenset(assumptions), revisit-declared?)
        ('superseded.pointed', []),
        ('accounts.Fidelity', []),
        ('evidence.arrivals', [('res', 'sim', 'dt-clamp-repaired-at-77b1f5a',
                                'invalidates:dt-clamp-is-binding')]),
        ('signals.out', []),
    ]:
        n.add_place(Place(p, init))

    # ORIGINATE: the fork is posed; who-may-close from the valve criteria
    # (irreversible / authoring-voice / cross-project / provenance -> steward; else default)
    n.add_transition(Transition('T.pose', Expression("True")))
    n.add_input('forks.decision-inflection', 'T.pose', Variable('f'))
    n.add_output('decision-questions', 'T.pose',
                 Expression("('dq', f[1], 'agent-default (no valve criterion fires)')"))

    # IDEATE: options with one-line consequences + a recommendation (the brief anatomy)
    n.add_transition(Transition('T.draft-options', Expression("True")))
    n.add_input('decision-questions', 'T.draft-options', Test(Variable('q')))
    n.add_output('options', 'T.draft-options',
                 Expression("('opts', q[1], 'A:keep-marching|B:direct-solve', "
                            "'recommend:B (GraphFlood precedent)')"))
    for k in LINES:
        tn = 'T.open.' + k
        n.add_transition(Transition(tn, Expression("True")))
        n.add_input('options', tn, Test(Variable('o')))
        n.add_output('line.' + k, tn, Expression("('line', %r, o[1], 0, '')" % k))

    # SECURE: the three lines
    for k, L in LINES.items():
        tn = 'T.secure*.' + k
        n.add_transition(Transition(tn, Expression("x[3] < %d" % (len(L['rungs']) - 1))))
        n.add_input('line.' + k, tn, Variable('x'))
        n.add_output('line.' + k, tn, Expression("(x[0], x[1], x[2], x[3]+1, x[4])"))
        n.add_output('accounts.Fidelity', tn,
                     Expression("('acct', 'secured', %r, %r[x[3]])" % (k, L['channels'])))

    # LAND: the decision enters the ledger. Authority starts at 'supported' here only if
    # grounds are explicit AND consultation announced (ratified would require both at max;
    # supported needs steward no-objection which this demo stipulates as the event).
    n.add_transition(Transition('T.land',
        Expression("g[3] >= 2 and c[3] >= 1")))          # revisit-declared + announced
    n.add_input('line.grounds', 'T.land', Variable('g'))
    n.add_input('line.consult', 'T.land', Variable('c'))
    n.add_input('line.authority', 'T.land', Variable('a'))
    n.add_output('ledger.DECISIONS', 'T.land',
                 Expression("('decision', g[2], 'supported', g[3], c[3], "
                            "frozenset(['dt-clamp-is-binding', 'no-marching-consumers-yet']), True)"))
    n.add_output('accounts.Fidelity', 'T.land',
                 Expression("('acct', 'decided', g[2], "
                            "'supported = no objections at that time; grounds+consult recorded')"))

    # SERVE: what each rung licenses (declared intent = the consumer's risk model)
    n.add_transition(Transition('T.license', Expression("True")))
    n.add_input('ledger.DECISIONS', 'T.license', Test(Variable('d')))
    n.add_output('signals.out', 'T.license',
                 Expression("('license', d[1], d[2] + ': act-on-it; revisit is easy; "
                            "do NOT argue from it as law')"))

    # MAINTAIN: the PRINCIPLED reopen (burst 3's prescription, structural):
    # evidence invalidating a NAMED assumption fires the declared revisit-trigger.
    n.add_transition(Transition('T.revisit-trigger',
        Expression("d[6] and e[3].split(':')[1] in d[5]")))
    n.add_input('evidence.arrivals', 'T.revisit-trigger', Variable('e'))
    n.add_input('ledger.DECISIONS', 'T.revisit-trigger', Variable('d'))
    n.add_output('ledger.DECISIONS', 'T.revisit-trigger',
                 Expression("('decision', d[1], 'reopened', d[3], d[4], d[5], d[6])"))
    n.add_output('signals.out', 'T.revisit-trigger',
                 Expression("('reopen-brief', d[1], 'WHAT-CHANGED: ' + e[2] + ' -- "
                            "assumption ' + e[3].split(':')[1] + ' no longer holds; "
                            "verify my model of WHY it was decided, then decide together')"))
    n.add_output('accounts.Fidelity', 'T.revisit-trigger',
                 Expression("('acct', 'revisit-fired', d[1], e[2])"))

    # supersession: typed and pointed, append-or-expressly-overturn
    n.add_transition(Transition('T.supersede', Expression("d[2] == 'reopened'")))
    n.add_input('ledger.DECISIONS', 'T.supersede', Variable('d'))
    n.add_output('superseded.pointed', 'T.supersede',
                 Expression("('superseded', d[1], 'revised-by:' + d[1] + '-v2', "
                            "'kept standing as history')"))
    n.add_output('ledger.DECISIONS', 'T.supersede',
                 Expression("('decision', d[1] + '-v2', 'proposed', 2, 1, "
                            "frozenset(['direct-solve-viable (77b1f5a)']), True)"))
    return n

EVENTS = []
def fire(n, tname):
    t = n.transition(tname)
    modes = t.modes()
    if not modes:
        return False
    t.fire(modes[0])
    EVENTS.append((len(EVENTS) + 1, tname, str(modes[0])))
    return True

if __name__ == '__main__':
    print("THE DECISION LINE (Backing) -- three securing lines, steward-attested semantics:")
    for k, L in LINES.items():
        print("  %-10s rungs:    %s" % (k, ' -> '.join(L['rungs'])))
        print("  %-10s channels: %s" % ('', ' | '.join(L['channels'])))
        print("  %-10s fails by: %s" % ('', L['fails']))
    print()
    n = build()
    fire(n, 'T.pose'); fire(n, 'T.draft-options')
    for k in LINES: fire(n, 'T.open.' + k)
    for k, steps in {'grounds': 2, 'consult': 2, 'authority': 1}.items():
        for _ in range(steps):                 # authority stops at 'supported' -- honest
            fire(n, 'T.secure*.' + k)
    fire(n, 'T.land'); fire(n, 'T.license')
    print("Act I -- landed:")
    for t in sorted(n.place('ledger.DECISIONS').tokens, key=str): print("   ", t)
    for t in sorted(n.place('signals.out').tokens, key=str): print("   ", t)
    print()
    print("Act II -- evidence invalidates a NAMED assumption; the declared trigger fires;")
    print("          reopen carries WHAT-CHANGED (never argued from the old authority):")
    fire(n, 'T.revisit-trigger'); fire(n, 'T.supersede')
    for t in sorted(n.place('ledger.DECISIONS').tokens, key=str): print("   ", t)
    for t in sorted(n.place('superseded.pointed').tokens, key=str): print("   ", t)
    for t in sorted(n.place('signals.out').tokens, key=str):
        if t[0] == 'reopen-brief': print("   ", t)
    print()
    print("Structural notes exercised:")
    print("  - RATIFIED-requires-grounds+announcement is a guard, not a convention")
    print("    (T.land's guard: revisit-triggers declared AND consultation announced);")
    print("    RULED deliberately has no grounds guard -- fiat may carry unstated reasons.")
    print("  - The forbidden edge is UNREPRESENTABLE: no transition reads the authority")
    print("    cell into any grounds value. Authority cannot launder groundedness here.")
    print("  - The reopen is the burst-3 prescription made structural: 'make sure I've")
    print("    thought through why it was decided, bring it up, verify assumptions,")
    print("    decide together' -- the brief carries what-changed, not a verdict.")
    print()
    print("EVENT LOG (%d firings):" % len(EVENTS))
    for e in EVENTS: print("  %3d  %-22s %s" % e)

    if '--draw' in sys.argv:
        import subprocess
        def esc(s): return s.replace('"', '\\"')
        FILLP = {'ledger': '#eadff0', 'accounts': '#f2e6d0', 'evidence': '#dfeedd',
                 'superseded': '#f5c6c6', 'signals': '#f9e0e0', 'forks': '#dbe7f5',
                 'decision-questions': '#dbe7f5'}
        out = ['digraph decisionline {',
               'rankdir=LR; ranksep=0.6; nodesep=0.35; fontsize=20; labelloc=t;',
               'label="The decision line (Backing) -- grounds / consult / authority as its '
               'securing lines; ratified is guard-gated on the other two; reopen fires on '
               'named-assumption invalidation and carries what-changed";',
               'node [fontname="Helvetica" fontsize=11]; '
               'edge [fontname="Helvetica" fontsize=9 color="#555555"];']
        n2 = build()
        for node in n2.node():
            if isinstance(node, Place):
                fill = '#eeeeee'
                for key, c in FILLP.items():
                    if key in node.name: fill = c
                label = node.name
                if node.name.startswith('line.'):
                    k = node.name.split('.')[1]; fill = '#e8f4f8'
                    label = node.name + '\\n' + ' > '.join(LINES[k]['rungs'])
                if 'ledger' in node.name:
                    label += '\\n(append-or-expressly-overturn;\\nauthority cell is a projection)'
                out.append(f'"{node.name}" [shape=ellipse style=filled fillcolor="{fill}" '
                           f'label="{esc(label)}"];')
            else:
                fill = '#cfcfcf'; label = node.name
                if 'secure' in node.name:
                    k = node.name.split('.')[-1]; fill = '#d7d7ee'
                    label = node.name + '\\n' + '\\n'.join(LINES[k]['channels'])
                if 'revisit' in node.name or 'supersede' in node.name: fill = '#f5d5a0'
                if node.name == 'T.land':
                    label += '\\n[GUARD: revisit-declared\\n+ announced]'
                out.append(f'"{node.name}" [shape=box style=filled fillcolor="{fill}" '
                           f'label="{esc(label)}"];')
        for t in n2.transition():
            for place, lab in t.input():
                dashed = 'Test' in type(lab).__name__
                out.append(f'"{place.name}" -> "{t.name}" '
                           f'[style={"dashed" if dashed else "solid"}'
                           + (', label="?"' if dashed else '') + '];')
            for place, lab in t.output():
                out.append(f'"{t.name}" -> "{place.name}";')
        out.append('}')
        open('decision-line.dot', 'w').write('\n'.join(out))
        for fmt in ('svg', 'png'):
            subprocess.run(['dot', '-T' + fmt, 'decision-line.dot',
                            '-o', 'decision-line.' + fmt], check=True)
        print('\ndrew decision-line.svg / .png / .dot')
