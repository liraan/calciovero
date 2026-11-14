#!/usr/bin/env bash
# Re-exec under bash when started as `sh script`.
[ -n "${BASH_VERSION:-}" ] || exec bash "$0" "$@"
# session.sh - the rituals of a session (context/RULES.md §E), one script.
#
#   start                       git alignment, gates (resume), pointers (E1)
#   close                       C2 in order, gates, capture of the closing (E4)
#   capture pause "<text>"      record a pause (E3)
#   capture task <id> "<text>"  record a task boundary (E3)
#
# It never writes STATE.md, never commits, never pushes: the bookmark is a
# synthesis, the commit a decision.
#
# Capture: a promotion exists only after the "->" marker
#   session.sh capture pause "discussion ... -> OI-79, IDEA-INFRA-023"
# Every OI, IDEA or ADR after "->" must exist, or the capture fails (exit 1).
# "-> none", or no marker, declares no promotion. Empty text writes a
# `filtered:none` row and no body.
# Output (append-only): context/captures/_index.md, context/captures/YYYY-MM.md

set -u
cd "$(dirname "$0")/../.." || { echo "session: cannot cd to repo root" >&2; exit 2; }

SS=context/STATE.md
IDEAS=context/IDEAS.md
CAP=context/captures

usage() {
  echo "usage: session.sh start | close | capture pause \"<text>\" | capture task <id> \"<text>\"" >&2
  exit 2
}

capture() {
  local mode="${1:-}"; shift || true
  local task_id text
  case "$mode" in
    pause) [ $# -eq 1 ] || usage; task_id="-"; text="$1" ;;
    task)  [ $# -eq 2 ] || usage; task_id="$1"; text="$2" ;;
    *) usage ;;
  esac
  [ -f "$SS" ] || { echo "capture: missing $SS" >&2; return 1; }
  local lcv now head n files files_list log status promos verify failp=0
  lcv=$(grep -m1 '^\*\*Last commit verified:\*\*' "$SS" | sed -E 's/.*`([0-9a-f]+)`.*/\1/')
  [ -n "$lcv" ] || { echo "capture: cannot read LCV from $SS" >&2; return 1; }
  now=$(date '+%Y-%m-%d %H:%M')
  head=$(git rev-parse --short HEAD 2>/dev/null || echo "?")
  n=$(git rev-list --count "$lcv..HEAD" 2>/dev/null || echo 0)
  files=$(git diff --name-only "$lcv..HEAD" 2>/dev/null | wc -l | tr -d ' ')
  files_list=$(git diff --name-only "$lcv..HEAD" 2>/dev/null | head -8 | tr '\n' ' ')
  log=$(git log --oneline "$lcv..HEAD" 2>/dev/null | head -8)
  local month="$CAP/$(date '+%Y-%m').md" idx="$CAP/_index.md"

  if [ -z "$text" ]; then
    status="filtered:none"; promos="-"; verify="(filtered: no text)"
  else
    status="written"; promos=""
    local p=""
    case "$text" in *"->"*) p="${text##*->}" ;; esac
    p=$(printf '%s' "$p" | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')
    case "$p" in ""|none|None|NONE) p="" ;; esac
    local id num
    for id in $(printf '%s\n' "$p" | grep -oE 'OI-[0-9]+' | sort -u); do
      if grep -qE "^\| $id " "$SS"; then promos="$promos$id(exists); "; else promos="$promos$id(MISSING); "; failp=1; fi
    done
    for id in $(printf '%s\n' "$p" | grep -oE 'IDEA-[A-Z]+-[0-9]+' | sort -u); do
      if grep -qE "^### $id" "$IDEAS"; then promos="$promos$id(exists); "; else promos="$promos$id(MISSING); "; failp=1; fi
    done
    for id in $(printf '%s\n' "$p" | grep -oE 'ADR-[0-9]+' | sort -u); do
      num=$(printf '%03d' "$((10#${id#ADR-}))")
      if ls doctrine/adr/"$num"-*.md >/dev/null 2>&1; then promos="$promos$id(exists); "; else promos="$promos$id(MISSING); "; failp=1; fi
    done
    [ -n "$promos" ] || promos="-"
    if [ -z "$p" ]; then verify="(no promotion declared)"
    elif [ "$failp" -eq 1 ]; then verify="FAIL: a promotion did not resolve"
    else verify="all promotions resolved"; fi
  fi

  mkdir -p "$CAP"
  [ -f "$idx" ] || printf '# Captures index (append-only)\n\n' > "$idx"
  printf '%s | %s %s | %s | %s | %s | %s | %s\n' "$now" "$mode" "$task_id" "$status" "$head" "$n" "$promos" "$month" >> "$idx"
  if [ "$status" = "written" ]; then
    [ -f "$month" ] || printf '# Captures - %s\n' "$(date '+%Y-%m')" > "$month"
    {
      printf '\n## %s | %s %s | HEAD %s\n\n' "$now" "$mode" "$task_id" "$head"
      printf '**Said.** %s\n\n**Promotions.** %s\n\n**Verify.** %s\n\n' "$text" "$promos" "$verify"
      printf '**Since LCV %s (%s commits, %s files).** `%s`\n\n' "$lcv" "$n" "$files" "$files_list"
      [ -n "$log" ] && printf '```\n%s\n```\n\n' "$log"
    } >> "$month"
  fi
  echo "capture: $mode $task_id -> $status (HEAD $head, $n commits since LCV)"
  [ "$failp" -eq 0 ] || { echo "capture: a promotion did not resolve; fix before committing" >&2; return 1; }
  return 0
}

case "${1:-}" in
  start)
    echo "=== start: git alignment ==="
    git status --short
    git pull --ff-only
    git log --oneline -5
    echo ""
    echo "=== start: gates ==="
    bash context/scripts/gates.sh resume
    rc=$?
    echo ""
    echo "=== next: report to the maintainer (RULES.md E1) before proposing anything ==="
    exit "$rc"
    ;;
  close)
    fail=0
    echo "=== close: C2 ==="
    cargo build --release || fail=1
    cargo clippy --workspace --all-targets -- -D warnings || fail=1
    cargo test --workspace || fail=1
    cargo fmt --check || fail=1
    cargo run --release --bin ds-test-runner -- --check --seed 42 || fail=1
    echo ""
    echo "=== close: gates ==="
    bash context/scripts/gates.sh closing || fail=1
    echo ""
    verdict=$([ "$fail" -eq 0 ] && echo green || echo RED)
    capture pause "closing: C2 and gates $verdict -> none" || fail=1
    echo ""
    if [ "$fail" -eq 0 ]; then
      echo "close: GREEN. Next: update STATE.md (LCV = HEAD), commit with Verified: C2-green, push to origin (the Mirror workflow copies main to codeberg; a manual push there is fine too)."
      exit 0
    fi
    echo "close: RED - fix before writing the bookmark"
    exit 1
    ;;
  capture)
    shift
    capture "$@"
    ;;
  *) usage ;;
esac
