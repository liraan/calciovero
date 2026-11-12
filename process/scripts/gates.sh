#!/usr/bin/env bash
# Re-exec under bash when started as `sh script` (dash has no here-strings).
[ -n "${BASH_VERSION:-}" ] || exec bash "$0" "$@"
# Gates A and B - mechanical checks from NEW-SESSION-PROMPT.md.
# Exit 0 = green (all mechanical checks pass), 1 = red.
# Run from anywhere: it moves to the repo root. Needs bash (here-strings).
#
# Usage: process/scripts/gates.sh [startup|closing|resume]
#   closing (default) - full set: A1, A3, B1, B3, B6, B7 + manual B2
#   resume            - pointers for a fresh session (LCV, next step, OIs)
#   startup           - fast set: A1, A3, B1, B3, B6, B7 + manual B2
#                        (B2 is only meaningful after work has been done)
#
# Manual checks are NOT reported as "ok": they are listed as "manual" and must
# be done in the report. Reporting them as "ok" without checking would be a
# leak (D5).

set -u
cd "$(dirname "$0")/../.." || { echo "gates: cannot cd to repo root" >&2; exit 2; }
MODE="${1:-closing}"
fail=0

say_ok()     { printf '  ok      %s\n' "$1"; }
say_fail()   { printf '  FAIL    %s\n' "$1"; fail=1; }
say_manual() { printf '  manual  %s\n' "$1"; }

case "$MODE" in
  startup|closing|resume) : ;;
  *) echo "gates: unknown mode '$MODE' (expected startup, closing or resume)" >&2; exit 2 ;;
esac

echo "Gates mode: $MODE"

# ---------- Gate A: documentation coherence ----------
echo "Gate A - documentation coherence"

a_structure=$(grep -cE '^\| 0[0-9][0-9] ' doctrine/STRUCTURE.md 2>/dev/null || echo 0)
a_index=$(grep -cE '^\| \[0[0-9][0-9]\]' doctrine/_index.md 2>/dev/null || echo 0)
a_files=$(ls doctrine/adr/*.md 2>/dev/null | grep -v '000-template' | wc -l | tr -d ' ')
if [ "$a_structure" = "$a_index" ] && [ "$a_index" = "$a_files" ]; then
  say_ok "A1 ADR counts: STRUCTURE=$a_structure, _index=$a_index, files=$a_files"
else
  say_fail "A1 ADR counts diverge: STRUCTURE=$a_structure, _index=$a_index, files=$a_files"
fi

# A2 removed (OI-15). `cargo test` runs in startup step 4 and in C2
# (closing step 1). A third run to compare the README badge with the real
# count is a documentation check, not a simulation invariant: the badge is
# declared in the closing report, where the real output is visible.

a3=$(grep -nE '[0-9]+ (tests?|crates?|ADRs?)\b|(tests?|crates?|ADRs?) [0-9]+' process/NEW-SESSION-PROMPT.md 2>/dev/null | head -5)
if [ -z "$a3" ]; then
  say_ok "A3 no hardcoded counts in NEW-SESSION-PROMPT.md"
else
  say_fail "A3 hardcoded counts found in NEW-SESSION-PROMPT.md:"
  printf '%s\n' "$a3"
fi

# A4 (2026-11-01): crates and ADRs cited in the 4 main docs must match the
# filesystem. Tests are NOT checked here: the real count comes from
# `cargo test` in C2 (closing step 1), and the README badge is the declared
# value, not a second source. Checking the badge against itself is circular.
# Excluded: lines with "as of" (historical).
a4_real_crates=$(ls -d L0-match/crates/ds-* tools/ds-* 2>/dev/null | wc -l | tr -d ' ')
a4_real_adrs=$(ls doctrine/adr/*.md 2>/dev/null | grep -v '000-template' | wc -l | tr -d ' ')
a4_bad=""
for a4_f in README.md doctrine/_index.md doctrine/STRUCTURE.md CONTRIBUTING.md; do
  [ -f "$a4_f" ] || continue
  for a4_p in crates ADRs; do
    case "$a4_p" in
      crates) a4_n=$a4_real_crates ;;
      ADRs) a4_n=$a4_real_adrs ;;
    esac
    a4_hit=$(grep -nE "[0-9]+ ${a4_p}" "$a4_f" 2>/dev/null | grep -v 'as of' | grep -vE "(^|[^0-9])${a4_n} ${a4_p}" | head -2)
    [ -n "$a4_hit" ] && a4_bad="${a4_bad}${a4_f}(${a4_p}=${a4_n}): ${a4_hit}; "
  done
done
if [ -z "$a4_bad" ]; then
  say_ok "A4 crates and ADRs in docs match reality (crates=$a4_real_crates, ADRs=$a4_real_adrs); tests verified in C2"
else
  say_fail "A4 numbers diverge: $a4_bad"
fi

# A4-coherence (2026-11-02): every `N tests` declared in the 4 main docs
# must be the same number. A4 above checks crates and ADRs against the
# filesystem; tests are not checked there (the real count comes from C2,
# and the badge is declared). Coherence is different: the docs must not
# contradict each other. Excluded: lines with "as of" (historical).
a4c_values=""
a4c_bad=""
for a4c_f in README.md doctrine/_index.md doctrine/STRUCTURE.md CONTRIBUTING.md; do
  [ -f "$a4c_f" ] || continue
  a4c_hit=$(grep -nE '[0-9]+ tests?' "$a4c_f" 2>/dev/null | grep -v 'as of')
  [ -z "$a4c_hit" ] && continue
  while IFS= read -r a4c_line; do
    a4c_n=$(printf '%s\n' "$a4c_line" | grep -oE '[0-9]+ tests?' | head -1 | grep -oE '[0-9]+')
    [ -z "$a4c_n" ] && continue
    a4c_values="${a4c_values} ${a4c_n}"
  done <<< "$a4c_hit"
done
# A4-coherence extended: the README badge `tests-N_passing` declares a
# number too. It must match the docs. Pattern: the badge has the number
# after `tests-`, not before.
a4c_badge=$(grep -oE 'badge/tests-[0-9]+_passing' README.md 2>/dev/null | grep -oE '[0-9]+' | head -1)
if [ -n "$a4c_badge" ]; then
  a4c_values="${a4c_values} ${a4c_badge}"
fi
a4c_uniq=$(printf '%s\n' "$a4c_values" | grep -oE '[0-9]+' | sort -u | tr '\n' ' ' | sed 's/ $//')
a4c_count=$(printf '%s\n' "$a4c_uniq" | tr ' ' '\n' | grep -c .)
if [ "$a4c_count" -le 1 ]; then
  say_ok "A4-coherence tests count in docs (tests=$a4c_uniq)"
else
  say_fail "A4-coherence tests count diverges in docs: $a4c_uniq"
fi

# ---------- Gate B: fresh-session simulation (mechanical subset) ----------
echo "Gate B - fresh-session simulation (mechanical subset)"

SS=process/SESSION-STATE.md
NSP=process/NEW-SESSION-PROMPT.md

# B1 (2026-11-02, revised): only [next] counts as "the next step".
# No fallback to [open]: that fallback returned 11 (a [deferred] step)
# whenever no [next] was declared, which is misleading. A Plan with no
# [next] is a valid state (between slices): B1 says so, green, and the
# maintainer sees it.
first_line=$(awk -F'|' 'NF>=7 && $2 ~ /\[next\]/ && $3 ~ /^[[:space:]]*([A-Za-z0-9]+([.][0-9]+)?[a-z0-9]*(-[A-Za-z0-9]+)*)[[:space:]]*$/ {print; exit}' "$SS" 2>/dev/null)
kind="next"
if [ -z "$first_line" ]; then
  say_ok "B1 no [next] step declared (valid between slices)"
else
  step=$(echo "$first_line" | awk -F'|' '{gsub(/^[[:space:]]+|[[:space:]]+$/,"",$3); print $3}')
  action=$(echo "$first_line" | awk -F'|' '{gsub(/^[[:space:]]+|[[:space:]]+$/,"",$6); print $6}')
  if [ -n "$action" ]; then
    say_ok "B1 first $kind step ($step) has a Prima azione"
  else
    say_fail "B1 first $kind step ($step) has an empty Prima azione"
  fi
fi

say_manual "B2 Plan vs Detail order - check in the report"

oi_count=$(grep -cE '^\| OI-[0-9]+ ' "$SS" 2>/dev/null || echo 0)
if [ "$oi_count" -gt 0 ]; then
  say_ok "B3 OI table present ($oi_count items)"
else
  say_fail "B3 no OI table in SESSION-STATE.md"
fi
# B3-bis (2026-11-02): every OI must have a non-empty "Prima azione"
# (third column). An OI without a first action is a step the next session
# will skip (bookmark contract, §F).
oi_empty=$(awk -F'|' '/^\| OI-[0-9]+ / { a=$4; gsub(/^[[:space:]]+|[[:space:]]+$/,"",a); if (a == "") print $2 }' "$SS" 2>/dev/null | tr -d ' ' | tr '\n' ' ')
if [ -n "$oi_empty" ]; then
  say_fail "B3-bis OI without a Prima azione: $oi_empty"
else
  say_ok "B3-bis all OIs have a Prima azione"
fi


proposed=0
surfaced=0
for f in doctrine/adr/*.md; do
  s=$(sed -n '3p' "$f" 2>/dev/null)
  if echo "$s" | grep -q 'Status:\*\* proposed$'; then
    proposed=$((proposed + 1))
    id=$(basename "$f" | cut -d- -f1)
    if grep -q "ADR-$id" "$SS"; then surfaced=$((surfaced + 1)); fi
  fi
done
if [ "$proposed" -eq 0 ]; then
  say_ok "B6 no proposed ADR"
elif [ "$surfaced" -eq "$proposed" ]; then
  say_ok "B6 proposed ADRs surfaced ($surfaced/$proposed)"
else
  say_fail "B6 proposed ADRs surfaced ($surfaced/$proposed)"
fi

uncited=0
for r in process/analysis/*.md; do
  [ -e "$r" ] || continue
  base=$(basename "$r")
  if ! grep -q "$base" "$SS"; then uncited=$((uncited + 1)); fi
done
if [ "$uncited" -eq 0 ]; then
  say_ok "B7 all reports in analysis/ are cited in SESSION-STATE.md"
else
  say_fail "B7 $uncited report(s) in analysis/ not cited in SESSION-STATE.md"
fi

echo ""
# E4b (2026-11-02): before a push, the bookmark update is the last commit.
# Mechanised: if there are commits ahead of the upstream, HEAD must touch
# process/SESSION-STATE.md. This failed twice on 2026-11-02: code
# pushed, bookmark not updated first.
if git rev-parse --abbrev-ref '@{u}' >/dev/null 2>&1; then
  e4b_ahead=$(git rev-list --count '@{u}..HEAD' 2>/dev/null || echo 0)
  if [ "${e4b_ahead:-0}" -gt 0 ]; then
    if git show --name-only --format='' HEAD 2>/dev/null | grep -qx 'process/SESSION-STATE.md'; then
      say_ok "E4b bookmark updated before push (HEAD touches SESSION-STATE.md)"
    else
      say_fail "E4b bookmark not updated: HEAD ($(git rev-parse --short HEAD)) does not touch SESSION-STATE.md, $e4b_ahead unpushed commit(s)"
    fi
  else
    say_ok "E4b no unpushed commits"
  fi
else
  say_ok "E4b no upstream configured (skipped)"
fi


echo ""
# B8 (2026-11-03, MD-32): every [done] micro-decision that declares a
# simplification must say which leaf removes it. The gate checks that
# `Removed-by:` exists and is non-empty; it does NOT check that the value
# is right (not mechanisable). `never` is a valid declared value.
b8_missing=0
while IFS= read -r line; do
  case "$line" in
    *"- [done]"*"[simpl]"*)
      if ! printf '%s' "$line" | grep -qE 'Removed-by:[[:space:]]*[^[:space:]]'; then
        b8_missing=$((b8_missing + 1))
      fi
      ;;
  esac
done < "$SS"
if [ "$b8_missing" -eq 0 ]; then
  say_ok "B8 simplifications declare Removed-by:"
else
  say_fail "B8 $b8_missing simplification(s) without a non-empty Removed-by: (MD-32)"
fi

# B9 (ADR-038, 2026-11-07): the simulation path is platform-free. The
# six simulation crates must not import platform modules (`std::fs`,
# `std::net`, `std::thread`, `std::process`, `std::env`, `std::os`,
# `std::path`) or platform crates (`tokio`, `rayon`, `async-std`,
# `smol`). The banned list lives here, not in the ADR, so changing it
# is one place. The gate greps `src/`, not `tests/`: a test may use
# the platform; the engine may not.
b9_banned='use std::(fs|net|thread|process|env|os|path)'
b9_banned_crates='use (tokio|rayon|async_std|smol)'
b9_sim_crates="L0-match/crates/ds-core/src L0-match/crates/ds-model/src L0-match/crates/ds-db/src L0-match/crates/ds-orchestrator/src L0-match/crates/ds-calculator/src L0-match/crates/ds-match/src L0-match/crates/ds-stats/src"
b9_hits=0
b9_missing=0
for d in $b9_sim_crates; do
  if [ -d "$d" ]; then
    n=$(grep -rEn "$b9_banned|$b9_banned_crates" "$d" 2>/dev/null | wc -l | tr -d ' ')
    b9_hits=$((b9_hits + n))
    if [ "$n" -gt 0 ]; then
      echo "  B9 hit in $d:"
      grep -rEn "$b9_banned|$b9_banned_crates" "$d" 2>/dev/null | sed 's/^/    /'
    fi
  else
    b9_missing=$((b9_missing + 1))
    echo "  B9 missing simulation crate path: $d"
  fi
done
if [ "$b9_missing" -gt 0 ]; then
  say_fail "B9 $b9_missing missing simulation crate path(s): the list is stale"
elif [ "$b9_hits" -eq 0 ]; then
  say_ok "B9 simulation path is platform-free (ADR-038)"
else
  say_fail "B9 $b9_hits platform import(s) in the simulation path (ADR-038)"
fi

# ---------- Resume mode: mechanical pointers for a fresh session ----------
if [ "$MODE" = "resume" ]; then
  echo ""
  echo "Resume - pointers read from the repo, not from chat"
  echo "LCV:    $(grep -m1 'Last commit verified' "$SS" 2>/dev/null | cut -c1-120)"
  echo "HEAD:   $(git rev-parse --short HEAD 2>/dev/null)"
  echo "REMOTE: $(git status -sb 2>/dev/null | head -1)"
  echo ""
  echo "Plan, next steps:"
  grep -nE '^\| \[(next|wip|urgent)\]' "$SS" 2>/dev/null | cut -c1-160
  echo ""
  echo "Open items: $(grep -cE '^\| OI-[0-9]+ ' "$SS" 2>/dev/null) total"
  echo "  newest: $(grep -oE 'OI-[0-9]+' "$SS" 2>/dev/null | sort -t- -k2 -n | tail -1)"
  echo "Method lessons (B1-bis), last 2:"
  awk '/Tool fallbacks \(observed\)/{f=1} /^- \*\*B2 —/{f=0} f && /^    - \*\*/' "$NSP" 2>/dev/null | tail -2 | cut -c1-140
  echo "Ideas backlog: $(grep -cE '^### IDEA-' process/IDEAS.md 2>/dev/null) entries"
  echo "Analysis reports: $(ls process/analysis/*.md 2>/dev/null | wc -l) in process/analysis/"
  echo "Direction: process/analysis/2026-11-04-direction-decisions.md (why the engine has this shape)"
  echo ""
  echo "Dirty tree: $(git status --short 2>/dev/null | wc -l) file(s)"
fi

if [ "$fail" -eq 0 ]; then
  if [ "$MODE" = "closing" ]; then
    echo "gates: GREEN (closing; manual: B2 - do it in the closing report)"
  else
    echo "gates: GREEN (startup; manual: B2)"
  fi
  exit 0
else
  echo "gates: RED - fix before pushing"
  exit 1
fi
