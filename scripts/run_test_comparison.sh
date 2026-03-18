#!/bin/bash
# Usage: ./scripts/run_test_comparison.sh <project>
# Example: ./scripts/run_test_comparison.sh commons-cli
#          ./scripts/run_test_comparison.sh go-edlib
#          ./scripts/run_test_comparison.sh strsim
# Or run all AlphaTrans: ./scripts/run_test_comparison.sh all
# Or run all Oxidizer:   ./scripts/run_test_comparison.sh all-oxidizer
# Or run all Skel:       ./scripts/run_test_comparison.sh all-skel

project=$1
# Set to true to compute embedding similarity (slow, loads model). Default: false
COMPUTE_SIMILARITY=true

ALPHATRANS_BASE_DIR="results/recodeagent_translations/data/tool_projects/alphatrans"
OXIDIZER_BASE_DIR="results/recodeagent_translations/data/tool_projects/oxidizer"
SKEL_BASE_DIR="results/recodeagent_translations/data/tool_projects/skel"

# ── AlphaTrans configuration ──────────────────────────────────────────────────

# Java test path is always the same
JAVA_TEST_PATH="java/src/test/java"

# Python test paths vary per project
declare -A PYTHON_TEST_PATHS
PYTHON_TEST_PATHS["commons-cli"]="python/src/test/python"
PYTHON_TEST_PATHS["commons-csv"]="python/src/test/python"
PYTHON_TEST_PATHS["commons-fileupload"]="python/test"
PYTHON_TEST_PATHS["commons-validator"]="python/src/test/python"

# Superclass mappings for each project (class:superclass,class:superclass,...)
# Used to look up inherited test methods
declare -A SUPERCLASS_MAPPINGS
# commons-cli parser tests inherit from ParserTestCase
SUPERCLASS_MAPPINGS["commons-cli"]="BasicParserTest:ParserTestCase,\
DefaultParserTest:ParserTestCase,\
GnuParserTest:ParserTestCase,\
PosixParserTest:ParserTestCase"
SUPERCLASS_MAPPINGS["commons-csv"]=""
SUPERCLASS_MAPPINGS["commons-fileupload"]=""
# commons-validator has many test classes that inherit from abstract test classes
SUPERCLASS_MAPPINGS["commons-validator"]="BigDecimalValidatorTest:AbstractNumberValidatorTest,\
BigIntegerValidatorTest:AbstractNumberValidatorTest,\
ByteValidatorTest:AbstractNumberValidatorTest,\
DoubleValidatorTest:AbstractNumberValidatorTest,\
FloatValidatorTest:AbstractNumberValidatorTest,\
IntegerValidatorTest:AbstractNumberValidatorTest,\
LongValidatorTest:AbstractNumberValidatorTest,\
ShortValidatorTest:AbstractNumberValidatorTest,\
CalendarValidatorTest:AbstractCalendarValidatorTest,\
DateValidatorTest:AbstractCalendarValidatorTest,\
ABANumberCheckDigitTest:AbstractCheckDigitTest,\
CUSIPCheckDigitTest:AbstractCheckDigitTest,\
EAN13CheckDigitTest:AbstractCheckDigitTest,\
IBANCheckDigitTest:AbstractCheckDigitTest,\
ISBN10CheckDigitTest:AbstractCheckDigitTest,\
ISBNCheckDigitTest:AbstractCheckDigitTest,\
ISINCheckDigitTest:AbstractCheckDigitTest,\
ISSNCheckDigitTest:AbstractCheckDigitTest,\
LuhnCheckDigitTest:AbstractCheckDigitTest,\
ModulusTenABACheckDigitTest:AbstractCheckDigitTest,\
ModulusTenCUSIPCheckDigitTest:AbstractCheckDigitTest,\
ModulusTenEAN13CheckDigitTest:AbstractCheckDigitTest,\
ModulusTenLuhnCheckDigitTest:AbstractCheckDigitTest,\
ModulusTenSedolCheckDigitTest:AbstractCheckDigitTest,\
SedolCheckDigitTest:AbstractCheckDigitTest,\
VerhoeffCheckDigitTest:AbstractCheckDigitTest"

# ── Oxidizer configuration ────────────────────────────────────────────────────

OXIDIZER_PROJECTS=("go-edlib" "stats" "gonameparts" "gohistogram" "checkdigit" "textrank")

# ── Skel configuration ────────────────────────────────────────────────────────

SKEL_PROJECTS=("bst" "colorsys" "heapq" "html" "mathgen" "rbt" "strsim" "toml")

# ── Helper: build similarity arg ─────────────────────────────────────────────

similarity_arg=""
if [ "$COMPUTE_SIMILARITY" = "true" ]; then
    similarity_arg="--compute_similarity"
fi

# ── AlphaTrans runner ─────────────────────────────────────────────────────────

run_alphatrans_comparison() {
    local proj=$1
    local python_path="${PYTHON_TEST_PATHS[$proj]}"
    local superclass_map="${SUPERCLASS_MAPPINGS[$proj]}"

    if [ -z "$python_path" ]; then
        echo "Error: Unknown AlphaTrans project '$proj'"
        echo "Available projects: commons-cli, commons-csv, commons-fileupload, commons-validator"
        return 1
    fi

    echo "Processing $proj (AlphaTrans: Java → Python)..."
    echo "  Java path:   ${ALPHATRANS_BASE_DIR}/${proj}/${JAVA_TEST_PATH}"
    echo "  Python path: ${ALPHATRANS_BASE_DIR}/${proj}/${python_path}"

    local superclass_arg=""
    if [ -n "$superclass_map" ]; then
        superclass_arg="--superclass_map ${superclass_map}"
        echo "  Superclass mappings: ${superclass_map}"
    fi

    if [ "$COMPUTE_SIMILARITY" = "true" ]; then
        echo "  Embedding similarity: enabled"
    fi

    python src/analysis/compare_tests.py \
        --mapping_csv "${ALPHATRANS_BASE_DIR}/${proj}/test_name_mapping.csv" \
        --source_lang "${ALPHATRANS_BASE_DIR}/${proj}/${JAVA_TEST_PATH}" \
        --target_lang "${ALPHATRANS_BASE_DIR}/${proj}/${python_path}" \
        --output "${ALPHATRANS_BASE_DIR}/${proj}/test_comparison_report.json" \
        $superclass_arg $similarity_arg
    echo ""
}

# ── Oxidizer runner ───────────────────────────────────────────────────────────

run_oxidizer_comparison() {
    local proj=$1
    local proj_dir="${OXIDIZER_BASE_DIR}/${proj}"

    if [ ! -d "$proj_dir" ]; then
        echo "Error: Unknown Oxidizer project '$proj' (directory not found: $proj_dir)"
        echo "Available projects: ${OXIDIZER_PROJECTS[*]}"
        return 1
    fi

    echo "Processing $proj (Oxidizer: Go → Rust)..."
    echo "  Go path:   ${proj_dir}/go"
    echo "  Rust path: ${proj_dir}/rust"

    if [ "$COMPUTE_SIMILARITY" = "true" ]; then
        echo "  Embedding similarity: enabled"
    fi

    python src/analysis/compare_tests.py \
        --mapping_csv "${proj_dir}/test_name_mapping.csv" \
        --source_lang "${proj_dir}/go" \
        --target_lang "${proj_dir}/rust" \
        --output "${proj_dir}/test_comparison_report.json" \
        $similarity_arg
    echo ""
}

# ── Skel runner ───────────────────────────────────────────────────────────────

run_skel_comparison() {
    local proj=$1
    local proj_dir="${SKEL_BASE_DIR}/${proj}"

    if [ ! -d "$proj_dir" ]; then
        echo "Error: Unknown Skel project '$proj' (directory not found: $proj_dir)"
        echo "Available projects: ${SKEL_PROJECTS[*]}"
        return 1
    fi

    echo "Processing $proj (Skel: Python → JavaScript)..."
    echo "  Python path:     ${proj_dir}/python"
    echo "  JavaScript path: ${proj_dir}/javascript"

    if [ "$COMPUTE_SIMILARITY" = "true" ]; then
        echo "  Embedding similarity: enabled"
    fi

    python src/analysis/compare_tests.py \
        --mapping_csv "${proj_dir}/test_name_mapping.csv" \
        --source_lang "${proj_dir}/python" \
        --target_lang "${proj_dir}/javascript" \
        --output "${proj_dir}/test_comparison_report.json" \
        $similarity_arg
    echo ""
}

# ── Dispatch ──────────────────────────────────────────────────────────────────

is_oxidizer_project() {
    local proj=$1
    for p in "${OXIDIZER_PROJECTS[@]}"; do
        [ "$p" = "$proj" ] && return 0
    done
    return 1
}

is_skel_project() {
    local proj=$1
    for p in "${SKEL_PROJECTS[@]}"; do
        [ "$p" = "$proj" ] && return 0
    done
    return 1
}

if [ -z "$project" ]; then
    echo "Usage: ./scripts/run_test_comparison.sh <project|all|all-oxidizer|all-skel>"
    echo ""
    echo "AlphaTrans projects (Java → Python):"
    echo "  commons-cli, commons-csv, commons-fileupload, commons-validator"
    echo ""
    echo "Oxidizer projects (Go → Rust):"
    echo "  ${OXIDIZER_PROJECTS[*]}"
    echo ""
    echo "Skel projects (Python → JavaScript):"
    echo "  ${SKEL_PROJECTS[*]}"
    exit 1
fi

if [ "$project" = "all" ]; then
    for proj in commons-cli commons-csv commons-fileupload commons-validator; do
        run_alphatrans_comparison "$proj"
    done
elif [ "$project" = "all-oxidizer" ]; then
    for proj in "${OXIDIZER_PROJECTS[@]}"; do
        run_oxidizer_comparison "$proj"
    done
elif [ "$project" = "all-skel" ]; then
    for proj in "${SKEL_PROJECTS[@]}"; do
        run_skel_comparison "$proj"
    done
elif is_oxidizer_project "$project"; then
    run_oxidizer_comparison "$project"
elif is_skel_project "$project"; then
    run_skel_comparison "$project"
else
    run_alphatrans_comparison "$project"
fi
