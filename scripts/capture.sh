#!/bin/sh
# capture.sh - register "what was said" at a pause or task boundary.
# Not a second gates.sh resume: it does NOT count tests/ADRs/crates/ideas,
# does NOT read remote, does NOT run cargo. Those are gates.sh's job.
# It records: the event, the detto, the promotions, and the proof of the
# promotions (git log LCV..HEAD --oneline + --name-only).
#
# Promotion contract: a promotion exists ONLY when it follows the "->" marker.
#   capture.sh pause "discussion ... -> OI-79, IDEA-INFRA-023"
# Anything before "->" is narrative, not a promise. Use "-> none" or omit "->"
# to declare no promotions.
#
# Usage:
#   scripts/capture.sh pause "<testo>"
#   scripts/capture.sh task <id> "<testo>"
#
# Output files (append-only):
#   docs/context/captures/_index.md  -- one line per event
#   docs/context/captures/YYYY-MM.md -- body of the written events
#
# Exit 0 = captured (or filtered), 1 = refused (promotion missing), 2 = usage.

set -u

if [ $# -lt 2 ]; then
  echo "usage: scripts/capture.sh pause \"<testo>\" | scripts/capture.sh task <id> \"<testo>\"" >&2
  exit 2
fi

MODE="$1"
shift
case "$MODE" in
  pause) [ $# -ne 1 ] && { echo "capture: pause needs exactly one text arg" >&2; exit 2; }; TASK_ID="-"; TEXT="$1" ;;
  task)  [ $# -ne 2 ] && { echo "capture: task needs <id> \"<text>\"" >&2; exit 2; }; TASK_ID="$1"; TEXT="$2" ;;
  *) echo "capture: unknown mode '$MODE' (pause|task)" >&2; exit 2 ;;
esac

cd "$(dirname "$0")/.." || { echo "capture: cannot cd to repo root" >&2; exit 1; }

SS=docs/context/SESSION-STATE.md
IDEAS=docs/context/IDEAS.md
IDX=docs/context/captures/_index.md
MONTH_FILE="docs/context/captures/$(date '+%Y-%m').md"

[ -f "$SS" ]   || { echo "capture: missing $SS" >&2; exit 1; }
[ -f "$IDEAS" ] || { echo "capture: missing $IDEAS" >&2; exit 1; }

LCV=$(grep -m1 '^\*\*Last commit verified:\*\*' "$SS" | sed -E 's/.*`([0-9a-f]+)`.*/\1/')
[ -z "$LCV" ] && { echo "capture: cannot read LCV from $SS" >&2; exit 1; }

NOW=$(date '+%Y-%m-%d %H:%M')
HEAD=$(git rev-parse --short HEAD 2>/dev/null || echo "?")
N_COMMITS=$(git rev-list --count "$LCV..HEAD" 2>/dev/null || echo 0)
FILES=$(git diff --name-only "$LCV..HEAD" 2>/dev/null | wc -l | tr -d ' ')
FILES_LIST=$(git diff --name-only "$LCV..HEAD" 2>/dev/null | head -8 | tr '\n' ' ')
LOG=$(git log --oneline "$LCV..HEAD" 2>/dev/null | head -8)

# Filter: empty text = filtered:none (the row exists, the body does not).
if [ -z "$TEXT" ]; then
  STATUS="filtered:none"
  PROMOS="-"
  VERIFY="(filtered: no text, no promotion to verify)"
else
  STATUS="written"

  # Promotions: parsed from the "->" marker only. Before "->" is narrative.
  PROMO_TEXT=""
  case "$TEXT" in
    *"->"*) PROMO_TEXT="${TEXT##*->}" ;;
  esac
  PROMO_TEXT=$(printf '%s\n' "$PROMO_TEXT" | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')
  case "$PROMO_TEXT" in
    ""|"none"|"None"|"NONE") PROMO_TEXT="" ;;
  esac

  PROMOS=""
  FAILP=0
  if [ -z "$PROMO_TEXT" ]; then
    PROMOS="-"
    VERIFY="(no promotions declared via '->' marker)"
  else
    for OI in $(printf '%s\n' "$PROMO_TEXT" | grep -oE 'OI-[0-9]+' | sort -u); do
      if grep -qE "^\| $OI " "$SS"; then
        PROMOS="${PROMOS}${OI}(exists); "
      else
        PROMOS="${PROMOS}${OI}(MISSING); "
        FAILP=1
      fi
    done
    for ID in $(printf '%s\n' "$PROMO_TEXT" | grep -oE 'IDEA-[A-Z]+-[0-9]+' | sort -u); do
      if grep -qE "^### $ID" "$IDEAS"; then
        PROMOS="${PROMOS}${ID}(exists); "
      else
        PROMOS="${PROMOS}${ID}(MISSING); "
        FAILP=1
      fi
    done
    for AD in $(printf '%s\n' "$PROMO_TEXT" | grep -oE 'ADR-[0-9]+' | sort -u); do
      NUM=$(printf '%s\n' "$AD" | grep -oE '[0-9]+')
      if ls docs/adr/0*"$NUM"*.md >/dev/null 2>&1; then
        PROMOS="${PROMOS}${AD}(exists); "
      else
        PROMOS="${PROMOS}${AD}(MISSING); "
        FAILP=1
      fi
    done
    [ -z "$PROMOS" ] && PROMOS="-"
    if [ "$FAILP" -eq 1 ]; then
      VERIFY="FAIL: promotion did not resolve (see PROMOS above)"
    else
      VERIFY="all promotions resolved"
    fi
  fi
fi

mkdir -p docs/context/captures
[ -f "$IDX" ] || printf '# Captures index (append-only)\n\nformat: YYYY-MM-DD HH:MM | pause|task <id> | status | HEAD | commits LCV..HEAD | promos | month-file\n\n' > "$IDX"

printf '%s | %s %s | %s | %s | %s | %s | %s\n' \
  "$NOW" "$MODE" "$TASK_ID" "$STATUS" "$HEAD" "$N_COMMITS" "$PROMOS" "$MONTH_FILE" >> "$IDX"

if [ "$STATUS" = "written" ]; then
  [ -f "$MONTH_FILE" ] || printf '# Captures - %s\n' "$(date '+%Y-%m')" > "$MONTH_FILE"
  {
    printf '\n## %s | %s %s | HEAD %s\n\n' "$NOW" "$MODE" "$TASK_ID" "$HEAD"
    printf '**Trigger.** %s %s at %s\n\n' "$MODE" "$TASK_ID" "$NOW"
    printf '**Detto.** %s\n\n' "$TEXT"
    printf '**Promotions.** %s\n\n' "$PROMOS"
    printf '**Verify.** %s\n\n' "$VERIFY"
    printf '**Proof (LCV %s..%s, %s commits, %s files).** `%s`\n\n' "$LCV" "$HEAD" "$N_COMMITS" "$FILES" "$FILES_LIST"
    if [ -n "$LOG" ]; then
      printf '```\n%s\n```\n\n' "$LOG"
    fi
  } >> "$MONTH_FILE"
fi

echo "capture: $MODE $TASK_ID -> $STATUS (HEAD $HEAD, LCV..HEAD = $N_COMMITS commits, $FILES files)"
echo "capture: wrote $IDX"
[ "$STATUS" = "written" ] && echo "capture: wrote $MONTH_FILE"
[ "$STATUS" = "written" ] && [ "${FAILP:-0}" -eq 1 ] && { echo "capture: PROMOTION FAILED to resolve - fix before commit" >&2; exit 1; }
exit 0
