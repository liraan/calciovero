#!/usr/bin/env bash
# Re-exec under bash when started as `sh script` (dash has no here-strings).
[ -n "${BASH_VERSION:-}" ] || exec bash "$0" "$@"
# closing.sh - one command for the closing ritual (E2), AFTER the maintainer
# has written "chiudi". Calls C2 (the five commands, in order), gates.sh
# closing, and capture.sh for the closing event. Adds ONLY the NEXT note.
# It does NOT write SESSION-STATE.md, does NOT commit, does NOT push:
# the bookmark is synthesis, the commit is the maintainer's decision.
#
# Rule (maintainer, 2026-11-07): a command that duplicates a field already
# printed by gates.sh resume/closing is noise. Cite, do not repeat.
#
# Exit 0 = all green, 1 = a check failed (fix before continuing).

set -u
cd "$(dirname "$0")/.." || { echo "closing: cannot cd to repo root" >&2; exit 1; }

FAIL=0

echo "=== closing: git verify ==="
git status --short
git log --oneline -5
git rev-parse HEAD
echo ""

echo "=== closing: C2 (the five commands, in order) ==="
echo "--- build --release ---"
cargo build --release || FAIL=1
echo "--- clippy --workspace --all-targets -- -D warnings ---"
cargo clippy --workspace --all-targets -- -D warnings || FAIL=1
echo "--- test --workspace ---"
cargo test --workspace || FAIL=1
echo "--- fmt --check ---"
cargo fmt --check || FAIL=1
echo "--- test-runner --check --seed 42 ---"
cargo run --release --bin ds-test-runner -- --check --seed 42 || FAIL=1
echo ""

echo "=== closing: gates (full set) ==="
bash scripts/gates.sh closing || FAIL=1
echo ""

echo "=== closing: capture (record the closing event) ==="
bash scripts/capture.sh pause "closing ritual executed via scripts/closing.sh; C2 $( [ $FAIL -eq 0 ] && echo green || echo RED ); gates $( [ $FAIL -eq 0 ] && echo green || echo RED ); bookmark written next, commit/push by maintainer. -> none" || FAIL=1
echo ""

if [ "$FAIL" -eq 0 ]; then
  echo "=== NEXT: write bookmark, then commit and push ==="
  echo "  1. Update SESSION-STATE.md §1-3 (LCV = commit before this one; OIs; Plan)."
  echo "  2. git add <path> (one path per call, B1-bis)"
  echo "     - docs/context/SESSION-STATE.md"
  echo "     - docs/context/captures/_index.md"
  echo "     - docs/context/captures/YYYY-MM.md"
  echo "     - scripts/ (only the scripts touched)"
  echo "  3. git commit -t .gitmessage  (trailers: Verified: C2-green, Rules-Audit: ...)"
  echo "  4. git push"
  echo "  5. git status -sb   (must show '## main...origin/main', no [ahead])"
  echo ""
  echo "closing: GREEN"
  exit 0
else
  echo "closing: RED - fix before writing the bookmark"
  exit 1
fi
