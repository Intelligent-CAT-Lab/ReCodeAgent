#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../../include/vm/hashmap.h"
#include "../../include/logging/logger.h"
#include "../../include/vm/common.h"


void init_hash_map(struct hash_map* map) {
    for (int i = 0; i < TABLE_SIZE; i++)
    {
        map->table[i] = NULL;
    }
    
}


unsigned int hash(const char* path) {
    unsigned int hash_value = 0;
    for (int i = 0; i < strlen(path); ++i) {
        hash_value = hash_value * 31 + path[i];
    }

    return hash_value % TABLE_SIZE;
}

void insert_map(struct hash_map* map, const char* path, const char* hash_string, const char* raw_path) {
    unsigned int index = hash(path);

    struct key_value* newNode = (struct key_value*)malloc(sizeof(struct key_value));

    // Ensure that the size of newNode->key and newNode->value can accommodate the strings
    strcpy(newNode->path, path);
    strcpy(newNode->hash, hash_string);
    strcpy(newNode->raw_path, raw_path);

    newNode->next = map->table[index];
    map->table[index] = newNode;
}


char* get_hash_from_path(struct hash_map* map, const char* path) {
    unsigned int index = hash(path);

    struct key_value* current = map->table[index];
    while (current != NULL) {  
        if (strcmp(current->path, path) == 0) {
    
            return current->hash;
        }
        current = current->next;
    }
     
    return NULL;
}

void delete_from_map(struct hash_map* map, const char* path) {
    unsigned int index = hash(path);

    struct key_value* current = map->table[index];
    struct key_value* previous = NULL;

    while (current != NULL) {
        if (strcmp(current->path, path) == 0) {
            if (previous == NULL) {
                map->table[index] = current->next;
            } else {
                previous->next = current->next;
            }

            free(current);
            return;
        }

        previous = current;
        current = current->next;
    }
}

void clear_hash_map(struct hash_map* map) {
    for (int i = 0; i < TABLE_SIZE; i++) {
        struct key_value* current = map->table[i];
        while (current != NULL) {
            struct key_value* temp = current;
            current = current->next;
            free(temp);
        }
        map->table[i] = NULL;
    }
}


int populate_hashmap_from_file(struct hash_map* map, const char* raw_path, const char* filename) {
    FILE* file = fopen(filename, "r");
    if (file == NULL) {
        logger(ERROR_TAG, "Can not populate a hashmap from such as this file");
        return FAIL;
    }

    char line[HASH_LEN + 1 + MAX_PATH];
    fgets(line, sizeof(line), file);
    fgets(line, sizeof(line), file);
    while (fgets(line, sizeof(line), file) != NULL) {
        char* spacePos = strchr(line, ' ');
        if (spacePos != NULL) {
            *spacePos = '\0';  // Null-terminate the hash
            char* path = line;
            char* hash = spacePos + 1;
            hash[strlen(hash) - 1] = '\0';  // Remove the newline character

            char src_file_path[MAX_PATH];
            strcpy(src_file_path, raw_path);
            strcat(src_file_path, "/");
            strcat(src_file_path, hash);

            insert_map(map, path, hash, src_file_path);
        }
    }

    fclose(file);
    return SUCCESS;
}
