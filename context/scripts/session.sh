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
  echo "usage: session.sh start | close [full|reduced [<cargo selectors>]] | check <module> | check changed | capture pause \"<text>\" | capture task <id> \"<text>\"" >&2
  exit 2
}

CORE_CRATES="ds-core ds-model ds-db ds-orchestrator ds-calculator ds-match ds-stats"

# Tests run to completion: a deadline killed the second compile after all
# guards were already green, turning a passing check into a failure
# (maintainer, 2026-11-21: "togli il limite di secondi altrimenti fa saltare tutto").
run_cargo_test() {
  cargo test "$@"
}

# Test-result cache (maintainer, 2026-11-20): a file whose current bytes were
# already covered by a green `check changed` is not tested again. The key is
# the file's content hash, so reverting a change reuses the old verdict. The
# cache lives in .cache/ (gitignored) and follows the same guard map below:
# it never invents coverage for a file the map would have sent to the suite.
TEST_CACHE_DIR=.cache/session-tests
TEST_CACHE_FILE="$TEST_CACHE_DIR/verified.tsv"
CURRENT_FP="$TEST_CACHE_DIR/.current"
CACHE_MODE=worktree

# One hash pass for all files, not one process per file: on Windows the
# process spawn is slower than the hashing itself.
cache_write_current() {
  mkdir -p "$TEST_CACHE_DIR"
  : > "$CURRENT_FP"
  local f h line existing=()
  if [ "$CACHE_MODE" = worktree ]; then
    for f in "${CHANGED_FILES[@]}"; do [ -f "$f" ] && existing+=("$f"); done
    if [ ${#existing[@]} -gt 0 ] && command -v sha1sum >/dev/null 2>&1; then
      while IFS= read -r line; do
        h="${line:0:40}"
        f="${line:42}"
        printf '%s\t%s\n' "$f" "$h" >> "$CURRENT_FP"
      done < <(sha1sum -- "${existing[@]}")
    else
      for f in "${existing[@]}"; do
        printf '%s\t%s\n' "$f" "$(git hash-object --no-filters -- "$f")" >> "$CURRENT_FP"
      done
    fi
    for f in "${CHANGED_FILES[@]}"; do
      [ -f "$f" ] || printf '%s\tabsent\n' "$f" >> "$CURRENT_FP"
    done
  else
    for f in "${CHANGED_FILES[@]}"; do
      h=$(git show ":$f" 2>/dev/null | git hash-object --stdin) || h=absent
      printf '%s\t%s\n' "$f" "$h" >> "$CURRENT_FP"
    done
  fi
}

cache_pending() {
  local out="$1"
  : > "$out"
  if [ -f "$TEST_CACHE_FILE" ]; then
    awk -F '\t' 'NR == FNR { ok[$1 FS $2] = 1; next } !(($1 FS $2) in ok) { print $1 }' \
      "$TEST_CACHE_FILE" "$CURRENT_FP" > "$out"
  else
    cut -f1 "$CURRENT_FP" > "$out"
  fi
}

cache_mark_verified() {
  local pending="$1" tmp="$TEST_CACHE_FILE.tmp"
  mkdir -p "$TEST_CACHE_DIR"
  if [ ! -f "$TEST_CACHE_FILE" ]; then
    printf '# map:%s\n' "$(git hash-object --no-filters -- context/scripts/session.sh)" > "$TEST_CACHE_FILE"
  fi
  awk -F '\t' 'NR == FNR { drop[$0] = 1; next } !($1 in drop) { print }' \
    "$pending" "$TEST_CACHE_FILE" > "$tmp"
  awk -F '\t' 'NR == FNR { keep[$0] = 1; next } ($1 in keep) { print }' \
    "$pending" "$CURRENT_FP" >> "$tmp"
  mv "$tmp" "$TEST_CACHE_FILE"
}

# One source of truth: the workspace members in Cargo.toml. No second list.
workspace_members() {
  sed -n '/^members = \[/,/^\]/p' Cargo.toml | sed -n 's/^[[:space:]]*"\([^"]*\)".*/\1/p'
}

# A changed crate selects its own test. Sets CHANGED_SELECTORS (-p <crate>...)
# and CHANGED_CORE (1 when the simulation itself changed).
# `staged` reads the index (pre-commit); the default reads the working tree.
changed_selectors() {
  CHANGED_SELECTORS=()
  CHANGED_CORE=0
  local mode="${1:-worktree}" from_list="${2:-}"
  local lcv f m crate seen=" "
  if [ "$from_list" != from-list ]; then
    lcv=$(grep -m1 '^\*\*Last commit verified:\*\*' "$SS" | sed -E 's/.*`([0-9a-f]+)`.*/\1/')
    local files=()
    while IFS= read -r f; do [ -n "$f" ] && files+=("$f"); done < <(
      if [ "$mode" = staged ]; then
        git diff --cached --name-only --diff-filter=ACMRD
      else
        { git diff --name-only HEAD; git ls-files --others --exclude-standard;
          [ -n "$lcv" ] && git diff --name-only "$lcv..HEAD"; } 2>/dev/null | sort -u
      fi
    )
    CHANGED_FILES=("${files[@]}")
  fi
  for f in "${CHANGED_FILES[@]}"; do
    case "$f" in Cargo.toml|Cargo.lock) CHANGED_CORE=1 ;; esac
    for m in $(workspace_members); do
      case "$f" in
        "$m"/*)
          crate="${m##*/}"
          case "$seen" in *" $crate "*) ;; *) seen="$seen$crate "; CHANGED_SELECTORS+=(-p "$crate") ;; esac
          case " $CORE_CRATES " in *" $crate "*) CHANGED_CORE=1 ;; esac
          ;;
      esac
    done
  done
}

# A changed source file selects the *single test functions* that guard it —
# not their whole module, and never the golden unless the golden itself is
# touched. The table lives here, next to the check that uses it: an unknown
# or shared file selects the whole suite, because under-testing is worse than
# a slow check. Sets CHANGED_FUNCS (cases::mod::fn) and CHANGED_ALL.
changed_test_functions() {
  CHANGED_FUNCS=()
  CHANGED_ALL=0
  local f fn
  add_fn() {
    case " ${CHANGED_FUNCS[*]:-} " in *" $1 "*) ;; *) CHANGED_FUNCS+=("$1") ;; esac
  }
  add_golden() {
    add_fn golden_l0::golden_l0_is_deterministic
    add_fn golden_l0::golden_l0_with_instructions_is_deterministic
    add_fn golden_l0::golden_l0_cup_is_deterministic
  }
  for f in "${CHANGED_FILES[@]}"; do
    case "$f" in
      Cargo.toml|Cargo.lock|*/Cargo.toml)
        # A dependency change is guarded by the golden plus the touched
        # crates; the whole workspace is not needed for that.
        add_golden ;;
      L0-match/crates/ds-match/tests/suite.rs|L0-match/crates/ds-match/tests/support/*)
        # The harness itself: the golden is its guard, not the whole workspace.
        add_golden ;;
      L0-match/crates/ds-match/tests/cases/*.rs)
        # A touched test file selects its own #[test] functions, not the whole
        # module: maximal granularity, and the untracked redesign does not turn
        # into a full-suite run.
        mod="${f##*/}"; mod="${mod%.rs}"
        while IFS= read -r fn; do
          [ -n "$fn" ] && add_fn "$mod::$fn"
        done < <(awk '
          /^[[:space:]]*#\[test\]/ { want=1; next }
          want && /^[[:space:]]*fn [A-Za-z0-9_]+/ {
            line=$0; sub(/^.*fn[[:space:]]+/, "", line); sub(/\(.*/, "", line)
            print line; want=0; next
          }
          want && /^[[:space:]]*#\[/ && /\)[[:space:]]*$/ { next }
        ' "$f") ;;
      # Shared formulas have named guards in the integration suite; a whole
      # workspace run is not needed to check a known shared file.
      L0-match/crates/ds-core/src/lib.rs|L0-match/crates/ds-calculator/src/kick.rs|L0-match/crates/ds-calculator/src/duel.rs|L0-match/crates/ds-calculator/src/fatigue.rs|L0-match/crates/ds-calculator/src/save.rs|L0-match/crates/ds-calculator/src/thought.rs|L0-match/crates/ds-calculator/tests/reach_has_one_value.rs|L0-match/crates/ds-orchestrator/src/lib.rs|L0-match/crates/ds-orchestrator/src/actor.rs|L0-match/crates/ds-orchestrator/src/read.rs|L0-match/crates/ds-orchestrator/src/offball.rs|L0-match/crates/ds-orchestrator/src/outcome.rs)
        add_golden
        add_fn shot_forced::resolve_shot_pressure_widens_the_deviation ;;
      L0-match/crates/ds-core/src/rng.rs)
        # Deleted: it was carried but never drawn from (ADR-035).
        add_golden ;;
      L0-match/crates/ds-core/src/hash.rs|L0-match/crates/ds-core/src/identity.rs)
        add_golden ;;
      L0-match/crates/ds-core/*|L0-match/crates/ds-model/*|L0-match/crates/ds-db/*|L0-match/crates/ds-orchestrator/*|L0-match/crates/ds-calculator/*|L0-match/crates/ds-stats/*)
        CHANGED_ALL=1 ;;
      L0-match/crates/ds-match/src/*)
        case "$f" in
          */simulate.rs|*/engine.rs|*/driver.rs|*/state.rs|*/lib.rs|*/context.rs|*/registry.rs|*/fact.rs|*/shape.rs|*/balance.rs|*/plugins/mod.rs|*/test_support.rs)
            # Any change to the simulation core moves the golden hashes: those
            # three tests are its anchor, and they are the guard here.
            add_golden ;;
          */plugins/technique.rs)
            add_golden
            add_fn feet::swapped_wide_men_strike_with_the_feet_they_have
            add_fn shot_forced::resolve_shot_contact_height_picks_the_vertical_direction
            add_fn golden_scenes::in_a_dribble_the_ball_is_ahead ;;
          */plugins/duels.rs)
            add_golden
            add_fn knock::a_knocked_man_leaves_the_pitch
            add_fn knock::the_same_contact_knocks_the_frailer_body
            add_fn narrative_fields::duel_lost_when_winner_is_not_actor ;;
          */plugins/keeper.rs)
            add_golden
            add_fn save_forced::a_ball_at_his_body_is_stopped
            add_fn save_forced::far_outside_any_dive_is_never_stopped ;;
          */plugins/bench.rs)
            add_fn coach::the_coach_reads_the_match ;;
          */plugins/thought.rs)
            add_golden
            add_fn instructions::a_striker_asked_to_run_waits_on_the_last_line
            add_fn channels::by_voice_at_once_by_a_gesture_when_they_look ;;
          */plugins/set_pieces.rs)
            add_golden
            add_fn extra_time::a_cup_match_level_after_extra_time_goes_to_penalties
            add_fn golden_scenes::no_restart_stops_the_match ;;
          */plugins/referee.rs)
            add_fn extra_time::a_cup_match_won_at_ninety_ends_there
            add_fn tick_record::the_laws_call_fouls_and_cards ;;
          */plugins/fatigue.rs)
            add_fn tick_record::the_players_tire_and_half_time_rests_them ;;
          */plugins/ball.rs)
            add_fn functional_l0::functional_l0_agents_move_and_do_not_cluster
            add_fn tick_record::the_ball_is_mostly_at_someone_s_feet ;;
          */plugins/perception.rs)
            add_fn plugins_off::the_match_is_played_with_psychology_switched_off ;;
          */plugins/body.rs)
            add_fn golden_scenes::nobody_moves_faster_than_his_legs
            add_fn knock::the_same_contact_knocks_the_frailer_body ;;
          */plugins/psychology.rs)
            add_fn plugins_off::the_match_is_played_with_psychology_switched_off ;;
          */spatial.rs)
            # move_agents lives here: a change to how men run moves the hashes.
            add_golden
            add_fn golden_scenes::nobody_moves_faster_than_his_legs
            add_fn feet::swapped_wide_men_strike_with_the_feet_they_have ;;
          */record.rs|*/recording.rs)
            add_fn tick_record::recording_does_not_change_the_match
            add_fn golden_scenes::in_a_dribble_the_ball_is_ahead ;;
          */save.rs)
            add_fn save_forced::the_same_shot_gives_the_same_save
            add_fn anti_leak::save_contains_no_latent_field_name
            add_fn instructions::a_saved_match_replays_its_instructions ;;
          */narrative.rs)
            add_fn narrative_fields::tackle_text_uses_from_field_and_event_keeps_it ;;
          */result.rs)
            add_fn anti_leak::save_contains_no_latent_field_name ;;
          *) CHANGED_ALL=1 ;;
        esac ;;
    esac
  done
  [ "$CHANGED_ALL" -eq 1 ] && CHANGED_FUNCS=()
  return 0
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

# The guard map says *which* tests cover a changed file; the cache says
# *whether that file still needs them*. Only unverified files are fed to the
# map, so a file already covered by a green run is not paid for twice.
check_changed() {
  local mode="${1:-worktree}" f ok=0 map_hash cached_map cached
  local use_cache=1
  local pending=() pending_file="$TEST_CACHE_DIR/.pending"
  CACHE_MODE="$mode"
  changed_selectors "$mode"
  changed_test_functions

  # A closing C2 must really run (D1: `C2-green` needs a gate that ran now).
  [ "${CACHE_DISABLED:-0}" -eq 1 ] && use_cache=0
  if [ "$use_cache" -eq 1 ]; then
    # If the guard map itself changed, cached verdicts may name the wrong tests.
    map_hash=$(git hash-object --no-filters -- context/scripts/session.sh)
    cached_map=$([ -f "$TEST_CACHE_FILE" ] && head -n 1 "$TEST_CACHE_FILE" || echo "")
    [ "$cached_map" = "# map:$map_hash" ] || rm -f "$TEST_CACHE_FILE"

    cache_write_current
    cache_pending "$pending_file"
    while IFS= read -r f; do [ -n "$f" ] && pending+=("$f"); done < "$pending_file"
    cached=$((${#CHANGED_FILES[@]} - ${#pending[@]}))
  else
    pending=("${CHANGED_FILES[@]}")
    cached=0
  fi
  if [ ${#pending[@]} -eq 0 ]; then
    echo "changed: ${cached} file(s) already verified in cache - no test to rerun"
    return 0
  fi
  [ "$cached" -eq 0 ] || echo "changed: ${cached} file(s) already verified, ${#pending[@]} to check"

  CHANGED_FILES=("${pending[@]}")
  changed_selectors "$mode" from-list
  changed_test_functions

  if [ "$CHANGED_ALL" -eq 1 ]; then
    echo "changed: file globale toccato -> suite completa"
    run_cargo_test --workspace || ok=1
  elif [ "$CHANGED_CORE" -eq 1 ] && [ ${#CHANGED_FUNCS[@]} -gt 0 ]; then
    local filters=() other=() i
    for f in "${CHANGED_FUNCS[@]}"; do filters+=("cases::$f"); done
    echo "changed functions: ${CHANGED_FUNCS[*]}"
    run_cargo_test --release -p ds-match --test suite -- "${filters[@]}" || ok=1
    for ((i=0; i<${#CHANGED_SELECTORS[@]}; i+=2)); do
      [ "${CHANGED_SELECTORS[i+1]}" = ds-match ] || other+=("${CHANGED_SELECTORS[i]}" "${CHANGED_SELECTORS[i+1]}")
    done
    [ ${#other[@]} -eq 0 ] || check test --release "${other[@]}" || ok=1
  elif [ ${#CHANGED_SELECTORS[@]} -gt 0 ]; then
    echo "changed crates: ${CHANGED_SELECTORS[*]}"
    check test "${CHANGED_SELECTORS[@]}" || ok=1
  else
    echo "changed: no crate touched (docs only) - no test to run"
  fi

  if [ "$ok" -eq 0 ] && [ "$use_cache" -eq 1 ]; then cache_mark_verified "$pending_file"; fi
  return "$ok"
}

# C2 modules (RULES.md §C2): one check, one command. `close` composes them;
# `session.sh check <module>` runs one alone. No second list to keep in sync.
check() {
  case "${1:-}" in
    fmt)    cargo fmt --check ;;
    clippy) cargo clippy --workspace --all-targets -- -D warnings ;;
    build)  cargo build --release ;;
    test)   shift; if [ $# -gt 0 ]; then run_cargo_test "$@"; else check changed worktree; fi ;;
    golden) run_cargo_test --release -p ds-match --test suite golden_l0 ;;
    runner) cargo run --release --bin ds-test-runner -- --check --seed 42 ;;
    gates)  shift; bash context/scripts/gates.sh "${1:-closing}" ;;
    changed) check_changed "${2:-worktree}" ;;
    *)      echo "check: unknown module '${1:-}' (fmt|clippy|build|test|golden|runner|gates|changed)" >&2; return 2 ;;
  esac
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
    profile="${2:-full}"
    selectors=()
    if [ "$profile" = reduced ]; then
      changed_selectors
      if [ "$CHANGED_CORE" -eq 1 ]; then
        echo "close: nocciolo toccato -> profilo full (C2 hard stop)"
        profile=full
      elif [ $# -ge 3 ]; then
        selectors=("${@:3}")
      else
        selectors=("${CHANGED_SELECTORS[@]}")
      fi
    fi
    run_module() { echo "--- check $*"; check "$@" || fail=1; }
    echo "=== close: C2 ($profile) ==="
    if [ "$profile" = reduced ]; then
      run_module fmt
      run_module clippy
      if [ ${#selectors[@]} -gt 0 ]; then
        run_module test "${selectors[@]}"
      else
        echo "--- no crate touched (docs only): test skipped"
      fi
      run_module golden
    else
      run_module build
      run_module clippy
      CACHE_DISABLED=1 run_module test
      run_module fmt
      run_module runner
    fi
    echo ""
    echo "=== close: gates ==="
    run_module gates closing
    echo ""
    verdict=$([ "$fail" -eq 0 ] && echo green || echo RED)
    capture pause "closing: C2 ($profile) and gates $verdict -> none" || fail=1
    echo ""
    if [ "$fail" -eq 0 ]; then
      trailer=$([ "$profile" = reduced ] && echo C2-ridotto || echo C2-green)
      echo "close: GREEN ($profile). Next: update STATE.md (LCV = HEAD), commit with Verified: $trailer, push to origin (the Mirror workflow copies main to codeberg; a manual push there is fine too)."
      exit 0
    fi
    echo "close: RED - fix before writing the bookmark"
    exit 1
    ;;
  check)
    shift
    check "$@"
    ;;
  capture)
    shift
    capture "$@"
    ;;
  *) usage ;;
esac
