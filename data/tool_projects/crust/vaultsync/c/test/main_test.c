#include "unity/unity.h"
#include "../src/logging/logger.c"  // Assuming you have a file module to test

#include "../include/vm/repository.h"
#include "../include/vm/file.h"
#include "../include/vm/common.h"
#include "../include/vm/commit.h"
#include "../include/vm/author.h"
#include "../include/vm/file.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>



void setUp() {

}

void tearDown() {
    
}

void init_repository() {
    struct author* author = (struct author*) malloc(sizeof(struct author));
    strcpy(author->mail, "john@example.com");
    strcpy(author->username, "john");
    // Creating new commit
    struct repository* repo = NULL;
    
    if (init_repo(repo, author) == SUCCESS) {
        printf("Init done\n");
    }
}

void adding_changes() {
    struct author* author = (struct author*) malloc(sizeof(struct author));
    strcpy(author->mail, "john@example.com");
    strcpy(author->username, "john");
    // Creating new commit
    struct repository* repo = NULL;

    // adding changes
    struct file_node* file = NULL;
    file = addFile(file, "/home/elhalili/workspace/VaultSync/build/mypr/hello.txt");

    repo = (struct repository*) malloc(sizeof(struct repository));
    repo->author = author;
    strcpy(repo->dir, "/home/elhalili/workspace/VaultSync/build/mypr");
    strcpy(repo->name, "foo");


    if (add_changes(repo, file) == SUCCESS) {
        printf("Add changes done\n");
    }
}


void made_commit() {
    struct author* author = (struct author*) malloc(sizeof(struct author));
    strcpy(author->mail, "john@example.com");
    strcpy(author->username, "john");
    // Creating new commit
    struct repository* repo = (struct repository*) malloc(sizeof(struct repository));
    repo->author = author;
    strcpy(repo->dir, "/home/elhalili/workspace/VaultSync/build/mypr");
    strcpy(repo->name, "foo");

    struct commit* commit = (struct commit*) malloc(sizeof(struct commit));
    commit->author = author;
    strcpy(commit->hash, "0");
    strcpy(commit->parent_hash, "-");

    repo->last_commit = commit;


    struct commit* new_commit = (struct commit*) malloc(sizeof(struct commit));
    if (make_commit(repo, author, new_commit) == SUCCESS) {
        printf("Commit done\n");
    }
}

int main(void) {
    UNITY_BEGIN();

    adding_changes();
    made_commit();

    return UNITY_END();
}
