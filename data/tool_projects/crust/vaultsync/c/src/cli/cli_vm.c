#include "../../include/cli/cli_vm.h"
#include "../../include/logging/logger.h"
#include "../../include/vm/author.h"
#include "../../include/vm/repository.h"
#include "../../include/vm/common.h"
#include "../../include/vm/commit.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>


void init_repository() {
    struct author* author = NULL;
    if ((author = load_author()) == NULL) {
        logger(ERROR_TAG, "Can not getting the author, check the default config at ~/.vsync_rc");
        return;
    }
    
    struct repository* repo = NULL;
    if (init_repo(repo, author) == SUCCESS) {
        logger(INFO_TAG, "Initialization done");
    }
}

void track_changes(int n, char const *paths[]) {
    int i = 2;
    struct file_node* files = NULL;
    while (i < n) {
        files = addFile(files, paths[i++]);
    }

    // TODO: control the entered file before consume them
    struct repository* repo = NULL;
    if ((repo = load_repository()) == NULL) {
        logger(ERROR_TAG, "Can not getting the repo info");
        return;            
    }

    if (add_changes(repo, files) == SUCCESS) {
        logger(INFO_TAG, "Files has been tracked successfully");
    }
}

void commit_changes() {
    struct commit* new_commit = (struct commit*) malloc(sizeof(struct commit));
    struct repository* repo = NULL;
    struct author* author;

    if ((author = load_author()) == NULL) {
        logger(ERROR_TAG, "Can not getting the author, check the default config at ~/.vsync_rc");
        return;
    }

    if ((repo = load_repository()) == NULL) {
        logger(ERROR_TAG, "Can not getting the repo info");
        return;            
    }

    if (make_commit(repo, author, new_commit) == SUCCESS) {
        logger(INFO_TAG, "The commit has been done successfully");
    } 
}

void rollback_changes(const char* hash) {
    struct repository* repo = NULL;
    if ((repo = load_repository()) == NULL) {
        logger(ERROR_TAG, "Can not getting the repo info");
        return;            
    }

    if (rollback(repo, hash) == SUCCESS) {
        logger(INFO_TAG, "The rollback has been successfully done");
    }
}