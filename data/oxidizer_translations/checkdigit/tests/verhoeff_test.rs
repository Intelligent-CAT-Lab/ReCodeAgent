// tests/verhoeff_test.rs
// Verhoeff algorithm tests

use wspace::verhoeff::Verhoeff;
use wspace::__synthetic::{__Synth0__generate, __Synth1__verify};

#[test]
fn TestVerhoeff_Verify() {
    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: bool,
    }

    let cases = vec![
        TestCase {
            name: "Regular 1",
            input: "938472210036",
            expected: true,
        },
        TestCase {
            name: "Regular 2",
            input: "0973652",
            expected: true,
        },
        TestCase {
            name: "Regular 3",
            input: "27",
            expected: true,
        },
        TestCase {
            name: "Irregular 1",
            input: "2361",
            expected: false,
        },
        TestCase {
            name: "Irregular 2",
            input: "",
            expected: false,
        },
        TestCase {
            name: "Irregular 3",
            input: "a",
            expected: false,
        },
    ];

    for case in cases {
        let ret = Verhoeff::new_verhoeff().verify(case.input);
        assert_eq!(
            case.expected, ret,
            "Test '{}' failed: expected = {}, given = {}",
            case.name, case.expected, ret
        );
    }
}

#[test]
fn TestVerhoeff_Generate() {
    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: i32,
        is_error: bool,
    }

    let cases = vec![
        TestCase {
            name: "Regular 1",
            input: "236",
            expected: 3,
            is_error: false,
        },
        TestCase {
            name: "Regular 2",
            input: "097365",
            expected: 2,
            is_error: false,
        },
        TestCase {
            name: "Regular 3",
            input: "93847221003",
            expected: 6,
            is_error: false,
        },
        TestCase {
            name: "Regular 4",
            input: "2",
            expected: 7,
            is_error: false,
        },
        TestCase {
            name: "Irregular 1",
            input: "",
            expected: 0,
            is_error: true,
        },
        TestCase {
            name: "Irregular 2",
            input: "a",
            expected: 0,
            is_error: true,
        },
    ];

    for case in cases {
        let result = Verhoeff::new_verhoeff().generate(case.input);

        if case.is_error {
            assert!(
                result.is_err(),
                "Test '{}' failed: expected error but got success",
                case.name
            );
        } else {
            let r = result.expect(&format!("Test '{}' failed: unexpected error", case.name));
            assert_eq!(
                case.expected, r,
                "Test '{}' failed: expected = {}, given = {}",
                case.name, case.expected, r
            );
        }
    }
}
