use blt::blt::*;

/// Splits a string `s` on the space character (including empty tokens)
/// and calls the provided closure for each token.
fn split<F: FnMut(String)>(s: &str, mut fun: F) {
    if s.is_empty() {
        return;
    }
    let mut start = 0;
    for (i, ch) in s.char_indices() {
        if ch == ' ' {
            fun(s[start..i].to_string());
            start = i + 1;
        }
    }
    fun(s[start..].to_string());
}

struct Arr {
    p: Vec<String>,
}

impl Arr {
    fn new() -> Self {
        Arr { p: Vec::new() }
    }
    fn add(&mut self, s: String) {
        self.p.push(s);
    }
}

/// Creates a BLT tree from a line of space‐separated keys.
fn make_blt(line: &str) -> Blt {
    let mut tree = Blt::blt_new();
    split(line, |s| {
        // Insert key with data 0 (boxed integer).
        let _ = tree.blt_put(&s, Box::new(0));
    });
    tree
}

/// Creates an Arr from a line of space‐separated tokens.
fn make_arr(line: &str) -> Arr {
    let mut arr = Arr::new();
    split(line, |s| {
        arr.add(s);
    });
    arr
}

/// Tests in‐order traversal of the BLT tree.
fn test_traverse(line: &str) {
    let mut tree = Blt::blt_new();
    let mut arr = Arr::new();
    split(line, |s| {
        let _ = tree.blt_put(&s, Box::new(0));
        arr.add(s);
    });
    arr.p.sort();

    let mut n = 0;
    let mut count = 0;
    // Check that `forall` visits keys in sorted order.
    tree.blt_forall(|it: &BltIt| {
        while n + 1 < arr.p.len() && arr.p[n] == arr.p[n + 1] {
            n += 1;
        }
        assert!(n < arr.p.len(), "FAIL in forall");
        assert_eq!(it.key, arr.p[n], "Key mismatch in forall");
        n += 1;
        count += 1;
    });
    assert_eq!(count, tree.blt_size() as usize, "Size mismatch in forall");
    assert_eq!(n, arr.p.len(), "Count mismatch in forall");

    // Check traversal using first() and next().
    n = 0;
    count = 0;
    let mut it_opt = tree.blt_first();
    while let Some(it) = it_opt {
        while n + 1 < arr.p.len() && arr.p[n] == arr.p[n + 1] {
            n += 1;
        }
        assert!(n < arr.p.len(), "FAIL in first/next");
        assert_eq!(it.key, arr.p[n], "Key mismatch in first/next");
        n += 1;
        count += 1;
        it_opt = tree.blt_next(&it);
    }
    assert_eq!(
        count,
        tree.blt_size() as usize,
        "Size mismatch in first/next"
    );
    assert_eq!(n, arr.p.len(), "Count mismatch in first/next");

    // Check traversal using last() and prev().
    if !arr.p.is_empty() {
        n = arr.p.len() - 1;
    } else {
        n = 0;
    }
    count = 0;
    let mut it_opt = tree.blt_last();
    while let Some(it) = it_opt {
        while n > 0 && arr.p[n] == arr.p[n - 1] {
            n -= 1;
        }
        assert!(n < arr.p.len(), "FAIL in last/prev");
        assert_eq!(it.key, arr.p[n], "Key mismatch in last/prev");
        count += 1;
        if n == 0 {
            break;
        } else {
            n -= 1;
        }
        it_opt = tree.blt_prev(&it);
    }
    assert_eq!(
        count,
        tree.blt_size() as usize,
        "Size mismatch in last/prev"
    );
}

/// Checks that iterating over all keys with a given prefix returns the expected order.
fn check_prefix(tree: &Blt, prefix: &str, want: &str) {
    let arr = make_arr(want);
    let mut n = 0;
    let _ = tree.blt_allprefixed(prefix, |it: &BltIt| {
        while n + 1 < arr.p.len() && arr.p[n] == arr.p[n + 1] {
            n += 1;
        }
        assert!(n < arr.p.len(), "FAIL in check_prefix");
        assert_eq!(it.key, arr.p[n], "Key mismatch in check_prefix");
        n += 1;
        1 // continue iteration
    });
    assert_eq!(n, arr.p.len(), "Prefix count mismatch");
}

/// Generates a string with `n` groups of 'a' (each group length 1–12), separated by spaces.
fn generate_random_a_string(n: usize) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    for i in 0..n {
        if i > 0 {
            s.push(' ');
        }
        let len = rng.gen_range(1..=12);
        s.push_str(&"a".repeat(len));
    }
    s
}

/// Generates a string with `n` groups of random printable characters (each group length 1–12), separated by spaces.
fn generate_random_nonzero_string(n: usize) -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    for i in 0..n {
        if i > 0 {
            s.push(' ');
        }
        let len = rng.gen_range(1..=12);
        for _ in 0..len {
            // Generate a random printable ASCII character.
            let c = rng.gen_range(33..127) as u8 as char;
            s.push(c);
        }
    }
    s
}

#[test]
fn test() {
    test_traverse("");
    test_traverse("one-string");
    test_traverse("two strings");
    test_traverse("red string blue string");
    test_traverse("the quick brown fox jumps over the lazy dog");
    test_traverse("  2 spaces   means  empty   strings   are tested");

    let n = 32;
    let s = generate_random_a_string(n);
    test_traverse(&s);

    let s = generate_random_nonzero_string(n);
    test_traverse(&s);

    let tree = make_blt("a aardvark b ben blink bliss blt blynn");
    check_prefix(&tree, "b", "b ben blink bliss blt blynn");
    check_prefix(&tree, "bl", "blink bliss blt blynn");
    check_prefix(&tree, "bli", "blink bliss");
    check_prefix(&tree, "a", "a aardvark");
    check_prefix(&tree, "aa", "aardvark");
    check_prefix(&tree, "c", "");

    let ceil = tree.blt_ceil("blink").expect("Expected ceil for blink");
    assert_eq!(ceil.key, "blink", "blt_ceil mismatch for blink");

    let ceil = tree
        .blt_ceil("blink182")
        .expect("Expected ceil for blink182");
    assert_eq!(ceil.key, "bliss", "blt_ceil mismatch for blink182");

    let floor = tree.blt_floor("blink").expect("Expected floor for blink");
    assert_eq!(floor.key, "blink", "blt_floor mismatch for blink");

    let floor = tree
        .blt_floor("blink182")
        .expect("Expected floor for blink182");
    assert_eq!(floor.key, "blink", "blt_floor mismatch for blink182");
}
fn main() {}
