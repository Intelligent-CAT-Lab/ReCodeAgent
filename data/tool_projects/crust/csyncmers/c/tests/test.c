#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "../src/closed_syncmers.h"
#include "../src/closed_syncmers_naive.h"

// Function to generate a random sequence of specified length
void generate_sequence(char *seq, int len) {
    const char bases[] = "ACGT";
    for (int i = 0; i < len; i++) {
        seq[i] = bases[rand() % 4];
    }
    seq[len] = '\0';
}

// Main function
int main() {
    srand(time(NULL)); // Seed the random number generator

    int num_tests = 10;
    int sequence_length = 1050;
    char sequence[sequence_length + 1]; // +1 for the null terminator

    for (int test = 1; test <= num_tests; test++) {
        // Generate a random sequence
        generate_sequence(sequence, sequence_length);

        // Generate random 'a' such that 10 < a < 1000
        int a = rand() % 989 + 11; // Random integer between 11 and 999 inclusive

        // Calculate 'b' constraints: 6 < b < a and b < 100
        int b_min = 7;
        int b_max = (a - 1 < 63) ? a - 1 : 63;

        // Ensure b_min does not exceed b_max
        int b = (b_max < b_min) ? b_min : (rand() % (b_max - b_min + 1) + b_min);

        printf("Test %d: Sequence Length=%d, K=%d, S=%d\n", test, sequence_length, a, b);

        int num_results;
        MinimizerResult results[10000];
        compute_closed_syncmers(sequence, strlen(sequence), a, b, results, &num_results);

        printf("Closed Syncmers:\n");
        printf("%-20s %-20s\n", "Position", "Minimizer Hash");
        for (int i = 0; i < num_results; i++) {
            printf("%-20zu %-20llu\n", results[i].kmer_position, (unsigned long long)results[i].minimizer_hash);
        }

        // Compute closed syncmers using the naive method
        int num_naive_results;
        MinimizerResult naive_results[10000];
        compute_closed_syncmers_naive(sequence, strlen(sequence), a, b, naive_results, &num_naive_results);

        printf("\nClosed Syncmers (Naive):\n");
        printf("%-20s %-20s\n", "Position", "Minimizer Hash");
        for (int i = 0; i < num_naive_results; i++) {
            printf("%-20zu %-20llu\n", naive_results[i].kmer_position, (unsigned long long)naive_results[i].minimizer_hash);
        }

        // Compare the results
        if (num_results != num_naive_results) {
            printf("\nMismatch in number of closed syncmers: %d (original) vs %d (naive)\n", num_results, num_naive_results);
            return 1;
        } else {
            printf("\nNumber of closed syncmers matches: %d\n", num_results);
        }

        // Compare each result
        int mismatch = 0;
        for (int i = 0; i < num_results; i++) {
            if (results[i].kmer_position != naive_results[i].kmer_position ||
                results[i].minimizer_hash != naive_results[i].minimizer_hash) {
                printf("Mismatch at index %d:\n", i);
                printf("  Original -> Position: %zu, Hash: %llu\n",
                       results[i].kmer_position, (unsigned long long)results[i].minimizer_hash);
                printf("  Naive    -> Position: %zu, Hash: %llu\n",
                       naive_results[i].kmer_position, (unsigned long long)naive_results[i].minimizer_hash);
                mismatch = 1;
                break;
            }
        }
        if (mismatch) {
            printf("Test %d failed.\n", test);
            return 1;
        }

        printf("Test %d passed. All closed syncmers match between original and naive method.\n\n", test);
    }

    printf("All tests passed.\n");
    return 0;
}
