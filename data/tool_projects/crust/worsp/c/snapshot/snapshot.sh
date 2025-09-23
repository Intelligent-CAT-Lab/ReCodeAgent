#!/bin/bash

SCRIPT_DIR=$(dirname "$0")

SNAPSHOT_FILE="$SCRIPT_DIR/snapshot.json"

UPDATE_SNAPSHOT=false

while getopts "u" opt; do
  case $opt in
    u)
      UPDATE_SNAPSHOT=true
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
  esac
done

declare -a RESULTS

ALL_TESTS_PASSED=true

for FILE in $(find "$SCRIPT_DIR/fixtures" -name '*.wsp'); do
  FULL_OUTPUT=$(make --no-print-directory run-main WORSP_FILE="$FILE")
  OUTPUT=$(echo -e "$FULL_OUTPUT" | sed -n '2,$p')
  EXIT_CODE=$?

  if [ $EXIT_CODE -ne 0 ]; then
    echo "Test failed for $FILE."
    ALL_TESTS_PASSED=false
    break
  fi

  if [ "$UPDATE_SNAPSHOT" = true ]; then
    RESULTS+=("{\"fixture\":\"$FILE\",\"stdout\":\"$OUTPUT\"}")
  else
    EXPECTED_OUTPUT=$(jq -r --arg file "$FILE" '.[] | select(.fixture == $file) | .stdout' "$SNAPSHOT_FILE")

    if [ "$OUTPUT" != "$EXPECTED_OUTPUT" ]; then
      echo "Test failed for $FILE. Expected '$EXPECTED_OUTPUT', got '$OUTPUT'."
      ALL_TESTS_PASSED=false
    fi
  fi
done


if [ "$UPDATE_SNAPSHOT" = true ]; then
  JSON_STRING="["
  for i in "${!RESULTS[@]}"; do
    JSON_STRING+="${RESULTS[i]}"
    [ $i -lt $((${#RESULTS[@]} - 1)) ] && JSON_STRING+=","
  done
  JSON_STRING+="]"

  echo -n "$JSON_STRING" | jq '.' > "$SNAPSHOT_FILE"
fi

if [ "$ALL_TESTS_PASSED" = true ]; then
  echo "All tests passed."
else
  echo "Some tests failed."
  exit 1
fi
