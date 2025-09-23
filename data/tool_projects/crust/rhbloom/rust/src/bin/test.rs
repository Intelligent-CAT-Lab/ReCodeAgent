use rhbloom::rhbloom::{RHBloom};
use std::time::{Instant};
use std::fmt::Write;

fn murmurhash2(key: &[u8], seed: u32) -> u32 {
    let m = 0x5bd1e995;
    let r = 24;
    let mut h = seed ^ key.len() as u32;
    let mut data = key;
    while data.len() >= 4 {
        let mut k = u32::from_le_bytes(data[0..4].try_into().unwrap());
        k = k.wrapping_mul(m);
        k ^= k >> r;
        k = k.wrapping_mul(m);
        h = h.wrapping_mul(m);
        h ^= k;
        data = &data[4..];
    }
    match data.len() {
        3 => h ^= (data[2] as u32) << 16,
        2 => h ^= (data[1] as u32) << 8,
        1 => {
            h ^= data[0] as u32;
            h = h.wrapping_mul(m);
        }
        _ => {}
    }
    h ^= h >> 13;
    h = h.wrapping_mul(m);
    h ^= h >> 15;
    h
}

fn hash(x: i32) -> u64 {
    murmurhash2(&x.to_le_bytes(), 0) as u64
}

fn now() -> f64 {
    let now = Instant::now();
    now.elapsed().as_secs_f64()
}

fn commaize(n: u32) -> String {
    let mut s1 = n.to_string();
    let mut s2 = String::new();
    let mut j = 0;
    for c in s1.chars().rev() {
        if j % 3 == 0 && j != 0 {
            s2.insert(0, ',');
        }
        s2.insert(0, c);
        j += 1;
    }
    s2
}

fn bench_print(n: u32, start: f64, end: f64) {
    let elapsed = end - start;
    let nsop = elapsed / (n as f64) * 1e9;
    let pops = commaize(n);
    let psec = commaize((n as f64 / elapsed) as u32);
    println!(
        "{} ops in {:.3} secs {:6.1} ns/op {:13} op/sec",
        pops, elapsed, nsop, psec
    );
}

fn test_step(rhbloom: &mut RHBloom, n: i32, p: f64) {
    let nn = n + 1;
    for i in 0..nn {
        if !rhbloom.upgraded() {
            assert!(!rhbloom.test(hash(i)));
        }
        rhbloom.add(hash(i));
        if !rhbloom.upgraded() {
            assert!(rhbloom.test(hash(i)));
        }
    }
    assert!(rhbloom.upgraded());
    let mut hits = 0;
    for i in 0..nn {
        if rhbloom.test(hash(i)) {
            hits += 1;
        }
    }
    assert_eq!(hits, nn);
    hits = 0;
    for i in nn..nn * 2 {
        if rhbloom.test(hash(i)) {
            hits += 1;
        }
    }
    if (hits as f64 / n as f64 - p).abs() > 0.1 && n > 0 {
        println!(
            "n={} p={} hits={} \t({})",
            n, p, hits, hits as f64 / n as f64
        );
        println!(" ({})", hits as f64 / n as f64 - p);
        panic!("bad probability");
    }
}

#[test]
fn test() {
    for n in (0..100000).step_by(1000) {
        for p in (1..70).map(|x| x as f64 / 100.0) {
            let mut rhbloom = RHBloom::new(n as usize, p);
            test_step(&mut rhbloom, n, p);
            println!("-- clear --");
            rhbloom.clear();
            test_step(&mut rhbloom, n, p);
        }
    }
    println!("PASSED");
}

fn bench(args: Vec<String>) {
    let mut n = 1000000;
    let mut p = 0.01;
    if args.len() > 2 {
        n = args[2].parse().unwrap();
    }
    if args.len() > 3 {
        p = args[3].parse().unwrap();
    }
    let mut hashes: Vec<u64> = (0..n * 2).map(|i| hash(i as i32)).collect();
    let mut rhbloom = RHBloom::new(n, p);
    let mut start;
    let mut misses = 0;
    for j in 0..2 {
        if j > 0 {
            println!("-- clear --");
            rhbloom.clear();
        }
        println!("add          ");
        start = now();
        for i in 0..n {
            rhbloom.add(hashes[i]);
        }
        bench_print(n as u32, start, now());

        println!("test (yes)   ");
        start = now();
        for i in 0..n {
            assert!(rhbloom.test(hashes[i]));
        }
        bench_print(n as u32, start, now());

        println!("test (no)    ");
        misses = 0;
        start = now();
        for i in n..n * 2 {
            if rhbloom.test(hashes[i]) {
                misses += 1;
            }
        }
        bench_print(n as u32, start, now());
    }
    println!(
        "Misses {} ({:.4}% false-positive)",
        misses,
        misses as f64 / n as f64 * 100.0
    );
    println!("Memory {:.2} MB", rhbloom.memsize() as f64 / 1024.0 / 1024.0);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "bench" {
        bench(args);
    } else {
        // test();
    }
}
