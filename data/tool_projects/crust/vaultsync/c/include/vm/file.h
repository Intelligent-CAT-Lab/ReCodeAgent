#ifndef FILE_H
#define FILE_H

#define MAX_PATH 1024

/**
 * LinkedList of files
*/
struct file_node {
    char path[MAX_PATH];
    struct file_node* next;
};


struct file_node* addFile(struct file_node* head, const char* path);
struct file_node* removeFileByPath(struct file_node* head, const char* path);
struct file_node* findByPath(struct file_node* head, const char* path);
#endif
