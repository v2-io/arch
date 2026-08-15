#!/usr/bin/env python3
"""
Truthification cycle as a colored Petri net (SNAKES) -- v2, ladders fleshed out.

From Joseph's 2026-08-14 sketch: observe -> ideate -> concurrent truth-lines
(each line's [*] is a LADDER: a guarded self-loop whose guard vocabulary names
the securing channels that may fire) -> typed results -> async pay-in to a
standing claim-ledger -> decide/teach/apply, with loop closure.

v2 adds, per line, the estate's actual ladder content:

  math       rungs sketch->drafted->deps-verified->re-derived; ceiling exact-under-premises;
             conclusion is theorem/no-go/bound (a no-go also mints a question)
  sim        rungs pre-registered->run-once->swept->independently-reproduced; results ERA-KEYED
             (a kernel/version bump EXPIRES them: certificates decay)
  lit        rungs named-not-read->recalled->search-corroborated->verified-via-secondary->
             primary-verified; strength CAPPED at the source's own tier (transmitted ceiling law);
             walls move: a later primary fetch can re-verify upward
  testimony  witness POSITION is fixed at capture (only a closer witness changes it);
             what corroboration moves is the CREDIBILITY column -- two columns, never one grade
  analog     no adjudicative ladder: hunch->articulated->perturbation-tested; survival licenses
             continued generative use only; the line terminates by FEEDING IDEATION, not by result

Also demonstrated when run:
  - status-as-projection-of-events (every firing logged; no hand-set cell is expressible)
  - the Kahn obligation (pay-in order-invariance, verified over two arrival orders)
  - convergent lock arms only across failure-mode-INDEPENDENT families
  - certificate decay (era bump disarms the lock) and re-verification (primary fetch re-arms it)
"""
import sys
from snakes.nets import *

# ---------------------------------------------------------------- ladder declarations
# A ladder IS: its rung names (color ordering) + its channel vocabulary (the guard
# vocabulary: which named securing acts may fire, one per rung-step) + its family
# (failure-mode independence class, for the lock) + its ceiling law.
LADDERS = {
    'math': dict(
        family='derivation',
        rungs=['sketch', 'drafted', 'deps-verified', 're-derived'],
        channels=['formal-expression-written', 'premises-named-and-checked',
                  'independent-re-derivation'],
        ceiling='exact (under named premises); conditional if any premise is empirical'),
    'sim': dict(
        family='in-vivo-measurement',
        rungs=['pre-registered', 'run-once', 'swept', 'independently-reproduced'],
        channels=['first-run (era-keyed)', 'seed/level/footprint-sweep',
                  'independent-re-run'],
        ceiling='exact for the authored world (identifiability by construction); '
                'empirical for transfer to the wild; every number era-keyed'),
    'lit': dict(
        family='transmission:empirical-wild',   # WRAPPER: carriage over the source's own family;
                                                # never independent of that family for the lock
        rungs=['named-not-read', 'recalled', 'search-corroborated',
               'verified-via-secondary', 'primary-verified'],
        channels=['recall-marked-as-recall', 'convergent-secondaries',
                  'faithful-secondary-fetched', 'primary-fetched'],
        ceiling='min(rung, source own tier) -- transmitted ceiling caps at source; '
                'defeasible two ways: source wrong OR carriage infidelious; '
                'family = transmission:<source-family> -- a wrapper, never lock-independent of it'),
    'testimony': dict(
        family='testimony',
        rungs=['captured', 'corroborated', 'cross-kind-corroborated'],   # CREDIBILITY column
        channels=['independent-account-located', 'independent-kind-agrees'],
        ceiling='position fixed at capture (attested/reconstructed/secondhand); '
                'corroboration raises credibility only, never position'),
    'analog': dict(
        family='analogy',
        rungs=['hunch', 'articulated', 'perturbation-tested'],
        channels=['analogy-articulated', 'perturbation-test-passed'],
        ceiling='heuristic; survival licenses generative use, never assertion'),
}
SRC_TIER_LIT = 2   # this run's external source is itself robust-qualitative at best

def build():
    n = PetriNet('truthification-cycle-v2')
    for p, init in [
        ('observations.Fidelity', [('obs', 'water-fill-never-settles')]),
        ('questions.Disposition', [('q', 'does-the-fill-settle')]),
        ('counterfactuals',       [('cf', 'solve-equilibrium-directly')]),
        ('claims.Evidence',       []),
        ('line.math', []), ('line.sim', []), ('line.lit', []),
        ('line.testimony', []), ('line.analog', []),
        ('results', []),
        ('accounts.Fidelity', []),
        ('ledger.standing-surface', []),   # ('ledger', slug, frozenset((kind,strength,family,note)))
        ('expired.certificates', []),      # decayed entries, kept visible (never silently gone)
        ('decisions.Backing', []),
        ('pedagogy.Comprehension', []),
    ]:
        n.add_place(Place(p, init))

    # ---- ideation (generative; reads sources without consuming) ----
    n.add_transition(Transition('T.ideate', Expression("True")))
    n.add_input('observations.Fidelity', 'T.ideate', Test(Variable('o')))
    n.add_input('questions.Disposition', 'T.ideate', Test(Variable('q')))
    n.add_input('counterfactuals',       'T.ideate', Test(Variable('c')))
    n.add_output('claims.Evidence', 'T.ideate', Expression("('claim', q[1])"))
    n.add_output('ledger.standing-surface', 'T.ideate',
                 Expression("('ledger', q[1], frozenset())"))
    n.add_output('accounts.Fidelity', 'T.ideate', Expression("('acct','ideated',q[1],'')"))

    # ---- launch lines (fan-out; claim persists) ----
    for kind in LADDERS:
        tn = 'T.launch.' + kind
        n.add_transition(Transition(tn, Expression("True")))
        n.add_input('claims.Evidence', tn, Test(Variable('cl')))
        extra = "'k1'" if kind == 'sim' else ("'reconstructed'" if kind == 'testimony' else "''")
        n.add_output('line.' + kind, tn,
                     Expression("('line', %r, cl[1], 0, %s)" % (kind, extra)))

    # ---- the ladders: one guarded self-loop per line; the CHANNEL fired is a
    #      function of the current rung (the guard vocabulary made operational);
    #      every firing emits the securing event into accounts ----
    for kind, L in LADDERS.items():
        tn = 'T.secure*.' + kind
        n.add_transition(Transition(tn, Expression("x[3] < %d" % (len(L['rungs']) - 1))))
        n.add_input('line.' + kind, tn, Variable('x'))
        n.add_output('line.' + kind, tn, Expression("(x[0], x[1], x[2], x[3]+1, x[4])"))
        n.add_output('accounts.Fidelity', tn,
                     Expression("('acct','secured', %r, %r[x[3]])" % (kind, L['channels'])))

    # ---- conclusions, typed per line ----
    # math: at re-derived -> exact-under-premises result (strength 3) + a fresh question
    n.add_transition(Transition('T.conclude.math', Expression("x[3] >= 3")))
    n.add_input('line.math', 'T.conclude.math', Variable('x'))
    n.add_output('results', 'T.conclude.math',
                 Expression("('res','math', x[2], 3, 'derivation', 'exact|premises-named')"))
    n.add_output('questions.Disposition', 'T.conclude.math',
                 Expression("('q', 'what-does-the-bound-mean-for-' + x[2])"))
    # sim: at swept -> strength 2, era-keyed (reproduced would give 3)
    n.add_transition(Transition('T.conclude.sim', Expression("x[3] >= 2")))
    n.add_input('line.sim', 'T.conclude.sim', Variable('x'))
    n.add_output('results', 'T.conclude.sim',
                 Expression("('res','sim', x[2], (2 if x[3]==2 else 3), 'in-vivo-measurement', x[4])"))
    # lit: strength = min(rung-derived, source tier)  [transmitted ceiling law]
    n.add_transition(Transition('T.conclude.lit', Expression("x[3] >= 2")))
    n.add_input('line.lit', 'T.conclude.lit', Variable('x'))
    n.add_output('results', 'T.conclude.lit',
                 Expression("('res','lit', x[2], min(%d, 1 if x[3] < 3 else 2), "
                            "'transmission:empirical-wild', 'rung='+str(x[3]))" % SRC_TIER_LIT))
    # testimony: contributes at CREDIBILITY (rung), position rides in the note, fixed
    n.add_transition(Transition('T.conclude.testimony', Expression("x[3] >= 1")))
    n.add_input('line.testimony', 'T.conclude.testimony', Variable('x'))
    n.add_output('results', 'T.conclude.testimony',
                 Expression("('res','testimony', x[2], x[3], 'testimony', "
                            "'position='+x[4]+' (fixed); credibility='+str(x[3]))"))
    # analog: no result -- at perturbation-tested it FEEDS IDEATION and stands down
    n.add_transition(Transition('T.feed-ideation.analog', Expression("x[3] >= 2")))
    n.add_input('line.analog', 'T.feed-ideation.analog', Variable('x'))
    n.add_output('counterfactuals', 'T.feed-ideation.analog',
                 Expression("('cf', 'analogy-suggests:' + x[2])"))
    n.add_output('accounts.Fidelity', 'T.feed-ideation.analog',
                 Expression("('acct','analogy-fed-ideation', x[2], 'survived-perturbation')"))


    # ---- lit's OTHER two roles (steward correction, 2026-08-14): it feeds the other
    #      lines early, and can inject a scope-change/no-go acting on the claim itself ----
    n.add_transition(Transition('T.lit-informs-design', Expression("x[3] >= 1")))
    n.add_input('line.lit', 'T.lit-informs-design', Test(Variable('x')))
    n.add_output('counterfactuals', 'T.lit-informs-design',
                 Expression("('cf', 'prior-art-informs:' + x[2])"))
    n.add_transition(Transition('T.lit-scope-influence', Expression("x[3] >= 2")))
    n.add_input('line.lit', 'T.lit-scope-influence', Test(Variable('x')))
    n.add_output('questions.Disposition', 'T.lit-scope-influence',
                 Expression("('q', 'scope-check:' + x[2] + ':external-bound-found')"))

    # ---- async pay-in: bag union (order-invariant by construction) ----
    n.add_transition(Transition('T.pay-in', Expression("r[2] == l[1]")))
    n.add_input('results', 'T.pay-in', Variable('r'))
    n.add_input('ledger.standing-surface', 'T.pay-in', Variable('l'))
    n.add_output('ledger.standing-surface', 'T.pay-in',
                 Expression("('ledger', l[1], l[2] | frozenset([(r[1], r[3], r[4], r[5])]))"))

    # ---- decay: an era bump EXPIRES era-keyed certificates (visibly, never silently) ----
    n.add_transition(Transition('T.era-bump.k1-to-k2',
        Expression("any(e[0]=='sim' and e[3]=='k1' for e in l[2])")))
    n.add_input('ledger.standing-surface', 'T.era-bump.k1-to-k2', Variable('l'))
    n.add_output('ledger.standing-surface', 'T.era-bump.k1-to-k2',
                 Expression("('ledger', l[1], frozenset(e for e in l[2] "
                            "if not (e[0]=='sim' and e[3]=='k1')))"))
    n.add_output('expired.certificates', 'T.era-bump.k1-to-k2',
                 Expression("('expired', l[1], 'sim@k1', 'kernel-era-boundary')"))
    n.add_output('questions.Disposition', 'T.era-bump.k1-to-k2',
                 Expression("('q', 're-run-' + l[1] + '-under-k2')"))

    # ---- re-verification: the wall moved; a primary fetch raises lit to its cap ----
    n.add_transition(Transition('T.reverify.lit-primary-fetched',
        Expression("any(e[0]=='lit' and e[1] < %d for e in l[2])" % SRC_TIER_LIT)))
    n.add_input('ledger.standing-surface', 'T.reverify.lit-primary-fetched', Variable('l'))
    n.add_output('ledger.standing-surface', 'T.reverify.lit-primary-fetched',
                 Expression("('ledger', l[1], frozenset(e for e in l[2] if e[0]!='lit') "
                            "| frozenset([('lit', %d, 'transmission:empirical-wild', "
                            "'primary-fetched; capped-at-source-tier')]))" % SRC_TIER_LIT))
    n.add_output('accounts.Fidelity', 'T.reverify.lit-primary-fetched',
                 Expression("('acct','re-verified','lit','wall-moved: primary now fetchable')"))

    # ---- downstream (consult, never consume) + loop closure ----
    n.add_transition(Transition('T.decide', Expression("len(l[2]) > 0")))
    n.add_input('ledger.standing-surface', 'T.decide', Test(Variable('l')))
    n.add_output('decisions.Backing', 'T.decide', Expression("('decision', l[1], 'proposed')"))
    n.add_transition(Transition('T.teach', Expression("len(l[2]) > 0")))
    n.add_input('ledger.standing-surface', 'T.teach', Test(Variable('l')))
    n.add_output('pedagogy.Comprehension', 'T.teach', Expression("('primer', l[1])"))
    n.add_transition(Transition('T.apply', Expression("True")))
    n.add_input('decisions.Backing', 'T.apply', Test(Variable('d')))
    n.add_output('observations.Fidelity', 'T.apply', Expression("('obs', 'applied-' + d[1])"))
    return n

# ---------------------------------------------------------------- simulation
EVENTS = []
def fire(n, tname, pick=None):
    t = n.transition(tname)
    modes = t.modes()
    if not modes:
        return False
    mode = modes[0] if pick is None else pick(modes)
    t.fire(mode)
    EVENTS.append((len(EVENTS) + 1, tname, str(mode)))
    return True

CLIMB = {'math': 3, 'sim': 2, 'lit': 2, 'testimony': 1, 'analog': 2}  # rungs walked this run

def run(payin_order):
    global EVENTS
    EVENTS = []
    n = build()
    fire(n, 'T.ideate')
    for k in LADDERS: fire(n, 'T.launch.' + k)
    for k, steps in CLIMB.items():
        for _ in range(steps):
            fire(n, 'T.secure*.' + k)
    fire(n, 'T.lit-informs-design'); fire(n, 'T.lit-scope-influence')
    for k in ('math', 'sim', 'lit', 'testimony'):
        fire(n, 'T.conclude.' + k)
    fire(n, 'T.feed-ideation.analog')
    for kind in payin_order:
        fire(n, 'T.pay-in', pick=lambda ms, kind=kind: next(m for m in ms if m['r'][1] == kind))
    fire(n, 'T.decide'); fire(n, 'T.teach'); fire(n, 'T.apply')
    return n

def ledger_of(n):
    return list(n.place('ledger.standing-surface').tokens)[0]

def projections(ledger):
    entries = sorted(ledger[2])
    strengths = {e[0]: e[1] for e in entries}
    mx = max(strengths.values()) if strengths else 0
    floor = min(strengths.values()) if strengths else 0
    strong = [e for e in entries if e[1] >= 2]
    base = lambda fam: fam.split(':')[-1]          # look through the transmission wrapper
    lock = 'ARMED' if len({base(e[2]) for e in strong}) >= 2 else 'unarmed'
    return entries, mx, floor, lock

def show(tag, n):
    entries, mx, floor, lock = projections(ledger_of(n))
    print("  %-28s max=%d floor=%d lock=%-8s" % (tag, mx, floor, lock))
    for e in entries:
        print("      (%s, strength=%d, family=%s, %s)" % e)
    return mx, floor, lock

if __name__ == '__main__':
    print("LADDERS (guard vocabulary + rung ordering + ceiling law, per line):")
    for k, L in LADDERS.items():
        print("  %-10s family=%-20s rungs=%s" % (k, L['family'], ' -> '.join(L['rungs'])))
        print("  %-10s channels: %s" % ('', ' | '.join(L['channels'])))
        print("  %-10s ceiling:  %s" % ('', L['ceiling']))
    print()

    n1 = run(['sim', 'math', 'lit', 'testimony'])
    ev1 = list(EVENTS)
    n2 = run(['testimony', 'lit', 'math', 'sim'])
    p1, p2 = projections(ledger_of(n1)), projections(ledger_of(n2))
    print("KAHN OBLIGATION: reversed pay-in order ->",
          "HOLDS (identical ledgers + projections)" if p1 == p2 else "VIOLATED")
    print()
    print("Act I -- after all lines pay in:")
    show("(testimony@credibility=1 present but position fixed)", n1)
    print()
    print("Act II -- kernel era bump k1->k2 (certificates decay, visibly):")
    fire(n1, 'T.era-bump.k1-to-k2')
    show("sim@k1 expired -> question minted", n1)
    print("      expired-place holds:", sorted(n1.place('expired.certificates').tokens, key=str))
    print("      new question:", [t for t in n1.place('questions.Disposition').tokens if 'k2' in str(t)])
    print()
    print("Act III -- the wall moved: primary fetched, lit re-verified to its source-tier cap:")
    fire(n1, 'T.reverify.lit-primary-fetched')
    show("lit raised 1->2 (capped, base family empirical-wild): lock re-arms", n1)
    # counter-demo: had the external source been a DERIVATION, the wrapper look-through
    # would refuse to re-arm (math is already our derivation leg)
    hyp = frozenset([('math',3,'derivation','x'), ('lit',2,'transmission:derivation','x')])
    strong = [e for e in hyp if e[1] >= 2]
    hyp_lock = 'ARMED' if len({e[2].split(':')[-1] for e in strong}) >= 2 else 'unarmed'
    print("      counter-demo: same strengths but source=derivation ->", hyp_lock,
          "(transmitted derivation is not independent of our math leg)")
    print()
    print("EVENT LOG head (run 1, first 12 of %d firings):" % len(ev1))
    for e in ev1[:12]:
        print("  %3d  %-26s %s" % e)
    print()
    print("accounts substrate (the securing events, with their channel names):")
    for a in sorted(n1.place('accounts.Fidelity').tokens, key=str):
        print("   ", a)

    # ---------------------------------------------------------------- drawing
    if '--draw' in sys.argv:
        import snakes.plugins
        snakes.plugins.load('gv', 'snakes.nets', 'gvnets')
        # rebuild under the plugin-backed classes
        import importlib, types
        src = open(__file__).read()
        src = src.split('# ---------------------------------------------------------------- simulation')[0]
        src = src.replace('from snakes.nets import *', 'from gvnets import *')
        ns = {'__name__': 'netmod'}
        exec(compile(src, __file__, 'exec'), ns)
        gn = ns['build']()
        def place_attr(place, attr):
            attr['shape'] = 'ellipse'; attr['style'] = 'filled'
            name = place.name; attr['label'] = name; attr['fillcolor'] = '#eeeeee'
            for k, c in {'Fidelity': '#f2e6d0', 'Disposition': '#dbe7f5',
                         'Evidence': '#dfeedd', 'Backing': '#eadff0',
                         'Comprehension': '#fdf3c8', 'expired': '#f5c6c6'}.items():
                if k in name: attr['fillcolor'] = c
            if 'ledger' in name:
                attr['fillcolor'] = '#ffd9b3'; attr['peripheries'] = '2'
                attr['label'] = name + '\\n(standing surface;\\nmax/floor/lock are queries)'
            if 'line.' in name:
                L = LADDERS[name.split('.')[-1]]
                attr['fillcolor'] = '#e8f4f8'
                attr['label'] = name + '\\nrungs: ' + ' > '.join(L['rungs'])
        def trans_attr(trans, attr):
            attr['shape'] = 'box'; attr['style'] = 'filled'; attr['fillcolor'] = '#cfcfcf'
            if 'secure' in trans.name:
                k = trans.name.split('.')[-1]
                attr['fillcolor'] = '#d7d7ee'
                attr['label'] = (trans.name + '\\n[ladder: ' +
                                 ' | '.join(LADDERS[k]['channels']) + ']')
            if 'era-bump' in trans.name or 'reverify' in trans.name:
                attr['fillcolor'] = '#f5d5a0'
        def graph_attr(g, attr):
            attr['rankdir'] = 'TB'; attr['ranksep'] = '0.55'; attr['nodesep'] = '0.3'; attr['fontsize'] = '20'; attr['labelloc'] = 't'
            attr['label'] = ('Truthification cycle, v2 -- ladders as guard vocabularies '
                             '(colored Petri net, 2026-08-14)')
        for out in ('truthification_net_v2.svg', 'truthification_net_v2.png'):
            gn.draw(out, engine='dot', graph_attr=graph_attr,
                    place_attr=place_attr, trans_attr=trans_attr)
            print('drew', out)
