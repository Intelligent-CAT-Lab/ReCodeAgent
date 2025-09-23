#include "../../include/vm/common.h"
#include <stdio.h>
#include <dirent.h>
#include <string.h>
#include <stdarg.h>
#include <unistd.h>
#include <stdlib.h>
#include "../../include/crypto/hashing.h"


int create_file(const char* filename, const char* content) {
    FILE *file = fopen(filename, "w");
    if (file == NULL) return FAIL;
    if (fprintf(file, content) < 0) return FAIL;

    fclose(file);

    return SUCCESS;
}

int is_dir_exists(const char *parentDir, const char *targetDir) {

    DIR *dir = opendir(parentDir);
    if (dir == NULL) return FAIL;
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        if (entry->d_type == DT_DIR && strcmp(entry->d_name, targetDir) == 0) {
            closedir(dir);
            return 1; 
        }
    }

    closedir(dir);
    return 0;
}


int is_contains(char** arr, char* str) {
    while(*arr != NULL) {
        if (strcmp(*arr, str)) return 1;
    }

    return 0;
}


   
void make_string(char* dest, const char* src, ...) {
    strcpy(dest, src);

    // Get a pointer to the variable argument list
    va_list args;
    va_start(args, src);

    // Iterate through the remaining parameters and concatenate them
    const char* current;
    while ((current = va_arg(args, const char*)) != NULL) {
        strcat(dest, current);
    }

    // Clean up the variable argument list
    va_end(args);
}

void create_hash(char* str) {
    // Set a seed for the random number generator
    srand(time(NULL));

    char random_data[RANDOM_SIZE];
    generate_random_data(random_data, RANDOM_SIZE); 
    // Generate hash from random data
    char hash[HASH_LEN];
    generate_hash(random_data, HASH_LEN, hash);
    for (int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
        sprintf(&str[2 * i], "%02x", hash[i]);
    }
    str[HASH_LEN] = 0;
}
