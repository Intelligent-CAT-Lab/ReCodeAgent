use std::process;

use Bostree::bostree::*;
use Bostree::*;

fn cmp(a: &str, b: &str) -> i32 {
    a.cmp(b) as i32
}

/// Create a test tree by inserting keys from 'A' to 'Y' (i.e. 'A' .. 'Z').
fn test_tree() -> BOSTree {
    let mut tree = BOSTree::bostree_new(cmp, None);
    for c in 'A'..'Z' {
        let key = c.to_string();
        let _node = tree.bostree_insert(key, None);
    }
    tree
}

#[test]
fn test() {
    // For each removal key from 'A' to 'Y'
    for c in 'A'..'Z' {
        let mut tree = test_tree();
        let key = c.to_string();
        if let Some(node) = tree.bostree_lookup(&key) {
            tree.bostree_remove(&node);
        }
        test_tree_sanity(&tree);
        let expected_count = 'Z' as u32 - 'A' as u32 - 1;
        if tree.bostree_node_count() != expected_count {
            println!("Removed one node from a tree, but the node count did not decrease properly.");
            process::exit(1);
        }
    }
}
fn main() {}
