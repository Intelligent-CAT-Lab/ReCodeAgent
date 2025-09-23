
#define CBMT_NODE_I32

#include "merkle_tree.h"
#include <stdlib.h>
#include <stdio.h>
#include <assert.h>

#include <blake2b.h>

cbmt_node node_merge(void *merge_ctx,
                     cbmt_node *left,
                     cbmt_node *right) {
  cbmt_node ret;
#ifdef CBMT_NODE_I32
  int32_t left_value = *((int32_t *)left->bytes);
  int32_t right_value = *((int32_t *)right->bytes);
  int32_t value = right_value - left_value;
  memcpy(ret.bytes, &value, 4);
#else
  blake2b_state *blake2b_ctx = (blake2b_ctx *)merge_ctx;
  blake2b_init(blake2b_ctx, CBMT_NODE_SIZE);
  blake2b_update(blake2b_ctx, left->bytes, CBMT_NODE_SIZE);
  blake2b_update(blake2b_ctx, right->bytes, CBMT_NODE_SIZE);
  blake2b_final(blake2b_ctx, ret.bytes, CBMT_NODE_SIZE);
#endif
  return ret;
}

cbmt_node int32_to_node(int32_t value) {
  cbmt_node node;
  memcpy(node.bytes, &value, sizeof(value));
  return node;
}

int32_t node_to_int32(cbmt_node node) {
  return *((int32_t *)node.bytes);
}

void test_build_empty();
void test_build_five();
void test_build_root_directly_2leaves();
void test_build_root_directly();
void test_rebuild_proof();
void test_build_proof();

int main() {
  test_build_empty();
  test_build_five();
  test_build_root_directly_2leaves();
  test_build_root_directly();
  test_rebuild_proof();
  test_build_proof();
}

void test_build_empty() {
  cbmt_node nodes[1024];
  cbmt_node leaf_nodes[256];

  cbmt_buffer nodes_buffer;
  cbmt_buffer_init(&nodes_buffer, nodes, sizeof(nodes));

  int ret;
  cbmt_tree tree;
  cbmt_leaves leaves;
  cbmt_leaves_init(&leaves, leaf_nodes, 0);
  ret = cbmt_build_merkle_tree(&tree, &leaves, node_merge, NULL, nodes_buffer);
  assert(ret == 0);
  assert(node_to_int32(cbmt_tree_root(&tree)) == 0);
  return;
}

void test_build_five() {
  cbmt_node nodes[1024];
  cbmt_node leaf_nodes[256];

  cbmt_buffer nodes_buffer;
  cbmt_buffer_init(&nodes_buffer, nodes, sizeof(nodes));

  int ret;
  cbmt_tree tree;
  cbmt_leaves leaves;
  leaf_nodes[0] = int32_to_node(2);
  leaf_nodes[1] = int32_to_node(3);
  leaf_nodes[2] = int32_to_node(5);
  leaf_nodes[3] = int32_to_node(7);
  leaf_nodes[4] = int32_to_node(11);
  cbmt_leaves_init(&leaves, leaf_nodes, 5);
  ret = cbmt_build_merkle_tree(&tree, &leaves, node_merge, NULL, nodes_buffer);
  assert(ret == 0);

  for (size_t i = 0; i < tree.length; i++) {
    printf("tree.nodes[%ld]: %d\n", i, node_to_int32(tree.nodes[i]));
  }
  assert(node_to_int32(tree.nodes[0]) == 4);
  assert(node_to_int32(tree.nodes[1]) == -2);
  assert(node_to_int32(tree.nodes[2]) == 2);
  assert(node_to_int32(tree.nodes[3]) == 4);

  assert(node_to_int32(tree.nodes[4]) == 2);
  assert(node_to_int32(tree.nodes[5]) == 3);
  assert(node_to_int32(tree.nodes[6]) == 5);
  assert(node_to_int32(tree.nodes[7]) == 7);
  assert(node_to_int32(tree.nodes[8]) == 11);
  return;
}


void test_build_root_directly_2leaves() {
  cbmt_node nodes[1024];
  cbmt_node leaf_nodes[256];

  cbmt_buffer nodes_buffer;
  cbmt_buffer_init(&nodes_buffer, nodes, sizeof(nodes));

  int ret;
  cbmt_tree tree;
  cbmt_leaves leaves;
  leaf_nodes[0] = int32_to_node(2);
  leaf_nodes[1] = int32_to_node(3);
  cbmt_leaves_init(&leaves, leaf_nodes, 2);
  cbmt_node root;
  ret = cbmt_build_merkle_root(&root, &leaves, node_merge, NULL, nodes_buffer);
  assert(ret == 0);
  printf("root: %d\n", node_to_int32(root));
  assert(node_to_int32(root) == 1);
}

void test_build_root_directly() {
  cbmt_node nodes[1024];
  cbmt_node leaf_nodes[256];

  cbmt_buffer nodes_buffer;
  cbmt_buffer_init(&nodes_buffer, nodes, sizeof(nodes));

  int ret;
  cbmt_tree tree;
  cbmt_leaves leaves;
  leaf_nodes[0] = int32_to_node(2);
  leaf_nodes[1] = int32_to_node(3);
  leaf_nodes[2] = int32_to_node(5);
  leaf_nodes[3] = int32_to_node(7);
  leaf_nodes[4] = int32_to_node(11);
  cbmt_leaves_init(&leaves, leaf_nodes, 5);
  cbmt_node root;
  ret = cbmt_build_merkle_root(&root, &leaves, node_merge, NULL, nodes_buffer);
  assert(ret == 0);
  printf("root: %d\n", node_to_int32(root));
  assert(node_to_int32(root) == 4);
}

void test_rebuild_proof() {
  cbmt_node nodes[1024];
  cbmt_node leaf_nodes[256];

  cbmt_buffer nodes_buffer;
  cbmt_buffer_init(&nodes_buffer, nodes, sizeof(nodes));

  int ret;
  cbmt_tree tree;
  cbmt_leaves leaves;
  leaf_nodes[0] = int32_to_node(2);
  leaf_nodes[1] = int32_to_node(3);
  leaf_nodes[2] = int32_to_node(5);
  leaf_nodes[3] = int32_to_node(7);
  leaf_nodes[4] = int32_to_node(11);
  cbmt_leaves_init(&leaves, leaf_nodes, 5);
  ret = cbmt_build_merkle_tree(&tree, &leaves, node_merge, NULL, nodes_buffer);
  assert(ret == 0);

  cbmt_node root = cbmt_tree_root(&tree);
  cbmt_proof proof;
  uint32_t leaf_index_values[2] = { 0, 3 };
  cbmt_indices leaf_indices;
  cbmt_indices_init(&leaf_indices, leaf_index_values, 2, 2);

  uint32_t leaf_values_data[256];
  cbmt_node lemmas_nodes[256];
  cbmt_buffer indices_buffer;
  cbmt_buffer lemmas_buffer;
  cbmt_buffer_init(&indices_buffer, leaf_values_data, sizeof(leaf_values_data));
  cbmt_buffer_init(&lemmas_buffer, lemmas_nodes, sizeof(lemmas_nodes));
  ret = cbmt_tree_build_proof(&proof, &tree, &leaf_indices, indices_buffer, lemmas_buffer);
  assert(ret == 0);
  printf("proof.indices.length=%ld, proof.lemmas_length=%ld\n",
         proof.indices.length,
         proof.lemmas_length);
  assert(proof.indices.length == 2);
  assert(proof.indices.values[0] == 4);
  assert(proof.indices.values[1] == 7);
  assert(proof.lemmas_length == 2);
  assert(node_to_int32(proof.lemmas[0]) == 11);
  assert(node_to_int32(proof.lemmas[1]) == 2);

  cbmt_node needed_nodes[256];
  cbmt_leaves needed_leaves;
  cbmt_leaves_init(&needed_leaves, needed_nodes, leaf_indices.length);
  for (size_t i = 0; i < needed_leaves.length; i++) {
    needed_leaves.nodes[i] = tree.nodes[proof.indices.values[i]];
    printf("[needed] index=%d, node=%d\n",
           proof.indices.values[i],
           node_to_int32(tree.nodes[proof.indices.values[i]]));
  }

  cbmt_node nodes2[1024];
  cbmt_node_pair pairs[1024];
  cbmt_buffer nodes_buffer2;
  cbmt_buffer pairs_buffer;
  cbmt_buffer_init(&nodes_buffer2, nodes2, sizeof(nodes2));
  cbmt_buffer_init(&pairs_buffer, pairs, sizeof(pairs));
  ret = cbmt_proof_verify(&proof, &root, &needed_leaves, node_merge, NULL, nodes_buffer2, pairs_buffer);
  assert(ret == 0);
  return;
}

void test_build_proof() {
  return;
}
