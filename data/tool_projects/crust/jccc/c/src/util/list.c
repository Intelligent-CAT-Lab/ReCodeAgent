#include "list.h"

#include <stdlib.h> // memory

#include <util/out.h>

typedef struct ListBlock {
    void* * array;
    int size, full;
    struct ListBlock* next;
} ListBlock;

struct List {
    ListBlock *head, *tail;
    int blocksize;
};

List* create_list(int blocksize) {
    List* l = malloc(sizeof(List));
    if (!l) {
        PRINT_ERROR("internal: failed to allocate memory");
        return NULL;
    }
    l->blocksize = blocksize;
    l->head = l->tail = NULL;
    return l;
}

int destroy_list(List* l) {
    // Free all blocks.
    ListBlock *lb = l->head, *next;
    while (lb) {
        next = lb->next;
        free(lb->array);
        free(lb);
        lb = next;
    }
    free(l);
    return 0;
}

// Internal -- given a list and an index, produce the list block which that
// index references, and the index with respect to said list block.
static int lfind_index(List* l, ListBlock** lb, int* i) {
    *lb = l->head;
    if (*i < 0) {
        PRINT_ERROR("internal: list index was negative (%d)", *i);
        return -1;
    }
    // We see if the index is in range of the current block length. If not,
    // repeat with the next block.
    while (*i >= (*lb)->size) {
        if (lb) {
            *lb = (*lb)->next;
            *i -= (*lb)->size;
        } else {
            PRINT_ERROR("internal: list index %d out of bounds", *i);
            return -1;
        }
    }
    return 0;
}

void* lget_element(List* l, int index) {
    int i = index;
    ListBlock* lb;
    lfind_index(l, &lb, &i);
    // Now the index is in range and we are at the list block we wish to access.
    if (i >= lb->full) {
        PRINT_ERROR("internal: list index %d out of bounds", index);
        return NULL;
    }
    return lb->array[i];
}

int lset_element(List* l, int index, void* value) {
    int i = index, ret;
    ListBlock* lb;
    if ((ret = lfind_index(l, &lb, &i))) return ret;
    // Now the index is in range and we are at the list block we wish to access.
    if (i >= lb->full) {
        PRINT_ERROR("internal: list index %d out of bounds", index);
        return -1;
    }
    lb->array[i] = value;
    return 0;
}

// Helper function -- make a new array.
static ListBlock* new_block(List* l) {
    ListBlock* lb = malloc(sizeof(ListBlock));
    if (lb == NULL) return NULL;
    lb->array = malloc(l->blocksize * sizeof(void *));
    if (lb->array == NULL) {
        free(lb);
        return NULL;
    }
    lb->full = 0;
    lb->size = l->blocksize;
    lb->next = NULL;
    return lb;
}

// Add an element.
// We do this by finding the location of the next free slot. If the last array
// is full then we allocate a new one.
int ladd_element(List* l, void* element) {
    // First of all, if the list is empty, make a new array.
    if (l->head == NULL) {
        l->tail = new_block(l);
        l->head = l->tail;
    }
    // The last block is always where the free slots are.
    ListBlock* lb = l->tail;
    if (lb->full < lb->size) {
        lb->array[lb->full++] = element;
    } else {
        // Make a new block and put the element in there.
        l->tail = new_block(l);
        lb->next = l->tail;
        lb = lb->next;
        lb->array[lb->full++] = element;
    }
    return 0;
}

int literate(List* l, int (*fn)(void*)) {
    ListBlock* lb = l->head;
    int acc = 0;
    while (lb != NULL) {
        for (int i = 0; i < lb->full; ++i) {
            acc += fn(lb->array[i]);
        }
        lb = lb->next;
    }
    return acc;
}
