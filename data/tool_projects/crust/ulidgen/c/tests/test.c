#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <time.h>
#include "../src/ulid.h"
// Helper function to validate ULID structure
int is_valid_ulid(const char *ulid) {
    const char *b32alphabet = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";

    // Check length
    if (strlen(ulid) != 26)
        return 0;

    // Check characters
    for (int i = 0; i < 26; i++) {
        if (!strchr(b32alphabet, ulid[i]))
            return 0;
    }

    return 1;
}

// Test Cases
void test_ulid_length() {
    char ulid[27];
    ulidgen_r(ulid);
    assert(strlen(ulid) == 26 && "ULID length should be 26 characters");
}

void test_ulid_structure() {
    char ulid[27];
    ulidgen_r(ulid);
    printf("Generated ULID: %s\n", ulid);
    assert(is_valid_ulid(ulid) && "ULID should only contain valid Base32 characters");
}

void test_ulid_uniqueness() {
    char ulid1[27], ulid2[27];
    ulidgen_r(ulid1);
    ulidgen_r(ulid2);

    // Ensure two ULIDs generated consecutively are not the same
    assert(strcmp(ulid1, ulid2) != 0 && "Consecutive ULIDs should be unique");
}

void test_ulid_sortability() {
    char ulid1[27], ulid2[27];
    ulidgen_r(ulid1);

    // Simulate a delay to ensure different timestamps
    struct timespec ts = { .tv_sec = 0, .tv_nsec = 1500000 }; // 1.5 ms
    nanosleep(&ts, NULL);

    ulidgen_r(ulid2);
    assert(strcmp(ulid1, ulid2) < 0 && "ULIDs should be lexicographically sortable based on time");
}

int main() {
    test_ulid_length();
    printf("Test passed: ULID length is valid\n");

    // test_ulid_structure();
    // printf("Test passed: ULID structure is valid\n");

    test_ulid_uniqueness();
    printf("Test passed: ULID uniqueness is valid\n");

    test_ulid_sortability();
    printf("Test passed: ULID sortability is valid\n");

    printf("All tests passed successfully.\n");
    return 0;
}