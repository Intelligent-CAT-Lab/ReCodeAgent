#!/bin/bash
# Usage: ./scripts/run_test_comparison.sh <project>
# Example: ./scripts/run_test_comparison.sh commons-cli
# Or run all: ./scripts/run_test_comparison.sh all

project=$1
# Set to true to compute embedding similarity (slow, loads model). Default: false
COMPUTE_SIMILARITY=${COMPUTE_SIMILARITY:-false}

BASE_DIR="translations/data/tool_projects/alphatrans"

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

run_comparison() {
    local proj=$1
    local python_path="${PYTHON_TEST_PATHS[$proj]}"
    local superclass_map="${SUPERCLASS_MAPPINGS[$proj]}"
    
    if [ -z "$python_path" ]; then
        echo "Error: Unknown project '$proj'"
        echo "Available projects: commons-cli, commons-csv, commons-fileupload, commons-validator"
        return 1
    fi
    
    echo "Processing $proj..."
    echo "  Java path: ${BASE_DIR}/${proj}/${JAVA_TEST_PATH}"
    echo "  Python path: ${BASE_DIR}/${proj}/${python_path}"
    
    # Build superclass arg if mapping exists
    local superclass_arg=""
    if [ -n "$superclass_map" ]; then
        superclass_arg="--superclass_map ${superclass_map}"
        echo "  Superclass mappings: ${superclass_map}"
    fi

    # Build similarity arg if enabled
    local similarity_arg=""
    if [ "$COMPUTE_SIMILARITY" = "true" ]; then
        similarity_arg="--compute_similarity"
        echo "  Embedding similarity: enabled"
    fi
    
    python src/analysis/compare_tests.py \
        --mapping_csv "${BASE_DIR}/${proj}/test_name_mapping.csv" \
        --source_lang "${BASE_DIR}/${proj}/${JAVA_TEST_PATH}" \
        --target_lang "${BASE_DIR}/${proj}/${python_path}" \
        --output "${BASE_DIR}/${proj}/test_comparison_report.json" \
        $superclass_arg $similarity_arg
    echo ""
}

if [ -z "$project" ]; then
    echo "Usage: ./scripts/run_test_comparison.sh <project|all>"
    echo "Available projects: commons-cli, commons-csv, commons-fileupload, commons-validator"
    exit 1
fi

if [ "$project" = "all" ]; then
    for proj in commons-cli commons-csv commons-fileupload commons-validator; do
        run_comparison "$proj"
    done
else
    run_comparison "$project"
fi
