// tests/damm_test.rs
// Damm algorithm tests

use wspace::damm::Damm;
use wspace::__synthetic::{__Synth0__generate, __Synth1__verify};

#[test]
fn TestDamm_Verify() {
    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: bool,
    }

    let cases = vec![
        TestCase {
            name: "Regular 1",
            input: "224564332323",
            expected: true,
        },
        TestCase {
            name: "Regular 2",
            input: "543525432346",
            expected: true,
        },
        TestCase {
            name: "Regular 3",
            input: "37",
            expected: true,
        },
        TestCase {
            name: "Irregular 1",
            input: "835323233227",
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
        let damm = Damm::new_damm().expect("failed to create damm");
        let ret = damm.verify(case.input);
        assert_eq!(
            case.expected, ret,
            "Test '{}' failed: expected = {}, given = {}",
            case.name, case.expected, ret
        );
    }
}

#[test]
fn TestDamm_Generate() {
    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: i32,
        is_error: bool,
    }

    let cases = vec![
        TestCase {
            name: "Regular 1",
            input: "22456433232",
            expected: 3,
            is_error: false,
        },
        TestCase {
            name: "Regular 2",
            input: "54352543234",
            expected: 6,
            is_error: false,
        },
        TestCase {
            name: "Regular 3",
            input: "10493839530",
            expected: 5,
            is_error: false,
        },
        TestCase {
            name: "Regular 4",
            input: "08989435403",
            expected: 5,
            is_error: false,
        },
        TestCase {
            name: "Regular 5",
            input: "54994384990",
            expected: 4,
            is_error: false,
        },
        TestCase {
            name: "Regular 6",
            input: "3",
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
        let damm = Damm::new_damm().expect("failed to create damm");
        let result = damm.generate(case.input);

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
