#!/bin/bash

set -e 

generate_sequence() {
    local len=1050
    local seq=""
    local bases=(A C G T)
    for (( i=0; i<$len; i++ )); do
        seq+=${bases[$RANDOM % 4]}
    done
    echo "$seq"
}

for i in {1..10}
do
    # Generate a random sequence of length 50
    seq=$(generate_sequence)

    # Generate random 'a' such that 10 < a < 1000
    a=$(( RANDOM % 989 + 11 ))  # Random integer between 11 and 999 inclusive

    # Calculate 'b' constraints: 6 < b < a and b < 100
    b_min=7
    b_max=$(( a - 1 ))
    if (( b_max > 63 )); then
        b_max=63
    fi

    # Ensure b_min does not exceed b_max
    if (( b_max < b_min )); then
        b=$b_min
    else
        b=$(( RANDOM % (b_max - b_min + 1) + b_min ))
    fi

    # Execute the test command with the generated parameters
    echo "./test \"$seq\" $a $b"
    ./test "$seq" $a $b
done
echo "All tests OK."
