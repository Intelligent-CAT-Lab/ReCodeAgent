use std::fs;
use std::path::Path;
use std::collections::HashMap;
use mdb::mdb::{Mdb, MdbOptions, MdbError, MdbStatusCode};

const MDB_OK: MdbStatusCode = MdbStatusCode::MDB_OK;
const MDB_NO_KEY: MdbStatusCode = MdbStatusCode::MDB_NO_KEY;

use rand::Rng;

// Helper function to clean up test databases before/after tests
fn cleanup_db(db_name: &str) {
    let paths = [
        format!("{}.super", db_name),
        format!("{}.index", db_name),
        format!("{}.data", db_name)
    ];
    
    for path in paths.iter() {
        if Path::new(path).exists() {
            let _ = fs::remove_file(path);
        }
    }
}

#[test]
fn happy_test0() {
    println!("Starting misakawa happy test");
    
    // Clean up any existing test files
    cleanup_db("misakawa");
    
    let options = MdbOptions {
        db_name: "misakawa".to_string(),
        key_size_max: 64,
        data_size_max: 256,
        hash_buckets: 128,
        items_max: 166716,
    };
    
    // Create database
    let mut db = Mdb::create("misakawa", options).expect("Failed to create database");
    
    // Write a key-value pair
    db.write("misakawa", "mikoto").expect("Failed to write key-value");
    
    // Read the value back
    let mut buffer = vec![0u8; 257];
    let read_size = db.read("misakawa", &mut buffer).expect("Failed to read value");
    
    // Convert bytes to string for comparison
    let read_str = String::from_utf8_lossy(&buffer[0..read_size]).to_string();
    println!("\"misakawa\" => \"{}\"", read_str);
    assert_eq!("mikoto", read_str);
    
    // Delete the key
    db.delete("misakawa").expect("Failed to delete key");
    
    // Try to read the deleted key - should fail with KeyNotFound
    match db.read("misakawa", &mut buffer) {
        Err(MdbError::KeyNotFound) => (), // Expected error
        _ => panic!("Expected KeyNotFound error when reading deleted key"),
    }
    
    // Close the database (implicitly through drop)
    drop(db);
    
    println!("Finished misakawa happy test");
    
    // Clean up
    cleanup_db("misakawa");
}

#[test]
fn reopen_test1() {
    println!("Starting lambda re-open test");
    
    // Clean up any existing test files
    cleanup_db("lambda");
    
    let options = MdbOptions {
        db_name: "lambda".to_string(),
        key_size_max: 64,
        data_size_max: 256,
        hash_buckets: 128,
        items_max: 166716,
    };
    
    // Create database and write data
    {
        let mut db = Mdb::create("lambda", options.clone()).expect("Failed to create database");
        db.write("Lisp", "LambdaExpression").expect("Failed to write key-value");
        // Database closed when db goes out of scope
    }
    
    // Re-open the database
    let mut db1 = Mdb::open("lambda").expect("Failed to open existing database");
    
    // Verify options were preserved
    let db1_options = db1.get_options();
    assert_eq!(options.db_name, db1_options.db_name);
    assert_eq!(options.key_size_max, db1_options.key_size_max);
    assert_eq!(options.data_size_max, db1_options.data_size_max);
    assert_eq!(options.hash_buckets, db1_options.hash_buckets);
    assert_eq!(options.items_max, db1_options.items_max);
    
    // Read the previously written value
    let mut buffer = vec![0u8; 257];
    let read_size = db1.read("Lisp", &mut buffer).expect("Failed to read value");
    let read_str = String::from_utf8_lossy(&buffer[0..read_size]).to_string();
    assert_eq!("LambdaExpression", read_str);
    
    // Close the database (implicitly through drop)
    drop(db1);
    
    println!("Finished lambda re-open test");
    
    // Clean up
    cleanup_db("lambda");
}

#[test]
fn load_test2() {
    println!("Starting accelerator load test");
    
    // Clean up any existing test files
    cleanup_db("accelerator");
    
    let preset_values = [
        "misakawa", "kamijou", "accelerator", "index", "lyzh", "chuigda",
        "de_nuke", "de_mirage", "de_cache", "de_vertigo", "de_inferno", "de_cbble",
        "de_dust2", "fy_iceworld", "cs_assault", "komakan", "scarlet", "flandre"
    ];
    
    let mut values = Vec::with_capacity(1000);
    let mut rng = rand::thread_rng();
    
    let options = MdbOptions {
        db_name: "accelerator".to_string(),
        key_size_max: 8,
        data_size_max: 256,
        hash_buckets: 128,
        items_max: 166716,
    };
    
    let mut db = Mdb::create("accelerator", options).expect("Failed to create database");
    
    // Write 1000 key-value pairs
    let mut i = 0;
    for c1 in '0'..='9' {
        for c2 in '0'..='9' {
            for c3 in '0'..='9' {
                let key = format!("{}{}{}", c1, c2, c3);
                let value_idx = rng.gen_range(0..18);
                let value = preset_values[value_idx];
                values.push(value.to_string());
                
                db.write(&key, value).expect("Failed to write key-value");
                i += 1;
            }
        }
    }
    
    // Read all values back and verify
    i = 0;
    let mut buffer = vec![0u8; 257];
    for c1 in '0'..='9' {
        for c2 in '0'..='9' {
            for c3 in '0'..='9' {
                let key = format!("{}{}{}", c1, c2, c3);
                let read_size = db.read(&key, &mut buffer).expect("Failed to read value");
                let read_str = String::from_utf8_lossy(&buffer[0..read_size]).to_string();
                
                assert_eq!(values[i], read_str);
                i += 1;
            }
        }
    }
    
    // Close the database (implicitly through drop)
    drop(db);
    
    println!("Finished accelerator load test");
    
    // Clean up
    cleanup_db("accelerator");
}

#[test]
fn load_test3() {
    println!("Starting gabriel load test");
    
    // Clean up any existing test files
    cleanup_db("gabriel");
    
    let preset_values = [
        "misakawa", "kamijou", "accelerator", "index", "lyzh", "chuigda",
        "de_nuke", "de_mirage", "de_cache", "de_vertigo", "de_inferno", "de_cbble",
        "de_dust2", "fy_iceworld", "cs_assault", "komakan", "scarlet", "flandre"
    ];
    
    let mut values = vec![None; 1000];
    let mut values2 = vec![String::new(); 260];
    let mut deleted_entries = Vec::new();
    let mut rng = rand::thread_rng();
    
    let options = MdbOptions {
        db_name: "gabriel".to_string(),
        key_size_max: 8,
        data_size_max: 256,
        hash_buckets: 128,
        items_max: 166716,
    };
    
    let mut db = Mdb::create("gabriel", options).expect("Failed to create database");
    
    // Write 1000 key-value pairs with numeric keys
    let mut i = 0;
    for c1 in '0'..='9' {
        for c2 in '0'..='9' {
            for c3 in '0'..='9' {
                let key = format!("{}{}{}", c1, c2, c3);
                let value_idx = rng.gen_range(0..18);
                let value = preset_values[value_idx].to_string();
                values[i] = Some(value.clone());
                
                db.write(&key, &value).expect("Failed to write key-value");
                i += 1;
            }
        }
    }
    
    // Delete approximately 10% of entries (up to 128 max)
    let mut deleted_entries_cnt = 0;
    i = 0;
    for c1 in '0'..='9' {
        if deleted_entries_cnt >= 128 {
            break;
        }
        
        for c2 in '0'..='9' {
            if deleted_entries_cnt >= 128 {
                break;
            }
            
            for c3 in '0'..='9' {
                if deleted_entries_cnt >= 128 {
                    break;
                }
                
                if rng.gen_range(0..10) == 0 {
                    let key = format!("{}{}{}", c1, c2, c3);
                    db.delete(&key).expect("Failed to delete key");
                    values[i] = None;
                    deleted_entries.push(i);
                    deleted_entries_cnt += 1;
                }
                i += 1;
            }
        }
    }
    
    // Write 260 more key-value pairs with alphanumeric keys
    i = 0;
    for c1 in '0'..='9' {
        for c2 in 'A'..='Z' {
            let key = format!("{}{}", c1, c2);
            let value_idx = rng.gen_range(0..18);
            let value = preset_values[value_idx].to_string();
            values2[i] = value.clone();
            
            db.write(&key, &value).expect("Failed to write key-value");
            i += 1;
        }
    }
    
    // Verify all numeric keys
    i = 0;
    let mut buffer = vec![0u8; 257];
    for c1 in '0'..='9' {
        for c2 in '0'..='9' {
            for c3 in '0'..='9' {
                let key = format!("{}{}{}", c1, c2, c3);
                match values[i] {
                    None => {
                        // This key was deleted, should get KeyNotFound
                        match db.read(&key, &mut buffer) {
                            Err(MdbError::KeyNotFound) => (), // Expected
                            Ok(_) => panic!("Expected KeyNotFound for deleted key {}", key),
                            Err(e) => panic!("Unexpected error: {:?}", e),
                        }
                    },
                    Some(ref expected_value) => {
                        // Should be able to read this key
                        let read_size = db.read(&key, &mut buffer).expect("Failed to read value");
                        let read_str = String::from_utf8_lossy(&buffer[0..read_size]).to_string();
                        assert_eq!(expected_value, &read_str);
                    }
                }
                i += 1;
            }
        }
    }
    
    // Verify all alphanumeric keys
    i = 0;
    for c1 in '0'..='9' {
        for c2 in 'A'..='Z' {
            let key = format!("{}{}", c1, c2);
            let read_size = db.read(&key, &mut buffer).expect("Failed to read value");
            let read_str = String::from_utf8_lossy(&buffer[0..read_size]).to_string();
            assert_eq!(values2[i], read_str);
            i += 1;
        }
    }
    
    // Close the database (implicitly through drop)
    drop(db);
    
    println!("Finished gabriel load test");
    
    // Clean up
    cleanup_db("gabriel");
}


fn main() {
    println!("Running MDB tests...");
    // In Rust, we would use the built-in test framework
    // This main function could be used for manual testing if desired
}