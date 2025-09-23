use Simple_Sparsehash::simple_sparsehash;
use std::convert::TryInto;

// --- Helper function ---
/// Converts an i32 into a byte slice.
fn i32_to_bytes(n: i32) -> [u8; std::mem::size_of::<i32>()] {
    n.to_ne_bytes()
}

// --- Tests corresponding to test.c ---

#[test]
fn test_empty_array_does_not_blow_up() {
    let arr = simple_sparsehash::sparse_array_init(std::mem::size_of::<u64>(), 32)
        .expect("sparse_array_init failed to create an array");
    // Expect that getting index 0 returns None.
    assert!(simple_sparsehash::sparse_array_get(&arr, 0, None).is_none());
    assert!(simple_sparsehash::sparse_array_free(arr) != 0);
}

#[test]
fn test_cannot_set_outside_bounds() {
    let mut arr = simple_sparsehash::sparse_array_init(std::mem::size_of::<u64>(), 32)
        .expect("sparse_array_init failed");
    let test_num: u64 = 666;
    let test_bytes = test_num.to_ne_bytes();
    // Expect failure (0) when setting index 35 (out of bounds).
    assert_eq!(
        simple_sparsehash::sparse_array_set(&mut arr, 35, &test_bytes, std::mem::size_of::<u64>()),
        0,
        "Expected setting outside bounds to return 0"
    );
    assert!(simple_sparsehash::sparse_array_free(arr) != 0);
}

#[test]
fn test_cannot_get_outside_bounds() {
    let arr = simple_sparsehash::sparse_array_init(std::mem::size_of::<u64>(), 32)
        .expect("sparse_array_init failed");
    assert!(
        simple_sparsehash::sparse_array_get(&arr, 35, None).is_none(),
        "Expected getting outside bounds to return None"
    );
    assert!(simple_sparsehash::sparse_array_free(arr) != 0);
}

#[test]
fn test_cannot_set_bigger_elements() {
    let mut arr = simple_sparsehash::sparse_array_init(std::mem::size_of::<u8>(), 100)
        .expect("sparse_array_init failed");
    let test_num: u64 = 666;
    let test_bytes = test_num.to_ne_bytes();
    // Expect failure (0) when the value size is larger than element size.
    assert_eq!(
        simple_sparsehash::sparse_array_set(&mut arr, 0, &test_bytes, std::mem::size_of::<u64>()),
        0,
        "Expected setting element with bigger size to return 0"
    );
    assert!(simple_sparsehash::sparse_array_free(arr) != 0);
}

#[test]
fn test_array_set_backwards() {
    let array_size = 120;
    let mut arr =
        simple_sparsehash::sparse_array_init(std::mem::size_of::<i32>(), array_size as u32)
            .expect("sparse_array_init failed");

    // Set values in reverse order.
    for i in (0..array_size).rev() {
        let i_val = i as i32;
        let bytes = i32_to_bytes(i_val);
        assert!(
            simple_sparsehash::sparse_array_set(
                &mut arr,
                i as u32,
                &bytes,
                std::mem::size_of::<i32>()
            ) != 0,
            "Failed to set index {}",
            i
        );
        let mut size: usize = 0;
        let returned = simple_sparsehash::sparse_array_get(&arr, i as u32, Some(&mut size))
            .expect("Expected value");
        assert_eq!(returned.len(), std::mem::size_of::<i32>());
        let retrieved = i32::from_ne_bytes(returned.try_into().unwrap());
        assert_eq!(retrieved, i_val);
        assert_eq!(size, std::mem::size_of::<i32>());
    }

    // Retrieve values in reverse order.
    for i in (0..array_size).rev() {
        let mut size: usize = 0;
        let returned = simple_sparsehash::sparse_array_get(&arr, i as u32, Some(&mut size))
            .expect("Expected value");
        let retrieved = i32::from_ne_bytes(returned.try_into().unwrap());
        assert_eq!(retrieved, i as i32);
        assert_eq!(size, std::mem::size_of::<i32>());
    }

    assert!(simple_sparsehash::sparse_array_free(arr) != 0);
}

#[test]
fn test_array_set() {
    let array_size = 130;
    let mut arr =
        simple_sparsehash::sparse_array_init(std::mem::size_of::<i32>(), array_size as u32)
            .expect("sparse_array_init failed");

    for i in 0..array_size {
        let i_val = i as i32;
        let bytes = i32_to_bytes(i_val);
        assert!(
            simple_sparsehash::sparse_array_set(
                &mut arr,
                i as u32,
                &bytes,
                std::mem::size_of::<i32>()
            ) != 0,
            "Failed to set index {}",
            i
        );
        let mut size: usize = 0;
        let returned = simple_sparsehash::sparse_array_get(&arr, i as u32, Some(&mut size))
            .expect("Expected value");
        assert_eq!(returned.len(), std::mem::size_of::<i32>());
        let retrieved = i32::from_ne_bytes(returned.try_into().unwrap());
        assert_eq!(retrieved, i_val);
        assert_eq!(size, std::mem::size_of::<i32>());
    }

    for i in 0..array_size {
        let mut size: usize = 0;
        let returned = simple_sparsehash::sparse_array_get(&arr, i as u32, Some(&mut size))
            .expect("Expected value");
        let retrieved = i32::from_ne_bytes(returned.try_into().unwrap());
        assert_eq!(retrieved, i as i32);
        assert_eq!(size, std::mem::size_of::<i32>());
    }

    assert!(simple_sparsehash::sparse_array_free(arr) != 0);
}

#[test]
fn test_array_set_high_num() {
    let test_num: i32 = 65_555_555;
    let index = simple_sparsehash::GROUP_SIZE - 1;
    let mut arr = simple_sparsehash::sparse_array_init(std::mem::size_of::<i32>(), 140)
        .expect("sparse_array_init failed");

    let bytes = i32_to_bytes(test_num);
    assert!(
        simple_sparsehash::sparse_array_set(
            &mut arr,
            index as u32,
            &bytes,
            std::mem::size_of::<i32>()
        ) != 0,
        "Failed to set at high index"
    );
    let mut size: usize = 0;
    let returned = simple_sparsehash::sparse_array_get(&arr, index as u32, Some(&mut size))
        .expect("Expected value");
    let retrieved = i32::from_ne_bytes(returned.try_into().unwrap());
    assert_eq!(retrieved, test_num);
    assert_eq!(size, std::mem::size_of::<i32>());

    assert!(simple_sparsehash::sparse_array_free(arr) != 0);
}

#[test]
fn test_array_set_overwrites_old_values() {
    let test_num: i32 = 666;
    let test_num2: i32 = 1024;
    let mut arr = simple_sparsehash::sparse_array_init(std::mem::size_of::<i32>(), 150)
        .expect("sparse_array_init failed");

    let bytes1 = i32_to_bytes(test_num);
    let bytes2 = i32_to_bytes(test_num2);
    assert!(
        simple_sparsehash::sparse_array_set(&mut arr, 0, &bytes1, std::mem::size_of::<i32>()) != 0,
        "Failed to set first value"
    );
    assert!(
        simple_sparsehash::sparse_array_set(&mut arr, 0, &bytes2, std::mem::size_of::<i32>()) != 0,
        "Failed to overwrite value"
    );

    let returned = simple_sparsehash::sparse_array_get(&arr, 0, None).expect("Expected value");
    let retrieved = i32::from_ne_bytes(returned.try_into().unwrap());
    assert_eq!(retrieved, test_num2);

    assert!(simple_sparsehash::sparse_array_free(arr) != 0);
}

#[test]
fn test_array_get() {
    let test_num: i32 = 666;
    let mut arr = simple_sparsehash::sparse_array_init(std::mem::size_of::<i32>(), 200)
        .expect("sparse_array_init failed");
    let bytes = i32_to_bytes(test_num);
    assert!(
        simple_sparsehash::sparse_array_set(&mut arr, 0, &bytes, std::mem::size_of::<i32>()) != 0,
        "Failed to set value"
    );

    let mut size: usize = 0;
    let returned =
        simple_sparsehash::sparse_array_get(&arr, 0, Some(&mut size)).expect("Expected value");
    let retrieved = i32::from_ne_bytes(returned.try_into().unwrap());
    assert_eq!(retrieved, test_num);
    assert_eq!(size, std::mem::size_of::<i32>());

    assert!(simple_sparsehash::sparse_array_free(arr) != 0);
}

#[test]
fn test_dict_set() {
    let mut dict = simple_sparsehash::sparse_dict_init().expect("sparse_dict_init failed");
    assert!(
        simple_sparsehash::sparse_dict_set(
            &mut dict,
            "key",
            "key".len(),
            "value".as_bytes(),
            "value".len()
        ) != 0,
        "Failed to set dictionary entry"
    );
    assert!(simple_sparsehash::sparse_dict_free(dict) != 0);
}

#[test]
fn test_dict_get() {
    let mut dict = simple_sparsehash::sparse_dict_init().expect("sparse_dict_init failed");
    assert!(
        simple_sparsehash::sparse_dict_set(
            &mut dict,
            "key",
            "key".len(),
            "value".as_bytes(),
            "value".len()
        ) != 0,
        "Failed to set dictionary entry"
    );

    let mut outsize: usize = 0;
    let value = simple_sparsehash::sparse_dict_get(&dict, "key", "key".len(), Some(&mut outsize))
        .expect("Expected value");
    assert_eq!(outsize, "value".len());
    assert_eq!(value, "value".as_bytes());
    assert!(simple_sparsehash::sparse_dict_free(dict) != 0);
}

#[test]
fn test_dict_lots_of_set() {
    let mut dict = simple_sparsehash::sparse_dict_init().expect("sparse_dict_init failed");
    let iterations = 1_000_000;
    for i in 0..iterations {
        let key = format!("crazy hash{}", i);
        let val = format!("value{}", i);
        assert!(
            simple_sparsehash::sparse_dict_set(
                &mut dict,
                &key,
                key.len(),
                val.as_bytes(),
                val.len()
            ) != 0,
            "Failed to set dict entry at iteration {}",
            i
        );
        // Check that the bucket count matches the expected number.
        assert_eq!(
            dict.bucket_count,
            i as usize + 1,
            "Bucket count mismatch at iteration {}",
            i
        );
        let mut outsize = 0;
        let retrieved =
            simple_sparsehash::sparse_dict_get(&dict, &key, key.len(), Some(&mut outsize))
                .expect("Expected value");
        assert_eq!(outsize, val.len());
        assert_eq!(retrieved, val.as_bytes());
    }

    for i in (0..iterations).rev() {
        let key = format!("crazy hash{}", i);
        let val = format!("value{}", i);
        let mut outsize = 0;
        let retrieved =
            simple_sparsehash::sparse_dict_get(&dict, &key, key.len(), Some(&mut outsize))
                .expect("Expected value");
        assert_eq!(outsize, val.len());
        assert_eq!(retrieved, val.as_bytes());
    }

    assert!(simple_sparsehash::sparse_dict_free(dict) != 0);
}
fn main(){}
