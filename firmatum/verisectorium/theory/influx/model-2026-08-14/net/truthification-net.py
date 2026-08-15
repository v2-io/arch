#!/usr/bin/env python3
"""
Truthification cycle -- v3, PRINCIPLED derivation (no longer a transcription of the sketch).

The steward's directive (2026-08-14): "my diagram was very very unprincipled -- that's why I
handed it over to you to make it principled, not just to translate it into petri-net notation."

What v3 derives rather than copies:

  ORIGINATE   has TWO POLES, not one (steward correction: "other pregenerators besides
              aporia"). The CORRECTIVE pole reacts to disagreement: model-observation
              MISMATCH (aporia) | present-truth COLLISION | named GAP | STALENESS (decay +
              standing audit). The GENERATIVE pole (pregenerators) mints work with no
              mismatch anywhere: SPIKE-PROPOSAL (PROPOSED queues) | BRAINSTORM (steward
              streams) | WANDERING-GOLD (first-encounter reflections from reading) |
              COUNTERFACTUAL (what-if wondering -- its own generator, not an aporia
              sub-case) | CURIOSITY. Crossing both poles: DEMAND (from application),
              DIALOG (steward contact), and DECISION-INFLECTION (a fork demanding a call,
              often before any hypothesis exists -- Backing as an originator; also the
              signal type that VALUE-ORDERS questions, EVI-style). All converge to
              questions before minting.
  IDEATE      analogy is DEMOTED from truth-line to ideation generator (its perturbation test
              is a quality gate on candidate frames, not a securing act); counterfactuals
              likewise. Minting runs the epistemic triage as a gate, and the claim DECLARES
              its applicable foundation rungs at birth.
  SECURE      the fan-out is DERIVED: one line per declared foundation rung
              (instrument-follows-rung, structural):
                derivation        <- mathematically groundable
                authored-world    <- in-vivo testable (era-keyed)
                wild-empirical    <- world-measurable
                instance-check    <- well-modeled (map onto an ESTABLISHED model; verify the
                                     correspondence; check an instance prediction)
                testimonial       <- witnessed (credibility column; position fixed)
              plus TRANSMISSION, whose ladder secures CARRIAGE (its family wraps the
              source's; contribution caps at source tier; it also feeds design early and
              can inject scope questions).  Constitutive content takes NO Evidence line --
              it routes to Backing's ratification path (the per-currency generalization).
              Every ladder is SIGNED: refutation channels are first-class, and their
              firing yields no-gos / scope-bounds as RESULTS.
  LAND        the stage the sketch skipped: results pay into the standing ledger
              (order-invariant), and the claim LANDS in the population under write
              semantics -- where collisions and cascades live, and views regenerate.
  SERVE       downstream consumers read at their own clocks, each with a RETURN edge:
              comprehension-misses -> signals; application -> mismatch; (conviction
              ground-validation and efficacy firings noted, not modeled this cut).
  MAINTAIN    era decay + carriage re-verification + standing support-audit, all
              emitting STALENESS signals into origination -- the loop is closed
              structurally, not narratively.

Register: proposed. Run `python3 truthification-net.py` to re-execute every demonstration;
`--draw` regenerates the diagram.
"""
import sys
from snakes.nets import *

# ---------------------------------------------------------------- the derived line-set
# A line = the securing instrument of a foundation rung (instrument-follows-rung).
# Each: rung ordering + SIGNED channel vocabulary (+refute) + family + ceiling law.
LINES = {
    'derivation': dict(
        rung_of='mathematically-groundable',
        family='derivation',
        rungs=['sketch', 'drafted', 'deps-verified', 're-derived'],
        channels=['formal-expression-written', 'premises-named-and-checked',
                  'independent-re-derivation'],
        refute='counterexample-found -> no-go (a result, on the critical path)',
        ceiling='exact under named premises; conditional if any premise is empirical'),
    'authored-world': dict(
        rung_of='in-vivo-testable',
        family='in-vivo-measurement',
        rungs=['pre-registered', 'run-once', 'swept', 'independently-reproduced'],
        channels=['first-run (era-keyed)', 'seed/level/footprint-sweep',
                  'independent-re-run'],
        refute='prediction-miss (pre-registered) -> recorded miss, prevents flattering read',
        ceiling='exact for the authored world; every number era-keyed and decay-liable'),
    'wild-empirical': dict(
        rung_of='world-measurable',
        family='wild-measurement',
        rungs=['anecdotal', 'measured-once', 'population-robust', 'calibrated'],
        channels=['first-measurement', 'population-sweep', 'calibration-events'],
        refute='signed GRADE-shaped down-channels: bias-found / inconsistency / imprecision',
        ceiling='empirical; instrument honesty binds (empty != failed != unrun)'),
    'instance-check': dict(
        rung_of='well-modeled (instance of an ESTABLISHED model)',
        family='model-instance',
        rungs=['candidate-mapping', 'correspondence-verified', 'instance-prediction-checked'],
        channels=['map-claim-onto-model', 'verify-correspondence-by-perturbation'],
        refute='mapping breaks under perturbation -> SCOPE-BOUND (strengthen-first: the '
               'break is a result that narrows scope, not a failure)',
        ceiling='inherits the established model own tier, on the verified overlap only'),
    'testimonial': dict(
        rung_of='witnessed',
        family='testimony',
        rungs=['captured', 'corroborated', 'cross-kind-corroborated'],   # credibility column
        channels=['independent-account-located', 'independent-kind-agrees'],
        refute='account impeached (infidelity by omission/fabrication/misrepresentation)',
        ceiling='position fixed at capture; corroboration moves credibility only'),
    'transmission': dict(
        rung_of='(carrier, not a rung) -- carriage over an external source',
        family='transmission:wild-measurement',   # wraps the SOURCE family; lock looks through
        rungs=['named-not-read', 'recalled', 'search-corroborated',
               'verified-via-secondary', 'primary-verified'],
        channels=['recall-marked-as-recall', 'convergent-secondaries',
                  'faithful-secondary-fetched', 'primary-fetched'],
        refute='carriage infidelity found; or the source itself revised',
        ceiling='min(carriage rung, source own tier); never lock-independent of source family'),
}
SRC_TIER = 2   # external source's own tier this run

# The demo claim declares which rungs apply (the DERIVED fan-out):
CLAIM_RUNGS = frozenset(['derivation', 'authored-world', 'instance-check',
                         'testimonial', 'transmission'])
# wild-empirical deliberately NOT declared: the fill question is only measurable in the
# authored world -- so that line never launches. Fan-out follows the declaration.

def build():
    n = PetriNet('truthification-cycle-v3')
    for p, init in [
        # ORIGINATE -- typed mismatch signals, converging to questions
        ('signals.typed', [('mismatch', 'water-fill-residual-grows'),
                           ('decision-inflection', 'choose-marching-vs-direct-solve'),
                           ('spike-proposal', 'try-graphflood-style-priming')]),
        ('questions.Disposition', []),
        # IDEATE -- generators + the minting gate
        ('frames.candidate', []),          # analogy/counterfactual output, perturbation-gated
        ('claims.Evidence', []),           # ('claim', slug, declared-rungs)
        # SECURE -- one place per derived line
        *[('line.' + k, []) for k in LINES],
        ('results', []),
        ('accounts.Fidelity', []),
        # LAND
        ('ledger.standing-surface', []),
        ('population.segments', [('segment', 'diffusion-wave-model', 'established')]),
        ('expired.certificates', []),
        # SERVE
        ('decisions.Backing', []),
        ('pedagogy.Comprehension', []),
    ]:
        n.add_place(Place(p, init))

    # ---- ORIGINATE: any typed signal poses a question (uniform entry) ----
    n.add_transition(Transition('T.pose-question', Expression("True")))
    n.add_input('signals.typed', 'T.pose-question', Variable('s'))
    n.add_output('questions.Disposition', 'T.pose-question',
                 Expression("('q', s[1], 'from:' + s[0])"))

    # ---- IDEATE: generators (analogy demoted to here; perturbation gate embedded) ----
    n.add_transition(Transition('T.analogize', Expression("True")))
    n.add_input('population.segments', 'T.analogize', Test(Variable('m')))
    n.add_output('frames.candidate', 'T.analogize',
                 Expression("('frame', 'analogy:' + m[1], 'perturbation-tested')"))
    n.add_output('accounts.Fidelity', 'T.analogize',
                 Expression("('acct', 'frame-generated', m[1], 'survived-perturbation-gate')"))
    n.add_transition(Transition('T.counterfactualize', Expression("True")))
    n.add_input('questions.Disposition', 'T.counterfactualize', Test(Variable('q')))
    n.add_output('frames.candidate', 'T.counterfactualize',
                 Expression("('frame', 'what-if:' + q[1], 'perturbation-tested')"))
    n.add_output('accounts.Fidelity', 'T.counterfactualize',
                 Expression("('acct', 'frame-generated', q[1], 'counterfactual')"))
    # minting gate: the epistemic triage; the claim DECLARES its applicable rungs
    n.add_transition(Transition('T.mint', Expression("f[2] == 'perturbation-tested'")))
    n.add_input('questions.Disposition', 'T.mint', Test(Variable('q')))
    n.add_input('frames.candidate', 'T.mint', Test(Variable('f')))
    n.add_output('claims.Evidence', 'T.mint',
                 Expression("('claim', q[1], %r)" % CLAIM_RUNGS))
    n.add_output('ledger.standing-surface', 'T.mint',
                 Expression("('ledger', q[1], frozenset())"))
    n.add_output('accounts.Fidelity', 'T.mint',
                 Expression("('acct', 'minted', q[1], 'triage-run; rungs-declared')"))

    # ---- SECURE: launch is GUARDED BY THE DECLARATION (derived fan-out) ----
    for kind in LINES:
        tn = 'T.launch.' + kind
        n.add_transition(Transition(tn, Expression("%r in cl[2]" % kind)))
        n.add_input('claims.Evidence', tn, Test(Variable('cl')))
        extra = "'k1'" if kind == 'authored-world' else (
                "'reconstructed'" if kind == 'testimonial' else "''")
        n.add_output('line.' + kind, tn,
                     Expression("('line', %r, cl[1], 0, %s)" % (kind, extra)))
    # ladders: guarded self-loops; channel named per rung-step; every firing an event
    for kind, L in LINES.items():
        tn = 'T.secure*.' + kind
        n.add_transition(Transition(tn, Expression("x[3] < %d" % (len(L['rungs']) - 1))))
        n.add_input('line.' + kind, tn, Variable('x'))
        n.add_output('line.' + kind, tn, Expression("(x[0], x[1], x[2], x[3]+1, x[4])"))
        n.add_output('accounts.Fidelity', tn,
                     Expression("('acct', 'secured', %r, %r[x[3]])" % (kind, L['channels'])))
    # SIGNED: refutation channels are first-class; here, the instance-check mapping
    # breaks under perturbation at the lake case -> a SCOPE-BOUND result + a question
    n.add_transition(Transition('T.refute.instance-check', Expression("x[3] >= 1")))
    n.add_input('line.instance-check', 'T.refute.instance-check', Variable('x'))
    n.add_output('results', 'T.refute.instance-check',
                 Expression("('res', 'instance-check', x[2], 2, 'model-instance', "
                            "'SCOPE-BOUND: mapping fails near-horizontal (lake case)')"))
    n.add_output('questions.Disposition', 'T.refute.instance-check',
                 Expression("('q', 'rescope-' + x[2] + '-away-from-lakes', 'from:refutation')"))
    # conclusions (ascent path), typed per line
    n.add_transition(Transition('T.conclude.derivation', Expression("x[3] >= 3")))
    n.add_input('line.derivation', 'T.conclude.derivation', Variable('x'))
    n.add_output('results', 'T.conclude.derivation',
                 Expression("('res','derivation', x[2], 3, 'derivation', 'exact|premises-named')"))
    n.add_output('questions.Disposition', 'T.conclude.derivation',
                 Expression("('q', 'what-does-the-bound-mean-for-' + x[2], 'from:no-go')"))
    n.add_transition(Transition('T.conclude.authored-world', Expression("x[3] >= 2")))
    n.add_input('line.authored-world', 'T.conclude.authored-world', Variable('x'))
    n.add_output('results', 'T.conclude.authored-world',
                 Expression("('res','authored-world', x[2], (2 if x[3]==2 else 3), "
                            "'in-vivo-measurement', x[4])"))
    n.add_transition(Transition('T.conclude.transmission', Expression("x[3] >= 2")))
    n.add_input('line.transmission', 'T.conclude.transmission', Variable('x'))
    n.add_output('results', 'T.conclude.transmission',
                 Expression("('res','transmission', x[2], min(%d, 1 if x[3] < 3 else 2), "
                            "'transmission:wild-measurement', 'carriage-rung='+str(x[3]))" % SRC_TIER))
    n.add_transition(Transition('T.conclude.testimonial', Expression("x[3] >= 1")))
    n.add_input('line.testimonial', 'T.conclude.testimonial', Variable('x'))
    n.add_output('results', 'T.conclude.testimonial',
                 Expression("('res','testimonial', x[2], x[3], 'testimony', "
                            "'position='+x[4]+' (fixed); credibility='+str(x[3]))"))
    # transmission's OTHER roles: early design feed + independent scope influence
    n.add_transition(Transition('T.prior-art-informs', Expression("x[3] >= 1")))
    n.add_input('line.transmission', 'T.prior-art-informs', Test(Variable('x')))
    n.add_output('frames.candidate', 'T.prior-art-informs',
                 Expression("('frame', 'prior-art:' + x[2], 'perturbation-tested')"))
    n.add_transition(Transition('T.scope-influence', Expression("x[3] >= 2")))
    n.add_input('line.transmission', 'T.scope-influence', Test(Variable('x')))
    n.add_output('questions.Disposition', 'T.scope-influence',
                 Expression("('q', 'scope-check:' + x[2], 'from:external-bound')"))

    # ---- LAND: order-invariant pay-in, then landing into the population ----
    n.add_transition(Transition('T.pay-in', Expression("r[2] == l[1]")))
    n.add_input('results', 'T.pay-in', Variable('r'))
    n.add_input('ledger.standing-surface', 'T.pay-in', Variable('l'))
    n.add_output('ledger.standing-surface', 'T.pay-in',
                 Expression("('ledger', l[1], l[2] | frozenset([(r[1], r[3], r[4], r[5])]))"))
    n.add_transition(Transition('T.land', Expression("len(l[2]) > 0")))
    n.add_input('ledger.standing-surface', 'T.land', Test(Variable('l')))
    n.add_output('population.segments', 'T.land',
                 Expression("('segment', l[1], 'landed:replacement-semantics')"))
    n.add_output('accounts.Fidelity', 'T.land',
                 Expression("('acct', 'landed', l[1], "
                            "'delete-test + collision-surface now live; views regenerate')"))

    # ---- MAINTAIN: decay, re-verification, standing audit -> staleness signals ----
    n.add_transition(Transition('T.era-bump.k1-to-k2',
        Expression("any(e[0]=='authored-world' and e[3]=='k1' for e in l[2])")))
    n.add_input('ledger.standing-surface', 'T.era-bump.k1-to-k2', Variable('l'))
    n.add_output('ledger.standing-surface', 'T.era-bump.k1-to-k2',
                 Expression("('ledger', l[1], frozenset(e for e in l[2] "
                            "if not (e[0]=='authored-world' and e[3]=='k1')))"))
    n.add_output('expired.certificates', 'T.era-bump.k1-to-k2',
                 Expression("('expired', l[1], 'authored-world@k1', 'era-boundary')"))
    n.add_output('signals.typed', 'T.era-bump.k1-to-k2',
                 Expression("('staleness', 're-run-' + l[1] + '-under-k2')"))
    n.add_transition(Transition('T.reverify.carriage',
        Expression("any(e[0]=='transmission' and e[1] < %d for e in l[2])" % SRC_TIER)))
    n.add_input('ledger.standing-surface', 'T.reverify.carriage', Variable('l'))
    n.add_output('ledger.standing-surface', 'T.reverify.carriage',
                 Expression("('ledger', l[1], frozenset(e for e in l[2] if e[0]!='transmission') "
                            "| frozenset([('transmission', %d, 'transmission:wild-measurement', "
                            "'primary-fetched; capped-at-source-tier')]))" % SRC_TIER))
    n.add_transition(Transition('T.support-audit', Expression("s[2] != 'established'")))
    n.add_input('population.segments', 'T.support-audit', Test(Variable('s')))
    n.add_output('signals.typed', 'T.support-audit',
                 Expression("('staleness', 'support-audit:' + s[1] + ':deps-recheck')"))

    # ---- SERVE: downstream at own clocks, each with a return edge ----
    n.add_transition(Transition('T.decide', Expression("len(l[2]) > 0")))
    n.add_input('ledger.standing-surface', 'T.decide', Test(Variable('l')))
    n.add_output('decisions.Backing', 'T.decide', Expression("('decision', l[1], 'proposed')"))
    # Backing as originator: a fork short on grounds emits a decision-inflection signal
    n.add_transition(Transition('T.decision-inflection', Expression("d[2] == 'proposed'")))
    n.add_input('decisions.Backing', 'T.decision-inflection', Test(Variable('d')))
    n.add_output('signals.typed', 'T.decision-inflection',
                 Expression("('decision-inflection', 'grounds-needed-for:' + d[1])"))
    n.add_transition(Transition('T.teach', Expression("s[2] != 'established'")))
    n.add_input('population.segments', 'T.teach', Test(Variable('s')))
    n.add_output('pedagogy.Comprehension', 'T.teach', Expression("('primer', s[1])"))
    n.add_transition(Transition('T.wandering-gold', Expression("True")))
    n.add_input('pedagogy.Comprehension', 'T.wandering-gold', Test(Variable('p')))
    n.add_output('signals.typed', 'T.wandering-gold',
                 Expression("('wandering-gold', 'first-encounter-reflection:' + p[1])"))
    n.add_transition(Transition('T.comprehension-miss', Expression("True")))
    n.add_input('pedagogy.Comprehension', 'T.comprehension-miss', Test(Variable('p')))
    n.add_output('signals.typed', 'T.comprehension-miss',
                 Expression("('comprehension-miss', 'reader-confusion:' + p[1])"))
    n.add_transition(Transition('T.apply', Expression("True")))
    n.add_input('decisions.Backing', 'T.apply', Test(Variable('d')))
    n.add_output('signals.typed', 'T.apply',
                 Expression("('mismatch', 'application-feedback:' + d[1])"))
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

CLIMB = {'derivation': 3, 'authored-world': 2, 'transmission': 2,
         'testimonial': 1, 'instance-check': 1}

def run(payin_order):
    global EVENTS
    EVENTS = []
    n = build()
    fire(n, 'T.pose-question', pick=lambda ms: next(m for m in ms if m['s'][0]=='mismatch'))
    fire(n, 'T.pose-question')   # the decision-inflection poses its question too
    fire(n, 'T.analogize'); fire(n, 'T.counterfactualize')
    fire(n, 'T.mint', pick=lambda ms: next(m for m in ms if 'water-fill' in str(m['q'])))
    launched = [k for k in LINES if fire(n, 'T.launch.' + k)]
    for k, steps in CLIMB.items():
        if k in launched:
            for _ in range(steps):
                fire(n, 'T.secure*.' + k)
    fire(n, 'T.prior-art-informs'); fire(n, 'T.scope-influence')
    for k in ('derivation', 'authored-world', 'transmission', 'testimonial'):
        if k in launched:
            fire(n, 'T.conclude.' + k)
    fire(n, 'T.refute.instance-check')          # the signed channel firing
    for kind in payin_order:
        fire(n, 'T.pay-in', pick=lambda ms, kind=kind: next(m for m in ms if m['r'][1] == kind))
    fire(n, 'T.land')
    fire(n, 'T.decide'); fire(n, 'T.teach'); fire(n, 'T.apply')
    fire(n, 'T.comprehension-miss'); fire(n, 'T.wandering-gold'); fire(n, 'T.decision-inflection')
    return n, launched

def ledger_of(n):
    return list(n.place('ledger.standing-surface').tokens)[0]

def projections(ledger):
    entries = sorted(ledger[2])
    strengths = {e[0]: e[1] for e in entries}
    mx = max(strengths.values()) if strengths else 0
    floor = min(strengths.values()) if strengths else 0
    base = lambda fam: fam.split(':')[-1]
    strong = [e for e in entries if e[1] >= 2]
    lock = 'ARMED' if len({base(e[2]) for e in strong}) >= 2 else 'unarmed'
    return entries, mx, floor, lock

def show(tag, n):
    entries, mx, floor, lock = projections(ledger_of(n))
    print("  %-46s max=%d floor=%d lock=%s" % (tag, mx, floor, lock))
    for e in entries:
        print("      (%s, strength=%d, family=%s, %s)" % e)

if __name__ == '__main__':
    print("THE DERIVED LINE-SET (one line per foundation rung; instrument-follows-rung):")
    for k, L in LINES.items():
        print("  %-16s <- %-46s family=%s" % (k, L['rung_of'], L['family']))
        print("  %-16s    rungs:    %s" % ('', ' -> '.join(L['rungs'])))
        print("  %-16s    channels: %s" % ('', ' | '.join(L['channels'])))
        print("  %-16s    refute:   %s" % ('', L['refute']))
        print("  %-16s    ceiling:  %s" % ('', L['ceiling']))
    print()
    order1 = ['derivation', 'authored-world', 'transmission', 'testimonial', 'instance-check']
    n1, launched = run(order1)
    ev1 = list(EVENTS)
    n2, _ = run(list(reversed(order1)))
    same = (projections(ledger_of(n1)) == projections(ledger_of(n2))
            and ledger_of(n1) == ledger_of(n2))
    print("DERIVED FAN-OUT: claim declared rungs -> launched %s" % launched)
    print("  (wild-empirical declared inapplicable -> its line correctly never launched)")
    print()
    print("KAHN OBLIGATION over reversed pay-in order:", "HOLDS" if same else "VIOLATED")
    print()
    print("Act I -- ascent + one SIGNED refutation (instance mapping breaks at the lake case):")
    show("scope-bound lands as a RESULT, question minted", n1)
    print("      refutation-minted question:",
          [t for t in n1.place('questions.Disposition').tokens if 'rescope' in str(t)])
    print()
    print("Act II -- era decay -> staleness SIGNAL (loop closure through origination):")
    fire(n1, 'T.era-bump.k1-to-k2')
    show("authored-world@k1 expired", n1)
    print("      signals now queued:", sorted(n1.place('signals.typed').tokens, key=str))
    print()
    print("Act III -- carriage re-verified (wall moved); lock via look-through:")
    fire(n1, 'T.reverify.carriage')
    show("transmission raised to source-tier cap", n1)
    print()
    print("Return edges observed this run (the loop is structural):")
    for t in sorted(n1.place('signals.typed').tokens, key=str):
        print("   signal:", t)
    print()
    print("EVENT LOG head (run 1, first 10 of %d):" % len(ev1))
    for e in ev1[:10]:
        print("  %3d  %-24s %s" % e)

    # ---------------------------------------------------------------- drawing
    if '--draw' in sys.argv:
        import subprocess
        CL = {
            'ORIGINATE (two poles: corrective + generative pregenerators)': ['signals.typed', 'T.pose-question',
                                              'questions.Disposition'],
            'IDEATE (generators + minting gate)': ['T.analogize', 'T.counterfactualize', 'frames.candidate',
                                                   'T.mint', 'claims.Evidence'],
            'SECURE (lines derived from declared rungs; signed ladders)':
                sum([['T.launch.'+k, 'line.'+k, 'T.secure*.'+k] for k in LINES], [])
                + ['T.refute.instance-check', 'T.prior-art-informs', 'T.scope-influence'],
            'RESULTS': ['T.conclude.derivation','T.conclude.authored-world',
                        'T.conclude.transmission','T.conclude.testimonial','results'],
            'LAND (standing surface + population)': ['T.pay-in','ledger.standing-surface',
                        'T.land','population.segments'],
            'MAINTAIN (decay / re-verify / standing audit)': ['T.era-bump.k1-to-k2',
                        'expired.certificates','T.reverify.carriage','T.support-audit'],
            'SERVE (own clocks; return edges)': ['T.decide','decisions.Backing','T.decision-inflection','T.teach',
                        'T.wandering-gold',
                        'pedagogy.Comprehension','T.comprehension-miss','T.apply'],
        }
        FILL = {'Fidelity':'#f2e6d0','Disposition':'#dbe7f5','Evidence':'#dfeedd',
                'Backing':'#eadff0','Comprehension':'#fdf3c8','expired':'#f5c6c6',
                'signals':'#f9e0e0','population':'#d9ead9'}
        def esc(s): return s.replace('"','\\"')
        def pnode(name):
            fill = '#eeeeee'; label = name
            for k,c in FILL.items():
                if k in name: fill = c
            per = ''
            if name.startswith('line.'):
                k = name.split('.',1)[1]; L = LINES[k]; fill = '#e8f4f8'
                label = f"{name}\\nfamily: {L['family']}\\n" + " > ".join(L['rungs'])
            if 'ledger' in name:
                fill = '#ffd9b3'; per = ' peripheries=2'
                label = name + "\\n(max / floor / lock are queries;\\npay-in is order-invariant)"
            if 'population' in name:
                per = ' peripheries=2'
                label = name + "\\n(write semantics; collision surface;\\nviews regenerate)"
            return f'"{name}" [shape=ellipse style=filled fillcolor="{fill}"{per} label="{esc(label)}"];'
        def tnode(name):
            fill = '#cfcfcf'; label = name
            if 'secure' in name:
                k = name.split('.',1)[1].replace('secure*.','')
                k = name.split('T.secure*.')[-1]; fill = '#d7d7ee'
                label = name + "\\n" + "\\n".join(LINES[k]['channels'])
            if 'refute' in name:
                fill = '#f0c8c8'; label = name + "\\n[SIGNED channel]"
            if 'era-bump' in name or 'reverify' in name or 'support-audit' in name:
                fill = '#f5d5a0'
            if name in ('T.prior-art-informs','T.scope-influence','T.analogize','T.counterfactualize'):
                fill = '#e9dcc9'
            if name == 'T.mint':
                label = name + "\\n[epistemic triage;\\nrungs declared here]"
            return f'"{name}" [shape=box style=filled fillcolor="{fill}" label="{esc(label)}"];'
        n = build()
        lines_out = ['digraph v3 {',
          'rankdir=LR; ranksep=0.65; nodesep=0.32; fontsize=22; labelloc=t;',
          'label="Truthification cycle v3 -- principled: mismatch-typed origination, analogy demoted to ideation, '
          'fan-out derived from declared foundation rungs, signed ladders, landing stage, structural return edges";',
          'node [fontname="Helvetica" fontsize=11]; edge [fontname="Helvetica" fontsize=9 color="#555555"];']
        placed = set(); ci = 0
        for cname, members in CL.items():
            lines_out.append(f'subgraph cluster_{ci} {{ label="{cname}"; style=rounded; color="#999999"; fontsize=14;')
            for mname in members:
                try: node = n.node(mname)
                except Exception: continue
                placed.add(mname)
                lines_out.append(pnode(mname) if isinstance(node, Place) else tnode(mname))
            lines_out.append('}'); ci += 1
        for node in n.node():
            if node.name not in placed:
                lines_out.append(pnode(node.name) if isinstance(node, Place) else tnode(node.name))
        for t in n.transition():
            for place, label in t.input():
                dashed = 'Test' in type(label).__name__
                lines_out.append(f'"{place.name}" -> "{t.name}" [style={"dashed" if dashed else "solid"}'
                                 + (', label="?"' if dashed else '') + '];')
            for place, label in t.output():
                lines_out.append(f'"{t.name}" -> "{place.name}";')
        lines_out.append('}')
        open('truthification-net.dot','w').write('\n'.join(lines_out))
        for fmt in ('svg','png'):
            subprocess.run(['dot','-T'+fmt,'truthification-net.dot',
                            '-o','truthification-net.'+fmt], check=True)
        print('\ndrew truthification-net.svg / .png / .dot')
