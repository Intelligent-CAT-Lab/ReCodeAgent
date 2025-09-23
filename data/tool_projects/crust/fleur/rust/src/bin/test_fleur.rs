use fleur;
use fleur::fleur::BloomFilter;
use fleur::fleur::Header;
use rand::Rng;
use std::fs::File;
use std::io::Read;
use std::path::Path;

struct Tester {
    bf: BloomFilter,
    buf: Vec<Vec<u8>>,
}

fn generate_test_value(length: u64) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen::<u8>()).collect()
}

fn generate_example_filter(capacity: u64, p: f64, samples: u64) -> Tester {
    let mut bf = BloomFilter {
        version: 1,
        datasize: 6,
        h: Header {
            version: 1,
            n: capacity,
            p,
            k: 0,
            m: 0,
            n_value: 0,
        },
        m: 0,
        v: Vec::new(),
        data: b"foobar".to_vec(),
        modified: 0,
        error: 0,
    };

    let mut test_bufs = Vec::with_capacity(samples as usize);
    for _ in 0..samples {
        let buf = generate_test_value(100);
        bf.add(buf.as_slice());
        test_bufs.push(buf);
    }

    Tester { bf, buf: test_bufs }
}

#[test]
fn test_reading_header() {
    let mut file = File::open("header.bin").expect("Failed to open header.bin");
    let mut header_bytes = Vec::new();
    file.read_to_end(&mut header_bytes)
        .expect("Failed to read header");

    let h = unsafe { std::ptr::read(header_bytes.as_ptr() as *const Header) };

    assert_eq!(h.version, 1);
    assert_eq!(h.n, 354253067);
    assert_eq!(h.p, 0.0001);
    assert_eq!(h.k, 14);
    assert_eq!(h.m, 6791072655);
    assert_eq!(h.n_value, 354249652);
    assert!(h.check());
}

#[test]
fn test_reading_full() {
    let file = File::open("datatest.bloom").expect("Failed to open datatest.bloom");
    let mut bf = BloomFilter {
        version: 0,
        datasize: 0,
        h: Header {
            version: 0,
            n: 0,
            p: 0.0,
            k: 0,
            m: 0,
            n_value: 0,
        },
        m: 0,
        v: Vec::new(),
        data: Vec::new(),
        modified: 0,
        error: 0,
    };

    assert_eq!(bf.error, 0);
    assert_eq!(bf.m, 450);
    println!("{:?}", bf.to_string());
}

#[test]
fn test_writing() {
    let path = Path::new("writing-test.bloom");
    let file = File::create(path).expect("Failed to create file");

    let tester = generate_example_filter(1000, 0.001, 100);
    let ret = tester.bf.to_file(file);
    assert_eq!(ret, 1);
}

#[test]
fn test_fingerprint() {
    let mut bf = BloomFilter {
        version: 1,
        datasize: 0,
        h: Header {
            version: 1,
            n: 100000,
            p: 0.01,
            k: 7,
            m: 0,
            n_value: 0,
        },
        m: 0,
        v: Vec::new(),
        data: Vec::new(),
        modified: 0,
        error: 0,
    };

    let input = "bar";
    let fp = bf.fingerprint(input.as_bytes());
    let expected = vec![20311, 36825, 412501, 835777, 658914, 853361, 307361];
    assert_eq!(fp, expected);
}

#[test]
fn test_checking() {
    let capacity = 100000;
    let p = 0.001;
    let samples = 100000;

    let tester = generate_example_filter(capacity, p, samples);

    for buf in &tester.buf {
        assert_eq!(tester.bf.check(buf), 1);
    }

    let not_in_filter = b"this is not in the filter";
    assert_eq!(tester.bf.check(not_in_filter), 0);
}

#[test]
fn test_joining() {
    let mut file1 = File::open("join1.bloom").expect("Failed to open join1.bloom");
    let mut file2 = File::open("join2.bloom").expect("Failed to open join2.bloom");
    let mut file3 = File::open("join3.bloom").expect("Failed to open join3.bloom");

    let mut j1 = BloomFilter {
        version: 0,
        datasize: 0,
        h: Header {
            version: 0,
            n: 0,
            p: 0.0,
            k: 0,
            m: 0,
            n_value: 0,
        },
        m: 0,
        v: Vec::new(),
        data: Vec::new(),
        modified: 0,
        error: 0,
    };

    let mut j2 = j1.clone();
    let mut j3 = j1.clone();

    assert_eq!(j2.join(&j1), -1);
    assert_eq!(j3.join(&j1), 1);
}

fn main() {
}
