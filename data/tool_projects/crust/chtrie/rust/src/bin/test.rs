use std::collections::HashMap;
use chtrie::chtrie::Chtrie;

const N: usize = 65536;
const M: usize = 256;

static DICT1: [&str; 4] = ["", "the", "a", "an"];
static DICT2: [&str; 4] = ["he", "she", "his", "hers"];
static DICT3: [&str; 2] = ["this", "that"];
static STOP: [&str; 3] = ["the", "an", "a"];

fn add(tr: &mut Chtrie, term: &mut HashMap<i32, bool>, nchild: &mut HashMap<i32, i32>, s: &str) {
    let mut it = 0;
    for c in s.chars() {
        if tr.walk(it, c as i32, 0) == -1 {
            *nchild.entry(it).or_insert(0) += 1;
        }
        it = tr.walk(it, c as i32, 1);
        if it == -1 {
            panic!("walk failed");
        }
    }
    term.insert(it, true);
}

fn del(tr: &mut Chtrie, term: &mut HashMap<i32, bool>, nchild: &mut HashMap<i32, i32>, s: &str) {
    let mut nodes = Vec::new();
    let mut symbs = Vec::new();
    let mut it = 0;
    for c in s.chars() {
        nodes.push(it);
        symbs.push(c as i32);
        it = tr.walk(it, c as i32, 0);
        if it < 0 {
            return;
        }
    }
    if !term.get(&it).unwrap_or(&false) {
        return;
    }
    term.insert(it, false);
    while it > 0 && !term.get(&it).unwrap_or(&false) && *nchild.get(&it).unwrap_or(&0) == 0 {
        let n = nodes.pop().unwrap();
        let sym = symbs.pop().unwrap();
        tr.del(n, sym);
        it = n;
        *nchild.entry(it).or_insert(0) -= 1;
    }
}

fn query(tr: &mut Chtrie, term: &HashMap<i32, bool>, s: &str) -> bool {
    let mut it = 0;
    for c in s.chars() {
        it = tr.walk(it, c as i32, 0);
        if it < 0 {
            return false;
        }
    }
    *term.get(&it).unwrap_or(&false)
}

#[test]
fn test_chtrie() {
    let mut tr = Chtrie::new(N, M).expect("Failed to initialize Chtrie");
    let mut term = HashMap::new();
    let mut nchild = HashMap::new();

    for &word in DICT1.iter() {
        add(&mut tr, &mut term, &mut nchild, word);
    }
    for &word in DICT2.iter() {
        add(&mut tr, &mut term, &mut nchild, word);
    }
    for &word in STOP.iter() {
        del(&mut tr, &mut term, &mut nchild, word);
    }
    for &word in DICT3.iter() {
        add(&mut tr, &mut term, &mut nchild, word);
    }

    let test_cases = [
        ("hello", false), ("the", false), ("his", true), ("he", true),
        ("his", true), ("go", false), ("he", true), ("a", false),
        ("an", false), ("this", true), ("that", true), ("hey", false),
        ("she", true), ("hers", true),
    ];

    for &(test_case, expected) in test_cases.iter() {
        let result = query(&mut tr, &term, test_case);
        assert_eq!(result, expected, "Query: {}, Result: {}, Expected: {}", test_case, result, expected);
    }

    tr.free();
    println!("All tests passed!");
}
fn main() {
    //
}