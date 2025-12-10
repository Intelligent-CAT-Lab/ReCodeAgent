// tests/gtin_test.rs
// GTIN tests

use wspace::checkdigit::*;
use wspace::gtin::gtin;

#[test]
fn TestGtin_Verify() {
    struct TestCase<'a> {
        name: &'a str,
        input: &'a str,
        expected: bool,
        provider: Box<dyn Provider>,
    }

    let cases = vec![
        TestCase {
            name: "EAN8",
            provider: new_ean8().unwrap(),
            input: "96385074",
            expected: true,
        },
        TestCase {
            name: "EAN13",
            provider: new_ean13(),
            input: "5901234123457",
            expected: true,
        },
        TestCase {
            name: "JAN8",
            provider: Box::new(gtin::new_jan8()),
            input: "49968712",
            expected: true,
        },
        TestCase {
            name: "JAN13",
            provider: new_jan13(),
            input: "4569951116179",
            expected: true,
        },
        TestCase {
            name: "ITF",
            provider: new_itf().unwrap(),
            input: "14569951116176",
            expected: true,
        },
        TestCase {
            name: "UPC",
            provider: new_upc(),
            input: "012345678905",
            expected: true,
        },
        TestCase {
            name: "SSCC",
            provider: new_sscc(),
            input: "045699511100000016",
            expected: true,
        },
        TestCase {
            name: "Empty",
            provider: new_ean8().unwrap(),
            input: "",
            expected: false,
        },
    ];

    for case in cases {
        let ret = case.provider.verify(case.input);
        assert_eq!(
            case.expected, ret,
            "Test '{}' failed: expected = {}, given = {}",
            case.name, case.expected, ret
        );
    }
}

#[test]
fn TestGtin_Generate() {
    struct TestCase<'a> {
        name: &'a str,
        input: &'a str,
        expected: i32,
        is_error: bool,
        provider: Box<dyn Provider>,
    }

    let cases = vec![
        TestCase {
            name: "EAN8",
            provider: new_ean8().unwrap(),
            input: "9638112",
            expected: 0,
            is_error: false,
        },
        TestCase {
            name: "EAN13",
            provider: new_ean13(),
            input: "590123412345",
            expected: 7,
            is_error: false,
        },
        TestCase {
            name: "JAN8",
            provider: Box::new(gtin::new_jan8()),
            input: "4996871",
            expected: 2,
            is_error: false,
        },
        TestCase {
            name: "JAN13",
            provider: new_jan13(),
            input: "456995111617",
            expected: 9,
            is_error: false,
        },
        TestCase {
            name: "ITF",
            provider: new_itf().unwrap(),
            input: "1456995111617",
            expected: 6,
            is_error: false,
        },
        TestCase {
            name: "UPC",
            provider: new_upc(),
            input: "01234567890",
            expected: 5,
            is_error: false,
        },
        TestCase {
            name: "SSCC",
            provider: new_sscc(),
            input: "04569951110000001",
            expected: 6,
            is_error: false,
        },
        TestCase {
            name: "Empty",
            provider: new_ean8().unwrap(),
            input: "",
            expected: 0,
            is_error: true,
        },
        TestCase {
            name: "Alphabet",
            provider: new_ean8().unwrap(),
            input: "aaaaaaa",
            expected: 0,
            is_error: true,
        },
    ];

    for case in cases {
        let result = case.provider.generate(case.input);

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
