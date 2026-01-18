#!/bin/bash
set -e

# Report file
REPORT_FILE="test_report.md"
echo "# 🧪 EXRN Comprehensive Test Report" > "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "| Test Case | Status | Details |" >> "$REPORT_FILE"
echo "|-----------|--------|---------|" >> "$REPORT_FILE"

failures=0

log_result() {
    local name="$1"
    local status="$2"
    local details="$3"
    
    local icon="✅"
    if [ "$status" != "PASS" ]; then
        icon="❌"
        ((failures++))
    fi
    
    echo "| **$name** | $icon $status | $details |" >> "$REPORT_FILE"
    echo "Test '$name': $status - $details"
}

run_test() {
    local test_name="$1"
    local setup_cmd="$2"
    local run_cmd="$3"
    local verify_cmd="$4"
    
    # Setup
    eval "$setup_cmd"
    
    # Run
    eval "$run_cmd"
    
    # Verify
    if eval "$verify_cmd"; then
        log_result "$test_name" "PASS" "Verified successfully"
    else
        log_result "$test_name" "FAIL" "Verification failed"
    fi
    
    # Teardown (optional, using separate folders usually better)
}

# --- PREPARATION ---
EXRN_BIN="./target/release/exrn"
mkdir -p test_env

# --- TEST 1: Basic Renaming (Txt to Md) ---
setup_1="rm -rf test_env/t1 && mkdir -p test_env/t1 && touch test_env/t1/doc.txt"
run_1="$EXRN_BIN -s 'test_env/t1/*.txt' -r '(.*)\.txt' '\$1.md' -y"
verify_1="[ -f test_env/t1/doc.md ] && [ ! -f test_env/t1/doc.txt ]"
run_test "Basic Rename" "$setup_1" "$run_1" "$verify_1"

# --- TEST 2: Dry Run (No changes) ---
setup_2="rm -rf test_env/t2 && mkdir -p test_env/t2 && touch test_env/t2/safe.txt"
run_2="$EXRN_BIN -s 'test_env/t2/*.txt' -r 'safe' 'dangerous' --dry-run -y"
verify_2="[ -f test_env/t2/safe.txt ] && [ ! -f test_env/t2/dangerous.txt ]"
run_test "Dry Run" "$setup_2" "$run_2" "$verify_2"

# --- TEST 3: Chain Renaming (a->b, b->c) ---
setup_3="rm -rf test_env/t3 && mkdir -p test_env/t3 && touch test_env/t3/a test_env/t3/b"
# Rule: match whole name, append '_next'
# This is tricky with regex for specific files. Let's use specific rule.
# Regex: ^(.*)$ -> $1_next
# Expected: a -> a_next, b -> b_next. No, user wants Chain.
# Chain setup: match 'a' -> 'b', match 'b' -> 'c'.
# Regex: 'a' -> 'b', 'b' -> 'c'. Not easy with one regex.
# Let's use the prepend logic tested in unit tests: x->xx, xx->xxx
setup_3="rm -rf test_env/t3 && mkdir -p test_env/t3 && touch test_env/t3/x test_env/t3/xx"
# Regex: "^(.*)$" -> "x$1"
# x -> xx
# xx -> xxx
run_3="$EXRN_BIN -s 'test_env/t3/*' -r '^(.*)$' 'x\$1' -y"
verify_3="[ -f test_env/t3/xxx ] && [ -f test_env/t3/xx ] && [ ! -f test_env/t3/x ]"
run_test "Chain Rename (Prepend)" "$setup_3" "$run_3" "$verify_3"

# --- TEST 4: Special Characters (Spaces) ---
setup_4="rm -rf test_env/t4 && mkdir -p test_env/t4 && touch 'test_env/t4/file name.txt'"
run_4="$EXRN_BIN -s 'test_env/t4/*.txt' -r ' ' '_' -y"
verify_4="[ -f test_env/t4/file_name.txt ]"
run_test "Space Handling" "$setup_4" "$run_4" "$verify_4"

# --- TEST 5: Direct File Path (No Glob Quotes) ---
# Simulating shell expansion by passing direct file path
setup_5="rm -rf test_env/t5 && mkdir -p test_env/t5 && touch test_env/t5/direct.txt"
run_5="$EXRN_BIN -s test_env/t5/direct.txt -r 'direct' 'target' -y"
verify_5="[ -f test_env/t5/target.txt ]"
run_test "Direct Path" "$setup_5" "$run_5" "$verify_5"

# --- SUMMARY ---
echo "" >> "$REPORT_FILE"
if [ $failures -eq 0 ]; then
    echo "🎉 **All Tests Passed!**" >> "$REPORT_FILE"
    exit 0
else
    echo "⚠️ **$failures Tests Failed**" >> "$REPORT_FILE"
    exit 1
fi
