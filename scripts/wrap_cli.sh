#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROMPT_DIR="${SCRIPT_DIR}/wrap_cli"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

# --- Colors & helpers -------------------------------------------------------
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

info()    { echo -e "${CYAN}[info]${NC} $*"; }
success() { echo -e "${GREEN}[ok]${NC} $*"; }
warn()    { echo -e "${YELLOW}[warn]${NC} $*"; }
fail()    { echo -e "${RED}[fail]${NC} $*"; }

# --- Locate Claude Code runner ---------------------------------------------
find_runner() {
    if command -v claude &>/dev/null; then
        echo "claude"
    elif command -v pnpm &>/dev/null; then
        echo "pnpm dlx @anthropic-ai/claude-code"
    elif command -v npx &>/dev/null; then
        echo "npx @anthropic-ai/claude-code"
    else
        echo "Error: neither claude, pnpm, nor npx found." >&2
        exit 1
    fi
}

RUNNER="$(find_runner)"

# --- Parse arguments --------------------------------------------------------
TOOL=""
DRY_RUN=false
SKIP_TO=""

usage() {
    cat <<EOF
Usage: $(basename "$0") <tool-name> [options]

Create a fluent-{name} wrapper crate for a CLI tool.

Arguments:
  <tool-name>       The CLI tool to wrap (e.g., docker, terraform, kubectl)

Options:
  --dry-run         Print prompts without executing
  --skip-to PHASE   Skip to a specific phase (research|prd|scaffold|implement|assemble|fix)
  -h, --help        Show this help

Phases:
  1. research    — Explore the CLI and document its surface
  2. prd         — Write the PRD spec
  3. scaffold    — Create crate skeleton + workspace wiring
  4. implement   — Implement each operation group (parallel)
  5. assemble    — Write mod.rs files and entry point
  6. fix         — Build loop: just full-write && just full-check

Examples:
  $(basename "$0") docker
  $(basename "$0") terraform --skip-to implement
  $(basename "$0") kubectl --dry-run
EOF
    exit 0
}

for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=true ;;
        --skip-to) SKIP_TO="__next__" ;;
        -h|--help) usage ;;
        *)
            if [[ "$SKIP_TO" == "__next__" ]]; then
                SKIP_TO="$arg"
            elif [[ -z "$TOOL" ]]; then
                TOOL="$arg"
            fi
            ;;
    esac
done

if [[ -z "$TOOL" ]]; then
    echo "Error: tool name required. Run with --help for usage." >&2
    exit 1
fi

# Derive names
NAME="${TOOL}"  # crate name suffix (e.g., "brew" for fluent-brew)
# PascalCase for the entry point type (docker → Docker, cloud_sql → CloudSql)
ENTRY_TYPE="$(echo "${NAME}" | python3 -c "import sys; print(''.join(w.capitalize() for w in sys.stdin.read().strip().split('_')))")"

info "Tool: ${TOOL}"
info "Crate: fluent-${NAME}"
info "Entry type: ${ENTRY_TYPE}"
info "Runner: ${RUNNER}"
echo ""

# --- Template expansion -----------------------------------------------------
expand_template() {
    local file="$1"
    sed \
        -e "s/{{TOOL}}/${TOOL}/g" \
        -e "s/{{NAME}}/${NAME}/g" \
        -e "s/{{ENTRY_TYPE}}/${ENTRY_TYPE}/g" \
        "$file"
}

# Compose a prompt from role.md + a specific phase file
compose_prompt() {
    local phase_file="$1"
    local prompt=""
    prompt+="$(expand_template "${PROMPT_DIR}/role.md")"
    prompt+=$'\n\n'
    prompt+="$(expand_template "${phase_file}")"
    echo "$prompt"
}

# --- Run Claude Code --------------------------------------------------------
TOOLS_DEFAULT="Read Edit Write Bash Glob Grep"
TOOLS_WITH_WEB="Read Edit Write Bash Glob Grep WebFetch WebSearch"

run_claude() {
    local prompt="$1"
    local label="$2"
    local tools="${3:-$TOOLS_DEFAULT}"

    if [[ "$DRY_RUN" == true ]]; then
        echo "=== DRY RUN: ${label} ==="
        echo "${prompt}"
        echo "--- (tools: ${tools})"
        return 0
    fi

    info "Running: ${label}"
    cd "$PROJECT_DIR"
    echo "${prompt}" | ${RUNNER} --print --allowedTools "${tools}" 2>&1
}

# --- Phase gate: should we run this phase? ----------------------------------
should_run() {
    local phase="$1"
    if [[ -z "$SKIP_TO" ]]; then
        return 0  # no skip, run everything
    fi
    if [[ "$SKIP_TO" == "$phase" ]]; then
        SKIP_TO=""  # found the target, run from here
        return 0
    fi
    info "Skipping phase: ${phase}"
    return 1
}

# --- Phase 1: Research ------------------------------------------------------
if should_run "research"; then
    info "Phase 1: Research the CLI"
    if ! command -v "${TOOL}" &>/dev/null; then
        fail "${TOOL} is not installed. Install it first."
        exit 1
    fi

    PROMPT="$(compose_prompt "${PROMPT_DIR}/01_research.md")"
    run_claude "$PROMPT" "CLI Research" "$TOOLS_WITH_WEB"

    if [[ "$DRY_RUN" != true ]]; then
        if [[ -f "docs/prds/fluent-${NAME}.research.md" ]]; then
            success "Research output: docs/prds/fluent-${NAME}.research.md"
        else
            fail "Research file not created. Aborting."
            exit 1
        fi
    fi
    echo ""
fi

# --- Phase 2: PRD ----------------------------------------------------------
if should_run "prd"; then
    info "Phase 2: Write the PRD"
    PROMPT="$(compose_prompt "${PROMPT_DIR}/02_prd.md")"
    run_claude "$PROMPT" "PRD Generation"

    if [[ "$DRY_RUN" != true ]]; then
        if [[ -f "docs/prds/fluent-${NAME}.md" ]]; then
            success "PRD: docs/prds/fluent-${NAME}.md"
        else
            fail "PRD not created. Aborting."
            exit 1
        fi

        # Approval gate
        echo ""
        warn "Review the PRD before continuing."
        read -rp "Continue with implementation? [y/N] " answer
        if [[ "$answer" != "y" && "$answer" != "Y" ]]; then
            info "Stopping. Edit the PRD and re-run with: --skip-to scaffold"
            exit 0
        fi
    fi
    echo ""
fi

# --- Phase 3: Scaffold -----------------------------------------------------
if should_run "scaffold"; then
    info "Phase 3: Scaffold the crate"
    PROMPT="$(compose_prompt "${PROMPT_DIR}/03_scaffold.md")"
    run_claude "$PROMPT" "Crate Scaffolding"

    if [[ "$DRY_RUN" != true ]]; then
        if [[ -d "crates/fluent-${NAME}" ]]; then
            success "Crate scaffolded: crates/fluent-${NAME}/"
        else
            fail "Crate directory not created. Aborting."
            exit 1
        fi
    fi
    echo ""
fi

# --- Phase 4: Implement operation groups ------------------------------------
if should_run "implement"; then
    info "Phase 4: Implement operation groups"

    if [[ "$DRY_RUN" == true ]]; then
        PROMPT="$(compose_prompt "${PROMPT_DIR}/04_implement.md")"
        echo "=== DRY RUN: Implement (template — would run per group) ==="
        echo "${PROMPT}"
        echo "---"
    else
        # Extract operation groups from the PRD
        # Look for section headers under file structure or operation groups
        PRD_FILE="docs/prds/fluent-${NAME}.md"
        if [[ ! -f "$PRD_FILE" ]]; then
            fail "PRD not found at ${PRD_FILE}. Run earlier phases first."
            exit 1
        fi

        # Extract group names from the PRD ops file listing (e.g., "formula.rs", "query.rs")
        GROUPS=($(grep -oP 'ops/\K[a-z_]+(?=\.rs)' "$PRD_FILE" | sort -u))

        if [[ ${#GROUPS[@]} -eq 0 ]]; then
            warn "Could not auto-detect operation groups from PRD."
            read -rp "Enter groups (space-separated, e.g., 'formula query tap'): " -a GROUPS
        fi

        info "Operation groups: ${GROUPS[*]}"
        echo ""

        # Run each group implementation
        # Could be parallelized with background jobs, but sequential is safer
        # for shared file system and clearer error reporting
        for group in "${GROUPS[@]}"; do
            info "Implementing group: ${group}"
            GROUP_PROMPT="$(
                sed \
                    -e "s/{{TOOL}}/${TOOL}/g" \
                    -e "s/{{NAME}}/${NAME}/g" \
                    -e "s/{{ENTRY_TYPE}}/${ENTRY_TYPE}/g" \
                    -e "s/{{GROUP}}/${group}/g" \
                    "${PROMPT_DIR}/role.md" "${PROMPT_DIR}/04_implement.md"
            )"
            run_claude "$GROUP_PROMPT" "Implement: ${group}"
            success "Group done: ${group}"
            echo ""
        done
    fi
    echo ""
fi

# --- Phase 5: Assemble modules ---------------------------------------------
if should_run "assemble"; then
    info "Phase 5: Assemble modules and entry point"
    PROMPT="$(compose_prompt "${PROMPT_DIR}/05_assemble.md")"
    run_claude "$PROMPT" "Module Assembly"
    success "Modules assembled"
    echo ""
fi

# --- Phase 6: Build loop (deterministic) -----------------------------------
if should_run "fix"; then
    info "Phase 6: Build and fix"

    if [[ "$DRY_RUN" == true ]]; then
        echo "=== DRY RUN: Would run build loop ==="
        echo "  just full-write && just full-check (up to 5 attempts)"
        echo "  On failure: send errors to Claude for fixing"
        echo "---"
    else
        MAX_ATTEMPTS=5
        for attempt in $(seq 1 $MAX_ATTEMPTS); do
            info "Build attempt ${attempt}/${MAX_ATTEMPTS}"

            cd "$PROJECT_DIR"
            just full-write 2>&1 || true

            if just full-check 2>&1; then
                success "Build passes!"
                break
            else
                ERRORS="$(just full-check 2>&1 || true)"
                if [[ $attempt -eq $MAX_ATTEMPTS ]]; then
                    fail "Build still failing after ${MAX_ATTEMPTS} attempts."
                    echo "$ERRORS"
                    exit 1
                fi

                warn "Build failed. Sending errors to Claude for fixing..."
                FIX_PROMPT="$(cat <<FIXEOF
You are fixing compilation errors in the fluent-${NAME} crate.

The following errors occurred when running \`just full-check\`:

\`\`\`
${ERRORS}
\`\`\`

Read the failing files, understand the errors, and fix them.
Do NOT change the architecture — only fix the specific compilation/clippy errors.
After fixing, run \`just full-write\` to format.
FIXEOF
)"
                run_claude "$FIX_PROMPT" "Fix attempt ${attempt}"
            fi
        done
    fi
    echo ""
fi

# --- Done -------------------------------------------------------------------
echo ""
success "fluent-${NAME} crate is ready!"
info "Next steps:"
info "  1. Review the generated code"
info "  2. git add -A && git commit -m 'feat: add fluent-${NAME} crate wrapping ${TOOL} operations'"
info "  3. Create a PR if desired"
