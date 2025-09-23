#include "closed_syncmers.h"
#include "closed_syncmers_naive.h"

int main(int argc, char *argv[]) {
    if(argc < 4) {
        fprintf(stderr, "Usage: %s sequence K S\n", argv[0]);
        return 1;
    }
    char *sequence_input = argv[1];
    int K = atoi(argv[2]);
    int S = atoi(argv[3]);
    if(S >= K) {
        fprintf(stderr, "Error: S must be less than K\n");
        return 1;
    }
    int num_results;
    MinimizerResult results[10000];
    compute_closed_syncmers(sequence_input, strlen(sequence_input), K, S, results, &num_results);

    printf("Closed Syncmers:\n");
    printf("%-20s %-20s\n", "Position", "Minimizer Hash");
    for (int i = 0; i < num_results; i++) {
        printf("%-20zu %-20llu\n", results[i].kmer_position, (unsigned long long)results[i].minimizer_hash);
    }

    // Compute closed syncmers using the naive method
    int num_naive_results;
    MinimizerResult naive_results[10000];
    compute_closed_syncmers_naive(sequence_input, strlen(sequence_input), K, S, naive_results, &num_naive_results);

    printf("\nClosed Syncmers (Naive):\n");
    printf("%-20s %-20s\n", "Position", "Minimizer Hash");
    for (int i = 0; i < num_naive_results; i++) {
        printf("%-20zu %-20llu\n", naive_results[i].kmer_position, (unsigned long long)naive_results[i].minimizer_hash);
    }

    // Compare the results
    if (num_results != num_naive_results) {
        printf("\nMismatch in number of closed syncmers: %d (original) vs %d (naive)\n", num_results, num_naive_results);
    } else {
        printf("\nNumber of closed syncmers matches: %d\n", num_results);
    }
    // Compare each result
    int mismatch = 0;
    for (int i = 0; i < num_results; i++) {
        if (results[i].kmer_position != naive_results[i].kmer_position || results[i].minimizer_hash != naive_results[i].minimizer_hash) {
            printf("Mismatch at index %d:\n", i);
            printf("  Original -> Position: %zu, Hash: %llu\n", results[i].kmer_position, (unsigned long long)results[i].minimizer_hash);
            printf("  Naive    -> Position: %zu, Hash: %llu\n", naive_results[i].kmer_position, (unsigned long long)naive_results[i].minimizer_hash);
            mismatch = 1;
	    exit(1);
        }
    }
    if (!mismatch) {
        printf("All closed syncmers match between original and naive method.\n");
    }

    return 0;
}

