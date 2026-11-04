#!/bin/bash
# Hash-identity experiment (report §1.2). Builds a scratch store from a verbatim copy of ASF
# outside every repo, then runs each variant, restoring the copy in between.
# Usage: xstore.sh <empty scratch dir>. Needs vsect on PATH. Read-only on ASF.
set -e
ASF_ROOT=${ASF_ROOT:-$HOME/src/arch/asf}; T="$1"; HERE=$(cd "$(dirname "$0")" && pwd)
[ -n "$T" ] && [ ! -e "$T" ] || { echo "give a path that does not exist yet"; exit 2; }
mkdir -p "$T/src" "$T/.vsect"
cd "$ASF_ROOT"; cp 0*/src/*.md "$T/src/"; rm -f "$T"/src/old-*.md
cp -R .vsect/events .vsect/objects "$T/.vsect/"; cp -R reviews questions items "$T/"
python3 -I - "$ASF_ROOT/.vsect/kinds.yaml" "$T/.vsect/kinds.yaml" <<'PY'
import re,sys
k=open(sys.argv[1]).read().replace("name: asf","name: xtest")
k=re.sub(r'    find:\n      - "01-aat-core/src/\*.md"\n      - "02-tst-core/src/\*.md"\n      - "03-llm-core/src/\*.md"\n      - "04-eli-core/src/\*.md"\n    exclude:\n      - "0\*/src/old-\*.md"','    find:\n      - "src/*.md"',k)
k=re.sub(r'(?m)^planned:\n(  .*\n)+','',k); k=re.sub(r'(?m)^owners_from:.*\n','',k)
open(sys.argv[2],"w").write(k)
PY
cd "$T"; git init -q; git add -A; git -c user.email=x@x -c user.name=x commit -qm verbatim
H="$HERE/holdcount.sh"
echo "== verbatim copy, renamed store, flat src/"; $H strict; $H all
python3 -I - <<'PY'
import glob,re
for p in glob.glob("src/*.md"):
    t=open(p).read(); _,fm,body=t.split("---",2)
    body=re.split(r'(?m)^## Working Notes',body)[0]
    fm=re.sub(r'(?m)^(status|stage):.*\n','',fm)+"predecessor: asf@2133cf2e\n"
    open(p,"w").write("---"+fm+"---"+body)
PY
echo "== + Working Notes stripped, status/stage dropped, predecessor added"; $H strict; $H all
git -c user.email=x@x -c user.name=x commit -qam v1
python3 -I -c "
p='src/def-agent-environment.md'; t=open(p).read(); open(p,'w').write(t.replace('slug: def-agent-environment','form: definition\nslug: def-agent-environment',1))"
echo "== + form: definition on def-agent-environment"; $H strict; $H all; git checkout -q src/def-agent-environment.md
python3 -I -c "
p='src/def-agent-environment.md'; t=open(p).read(); open(p,'w').write(t.replace('The framework begins by drawing','*[Definition (coupling)]* The framework begins by drawing',1))"
echo "== + one carving tag in def-agent-environment"; $H strict; $H all; git checkout -q src/def-agent-environment.md
mkdir -p ../xstore-held; for f in src/*.md; do case $(basename $f .md) in def-agent-environment|def-action-transition|def-observation-function|def-chronica|scope-adaptive-system|scope-agency|post-causal-structure) ;; *) mv $f ../xstore-held/;; esac; done
echo "== only the seven Coupled Loop segments present"; $H strict; $H all
mv ../xstore-held/*.md src/; rmdir ../xstore-held
