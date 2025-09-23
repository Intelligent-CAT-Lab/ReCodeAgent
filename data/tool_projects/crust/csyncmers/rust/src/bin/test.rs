use csyncmers::closed_syncmers::{compute_closed_syncmers, MinimizerResult};
use csyncmers::closed_syncmers_naive::compute_closed_syncmers_naive;
use rand::Rng;

fn generate_sequence(len: usize) -> String {
    let bases = ['A', 'C', 'G', 'T'];
    let mut rng = rand::thread_rng();
    (0..len).map(|_| bases[rng.gen_range(0..4)]).collect()
}

#[test]
fn test() {
    let num_tests = 10;
    let sequence_length = 1050;

    for test in 1..=num_tests {
        let sequence = generate_sequence(sequence_length);
        let a = rand::thread_rng().gen_range(11..=999);
        let b_min = 7;
        let b_max = if a - 1 < 63 { a - 1 } else { 63 };
        let b = if b_max < b_min { b_min } else { rand::thread_rng().gen_range(b_min..=b_max) };

        println!("Test {}: Sequence Length={}, K={}, S={}", test, sequence_length, a, b);

        let mut results = Vec::new();
        let mut num_results = 0;
        compute_closed_syncmers(&sequence, sequence.len() as i32, a, b, &mut results, &mut num_results);

        println!("Closed Syncmers:");
        println!("{:<20} {:<20}", "Position", "Minimizer Hash");
        for result in &results {
            println!("{:<20} {:<20}", result.kmer_position, result.minimizer_hash);
        }

        let mut naive_results = Vec::new();
        let mut num_naive_results = 0;
        compute_closed_syncmers_naive(&sequence, sequence.len(), a, b, &mut naive_results, &mut num_naive_results);

        println!("\nClosed Syncmers (Naive):");
        println!("{:<20} {:<20}", "Position", "Minimizer Hash");
        for result in &naive_results {
            println!("{:<20} {:<20}", result.kmer_position, result.minimizer_hash);
        }

        assert_eq!(num_results, num_naive_results, "Mismatch in number of closed syncmers: {} (original) vs {} (naive)", num_results, num_naive_results);

        for (i, (result, naive_result)) in results.iter().zip(naive_results.iter()).enumerate() {
            assert_eq!(result.kmer_position, naive_result.kmer_position, "Mismatch at index {}: Position", i);
            assert_eq!(result.minimizer_hash, naive_result.minimizer_hash, "Mismatch at index {}: Hash", i);
        }

        println!("Test {} passed. All closed syncmers match between original and naive method.\n", test);
    }

    println!("All tests passed.");
}

fn main(){}