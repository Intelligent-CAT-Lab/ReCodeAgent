// Translation of entropy_test.go
// Tests for Entropy function

use wspace::data::Float64Data;
use wspace::entropy::entropy;

// Helper function for approximate float comparisons
fn tolerance(a: f64, b: f64, e: f64) -> bool {
    // Multiplying by e here can underflow denormal values to zero.
    // Check a==b so that at least if a and b are small and identical
    // we say they match.
    if a == b {
        return true;
    }
    let mut d = a - b;
    if d < 0.0 {
        d = -d;
    }

    // note: b is correct (expected) value, a is actual value.
    // make error tolerance a fraction of b, not a.
    let mut e = e;
    if b != 0.0 {
        e = e * b;
        if e < 0.0 {
            e = -e;
        }
    }
    d < e
}

fn veryclose(a: f64, b: f64) -> bool {
    tolerance(a, b, 4e-16)
}

fn makeFloatSlice(c: usize) -> Vec<f64> {
    let mut lf = Vec::with_capacity(c);
    for i in 0..c {
        let f = (i * 100) as f64;
        lf.push(f);
    }
    lf
}

#[test]
fn ExampleEntropy() {
    let d = Float64Data::from(vec![1.1, 2.2, 3.3]);
    let e = entropy(d.clone()).unwrap();
    // In GO: fmt.Println(e)
    // Output: 1.0114042647073518
    assert!(veryclose(e, 1.0114042647073518));
}

#[test]
fn TestEntropy() {
    struct TestCase {
        input: Float64Data,
        expected: f64,
    }

    let test_cases = vec![
        TestCase {
            input: Float64Data::from(vec![4.0, 8.0, 5.0, 1.0]),
            expected: 1.2110440167801229,
        },
        TestCase {
            input: Float64Data::from(vec![0.8, 0.01, 0.4]),
            expected: 0.6791185708986585,
        },
        TestCase {
            input: Float64Data::from(vec![0.8, 1.1, 0.0, 5.0]),
            expected: 0.7759393943707658,
        },
    ];

    for c in test_cases {
        let got = entropy(c.input.clone());
        assert!(got.is_ok(), "Returned an error");
        let got = got.unwrap();
        assert!(
            veryclose(got, c.expected),
            "Max({:?}) => {} != {}",
            c.input,
            got,
            c.expected
        );
    }

    // Test empty slice
    let result = entropy(Float64Data::from(vec![]));
    assert!(result.is_err(), "Empty slice didn't return an error");
}

#[test]
#[ignore] // Benchmarks are ignored by default in Rust
fn BenchmarkEntropySmallFloatSlice() {
    // In Rust, benchmarks typically use the criterion crate or built-in bencher
    // For now, we'll just run the operation to ensure it compiles and works
    for _ in 0..1000 {
        let _ = entropy(Float64Data::from(makeFloatSlice(5)));
    }
}

#[test]
#[ignore] // Benchmarks are ignored by default in Rust
fn BenchmarkEntropyLargeFloatSlice() {
    // In Rust, benchmarks typically use the criterion crate or built-in bencher
    // For now, we'll just run the operation to ensure it compiles and works
    let lf = Float64Data::from(makeFloatSlice(100000));
    for _ in 0..100 {
        let _ = entropy(lf.clone());
    }
}
