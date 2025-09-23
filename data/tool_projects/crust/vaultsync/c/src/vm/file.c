#include "../../include/vm/file.h"
#include <stdlib.h>


struct file_node* addFile(struct file_node* head, const char* path) {
    struct file_node* newNode = (struct file_node*)malloc(sizeof(struct file_node));
    strcpy(newNode->path, path);
    newNode->next = head;
    return newNode;
}


struct file_node* removeFileByPath(struct file_node* head, const char* path) {
    struct file_node* current = head;
    struct file_node* previous = NULL;

    while (current != NULL) {
        if (strcmp(current->path, path) == 0) {
            if (previous == NULL) {
                head = current->next;
            } else {
                previous->next = current->next;
            }

            free(current);
            return head; // Return the updated head
        }

        previous = current;
        current = current->next;
    }

    return head;
}



struct file_node* findByPath(struct file_node* head, const char* path) {
    struct file_node* current = head;

    while (current != NULL) {
        if (strcmp(current->path, path) == 0) {
            return current; 
        }

        current = current->next;
    }

    return NULL;
}
