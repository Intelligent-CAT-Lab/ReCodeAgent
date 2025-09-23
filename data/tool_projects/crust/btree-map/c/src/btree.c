#include "btree.h"
#include "string.h"

#define WEAK __attribute__((weak))

struct node {
    uint32_t key_hash;
    uint8_t p_key[BTREE_KEY_SIZE];
    size_t key_len;
    value_t value;

    node_t* child_left;
    node_t* child_right;
};

struct btree {
    node_t* node;
};


void* WEAK btree_malloc(size_t size) {
    void* ptr = malloc(size);
    memset(ptr, 0, size);
    return ptr;
}

void WEAK btree_free(void* ptr) {
    free (ptr);

    ptr = NULL;
}

size_t min_size(size_t a, size_t b) {
    if (a < b) {
        return a;
    }

    return b;
}

uint32_t WEAK calc_key_hash(void* key, size_t key_len) {
    uint32_t key_sum = 0;
    uint8_t* byte_key = (uint8_t*) key;
    for (size_t i=0; i<key_len; i++) {
        key_sum = key_sum % UINT32_MAX;
        key_sum += (byte_key[i] * (i + 1)) % UINT32_MAX;
    }

    return key_sum;
}

btree_t* new_btree() {
    btree_t* bt =  btree_malloc(sizeof(btree_t));
    bt->node = NULL;

    return bt;
};

node_t* new_node(void* key, size_t key_len, void* value, size_t value_len) {
    node_t* node = btree_malloc(sizeof(node_t));

    node->key_len = min_size(BTREE_KEY_SIZE, key_len);
    memcpy(node->p_key, key, node->key_len);

    node->value.value = btree_malloc(value_len);
    node->value.len = value_len;

    memcpy(node->value.value, value, node->value.len);

    node->key_hash = calc_key_hash(node->p_key, node->key_len);

    return node;
}

void free_node(node_t* self) {
    if (self == NULL) {
        return;
    }

    btree_free(self->value.value);

    free_node(self->child_left);
    free_node(self->child_right);

    btree_free(self);
}

void add_node(node_t* self, node_t* n_node) {
    if (n_node->key_hash > self->key_hash) {
        if (self->child_right == NULL) {
            self->child_right = n_node;
            return;
        }

        add_node(self->child_right, n_node);
        return;
    }

    if (n_node->key_hash == self->key_hash) {
        if (memcmp(self->p_key, n_node->p_key, n_node->key_len) == 0) {
            self->value.len = n_node->value.len;
            memcpy(self->value.value, n_node->value.value, n_node->value.len);

            free_node(n_node);
            return;
        }
    }
    
    if (self->child_left == NULL) {
        self->child_left = n_node;
        return;
    }

    add_node(self->child_left, n_node);
    return;
}

value_t* find_value(node_t* self, uint32_t key_hash, void* key, size_t key_len) {
    key_len = min_size(BTREE_KEY_SIZE, key_len);
    if (self->key_hash == key_hash) {
        if(memcmp(self->p_key, key, key_len) == 0) {
            return &self->value;
        }
    }

    if (key_hash > self->key_hash) {
        if (self->child_right == NULL) {
            return NULL;
        }

        return find_value(self->child_right, key_hash, key, key_len);
    }

    if(self->child_left == NULL) {
        return NULL;
    }

    return find_value(self->child_left, key_hash, key, key_len);
}

void list_node_entries(node_t* self, entry_list_t* list) {
    if (self == NULL) {
        return;
    }

    list_node_entries(self->child_left, list);

    if (list->len >= list->cap) {
        return;
    }

    entry_t* entry = btree_malloc(sizeof(entry_t));
    entry->key.key = btree_malloc(self->key_len);
    entry->key.len = self->key_len;
    memcpy(entry->key.key, self->p_key, self->key_len);

    entry->value.value = btree_malloc(self->value.len);
    entry->value.len = self->value.len;
    memcpy(entry->value.value, self->value.value, self->value.len);

    list->entries[list->len] = *entry;
    list->len += 1;

    list_node_entries(self->child_right, list);
} 

size_t get_node_count(node_t* self) {
    if (self == NULL) {
        return 0;
    }

    return 1 + get_node_count(self->child_left) + get_node_count(self->child_right);
}

void add_entry(btree_t* self,
 void* key, size_t key_len, 
 void* value, size_t value_len) {

    node_t* n_node = new_node(key, key_len, value, value_len);
    if (self->node == NULL) {
        self->node = n_node;
        return;
    }
    
    add_node(self->node, n_node);
}

value_t* find_entry(btree_t* self, void* key, size_t key_len) {
    if(self->node == NULL) {
        return NULL;
    }

    key_len = min_size(BTREE_KEY_SIZE, key_len);
    uint32_t key_hash = calc_key_hash(key, key_len);
    
    return find_value(self->node, key_hash, key, key_len);
}

node_t* delete_node(node_t* root, uint32_t key_hash, void* key, size_t key_len) {
    if (root == NULL) {
        return NULL;
    }

    key_len = min_size(BTREE_KEY_SIZE, key_len);

    // Navigate down the tree to find the node to delete
    if (key_hash < root->key_hash) {
        root->child_left = delete_node(root->child_left, key_hash, key, key_len);
    } else if (key_hash > root->key_hash) {
        root->child_right = delete_node(root->child_right, key_hash, key, key_len);
    } else {
        // Found the node to delete
        if (memcmp(root->p_key, key, key_len) == 0) {
            // Node with only one child or no child
            if (root->child_left == NULL) {
                node_t* temp = root->child_right;
                btree_free(root);
                return temp;
            } else if (root->child_right == NULL) {
                node_t* temp = root->child_left;
                btree_free(root);
                return temp;
            }

            // Node with two children
            node_t* temp = root->child_right;
            while (temp && temp->child_left != NULL) {
                temp = temp->child_left;
            }

            // Copy the inorder successor's content to this node
            root->key_hash = temp->key_hash;
            memcpy(root->p_key, temp->p_key, temp->key_len);
            root->key_len = temp->key_len;
            memcpy(root->value.value, temp->value.value, temp->value.len);
            root->value.len = temp->value.len;

            // Delete the inorder successor
            root->child_right = delete_node(root->child_right, temp->key_hash, temp->p_key, temp->key_len);
        }
    }

    return root;
}

void remove_entry(btree_t* self, void* key, size_t key_len) {
    if (self == NULL) {
        return;
    }

    key_len = min_size(BTREE_KEY_SIZE, key_len);
    uint32_t key_hash = calc_key_hash(key, key_len);

    self->node = delete_node(self->node, key_hash, key, key_len);
}

entry_list_t* list_entries(btree_t* self) {
    if (self == NULL) {
        return NULL;
    }

    entry_list_t* list = btree_malloc(sizeof(entry_list_t));
    list->len = 0;
    list->cap = get_entry_count(self);
    list->entries = btree_malloc(sizeof(entry_t) * list->cap);

    list_node_entries(self->node, list);
    return list;
}

size_t get_entry_count(btree_t* self) {
    if (self == NULL) {
        return 0;
    }

    return get_node_count(self->node);
}

void free_tree(btree_t** self) {
    free_node((*self)->node);

    btree_free(*self);
    (*self) = NULL;
    return;
}

void free_entry_list(entry_list_t** list) {
    for (size_t i=0; i<(*list)->len; i++) {
        btree_free((*list)->entries[i].key.key);
        btree_free((*list)->entries[i].value.value);
    }

    btree_free((*list)->entries);
    btree_free(*list);

    (*list) = NULL;
}



