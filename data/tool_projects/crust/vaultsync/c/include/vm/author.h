#ifndef AUTHOR_H
#define AUTHOR_H
#define MAX_UNAME 50
#define MAX_MAIL 100

/**
 * Author of a repository or a commit
*/
struct author {
    char username[MAX_UNAME];
    char mail[MAX_MAIL];
};

#endif