#!/usr/bin/env bash
# Re-exec under bash when started as `sh script` (dash has no here-strings).
[ -n "${BASH_VERSION:-}" ] || exec bash "$0" "$@"
# starting.sh - one command for the startup ritual (E1).
# Calls gates.sh resume (existing, prints pointers) and adds ONLY what
# gates.sh does not do: the three git commands for alignment, and the
# NEXT list for the agent. It does NOT read doctrine, files, or specs:
# it prints pointers, and the agent reads them (D5: the report is the proof).
#
# Rule (maintainer, 2026-11-07): a command that duplicates a field already
# printed by gates.sh resume/closing is noise. Cite, do not repeat.

set -u
cd "$(dirname "$0")/.." || { echo "starting: cannot cd to repo root" >&2; exit 1; }

echo "=== startup: git alignment ==="
git status --short
git pull --ff-only
git log --oneline -5
echo ""

echo "=== startup: gates (resume = fast subset + pointers) ==="
bash scripts/gates.sh resume
GATES=$?
echo ""

echo "=== NEXT: read (the agent reads; this only lists) ==="
echo "The numbered sections of NEW-SESSION-PROMPT.md, Startup ritual:"
sed -n '/^## Startup ritual/,/^## What NOT/p' docs/context/NEW-SESSION-PROMPT.md \
  | grep -E '^### ' \
  | sed 's/^### /  /'
echo ""

echo "=== NEXT: report to maintainer (step 5) ==="
echo "  - what was read, what was NOT read, and why (D1)"
echo "  - repo state: aligned? clean? numbers?"
echo "  - the [next] step according to SESSION-STATE.md"
echo "  - any divergence between docs and reality"
echo "  - do NOT propose a step before this report is done"

exit "$GATES"
