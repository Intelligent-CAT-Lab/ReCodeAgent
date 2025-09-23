#include "../../include/crypto/hashing.h"
#include "../../include/vm/commit.h"
#include <stdio.h>
#include <openssl/sha.h>
#include <openssl/rand.h>
#include <time.h>
#include <stdlib.h>

void generate_random_data(char *word, int length) {
    const char charset[] = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const size_t charset_size = sizeof(charset) - 1;

    for (int i = 0; i < length; i++) {
        if (RAND_bytes((unsigned char *)&word[i], 1) != 1) {
            fprintf(stderr, "Error generating random byte.\n");
            exit(EXIT_FAILURE);
        }

        // Map the random byte to the charset
        word[i] = charset[word[i] % charset_size];
    }

    word[length] = '\0';  // Null-terminate the string
}

void generate_hash(const char *data, int length, unsigned char *hash) {
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    SHA256_Update(&sha256, data, length);
    SHA256_Final(hash, &sha256);
}
