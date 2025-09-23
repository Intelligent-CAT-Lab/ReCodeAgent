use gfc::gfc::GFC; // Assuming the `GFC` struct is in a `gfc` module
use std::time::Instant;

/// Benchmark decryption performance
fn benchmark_dec(range: u64, rounds: u64, seed: u64, count: u64) -> f64 {
    let gfc = GFC::new(range, rounds, seed);

    let start = Instant::now();
    for i in 0..count {
        let _ = gfc.decrypt(i);
    }
    let duration = start.elapsed();
    
    duration.as_secs_f64() // Convert to seconds
}

/// Benchmark encryption performance
fn benchmark_enc(range: u64, rounds: u64, seed: u64, count: u64) -> f64 {
    let gfc = GFC::new(range, rounds, seed);

    let start = Instant::now();
    for i in 0..count {
        let _ = gfc.encrypt(i);
    }
    let duration = start.elapsed();

    duration.as_secs_f64() // Convert to seconds
}

fn main() {
    println!("range,mode,time");

    for i in 20..64 {
        let range = 2u64.pow(i);
        for j in 0..3 {
            let dec_s = benchmark_dec(range, 6, j, 1_048_576);
            let enc_s = benchmark_enc(range, 6, j, 1_048_576);
            println!("{},dec,{:.6}", range, dec_s);
            println!("{},enc,{:.6}", range, enc_s);
        }
    }
}
