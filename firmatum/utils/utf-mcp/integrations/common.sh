# Shared option parsing for the per-CLI launch scripts. Sourced, not run.
#
#   <cli>.sh [-m MODEL] [-l LOG] [-t TAG] [-w WORKDIR] [-x EXTRA_ARG]... PROMPT
#
#   -m MODEL    model to request (each script has a default)
#   -l LOG      utf-mcp call log to append to (default: WORKDIR/utf-mcp-calls.jsonl)
#   -t TAG      tag copied into every log record (default: <cli>-<timestamp>)
#   -w WORKDIR  per-run directory for configs/cwd (default: a fresh mktemp dir)
#   -x ARG      extra argument passed through to the CLI (repeatable)
#
# The CLI's own machine-readable output goes to stdout; a one-line summary of
# where things were written goes to stderr.

set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
UTF_MCP="${UTF_MCP:-$HERE/../target/release/utf-mcp}"
UTF_MCP="$(cd "$(dirname "$UTF_MCP")" && pwd)/$(basename "$UTF_MCP")"
[[ -x "$UTF_MCP" ]] || { echo "utf-mcp binary not found at $UTF_MCP (run: cargo build --release)" >&2; exit 2; }

MODEL="${DEFAULT_MODEL:-}"; LOG=""; TAG=""; WORK=""; EXTRA=()
while getopts "m:l:t:w:x:" o; do
  case "$o" in
    m) MODEL="$OPTARG" ;;
    l) LOG="$OPTARG" ;;
    t) TAG="$OPTARG" ;;
    w) WORK="$OPTARG" ;;
    x) EXTRA+=("$OPTARG") ;;
    *) exit 2 ;;
  esac
done
shift $((OPTIND - 1))
PROMPT="${1:?usage: $(basename "$0") [-m MODEL] [-l LOG] [-t TAG] [-w WORKDIR] [-x ARG]... PROMPT}"
WORK="${WORK:-$(mktemp -d "${TMPDIR:-/tmp}/utf-mcp-$CLI.XXXXXX")}"
mkdir -p "$WORK"; WORK="$(cd "$WORK" && pwd)"
LOG="${LOG:-$WORK/utf-mcp-calls.jsonl}"
case "$LOG" in /*) ;; *) LOG="$PWD/$LOG" ;; esac
TAG="${TAG:-$CLI-$(date -u +%Y%m%dT%H%M%SZ)}"

# The scripts write these values into JSON, TOML and (agy) a shell command line.
# Rather than escape for three syntaxes, refuse values that would need escaping.
[[ "$TAG" =~ ^[A-Za-z0-9._:+-]{1,128}$ ]] || { echo "tag must match [A-Za-z0-9._:+-]{1,128}: $TAG" >&2; exit 2; }
for v in "$UTF_MCP" "$LOG" "$WORK"; do
  [[ "$v" =~ ^[A-Za-z0-9._/:@+~,=-]+$ ]] || { echo "path has characters these scripts do not escape (quotes, spaces, \$, \\ ...): $v" >&2; exit 2; }
done

summary() { echo "utf-mcp[$CLI]: workdir=$WORK log=$LOG tag=$TAG" >&2; }
