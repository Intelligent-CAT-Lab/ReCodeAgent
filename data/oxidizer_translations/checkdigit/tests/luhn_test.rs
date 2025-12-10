// tests/luhn_test.rs
// Luhn algorithm tests

use wspace::checkdigit::*;

#[test]
fn TestLuhn_Verify() {
    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: bool,
    }

    let cases = vec![
        TestCase {
            name: "Regular 1",
            input: "4242424242424242",
            expected: true,
        },
        TestCase {
            name: "Regular 2",
            input: "5105105105105100",
            expected: true,
        },
        TestCase {
            name: "Regular 3",
            input: "34",
            expected: true,
        },
        TestCase {
            name: "Irregular 1",
            input: "510510510510511",
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
        let ret = new_luhn().verify(case.input);
        assert_eq!(
            case.expected, ret,
            "Test '{}' failed: expected = {}, given = {}",
            case.name, case.expected, ret
        );
    }
}

#[test]
fn TestLuhn_Generate() {
    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: i32,
        is_error: bool,
    }

    let cases = vec![
        TestCase {
            name: "Regular 1",
            input: "424242424242424",
            expected: 2,
            is_error: false,
        },
        TestCase {
            name: "Regular 2",
            input: "510510510510510",
            expected: 0,
            is_error: false,
        },
        TestCase {
            name: "Regular 3",
            input: "37144963539843",
            expected: 1,
            is_error: false,
        },
        TestCase {
            name: "Regular 4",
            input: "3056930902590",
            expected: 4,
            is_error: false,
        },
        TestCase {
            name: "Regular 5",
            input: "353011133330000",
            expected: 0,
            is_error: false,
        },
        TestCase {
            name: "Regular 6",
            input: "3",
            expected: 4,
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
        let result = new_luhn().generate(case.input);

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
