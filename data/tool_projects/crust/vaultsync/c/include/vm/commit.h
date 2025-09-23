#ifndef COMMIT_H
#define COMMIT_H
#include "author.h"
#include "file.h"
#include "hashmap.h"
#include "repository.h"

#define HASH_LEN 2 * SHA256_DIGEST_LENGTH + 1
#define TRACKED_DIR "tracked"

/**
 * Commit structure 
*/
struct commit {
    char hash[HASH_LEN];
    struct author* author;
    char parent_hash[HASH_LEN];
};

// TODO: show differences, commits history... and all related stuffs
/**
 * create a hashmap contains all the current files for the initialization
*/
int make_init_map_repo(struct repository* repo, struct hash_map* map, const char* path);

/**
 * Keep tracking files for the next commit
*/
int add_changes(struct repository *repo, struct file_node* files);

/**
 * create a hashmap contains the tracked files
*/
int make_changes(struct repository* repo, struct hash_map* map);

/**
 * creating a commit
 * you give a repository and commit object and a hashmap and it create the 
 * the commit dir and copying all data depending on the hashmap
*/
int create_commit(struct repository* repo, struct commit* commit, struct hash_map* map);

/**
 * Give a full repository object and a an empty commit and it will create the commit
*/
int make_commit(struct repository* repo, struct author* author, struct commit* commit);

/**
 * Delete the tracked directory
*/
int delete_tracked_dir(struct repository* repo);

/**
 * Rollback to a commit
*/
int rollback(struct repository* repo, const char* commit_hash);

/**
 * Delete all files and repositories
*/
int reset_repo_dir(const char* path, const char* root_path);

/**
 * create the files contains in hashmap
*/
int make_rollback_commit(struct hash_map* map);

/**
 * Load a commit
*/
struct commit* load_commit(struct repository* repo, const char* hash);
#endif
