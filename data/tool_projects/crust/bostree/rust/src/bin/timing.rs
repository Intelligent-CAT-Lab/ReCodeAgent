use std::time::{Duration, Instant};
use std::{env, process};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use Bostree::bostree::*;
use Bostree::*;

fn cmp(a: &str, b: &str) -> i32 {
    a.cmp(b) as i32
}

#[test]
fn test() {
    let args: Vec<String> = env::args().collect();
    let seed: u64 = if args.len() > 1 {
        let s = args[1].parse().unwrap_or(0);
        println!("Using seed {} from command line", s);
        s
    } else {
        let s = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        println!("Using seed {}", s);
        s
    };

    let mut rng = StdRng::seed_from_u64(seed);
    let mut tree = BOSTree::bostree_new(cmp, None);
    let mut gestime = Duration::new(0, 0);

    // Insert 10,000,000 elements.
    for i in 0..10_000_000 {
        let mut name: String = (0..32)
            .map(|_| {
                let r = rng.random_range(0..('Z' as u8 - 'A' as u8 + 1));
                (b'A' + r) as char
            })
            .collect();
        // Ensure the key is unique.
        while tree.bostree_lookup(&name).is_some() {
            name = (0..32)
                .map(|_| {
                    let r = rng.random_range(0..('Z' as u8 - 'A' as u8 + 1));
                    (b'A' + r) as char
                })
                .collect();
        }
        let start = Instant::now();
        let _node = tree.bostree_insert(name, None);
        gestime += start.elapsed();

        if i % 1_000_000 == 0 && i != 0 {
            println!(
                "{:07} elements, inserting a value takes {:.5e} s/op",
                tree.bostree_node_count(),
                gestime.as_secs_f64() / 1e9
            );
            test_tree_sanity(&tree);
            println!(" sanity check passed");
            gestime = Duration::new(0, 0);
        }
    }

    let mut selecttime = Duration::new(0, 0);
    let mut deletetime = Duration::new(0, 0);
    let mut count = tree.bostree_node_count();
    while count > 0 {
        let w = rng.random_range(0..count);
        let start = Instant::now();
        let node = tree.bostree_select(w);
        selecttime += start.elapsed();

        if node.is_none() {
            println!("Node missing!");
            test_tree_sanity(&tree);
            process::exit(1);
        }
        let node = node.unwrap();
        let start = Instant::now();
        tree.bostree_remove(&node);
        deletetime += start.elapsed();

        count -= 1;
        if count % 1_000_000 == 0 && count != 0 {
            println!(
                "{:07} elements, selecting a value takes {:.5e} s/op, removing a value takes {:.5e} s/op",
                tree.bostree_node_count(),
                selecttime.as_secs_f64() / 1e9,
                deletetime.as_secs_f64() / 1e9
            );
            test_tree_sanity(&tree);
            println!(" sanity check passed");
            selecttime = Duration::new(0, 0);
            deletetime = Duration::new(0, 0);
        }
    }
}
fn main(){}