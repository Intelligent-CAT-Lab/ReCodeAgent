#ifndef COMMON_H
#define COMMON_H

#define SUCCESS 1
#define FAIL -1
#include <openssl/sha.h>

#define HASH_LEN 2 * SHA256_DIGEST_LENGTH + 1

/**
 * creating a file with contents
*/
int create_file(const char* filename, const char* content);

/**
 * Check if a directory is exists or not
*/
int is_dir_exists(const char *parentDir, const char *targetDir);

/**
 * Check if an array of string contains a string
*/
int is_contains(char** arr, char* str);

/**
 * Make a string from a list of string
*/
void make_string(char* dest, const char* src, ...); 

/**
 * Creating a hash using SHA256
*/
void create_hash(char* str);

/**
 * Custom output function for debugging
*/
void custom_printf(const char *format, ...);
#endif
