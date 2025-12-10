// tests/checkdigit_test.rs
// Constructor and example tests

use wspace::checkdigit::*;
use wspace::damm::Damm;
use wspace::verhoeff::Verhoeff;
use wspace::gtin::gtin;
use wspace::__synthetic::{__Synth0__generate, __Synth1__verify};

#[test]
fn TestNewLuhn() {
    let p = new_luhn();
    assert!(true);
}

#[test]
fn TestNewDamm() {
    let p = Damm::new_damm();
    assert!(true);
}

#[test]
fn TestNewVerhoeff() {
    let p = Verhoeff::new_verhoeff();
    assert!(true);
}

#[test]
fn TestNewISBN10() {
    let v = NewISBN10();
    assert!(true);
}

#[test]
fn TestNewISBN13() {
    let p = new_isbn13();
    assert!(true);
}

#[test]
fn TestNewEAN8() {
    let p = new_ean8();
    assert!(true);
}

#[test]
fn TestNewEAN13() {
    let p = new_ean13();
    assert!(true);
}

#[test]
fn TestNewJAN8() {
    let p = gtin::new_jan8();
    assert!(true);
}

#[test]
fn TestNewJAN13() {
    let p = new_jan13();
    assert!(true);
}

#[test]
fn TestNewITF() {
    let p = new_itf();
    assert!(true);
}

#[test]
fn TestNewUPC() {
    let p = new_upc();
    assert!(true);
}

#[test]
fn TestNewSSCC() {
    let p = new_sscc();
    assert!(true);
}

#[test]
fn ExampleNewLuhn() {
    let p = new_luhn();

    let seed = "411111111111111";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let ok = p.verify(&format!("{}{}", seed, cd));
    println!("seed: {}, check digit: {}, verify: {}", seed, cd, ok);

    assert_eq!(cd, 1);
    assert!(ok);
}

#[test]
fn ExampleNewDamm() {
    let p = Damm::new_damm().expect("failed to create damm provider");

    let seed = "572";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let ok = p.verify(&format!("{}{}", seed, cd));
    println!("seed: {}, check digit: {}, verify: {}", seed, cd, ok);

    assert_eq!(cd, 4);
    assert!(ok);
}

#[test]
fn ExampleNewVerhoeff() {
    let p = Verhoeff::new_verhoeff();

    let seed = "236";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let ok = p.verify(&format!("{}{}", seed, cd));
    println!("seed: {}, check digit: {}, verify: {}", seed, cd, ok);

    assert_eq!(cd, 3);
    assert!(ok);
}

#[test]
fn ExampleNewISBN10() {
    let p = NewISBN10();

    let seed = "155860832";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let digit = if cd == 10 {
        "X".to_string()
    } else {
        cd.to_string()
    };

    let ok = p.verify(&format!("{}{}", seed, digit));
    println!("seed: {}, check digit: {}, verify: {}", seed, digit, ok);

    assert_eq!(cd, 10);
    assert_eq!(digit, "X");
    assert!(ok);
}

#[test]
fn ExampleNewISBN13() {
    let p = new_isbn13().expect("failed to create isbn13 provider");

    let seed = "978000271217";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let ok = p.verify(&format!("{}{}", seed, cd));
    println!("seed: {}, check digit: {}, verify: {}", seed, cd, ok);

    assert_eq!(cd, 0);
    assert!(ok);
}

#[test]
fn ExampleNewEAN8() {
    let p = new_ean8().expect("failed to create ean8 provider");

    let seed = "9638507";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let ok = p.verify(&format!("{}{}", seed, cd));
    println!("seed: {}, check digit: {}, verify: {}", seed, cd, ok);

    assert_eq!(cd, 4);
    assert!(ok);
}

#[test]
fn ExampleNewEAN13() {
    let p = new_ean13();

    let seed = "590123412345";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let ok = p.verify(&format!("{}{}", seed, cd));
    println!("seed: {}, check digit: {}, verify: {}", seed, cd, ok);

    assert_eq!(cd, 7);
    assert!(ok);
}

#[test]
fn ExampleNewJAN8() {
    let p = gtin::new_jan8();

    let seed = "4996871";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let ok = p.verify(&format!("{}{}", seed, cd));
    println!("seed: {}, check digit: {}, verify: {}", seed, cd, ok);

    assert_eq!(cd, 2);
    assert!(ok);
}

#[test]
fn ExampleNewJAN13() {
    let p = new_jan13();

    let seed = "456995111617";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let ok = p.verify(&format!("{}{}", seed, cd));
    println!("seed: {}, check digit: {}, verify: {}", seed, cd, ok);

    assert_eq!(cd, 9);
    assert!(ok);
}

#[test]
fn ExampleNewITF() {
    let p = new_itf().expect("failed to create itf provider");

    let seed = "1456995111617";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let ok = p.verify(&format!("{}{}", seed, cd));
    println!("seed: {}, check digit: {}, verify: {}", seed, cd, ok);

    assert_eq!(cd, 6);
    assert!(ok);
}

#[test]
fn ExampleNewUPC() {
    let p = new_upc();

    let seed = "01234567890";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let ok = p.verify(&format!("{}{}", seed, cd));
    println!("seed: {}, check digit: {}, verify: {}", seed, cd, ok);

    assert_eq!(cd, 5);
    assert!(ok);
}

#[test]
fn ExampleNewSSCC() {
    let p = new_sscc();

    let seed = "04569951110000001";
    let cd = p.generate(seed).expect("failed to generate check digit");

    let ok = p.verify(&format!("{}{}", seed, cd));
    println!("seed: {}, check digit: {}, verify: {}", seed, cd, ok);

    assert_eq!(cd, 6);
    assert!(ok);
}
