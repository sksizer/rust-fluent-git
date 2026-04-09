#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WRAP_CLI="${SCRIPT_DIR}/wrap_cli.sh"

# --- Colors -----------------------------------------------------------------
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

info()    { echo -e "${CYAN}[info]${NC} $*"; }
success() { echo -e "${GREEN}[ok]${NC} $*"; }
fail()    { echo -e "${RED}[fail]${NC} $*"; }

# --- Usage ------------------------------------------------------------------
usage() {
    cat <<EOF
Usage: $(basename "$0") [options] [tool ...] [-- wrap_cli options]

Wrap multiple CLI tools in sequence. Tools can be provided as arguments,
from a file, or both.

Arguments:
  tool ...          One or more CLI tool names (e.g., docker terraform kubectl)

Options:
  -f, --file FILE   Read tool names from FILE (one per line, # comments ok)
  --dry-run         Pass --dry-run to each wrap_cli.sh invocation
  -h, --help        Show this help

Everything after -- is forwarded to wrap_cli.sh (e.g., --skip-to, --dry-run).

File format (tools.txt):
  # Package managers
  docker
  terraform
  # kubectl  ← commented out, skipped

Examples:
  $(basename "$0") docker terraform kubectl
  $(basename "$0") -f tools.txt
  $(basename "$0") -f tools.txt -- --skip-to implement
  $(basename "$0") docker terraform -- --dry-run
EOF
    exit 0
}

# --- Parse arguments --------------------------------------------------------
TOOLS=()
TOOL_FILE=""
FORWARD_ARGS=()
PARSING_TOOLS=true

for arg in "$@"; do
    if [[ "$arg" == "--" ]]; then
        PARSING_TOOLS=false
        continue
    fi

    if [[ "$PARSING_TOOLS" == true ]]; then
        case "$arg" in
            -f|--file)
                # Next arg is the file — handled below via shift trick
                TOOL_FILE="__next__"
                ;;
            -h|--help)
                usage
                ;;
            --dry-run)
                # Shortcut: also forward to wrap_cli
                FORWARD_ARGS+=("--dry-run")
                ;;
            *)
                if [[ "$TOOL_FILE" == "__next__" ]]; then
                    TOOL_FILE="$arg"
                else
                    TOOLS+=("$arg")
                fi
                ;;
        esac
    else
        FORWARD_ARGS+=("$arg")
    fi
done

# Read tools from file if provided
if [[ -n "$TOOL_FILE" && "$TOOL_FILE" != "__next__" ]]; then
    if [[ ! -f "$TOOL_FILE" ]]; then
        fail "Tool file not found: ${TOOL_FILE}"
        exit 1
    fi
    while IFS= read -r line; do
        # Strip comments and whitespace
        line="${line%%#*}"
        line="$(echo "$line" | xargs)"
        [[ -z "$line" ]] && continue
        TOOLS+=("$line")
    done < "$TOOL_FILE"
fi

if [[ ${#TOOLS[@]} -eq 0 ]]; then
    echo "Error: no tools specified. Provide tool names or use -f FILE." >&2
    echo "Run with --help for usage." >&2
    exit 1
fi

# --- Run each tool ----------------------------------------------------------
TOTAL=${#TOOLS[@]}
SUCCEEDED=0
FAILED_TOOLS=()

info "Wrapping ${TOTAL} tool(s): ${TOOLS[*]}"
if [[ ${#FORWARD_ARGS[@]} -gt 0 ]]; then
    info "Forwarding args: ${FORWARD_ARGS[*]}"
fi
echo ""

for i in "${!TOOLS[@]}"; do
    tool="${TOOLS[$i]}"
    n=$((i + 1))

    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    info "[${n}/${TOTAL}] Wrapping: ${tool}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    if bash "$WRAP_CLI" "$tool" "${FORWARD_ARGS[@]+"${FORWARD_ARGS[@]}"}"; then
        success "[${n}/${TOTAL}] ${tool} complete"
        SUCCEEDED=$((SUCCEEDED + 1))
    else
        fail "[${n}/${TOTAL}] ${tool} failed"
        FAILED_TOOLS+=("$tool")
    fi
    echo ""
done

# --- Summary ----------------------------------------------------------------
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
success "Batch complete: ${SUCCEEDED}/${TOTAL} succeeded"

if [[ ${#FAILED_TOOLS[@]} -gt 0 ]]; then
    fail "Failed: ${FAILED_TOOLS[*]}"
    echo ""
    info "Re-run failed tools with:"
    info "  $(basename "$0") ${FAILED_TOOLS[*]} ${FORWARD_ARGS[*]+"${FORWARD_ARGS[*]}"}"
    exit 1
fi
