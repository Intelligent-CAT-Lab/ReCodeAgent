pub mod bostree;

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
use crate::bostree::*;
/// Runs several sanity checks on the given tree.
/// Panics if any check fails.
pub fn test_tree_sanity(tree: &BOSTree) {
    // Count nodes by traversing using select(0) and next_node().
    let mut count = 0;
    let mut current = tree.bostree_select(0);
    while let Some(node) = current {
        count += 1;
        current = bostree_next_node(&node);
    }
    let reported_count = tree.bostree_node_count();
    if count != reported_count {
        println!(
            "Manual node count yielded {}, order statistics reported {}",
            count, reported_count
        );
        panic!("Exiting due to errors.");
    }
    // For each in-order index, check various invariants.
    for i in 0..reported_count {
        let node_opt = tree.bostree_select(i);
        if node_opt.is_none() {
            println!(
                "Rank {} node not found, though count is {}",
                i, reported_count
            );
            panic!("Exiting due to errors.");
        }
        let node = node_opt.unwrap();
        let rank = bostree_rank(&node);
        if rank != i {
            println!(
                "Rank for node {} is {} but should be {}",
                node.borrow().key,
                rank,
                i
            );
            panic!("Exiting due to errors.");
        }
        let lookup_node = tree.bostree_lookup(&node.borrow().key);
        if let Some(ln) = lookup_node {
            if !Rc::ptr_eq(&node, &ln) {
                println!("Lookup for {} returned the wrong node.", node.borrow().key);
                panic!("Exiting due to errors.");
            }
        } else {
            println!("Lookup for {} returned None.", node.borrow().key);
            panic!("Exiting due to errors.");
        }
        let next_node = bostree_next_node(&node);
        let select_next = tree.bostree_select(i + 1);
        match (next_node, select_next) {
            (Some(n1), Some(n2)) => {
                if !Rc::ptr_eq(&n1, &n2) {
                    println!(
                        "Next node is {}, but select(i+1) is {}",
                        n1.borrow().key,
                        n2.borrow().key
                    );
                    panic!("Exiting due to errors.");
                }
            }
            (None, None) => {}
            _ => {
                println!("Mismatch between next node and select(i+1) for index {}", i);
                panic!("Exiting due to errors.");
            }
        }
        let prev_node = if i == 0 {
            None
        } else {
            tree.bostree_select(i - 1)
        };
        let node_prev = bostree_previous_node(&node);
        match (prev_node, node_prev) {
            (Some(n1), Some(n2)) => {
                if !Rc::ptr_eq(&n1, &n2) {
                    println!(
                        "Previous node is {}, but select(i-1) is {}",
                        n2.borrow().key,
                        n1.borrow().key
                    );
                    panic!("Exiting due to errors.");
                }
            }
            (None, None) => {}
            _ => {
                println!(
                    "Mismatch between previous node and select(i-1) for index {}",
                    i
                );
                panic!("Exiting due to errors.");
            }
        }
        // Check that if a node has a parent then it is one of its children.
        if let Some(ref parent_weak) = node.borrow().parent_node {
            if let Some(parent) = parent_weak.upgrade() {
                let left_ok = parent
                    .borrow()
                    .left_child_node
                    .as_ref()
                    .map_or(false, |left| Rc::ptr_eq(&node, left));
                let right_ok = parent
                    .borrow()
                    .right_child_node
                    .as_ref()
                    .map_or(false, |right| Rc::ptr_eq(&node, right));
                if !left_ok && !right_ok {
                    println!(
                        "Node {} has a parent, {}, but is not one of its children.",
                        node.borrow().key,
                        parent.borrow().key
                    );
                    panic!("Exiting due to errors.");
                }
            }
        }
        // Recursive functions to compute actual depth and count.
        fn actual_depth(node: &Rc<RefCell<BOSNode>>) -> u32 {
            let left_depth = node
                .borrow()
                .left_child_node
                .as_ref()
                .map_or(0, |left| actual_depth(left) + 1);
            let right_depth = node
                .borrow()
                .right_child_node
                .as_ref()
                .map_or(0, |right| actual_depth(right) + 1);
            max(left_depth, right_depth)
        }
        fn actual_count(node: &Rc<RefCell<BOSNode>>) -> u32 {
            let left_count = node
                .borrow()
                .left_child_node
                .as_ref()
                .map_or(0, |left| actual_count(left));
            let right_count = node
                .borrow()
                .right_child_node
                .as_ref()
                .map_or(0, |right| actual_count(right));
            left_count + right_count + 1
        }
        let actual_depth_val = actual_depth(&node);
        if actual_depth_val != node.borrow().depth {
            println!(
                "Node {} reports depth {} when the actual depth is {}",
                node.borrow().key,
                node.borrow().depth,
                actual_depth_val
            );
            panic!("Exiting due to errors.");
        }
        if let Some(ref left) = node.borrow().left_child_node {
            let left_count = actual_count(left);
            if left_count != node.borrow().left_child_count {
                println!(
                    "Node {} reports {} left children when there are {}",
                    node.borrow().key,
                    node.borrow().left_child_count,
                    left_count
                );
                panic!("Exiting due to errors.");
            }
        }
        if let Some(ref right) = node.borrow().right_child_node {
            let right_count = actual_count(right);
            if right_count != node.borrow().right_child_count {
                println!(
                    "Node {} reports {} right children when there are {}",
                    node.borrow().key,
                    node.borrow().right_child_count,
                    right_count
                );
                panic!("Exiting due to errors.");
            }
        }
        let left_depth = node
            .borrow()
            .left_child_node
            .as_ref()
            .map_or(0, |n| n.borrow().depth + 1);
        let right_depth = node
            .borrow()
            .right_child_node
            .as_ref()
            .map_or(0, |n| n.borrow().depth + 1);
        if (left_depth as i32 - right_depth as i32).abs() > 1 {
            println!(
                "Node {} violates the balance constraint: {} left depth vs. {} right depth",
                node.borrow().key,
                left_depth,
                right_depth
            );
            panic!("Exiting due to errors.");
        }
    }
}