#include "../../include/vm/common.h"
#include "../../include/vm/commit.h"
#include "../../include/logging/logger.h"
#include "../../include/vm/hashmap.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>


int add_changes(struct repository *repo, struct file_node* files) {

    // step1: check if track file is exist -> if not create it
    char track_dir_path[MAX_PATH];
    strcpy(track_dir_path, repo->dir);
    strcat(track_dir_path, "/.vsync");
    // check if .vsync exists
    DIR* dir = opendir(track_dir_path);
    if (dir == NULL)
    {
        logger(ERROR_TAG, "[add_changes] Can't find .vsync");
        return FAIL;
    }
    closedir(dir);

    strcat(track_dir_path, "/tracked"); 
    // check if tracked dir exists
    dir = opendir(track_dir_path);
    if (dir == NULL)
    {
        if (mkdir(track_dir_path, 0777) != 0)
        {
            logger(ERROR_TAG, "Can not create a tracking dir");
            return FAIL;
        }
    }
    closedir(dir);

    // check if track file exist + load infos + update changes
    struct hash_map* map = (struct hash_map*) malloc(sizeof(struct hash_map));
    init_hash_map(map);

    char track_file_path[MAX_PATH];
    strcpy(track_file_path, track_dir_path);
    strcat(track_file_path, "/track");
    if ((access(track_file_path, F_OK) == 0) && (populate_hashmap_from_file(map, track_dir_path, track_file_path) == FAIL)){
        return FAIL;
    }

    // traversal the changes and old changes for removing duplicated ones
    struct file_node* trav= files;
    while (trav != NULL) {
        // check if files existed
        if (access(trav->path, F_OK) != 0) {
            logger(ERROR_TAG, "Can not adding changes that not existed... check the added files");
            clear_hash_map(map);
            return FAIL;
        }
        
        // check if a tracked changes is existed for this file
        char* tr = get_hash_from_path(map, trav->path);

        // remove it
        if (tr != NULL) {

            char old_file_path[MAX_PATH];
            strcpy(old_file_path, track_dir_path);
            strcat(old_file_path, "/");
            strcat(old_file_path, tr);

            
            if (remove(old_file_path) != 0) {
                logger(ERROR_TAG, "[add_changes] Can not adding changes");
                return FAIL;
            }

            delete_from_map(map, trav->path);
        }
        
        // generate the hash
        char hash_string[HASH_LEN];  // +1 for null terminator
        create_hash(hash_string);
        insert_map(map, trav->path, hash_string, trav->path);

        trav = trav->next;
    }

    // add this changes
    if (make_changes(repo, map) == FAIL) {
        logger(ERROR_TAG, "[add_changes] Can not persist changes");
        return FAIL;
    }


    clear_hash_map(map);
    return SUCCESS;
}

int make_changes(struct repository* repo, struct hash_map* map) {
    char track_file_path[MAX_PATH];
    char track_dir_path[MAX_PATH];
    strcpy(track_dir_path, repo->dir);
    strcat(track_dir_path, "/.vsync/tracked");
    strcat(track_file_path, track_dir_path);
    strcat(track_file_path, "/track");

    FILE* track_file = fopen(track_file_path, "w");

    if (track_file == NULL) {
        logger(ERROR_TAG, "[make_changes] Can not open track file");
        return FAIL;
    }

    // write the first letter
    fprintf(track_file, "-\n");
    fprintf(track_file, "-\n");

    for (int i = 0; i < TABLE_SIZE; i++)
    {
        struct key_value* current = map->table[i];

        while (current != NULL){  
            char destination_path[MAX_PATH];
            strcpy(destination_path, track_dir_path);
            strcat(destination_path, "/");
            strcat(destination_path, current->hash);

            if (strcmp(current->raw_path, destination_path) != 0) {
                FILE* changes_file = fopen(current->raw_path, "rb");
                FILE* destination_file = fopen(destination_path, "wb");

                if (changes_file == NULL) {
                    logger(ERROR_TAG, "[make_changes] Can not open changes file");
                    return FAIL;            
                }
                if (destination_file == NULL) {
                    logger(ERROR_TAG, "[make_changes] Can not open destination file");
                    return FAIL;            
                }

                // copying from source to destination
                char buffer[1024];
                size_t bytesRead;
                while ((bytesRead = fread(buffer, 1, sizeof(buffer), changes_file)) > 0) {
                    fwrite(buffer, 1, bytesRead, destination_file);
                }

                fclose(changes_file);
                fclose(destination_file);
            }

            fprintf(track_file, "%s %s\n", current->path, current->hash);
            current = current->next;
        }
        
    }

    fclose(track_file);
    return SUCCESS;
}

int make_init_map_repo(struct repository* repo, struct hash_map* map, const char* path) {
    DIR *dir = opendir(path);
    struct dirent *entry;

    // Check if the directory can be opened
    if (dir == NULL) {
        perror("[make_init_map_repo] Can not open the init commit dir");
        return FAIL;
    }


    // Read each entry in the directory
    while ((entry = readdir(dir)) != NULL) {
        // Ignore "." and ".." directories
        if (strcmp(entry->d_name, ".") != 0 
            && strcmp(entry->d_name, "..") != 0
            && strcmp(entry->d_name, ".vsync") != 0) {

            // Construct the full path of the entry
            char fullpath[PATH_MAX];
            snprintf(fullpath, sizeof(fullpath), "%s/%s", path, entry->d_name);

            // If the entry is a directory, recursively call the function
            if (entry->d_type == DT_DIR) {
                make_init_map_repo(repo, map, fullpath);
            } else {
                // generate the hash
                char hash_string[HASH_LEN];  // +1 for null terminator
                create_hash(hash_string);

                insert_map(map, fullpath, hash_string, fullpath);
            }
        }
    }
    
    return SUCCESS;
}


int create_commit(struct repository* repo, struct commit* commit, struct hash_map* map) {
    // create the commit dir
    char commit_path[MAX_PATH];
    strcpy(commit_path, repo->dir);
    strcat(commit_path, "/.vsync/");
    strcat(commit_path, commit->hash);

    if (mkdir(commit_path, 0777) != 0) {
        logger(ERROR_TAG, "[create_commit] Can not create the commit dir");
        return FAIL;
    }
    
    // create the commit file and copy the raw the data
    strcat(commit_path, "/commit");

    FILE* commit_file = fopen(commit_path, "w");

    if (commit_file == NULL) {
        logger(ERROR_TAG, "[create_commit] Can not create the commit file");
        return FAIL;
    }

    // write the basics info
    fprintf(commit_file, "%s\n", commit->parent_hash);
    fprintf(commit_file, "%s %s\n", commit->author->username, commit->author->mail);

    // make commit path = ...../.vsync/hash/
    commit_path[strlen(commit_path) - 6] = 0;
    
    for(int i = 0; i < TABLE_SIZE; i++) {
        struct key_value* current = map->table[i];
        
        while (current != NULL)
        {
            // open the src file
            FILE* source_file = fopen(current->raw_path, "rb");
            if (source_file == NULL)
            {
                logger(ERROR_TAG, "[create_commit] can not open the src file");
                return FAIL;
            }
            

            // open the dest file
            char destination_path[MAX_PATH];
            strcpy(destination_path, commit_path);
            strcat(destination_path, current->hash);

            FILE* destination_file = fopen(destination_path, "wb");
            if (destination_file == NULL)
            {
                logger(ERROR_TAG, "[create_commit] can not open the destination file");
                return FAIL;
            }
            

            // copying from source to destination
            char buffer[1024];
            size_t bytesRead;
            while ((bytesRead = fread(buffer, 1, sizeof(buffer), source_file)) > 0) {
                fwrite(buffer, 1, bytesRead, destination_file);
            }

            fclose(source_file);
            fclose(destination_file);

            fprintf(commit_file, "%s %s\n", current->path, current->hash);

            current = current->next;
        }
    }


    fclose(commit_file);

    return SUCCESS;
}


int make_commit(struct repository* repo, struct author* author, struct commit* commit) {
    create_hash(commit->hash);
    strcpy(commit->parent_hash, repo->last_commit->hash);
    commit->author = author;
 
    // step1 looking at changes
    // creating the hashmap
    struct hash_map* changes_map = (struct hash_map*) malloc(sizeof(struct hash_map));
    init_hash_map(changes_map);

    char track_dir_path[MAX_PATH];
    strcpy(track_dir_path, repo->dir);
    strcat(track_dir_path, "/.vsync/tracked");

    char track_file_path[MAX_PATH];
    strcpy(track_file_path, track_dir_path);
    strcat(track_file_path, "/track");

    if ((access(track_file_path, F_OK) == 0) 
    && (populate_hashmap_from_file(changes_map, track_dir_path, track_file_path) == FAIL)){
        logger(ERROR_TAG, "[make_commit] Can not load changes to the map");
        return FAIL;
    }

    // step2 looking for the last commit
    char last_commit_dir_path[MAX_PATH];
    char last_commit_file_path[MAX_PATH];

    strcpy(last_commit_dir_path, repo->dir);
    strcat(last_commit_dir_path, "/.vsync/");
    strcat(last_commit_dir_path, repo->last_commit->hash);
    
    strcpy(last_commit_file_path, last_commit_dir_path);
    strcat(last_commit_file_path, "/commit");

    struct hash_map* last_commit_map = (struct hash_map*) malloc(sizeof(struct hash_map));
    init_hash_map(last_commit_map);

    if (populate_hashmap_from_file(last_commit_map, last_commit_dir_path, last_commit_file_path) == FAIL) {
        logger(ERROR_TAG, "[make_commit] can not load commit file to the map");
        return FAIL;
    }
    
    // update changes and last commit
    for (int i = 0; i < TABLE_SIZE; i++)
    {
        struct key_value* current = changes_map->table[i];
        
        while (current != NULL) {
            char* hash = get_hash_from_path(last_commit_map, current->path);

            if (hash != NULL)
            {
                delete_from_map(last_commit_map, current->path);
            }
            
            insert_map(last_commit_map, current->path, current->hash, current->raw_path);

            current = current->next;
        }
    }
    
    if (create_commit(repo, commit, last_commit_map) == FAIL)
    {
        logger(ERROR_TAG, "[make_commit] Can not create the commit");
        return FAIL;
    }
    
    clear_hash_map(last_commit_map);
    clear_hash_map(changes_map);

    repo->last_commit = commit;

    if (write_repository_file(repo) == FAIL)
    {
        logger(ERROR_TAG, "[make_commit] Can not update the repository file");
        return FAIL;
    }
    
    if (delete_tracked_dir(repo) == FAIL)
    {
        logger(ERROR_TAG, "[make_commit] Can not delete old tracked files");
        return FAIL;
    }
    
    return SUCCESS;
}

int delete_tracked_dir(struct repository* repo) {
    char tracked_dir_path[MAX_PATH];
    strcpy(tracked_dir_path, repo->dir);
    strcat(tracked_dir_path, "/.vsync/tracked");

    DIR* tracked_dir = opendir(tracked_dir_path);
    if (tracked_dir == NULL) return SUCCESS;
    
    struct dirent* entry;

    while((entry = readdir(tracked_dir)) != NULL) {
        if (strcmp(entry->d_name, ".") != 0
            && strcmp(entry->d_name, "..") != 0) {
            char file_path[MAX_PATH];
            strcpy(file_path, tracked_dir_path);
            strcat(file_path, "/");
            strcat(file_path, entry->d_name);

            if (remove(file_path) != 0) {
                logger(ERROR_TAG, "[delete_tracked_dir]");
                closedir(tracked_dir);
                return FAIL;
            }
        }
    }   

    closedir(tracked_dir);

    if (rmdir(tracked_dir_path) != 0) {
        logger(ERROR_TAG, "[delete_tracked_dir]");
        return FAIL;
    }
    
    return SUCCESS;
}

int rollback(struct repository* repo, const char* commit_hash) {
    // step1: check if the dir exists
    char commit_dir_path[MAX_PATH];
    strcpy(commit_dir_path, repo->dir);
    strcat(commit_dir_path, "/.vsync/");
    strcat(commit_dir_path, commit_hash);

    DIR* commit_dir = opendir(commit_dir_path);

    if (commit_dir == NULL) {
        logger(ERROR_TAG, "[rollback] Commit not found");
        return FAIL;
    }
    closedir(commit_dir);

    // step2: check if the commit file is exists then populate a hashmap from it
    struct hash_map* commit_map = (struct hash_map*)malloc(sizeof(struct hash_map));
    init_hash_map(commit_map);
    
    char commit_file_path[MAX_PATH];
    strcpy(commit_file_path, commit_dir_path);
    strcat(commit_file_path, "/commit");
    if (access(commit_file_path, F_OK) != 0) {
        logger(ERROR_TAG, "[rollback] Can not found the commit file");
        return FAIL;
    }

    if (populate_hashmap_from_file(commit_map, commit_dir_path, commit_file_path) == FAIL) {
        logger(ERROR_TAG, "[rollback] Can not populate the hashmap from the file");
        return FAIL;    
    }
    
    // step3: delete all the files and directories
    if (reset_repo_dir(repo->dir, repo->dir) == FAIL) {
        logger(ERROR_TAG, "[rollback] Can not delete files from the repository");
        return FAIL; 
    }
    
    // step4: create from the map the files and start copying them
    if (make_rollback_commit(commit_map) == FAIL) {
        logger(ERROR_TAG, "[rollback] Can not rollback files from the commit");
        return FAIL;     
    }

    // step5: update the repository file
    struct commit* commit = NULL;
    if ((commit = load_commit(repo, commit_hash)) == NULL) {
        logger(ERROR_TAG, "[rollback] Can not load the commit");
        return FAIL; 
    }
    
    free(repo->last_commit);
    repo->last_commit = commit;

    if (write_repository_file(repo) == FAIL) {
        logger(ERROR_TAG, "[rollback] Can not update the repository file");
        return FAIL;
    }
    
    return SUCCESS;
}

int reset_repo_dir(const char* path, const char* root_path) {
    DIR* dir = opendir(path);

    if (dir != NULL) {
        struct dirent* entry;

        while ((entry = readdir(dir)) != NULL) {
            if (strcmp(entry->d_name, ".") != 0
                && strcmp(entry->d_name, "..") != 0
                && strcmp(entry->d_name, ".vsync") != 0) {
                char full_path[PATH_MAX];
                snprintf(full_path, sizeof(full_path), "%s/%s", path, entry->d_name);

                if (entry->d_type == DT_DIR) {
                    // Recursively delete subdirectories
                    if (reset_repo_dir(full_path, root_path) == FAIL) {
                        closedir(dir);
                        return FAIL;
                    }
                } else {
                    // Delete regular files
                    if (remove(full_path) != 0) {
                        closedir(dir);
                        return FAIL;
                    }
                }
            }
        }

        closedir(dir);

        // Remove the empty directory itself, but not the root directory
        if (strcmp(path, root_path) != 0 && rmdir(path) != 0) {
            return FAIL;
        }

        return SUCCESS;
    }

    return FAIL;
}

int make_rollback_commit(struct hash_map* map) {
    for (int i = 0; i < TABLE_SIZE; i++) {
        struct key_value* current = map->table[i];

        while (current != NULL) {
            if (create_directories(current->path) == FAIL) {
                logger(ERROR_TAG, "[make_rollback_commit] can not create directories for copying");
                return FAIL;
            }
            

            FILE* src_file = fopen(current->raw_path, "rb");
            if (src_file == NULL) {
                logger(ERROR_TAG, "[make_rollback_commit] can not open the raw file");
                return FAIL;
            }

            if (create_directories(current->path) == FAIL) {
                logger(ERROR_TAG, "[make_rollback_commit] can not creating dirs for dest file");
                fclose(src_file);
                return FAIL;    
            }
            
            FILE* dest_file = fopen(current->path, "wb");
            if (dest_file == NULL) {
                logger(ERROR_TAG, "[make_rollback_commit] can not open the destination file");
                fclose(src_file);
                return FAIL;            
            }
            
            // copying from source to destination
            char buffer[1024];
            size_t bytesRead;
            while ((bytesRead = fread(buffer, 1, sizeof(buffer), src_file)) > 0) {
                fwrite(buffer, 1, bytesRead, dest_file);
            }

            fclose(src_file);
            fclose(dest_file);
            current = current->next;
        }
    }

    return SUCCESS;
}


int create_directories(const char* path) {
    if (path == NULL || strcmp(path, "") == 0) {
        return SUCCESS;  // Empty path, nothing to create
    }

    char* path_copy = strdup(path);
    if (path_copy == NULL) {
        return FAIL;  // Memory allocation failure
    }

    char* last_slash = strrchr(path_copy, '/');
    if (last_slash != NULL) {
        // Exclude the file part from the path
        *last_slash = '\0';
        if (strcmp(path_copy, "") == 0) return SUCCESS;
        // Recursively create parent directories
        int parent_result = create_directories(path_copy);
        if (parent_result != 0) {
            free(path_copy);
            return parent_result;  // Propagate failure from parent directory creation
        }

        // Check if the directory already exists
        struct stat st;
        if (stat(path_copy, &st) != 0) {
            // Directory does not exist, create it

            if (mkdir(path_copy, 0777) != 0) {
                free(path_copy);
                return FAIL;  // Failed to create directory
            }
        }
    }

    free(path_copy);
    return SUCCESS;  // Success
}



struct commit* load_commit(struct repository* repo, const char* hash) {

    char commit_file_path[MAX_PATH];
    strcpy(commit_file_path, repo->dir);
    strcat(commit_file_path, "/.vsync/");
    strcat(commit_file_path, hash);
    strcat(commit_file_path, "/commit");

    FILE* commit_file = fopen(commit_file_path, "r");

    if (commit_file == NULL) {
        logger(ERROR_TAG, "[load_commit] Commit no found");
        return NULL;
    }

    struct commit* commit = (struct commit*) malloc(sizeof(struct commit));
    strcpy(commit->hash, hash);

    struct author* author = (struct author*) malloc(sizeof(struct author));

    char line[MAX_PATH];
    fgets(line, MAX_PATH, commit_file);
    line[strlen(line) - 1] = 0;

    strcpy(commit->parent_hash, line);

    fgets(line, MAX_PATH, commit_file);
    line[strlen(line) - 1] = 0;

    char* spacePos = strchr(line, ' ');
    if (spacePos == NULL) {
        logger(ERROR_TAG, "[load_commit] Author of commit not found");
        free(commit);
        free(author);
        fclose(commit_file);
        return NULL;
    }
    *spacePos = 0;

    strcpy(author->username, line);
    strcpy(author->mail, spacePos + 1);


    commit->author = author;
    

    return commit;
}
