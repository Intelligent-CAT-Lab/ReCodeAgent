#ifndef CB_MERKLE_TREE_H_
#define CB_MERKLE_TREE_H_

#include <stdbool.h>
#include <string.h>
#include <stdint.h>

/* CBMT_NODE_I32 is for test purpose */
#ifdef CBMT_NODE_I32
#define CBMT_NODE_SIZE 4
#else
#define CBMT_NODE_SIZE 32
#endif

#define CBMT_ERROR_OVER_CAPACITY -1
#define CBMT_ERROR_QUEUE_EMPTY -2
#define CBMT_ERROR_PROOF_ROOT -3
#define CBMT_ERROR_BUILD_PROOF -4
#define CBMT_ERROR_INVALID_CAPACITY -5
#define CBMT_ERROR_VERIFY_FAILED -6
/* This happend only when have bug */
#define CBMT_FATAL_BUILD_PROOF -99

#define cbmt_is_left(index)  (((index) & 1) == 1)
#define cbmt_parent(index)   ((index) == 0 ? 0 : ((index) - 1) >> 1)
#define cbmt_sibling(index)  ((index) == 0 ? 0 : ((index + 1) ^ 1) - 1)

typedef int (*cbmt_cmp_fn)(const void *, const void *);

void cbmt_universal_swap(void *left, void *right, size_t width) {
  uint8_t tmp[128];
  size_t length;
  while (width) {
    length = sizeof(tmp) < width ? sizeof(tmp) : width;
    memcpy(tmp, left, length);
    memcpy(left, right, length);
    memcpy(right, tmp, length);
    left += length;
    right += length;
    width -= length;
  }
}
void cbmt_simple_bubble_sort(void *base, size_t length, size_t width, cbmt_cmp_fn cmp) {
  for (size_t i = 0; i < length - 1; i++) {
    for (size_t j = i + 1; j < length; j++) {
      void *left = base + (i * width);
      void *right = base + (j * width);
      if (cmp(left, right) > 0) {
        cbmt_universal_swap(left, right, width);
      }
    }
  }
}
int cbmt_uint32_reverse_cmp(const void *void_left, const void *void_right) {
  const uint32_t left = *((const uint32_t *)void_left);
  const uint32_t right = *((const uint32_t *)void_right);
  /* reverse order */
  return right - left;
}

typedef struct {
  void *data;
  size_t capacity;
} cbmt_buffer;

typedef struct {
  uint8_t bytes[CBMT_NODE_SIZE];
} cbmt_node;

typedef struct {
  uint32_t *values;
  size_t length;
  size_t capacity;
} cbmt_indices;

typedef struct {
  cbmt_indices indices;
  cbmt_node *lemmas;
  size_t lemmas_length;
} cbmt_proof;

typedef struct {
  cbmt_node *nodes;
  /* nodes length */
  size_t length;
  size_t capacity;
} cbmt_tree;

typedef struct {
  cbmt_node *nodes;
  size_t length;
  /* size_t capacity; */
} cbmt_leaves;


typedef struct {
  cbmt_buffer buffer;
  size_t width;
  size_t length;
  size_t capacity;
  /* tail index */
  size_t tail;
  /* head index */
  size_t head;
} cbmt_queue;

typedef struct {
  uint32_t index;
  cbmt_node node;
} cbmt_node_pair;


typedef cbmt_node (*cbmt_node_merge_fn)(void *merge_ctx, cbmt_node *left, cbmt_node *right);

void cbmt_buffer_init(cbmt_buffer *buffer, void *data, size_t capacity) {
  buffer->data = data;
  buffer->capacity = capacity;
}
void cbmt_leaves_init(cbmt_leaves *leaves, cbmt_node *nodes, size_t length) {
  leaves->length = length;
  leaves->nodes = nodes;
}
void cbmt_indices_init(cbmt_indices *indices, uint32_t *values, size_t length, size_t capacity) {
  indices->values = values;
  indices->length = length;
  indices->capacity = capacity;
}

int cbmt_queue_init(cbmt_queue *queue,
                   cbmt_buffer buffer,
                   size_t width,
                   size_t capacity) {
  if (capacity * width > buffer.capacity) {
    return CBMT_ERROR_OVER_CAPACITY;
  }
  if (buffer.capacity % width != 0) {
    return CBMT_ERROR_INVALID_CAPACITY;
  }
  queue->buffer = buffer;
  queue->capacity = capacity;
  queue->width = width;
  queue->length = 0;
  queue->head = 0;
  queue->tail = 0;
  return 0;
}

int cbmt_queue_push_back(cbmt_queue *queue, void *item) {
  if (queue->length >= queue->capacity) {
    return CBMT_ERROR_OVER_CAPACITY;
  }
  void *head = queue->buffer.data + (queue->head * queue->width);
  memcpy(head, item, queue->width);
  queue->head = (queue->head + 1) % queue->capacity;
  queue->length += 1;
  return 0;
}

int cbmt_queue_push_front(cbmt_queue *queue, void *item) {
  if (queue->length >= queue->capacity) {
    return CBMT_ERROR_OVER_CAPACITY;
  }
  queue->tail = (queue->tail + queue->capacity - 1) % queue -> capacity;
  void *tail = queue->buffer.data + (queue->tail * queue->width);
  memcpy(tail, item, queue->width);
  queue->length += 1;
  return 0;
}

int cbmt_queue_pop_front(cbmt_queue *queue, void *item) {
  if (queue->length == 0) {
    return CBMT_ERROR_QUEUE_EMPTY;
  }
  void *current_tail = queue->buffer.data + (queue->tail * queue->width);
  memcpy(item, current_tail, queue->width);
  queue->tail = (queue->tail + 1) % queue->capacity;
  queue->length -= 1;
  return 0;
}
void* cbmt_queue_front(cbmt_queue *queue) {
  if (queue->length == 0) {
    return NULL;
  }
  return queue->buffer.data + (queue->tail * queue->width);
}

void cbmt_node_copy(cbmt_node *dest, cbmt_node *src) {
  memcpy(dest->bytes, src->bytes, CBMT_NODE_SIZE);
}
int cbmt_node_cmp(const void *void_left, const void *void_right) {
  const cbmt_node *left = (const cbmt_node *)void_left;
  const cbmt_node *right = (const cbmt_node *)void_right;
#ifdef CBMT_NODE_I32
  int32_t left_value = *((int32_t *)left->bytes);
  int32_t right_value = *((int32_t *)right->bytes);
  return left_value - right_value;
#else
  for (int i = 0; i < CBMT_NODE_SIZE; i++) {
    int cmp_result = left->bytes[i] - right->bytes[i];
    if (cmp_result != 0) {
      return cmp_result;
    }
  }
  return 0;
#endif
}

int cbmt_node_pair_reverse_cmp(const void *void_left, const void *void_right) {
  const cbmt_node_pair *left = (const cbmt_node_pair *)void_left;
  const cbmt_node_pair *right = (const cbmt_node_pair *)void_right;
  /* reverse order */
  return right->index - left->index;
}


int cbmt_tree_build_proof(cbmt_proof *proof,
                          cbmt_tree *tree,
                          cbmt_indices *leaf_indices,
                          /* for saving indices in proof */
                          cbmt_buffer indices_buffer,
                          /* for saving lemmas in proof */
                          cbmt_buffer lemmas_buffer) {
  if (leaf_indices->length * sizeof(uint32_t) > indices_buffer.capacity) {
    return CBMT_ERROR_OVER_CAPACITY;
  }
  if (tree->length == 0 || leaf_indices->length == 0) {
    return CBMT_ERROR_BUILD_PROOF;
  }
  int ret;
  const uint32_t leaves_count = (uint32_t)((tree->length >> 1) + 1);

  cbmt_queue queue;
  ret = cbmt_queue_init(&queue,
                        indices_buffer,
                        sizeof(uint32_t),
                        indices_buffer.capacity / sizeof(uint32_t));
  if (ret != 0) {
    return ret;
  }
  for (size_t i = 0; i < leaf_indices->length; i++) {
    uint32_t value = leaf_indices->values[i] + (leaves_count - 1);
    ret = cbmt_queue_push_back(&queue, &value);
    if (ret != 0) {
      return ret;
    }
  }
  cbmt_simple_bubble_sort(queue.buffer.data,
                          leaf_indices->length,
                          sizeof(uint32_t),
                          cbmt_uint32_reverse_cmp);
  uint32_t first_value = *((uint32_t *)cbmt_queue_front(&queue));
  if (first_value >= ((leaves_count << 1) - 1)) {
    return CBMT_ERROR_BUILD_PROOF;
  }

  proof->lemmas = (cbmt_node *)lemmas_buffer.data;
  proof->lemmas_length = 0;
  uint32_t index;
  while (queue.length > 0) {
    ret = cbmt_queue_pop_front(&queue, &index);
    if (ret != 0) {
      return ret;
    }
    if (index == 0) {
      if (queue.length != 0) {
        return CBMT_FATAL_BUILD_PROOF;
      }
      break;
    }

    uint32_t sibling = cbmt_sibling(index);
    uint32_t *front = (uint32_t *)cbmt_queue_front(&queue);
    if (front != NULL && *front == sibling) {
      uint32_t tmp;
      ret = cbmt_queue_pop_front(&queue, &tmp);
      if (ret != 0) {
        return ret;
      }
    } else {
      cbmt_node *dest_lemma = proof->lemmas + proof->lemmas_length;
      cbmt_node *src_lemma = tree->nodes + sibling;
      cbmt_node_copy(dest_lemma, src_lemma);
      proof->lemmas_length += 1;
    }

    uint32_t parent = cbmt_parent(index);
    if (parent != 0) {
      ret = cbmt_queue_push_back(&queue, &parent);
      if (ret != 0) {
        return ret;
      }
    }
  }

  cbmt_indices indices;
  indices.values = (uint32_t *)indices_buffer.data;
  indices.length = leaf_indices->length;
  indices.capacity = indices_buffer.capacity / sizeof(uint32_t);
  for (size_t i = 0; i < indices.length; i++) {
    indices.values[i] = leaf_indices->values[i] + (leaves_count - 1);
  }

  for (size_t i = 0; i < indices.length - 1; i++) {
    for (size_t j = i + 1; j < indices.length; j++) {
      uint32_t left_index = indices.values[i];
      uint32_t right_index = indices.values[j];
      int order = cbmt_node_cmp(tree->nodes + left_index, tree->nodes + right_index);
      if (order > 0) {
        indices.values[i] = right_index;
        indices.values[j] = left_index;
      }
    }
  }
  proof->indices = indices;
  return 0;
}

cbmt_node cbmt_tree_root(cbmt_tree *tree) {
  cbmt_node node;
  if (tree->length == 0) {
    memset(node.bytes, 0, CBMT_NODE_SIZE);
  } else {
    cbmt_node_copy(&node, tree->nodes);
  }
  return node;
}

int cbmt_proof_root(cbmt_proof *proof,
                    cbmt_node *root,
                    cbmt_leaves *leaves,
                    cbmt_node_merge_fn merge,
                    void *merge_ctx,
                    /* for saving nodes in cloned leaves */
                    cbmt_buffer nodes_buffer,
                    /* for saving (index, node) pairs */
                    cbmt_buffer pairs_buffer) {
  if (leaves->length * sizeof(cbmt_node) > nodes_buffer.capacity) {
    return CBMT_ERROR_OVER_CAPACITY;
  }
  if (leaves->length * sizeof(cbmt_node_pair) > pairs_buffer.capacity) {
    return CBMT_ERROR_OVER_CAPACITY;
  }
  if (leaves->length != proof->indices.length || leaves->length == 0) {
    return CBMT_ERROR_PROOF_ROOT;
  }

  cbmt_leaves leaves_clone;
  leaves_clone.nodes = (cbmt_node *)nodes_buffer.data;
  leaves_clone.length = leaves->length;
  for (size_t i = 0; i < leaves->length; i++) {
    cbmt_node_copy(leaves_clone.nodes + i, leaves->nodes + i);
  }
  /* sort to align with indices */
  cbmt_simple_bubble_sort(leaves_clone.nodes,
                          leaves_clone.length,
                          sizeof(cbmt_node),
                          cbmt_node_cmp);

  int ret;
  cbmt_queue queue;
  ret = cbmt_queue_init(&queue, pairs_buffer, sizeof(cbmt_node_pair), leaves->length);
  if (ret != 0) {
    return ret;
  }
  for (size_t i = 0; i < leaves->length; i++) {
    cbmt_node_pair pair;
    pair.index = proof->indices.values[i];
    cbmt_node_copy(&pair.node, leaves_clone.nodes + i);
    ret = cbmt_queue_push_back(&queue, &pair);
    if (ret != 0) {
      return ret;
    }
  }
  cbmt_simple_bubble_sort(queue.buffer.data,
                          queue.length,
                          sizeof(cbmt_node_pair),
                          cbmt_node_pair_reverse_cmp);
  size_t lemmas_offset = 0;
  cbmt_node_pair pair_current;
  cbmt_node_pair pair_sibling;
  uint32_t index;
  cbmt_node *node;
  while (queue.length > 0) {
    ret = cbmt_queue_pop_front(&queue, &pair_current);
    if (ret != 0) {
      return ret;
    }
    index = pair_current.index;
    node = &pair_current.node;

    if (index == 0) {
      if (proof->lemmas_length == lemmas_offset && queue.length == 0) {
        cbmt_node_copy(root, node);
        return 0;
      } else {
        return CBMT_ERROR_PROOF_ROOT;
      }
    }

    cbmt_node_pair *pair_front = (cbmt_node_pair *)cbmt_queue_front(&queue);
    cbmt_node *sibling = NULL;
    if (pair_front != NULL && pair_front->index == cbmt_sibling(index)) {
      ret = cbmt_queue_pop_front(&queue, &pair_sibling);
      if (ret != 0) {
        return ret;
      }
      sibling = &pair_sibling.node;
    } else {
      if (lemmas_offset < proof->lemmas_length) {
        sibling = proof->lemmas + lemmas_offset;
        lemmas_offset += 1;
      }
    }
    if (sibling != NULL) {
      cbmt_node parent = cbmt_is_left(index)
        ? merge(merge_ctx, node, sibling)
        : merge(merge_ctx, sibling, node);
      cbmt_node_pair pair_parent;
      pair_parent.index = cbmt_parent(index);
      pair_parent.node = parent;
      ret = cbmt_queue_push_back(&queue, &pair_parent);
      if (ret != 0) {
        return ret;
      }
    }
  }
  return 0;
}

int cbmt_proof_verify(cbmt_proof *proof,
                      cbmt_node *root,
                      cbmt_leaves *leaves,
                      cbmt_node_merge_fn merge,
                      void *merge_ctx,
                      /* for saving nodes in cloned leaves */
                      cbmt_buffer nodes_buffer,
                      /* for saving (index, node) pairs */
                      cbmt_buffer pairs_buffer) {
  cbmt_node target_root;
  int ret = cbmt_proof_root(proof, &target_root, leaves, merge, merge_ctx, nodes_buffer, pairs_buffer);
  if (ret != 0) {
    return ret;
  }
  if (memcmp(target_root.bytes, root->bytes, CBMT_NODE_SIZE) != 0) {
    return CBMT_ERROR_VERIFY_FAILED;
  }
  return 0;
}

int cbmt_build_merkle_root(cbmt_node *root,
                           cbmt_leaves *leaves,
                           cbmt_node_merge_fn merge,
                           void *merge_ctx,
                           /* for saving nodes in queue */
                           cbmt_buffer nodes_buffer) {
  size_t length = leaves->length;
  if (length == 0) {
    memset(root->bytes, 0, CBMT_NODE_SIZE);
    return 0;
  }

  size_t capacity = (length + 1) >> 1;
  if (capacity > nodes_buffer.capacity) {
    return CBMT_ERROR_OVER_CAPACITY;
  }
  int ret;
  cbmt_queue queue;
  ret = cbmt_queue_init(&queue, nodes_buffer, sizeof(cbmt_node), capacity);
  if (ret != 0) {
    return ret;
  }
  for (int i = length - 1; i > 0; i -= 2) {
    cbmt_node *left = leaves->nodes + i - 1;
    cbmt_node *right = leaves->nodes + i;
    cbmt_node merged = merge(merge_ctx, left, right);
    ret = cbmt_queue_push_back(&queue, &merged);
    if (ret != 0) {
      return ret;
    }
  }
  if (length % 2 == 1) {
    ret = cbmt_queue_push_front(&queue, leaves->nodes);
    if (ret != 0) {
      return ret;
    }
  }

  while (queue.length > 1) {
    cbmt_node left;
    cbmt_node right;
    ret = cbmt_queue_pop_front(&queue, &right);
    if (ret != 0) {
      return ret;
    }
    ret = cbmt_queue_pop_front(&queue, &left);
    if (ret != 0) {
      return ret;
    }
    cbmt_node merged = merge(merge_ctx, &left, &right);
    ret = cbmt_queue_push_back(&queue, &merged);
    if (ret != 0) {
      return ret;
    }
  }
  ret = cbmt_queue_pop_front(&queue, root);
  if (ret != 0) {
    return ret;
  }
  return 0;
}

int cbmt_build_merkle_tree(cbmt_tree *tree,
                           cbmt_leaves *leaves,
                           cbmt_node_merge_fn merge,
                           void *merge_ctx,
                           /* for saving nodes in tree */
                           cbmt_buffer nodes_buffer) {
  tree->nodes = (cbmt_node *)nodes_buffer.data;
  tree->capacity = nodes_buffer.capacity / sizeof(cbmt_node);
  if (leaves->length > 0) {
    size_t length = leaves->length * 2 - 1;
    if (length > tree->capacity) {
      return CBMT_ERROR_OVER_CAPACITY;
    }
    tree->length = length;

    size_t offset = leaves->length - 1;
    for (size_t i = 0; i < leaves->length; i++) {
      cbmt_node *dest_node = tree->nodes + offset + i;
      cbmt_node *src_node = leaves->nodes + i;
      cbmt_node_copy(dest_node, src_node);
    }

    for (size_t i = 0; i < leaves->length - 1; i++) {
      size_t rev_idx = leaves->length - 2 - i;
      cbmt_node *target_node = tree->nodes + rev_idx;
      cbmt_node *left = tree->nodes + ((rev_idx << 1) + 1);
      cbmt_node *right = tree->nodes + ((rev_idx << 1) + 2);
      *target_node = merge(merge_ctx, left, right);
    }
  } else {
    tree->length = 0;
  }
  return 0;
}

/* Allocate memory yourself */
int cbmt_build_merkle_proof(cbmt_proof *proof,
                            cbmt_leaves *leaves,
                            cbmt_indices *leaf_indices,
                            cbmt_node_merge_fn merge,
                            void *merge_ctx,
                            /* for saving nodes in tree */
                            cbmt_buffer nodes_buffer,
                            /* for saving indices in proof */
                            cbmt_buffer indices_buffer,
                            /* for saving lemmas in proof */
                            cbmt_buffer lemmas_buffer) {
  cbmt_tree tree;
  int ret = cbmt_build_merkle_tree(&tree, leaves, merge, merge_ctx, nodes_buffer);
  if (ret != 0) {
    return ret;
  } else {
    return cbmt_tree_build_proof(proof, &tree, leaf_indices, indices_buffer, lemmas_buffer);
  }
}

#endif /* CB_MERKLE_TREE_H_ */
