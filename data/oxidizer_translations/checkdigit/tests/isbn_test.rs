// tests/isbn_test.rs
// ISBN tests

use wspace::checkdigit::*;

#[test]
fn TestIsbn10_Verify() {
    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: bool,
    }

    let cases = vec![
        TestCase {
            name: "Regular 1",
            input: "0026515628",
            expected: true,
        },
        TestCase {
            name: "Regular 2",
            input: "007231592X",
            expected: true,
        },
        TestCase {
            name: "Regular 3",
            input: "155860832X",
            expected: true,
        },
        TestCase {
            name: "Irregular 1",
            input: "155860831X",
            expected: false,
        },
        TestCase {
            name: "Irregular 2",
            input: "9780002715096",
            expected: false,
        },
        TestCase {
            name: "Irregular 3",
            input: "155860831",
            expected: false,
        },
        TestCase {
            name: "Irregular 4",
            input: "",
            expected: false,
        },
        TestCase {
            name: "Irregular 5",
            input: "aaaaaaaaaa",
            expected: false,
        },
    ];

    for case in cases {
        let ret = NewISBN10().verify(case.input);
        assert_eq!(
            case.expected, ret,
            "Test '{}' failed: expected = {}, given = {}",
            case.name, case.expected, ret
        );
    }
}

#[test]
fn TestIsbn10_Generate() {
    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: i32,
        is_error: bool,
    }

    let cases = vec![
        TestCase {
            name: "Regular 1",
            input: "002651562",
            expected: 8,
            is_error: false,
        },
        TestCase {
            name: "Regular 2",
            input: "007231592",
            expected: 10,
            is_error: false,
        },
        TestCase {
            name: "Regular 3",
            input: "155860832",
            expected: 10,
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
            input: "9780002715096",
            expected: 0,
            is_error: true,
        },
        TestCase {
            name: "Irregular 3",
            input: "15586",
            expected: 0,
            is_error: true,
        },
        TestCase {
            name: "Irregular 4",
            input: "155860832X",
            expected: 0,
            is_error: true,
        },
        TestCase {
            name: "Irregular 5",
            input: "aaaaaaaaa",
            expected: 0,
            is_error: true,
        },
    ];

    for case in cases {
        let result = NewISBN10().generate(case.input);

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

#[test]
fn TestIsbn13_Verify() {
    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: bool,
    }

    let cases = vec![
        TestCase {
            name: "Regular 1",
            input: "9780002712095",
            expected: true,
        },
        TestCase {
            name: "Regular 2",
            input: "9780002715096",
            expected: true,
        },
        TestCase {
            name: "Regular 3",
            input: "9780002713306",
            expected: true,
        },
        TestCase {
            name: "Irregular 1",
            input: "155860831X",
            expected: false,
        },
        TestCase {
            name: "Irregular 2",
            input: "9780002712520",
            expected: false,
        },
        TestCase {
            name: "Irregular 3",
            input: "9780002712709",
            expected: false,
        },
        TestCase {
            name: "Irregular 4",
            input: "",
            expected: false,
        },
        TestCase {
            name: "Irregular 5",
            input: "aaaaaaaaaaaaa",
            expected: false,
        },
    ];

    for case in cases {
        let ret = new_isbn13().unwrap().verify(case.input);
        assert_eq!(
            case.expected, ret,
            "Test '{}' failed: expected = {}, given = {}",
            case.name, case.expected, ret
        );
    }
}

#[test]
fn TestIsbn13_Generate() {
    struct TestCase {
        name: &'static str,
        input: &'static str,
        expected: i32,
        is_error: bool,
    }

    let cases = vec![
        TestCase {
            name: "Regular 1",
            input: "978000271217",
            expected: 0,
            is_error: false,
        },
        TestCase {
            name: "Regular 2",
            input: "978000271330",
            expected: 6,
            is_error: false,
        },
        TestCase {
            name: "Regular 3",
            input: "978000271363",
            expected: 4,
            is_error: false,
        },
        TestCase {
            name: "Regular 4",
            input: "978000271236",
            expected: 1,
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
            input: "0026515628",
            expected: 0,
            is_error: true,
        },
        TestCase {
            name: "Irregular 3",
            input: "155860832X",
            expected: 0,
            is_error: true,
        },
        TestCase {
            name: "Irregular 4",
            input: "a",
            expected: 0,
            is_error: true,
        },
    ];

    for case in cases {
        let result = new_isbn13().unwrap().generate(case.input);

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
