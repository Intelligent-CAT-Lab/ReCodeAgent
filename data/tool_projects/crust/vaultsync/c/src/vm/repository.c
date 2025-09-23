#include "../../include/vm/repository.h"
#include "../../include/vm/common.h"
#include "../../include/vm/commit.h"
#include "../../include/logging/logger.h"
#include "../../include/vm/author.h"
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <sys/stat.h>
#include <string.h>
#include <dirent.h>

int init_repo(struct repository* repo, struct author* author) {

    // TODO: ensure the buffer size is appropriate (size = enough size + "/.vsync"), it can lead to many problems
    repo = (struct repository*) malloc(sizeof(struct repository));

    strcpy(repo->name, "foo");

    if (getcwd(repo->dir, MAX_PATH) == NULL) {
        logger(ERROR_TAG, "[init_repo] Can not get the current working directory");
        return FAIL;
    };
    
    repo->author = author;

    struct commit* first_commit = (struct commit*) malloc(sizeof(struct commit));
    strcpy(first_commit->hash, "0");
    stpcpy(first_commit->parent_hash, "-");
    first_commit->author = author;

    repo->last_commit = first_commit;
    


    char dotdir_path[MAX_PATH];
    strcpy(dotdir_path, repo->dir);
    strcat(dotdir_path, "/.vsync");

    // TODO: check the existence of an old setup
    if (mkdir(dotdir_path, 0777) != 0) {
        logger(ERROR_TAG, "[init_repo] Can not create .vsync dir");
        return FAIL;
    }
    
    if (write_repository_file(repo) == FAIL)
    {
        logger(ERROR_TAG, "[init_repo] Can not create repository file");
        return FAIL;
    }
    
    // make the first commit
    struct hash_map* map = (struct hash_map*)malloc(sizeof(struct hash_map));
    init_hash_map(map);

    // TODO: refactor this (merge the code that copies files and the info commit file)
    // in
    if (make_init_map_repo(repo, map, repo->dir) == FAIL) {
        logger(ERROR_TAG, "[init_repo] can not create the hashmap for the first commit");
        return FAIL;
    }
    
    if (create_commit(repo, first_commit, map) == FAIL)
    {
        logger(ERROR_TAG, "[init_repo] can not create the first commit");
        return FAIL;
    }
    
    // free resources
    clear_hash_map(map);
    return SUCCESS;
}


int write_repository_file(struct repository* repo) {
    char repo_file_path[MAX_PATH];
    strcpy(repo_file_path, repo->dir);
    strcat(repo_file_path, "/.vsync/repository");

    FILE* repo_file = fopen(repo_file_path, "w");
    if (repo_file == NULL)
    {
        logger(ERROR_TAG, " [write_repository_file] Can not open the repository file");
        return FAIL;
    }

    fprintf(repo_file, "%s\n", repo->dir);
    fprintf(repo_file, "%s\n", repo->last_commit->hash);
    fprintf(repo_file, "%s\n", repo->name);
    fprintf(repo_file, "%s %s", repo->author->username, repo->author->mail);

    fclose(repo_file);

    return SUCCESS;
}

// TODO: search recursively for .vsync
struct repository* load_repository() {
    struct repository* repo = (struct repository*) malloc(sizeof(struct repository));
    struct commit* last_commit = (struct commit*) malloc(sizeof(struct commit));
    struct author* author = (struct author*) malloc(sizeof(struct author));

    // bind last commit with repo
    repo->last_commit = last_commit;
    repo->author = author;

    if (getcwd(repo->dir, MAX_PATH) == NULL) {
        logger(ERROR_TAG, "[load_repository] Can not get the current working directory");
        return NULL;
    }
    
    char repository_file_path[MAX_PATH];
    strcpy(repository_file_path, repo->dir);
    strcat(repository_file_path, "/.vsync/repository");

    FILE* repository_file = fopen(repository_file_path, "r");
    if (repository_file == NULL) {
        logger(ERROR_TAG, "[load_repository] Can not load the repository file");
        free(author);
        free(last_commit);
        free(repo);
        return NULL;
    }
    
    char line[MAX_PATH];

    // read the dir of repo
    fgets(line, MAX_PATH, repository_file);

    // reading the last commit
    fgets(line, MAX_PATH, repository_file);
    line[strlen(line) - 1] = 0;

    // getting the commit info
    char last_commit_path[MAX_PATH];
    strcpy(last_commit_path, repo->dir);
    strcat(last_commit_path, "/.vsync/");
    strcat(last_commit_path, line);
    strcat(last_commit_path, "/commit");



    FILE* last_commit_file = fopen(last_commit_path, "r");
    if (last_commit_file == NULL) {
        logger(ERROR_TAG, "[load_repository] Can not load the last commit");
        fclose(repository_file);
        free(author);
        free(last_commit);
        free(repo);
        return NULL;
    }
    
    strcpy(last_commit->hash, line);

    // reading the name of repo
    fgets(line, MAX_REP_NAME, repository_file);
    line[strlen(line) - 1] = 0;
    strcpy(repo->name, line);

    // reading the parent commit from the commit file
    fgets(line, MAX_PATH, last_commit_file);
    line[strlen(line) - 1] = 0;

    strcpy(last_commit->parent_hash, line);

    struct author* last_commit_author = (struct author*) malloc(sizeof(struct author));
    // reading the author's info of last commit 
    fgets(line, MAX_MAIL + 1 + MAX_UNAME, last_commit_file);
    line[strlen(line) - 1] = 0;


    char* spacePos = strchr(line, ' ');
    if (spacePos == NULL) {
        logger(ERROR_TAG, "[load_repository] Can not load the author of last commit");
        fclose(repository_file);
        fclose(last_commit_file);
        free(last_commit_author);
        free(author);
        free(last_commit);
        free(repo);
        return NULL;
    }

    *spacePos = 0;

    strcpy(last_commit_author->username, line);
    strcpy(last_commit_author->mail, spacePos + 1);
    last_commit->author = last_commit_author;

    // getting the author of the repository
    fgets(line, MAX_MAIL + 1 + MAX_UNAME, repository_file);
    line[strlen(line) - 1] = 0;

    spacePos = strchr(line, ' ');
    if (spacePos == NULL) {
        logger(ERROR_TAG, "[load_repository] Can not load the author of repository");
        fclose(repository_file);
        fclose(last_commit_file);
        free(last_commit_author);
        free(author);
        free(last_commit);
        free(repo);
        return NULL;
    }

    *spacePos = 0;
    strcpy(author->username, line);
    strcpy(author->mail, spacePos + 1);

    fclose(repository_file);
    fclose(last_commit_file);    
    
    return repo;
}

struct author* load_author() {
    struct author* author  = (struct author*) malloc(sizeof(struct author));
    const char *vsync_config_path = getenv("VSYNC_CONFIG_PATH");

    FILE* vsync_config_file ;
    char line[MAX_UNAME + 1 + MAX_MAIL];

    if (vsync_config_path != NULL) {
        vsync_config_file = fopen(vsync_config_path, "r");
        fgets(line, MAX_UNAME + 1 + MAX_MAIL, vsync_config_file);

        char* spacePos = strchr(line, ' ');

        if (spacePos != NULL) {
            *spacePos = 0;

            strcpy(author->username, line);
            strcpy(author->mail, spacePos + 1);

            author->mail[strlen(author->mail) - 1] = 0; 
            
            return author;
        }
    }
    
    free(author);
    logger(ERROR_TAG, "[load_author] The global config file does not existed");
    return NULL;
}