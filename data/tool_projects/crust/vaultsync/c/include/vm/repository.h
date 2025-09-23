#ifndef REPOSITORY_H
#define REPOSITORY_H

#define CONFIG_FILE "/config"
#define REPOSITORY_FILE "/repository"
#define LOG_FILE "/logs"
#define MAX_REPOSITORY_FILE_INIT 1024
#define MAX_CWD 4096
#define MAX_REP_NAME 80

#include "author.h"
#include "file.h"

// TODO : add remote related info
struct repository {
    char name[MAX_REP_NAME];
    struct author* author;
    char dir[MAX_PATH];
    struct commit* last_commit;
};

/**
 * responsable for doing the initilization stuffs
*/
int init_repo(struct repository* repo, struct author* author);

/**
 * show status of tracked and untracked files in a given moment
*/
int get_status(struct repository* repo);

/**
 * Write the repository file
*/
int write_repository_file(struct repository* repo);

/**
 * Load repository info
*/
struct repository* load_repository();

/**
 * Load an author
*/
struct author* load_author();
#endif
