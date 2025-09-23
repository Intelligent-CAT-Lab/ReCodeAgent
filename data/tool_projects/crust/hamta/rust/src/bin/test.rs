use hamta::hamta::{Hamt, KeyValue, hamt_int_hash, hamt_int_equals, hamt_str_hash, hamt_str_equals};

#[test]
fn test_empty() {
    let mut h: Hamt<i32, i32> = Hamt::new_hamt(hamt_int_hash::<i32>, hamt_int_equals::<i32>);
    assert!(h.root.is_none(), "error, hamt not initialized");
    h.hamt_destroy(|ptr| (), |ptr| () );
}
#[test]
fn test_big() {
    let mut h = Hamt::new_hamt(hamt_int_hash, hamt_int_equals);
    let n = 10000;
    for i in 0..n {
    let mut key = Box::new(i % (n / 1337) + 1);
    let mut value = Box::new(i * i + 10);
    let mut conflict_kv = KeyValue {
        key: &mut Box::new(0),
        value: &mut Box::new(0),
    };
    let conflict = h.hamt_set(&mut key, &mut value, &mut conflict_kv);
    if conflict {
    // In Rust, memory is automatically managed, so no need to free
    }
    let found = h.hamt_search(&mut key);
        assert!(found.is_some(), "value not found");
        assert_eq!(*(found.unwrap().value), value, "value inserted and retrieved don't match!");
    }
    h.hamt_destroy(|ptr|(), |ptr|());
}
#[test]
fn test_big2() {
    let mut h = Hamt::new_hamt(hamt_int_hash, hamt_int_equals);
    let n = 10000;
    for i in 0..n {
        let mut key = Box::new(i % (n / 1531));
        let mut value = Box::new(i * i * i);
        let mut conflict_kv = KeyValue {
            key: &mut Box::new(0),
            value: &mut Box::new(0),
        };
        let conflict = h.hamt_set( &mut key, &mut value, &mut conflict_kv);
        if conflict {
        // In Rust, memory is automatically managed, so no need to free
        }
        let found = h.hamt_search(&mut key);
        assert!(found.is_some(), "value not found");
        assert_eq!(*found.unwrap().value, value, "value inserted and retrieved don't match!");
    }
    for i in 0..n {
    let mut removed_kv = KeyValue {
        key: &mut Box::new(0),
        value: &mut Box::new(0),
    };
    let removed = h.hamt_remove( &mut Box::new(i), &mut removed_kv);
    if removed {
        // do nothing
    }
    }
    h.hamt_destroy(|ptr|(), |ptr|());
}
#[test]
fn test_create() {
    let mut h = Hamt::new_hamt(hamt_str_hash, hamt_str_equals);
    let mut x = Box::new("xx".to_string());
    let mut y = Box::new("yy".to_string());
    let mut x1 = Box::new("xx".to_string());
    let mut y1 = Box::new("yy".to_string());
    let mut conflict_kv = KeyValue {
        key: &mut Box::new("".to_string()),
        value: &mut Box::new("".to_string()),
    };
    h.hamt_set(&mut x, &mut x1, &mut conflict_kv);
    h.hamt_set(&mut y, &mut y1, &mut conflict_kv);
    h.hamt_set(&mut x, &mut y, &mut conflict_kv);
    h.hamt_set(&mut y, &mut x, &mut conflict_kv);
    let removed = h.hamt_remove(&mut x, &mut conflict_kv);
    if removed {
        // do nothing 
    }
    h.hamt_destroy(|_| {}, |_| {});
}
#[test]
fn test_search_destroy() {
    let mut h = Hamt::new_hamt(hamt_str_hash, hamt_str_equals);
    let mut num_elements = 0;
    let strings = vec![
        "aut", "bus", "vlak", "kokos", "banan", "losos", "bro", "b", "bubakov",
    ];
    let mut conflict_kv = KeyValue {
        key: &mut Box::new("".to_string()),
        value: &mut Box::new("".to_string()),
    };
    for s in &strings {
        h.hamt_set(&mut Box::new(s.to_string()), &mut Box::new(s.to_string()), &mut conflict_kv);
        num_elements += 1;
    }
    assert_eq!(h.hamt_size(), num_elements, "error, hamt size doesn't match 1");
    for s in &strings {
        let found = h.hamt_search(&mut Box::new(s.to_string()));
        assert!(found.is_some(), "error, not found");
        assert_eq!(**(found.unwrap().value), s.to_string(), "error, didn't find the correct key");
    }
    assert_eq!(h.hamt_size(), num_elements, "error, hamt size doesn't match");
    for s in &strings {
        let mut removed_kv = KeyValue {
            key: &mut Box::new("".to_string()),
            value: &mut Box::new("".to_string()),
        };
        let removed = h.hamt_remove(&mut Box::new(s.to_string()), &mut removed_kv);
        assert!(removed, "error, returned element is NULL");
        num_elements -= 1;
        assert_eq!(h.hamt_size(), num_elements, "error, hamt size doesn't match after removal");
    }
}
#[test]
fn test_hamta2() {
    let mut h = Hamt::new_hamt(hamt_str_hash, hamt_str_equals);
    let strings = vec![
    "a", "bb", "auto", "bus", "vlak", "kokos", "banan", "losos", "bubakov", "korkodyl", "x", "__x__", "y",
    ];
    let mut conflict_kv = KeyValue {
        key: &mut Box::new("".to_string()),
        value: &mut Box::new("".to_string()),
    };
    for s in &strings {
        #[cfg(debug_assertions)]
        //h.hamt_print(|x| x.downcast_ref::<String>().unwrap().clone());
        h.hamt_set(&mut Box::new(s.to_string()), &mut Box::new(s.to_string()), &mut conflict_kv);
    }
    #[cfg(debug_assertions)]
    //hamta::hamt_print(&h, |x| x.downcast_ref::<String>().unwrap().clone());
    assert_eq!(h.hamt_size(), strings.len() as i32, "error, hamt size doesn't match");
}
fn main() {
}