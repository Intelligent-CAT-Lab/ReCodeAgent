use gfc::gfc::GFC; // Assuming the `GFC` struct is in a `gfc` module.

fn test_range(range: u64, rounds: u64) {
    if range == 0 {
        return; // No meaningful test for an empty range.
    }

    let mut counts = vec![0u64; range as usize];
    let gfc = GFC::new(range, rounds, 42);

    for i in 0..range {
        let enc = gfc.encrypt(i);
        let dec = gfc.decrypt(enc);
        
        // Ensure decryption returns the original value
        assert_eq!(dec, i, "Decryption failed for i = {}", i);
        
        // Count occurrences of encrypted values
        counts[enc as usize] += 1;
    }

    for (i, &count) in counts.iter().enumerate() {
        // Ensure each number appears exactly once
        assert_eq!(count, 1, "Incorrect count for index {}: {}", i, count);
    }
}
#[test]
fn test() {
    test_range(0, 0);
    test_range(0, 1);
    test_range(1, 0);
    test_range(1, 1);
    test_range(10, 1);
    test_range(100, 1);
    test_range(100, 6);
    test_range(500 * 1000, 6);
}
fn main(){}
