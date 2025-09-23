use bigint::bigint::BigInt;

#[test]
fn test1() {
    let a = BigInt::from_str("123456789012345678901234567890");
    let b = BigInt::from_int(1234567890);
    let _c = a.clone();
}

#[test]
fn test2() {
    let a = BigInt::from_str("123456789012345678901234567890");
    let b = BigInt::from_str("987654321098765432109876543210");

    let _c = a.add(&b);
    let _d = a.sub(&b);
    let _e = a.mul(&b);
    let _f = a.div(&b);
    let _g = a.r#mod(&b);
}

#[test]
fn test3() {
    let a = BigInt::from_str("123456789012345678901234567890");
    let b = BigInt::from_str("987654321098765432109876543210");

    if a == b {
        println!("a == b");
    } else if a < b {
        println!("a < b");
    } else if a > b {
        println!("a > b");
    }
}

#[test]
fn test4() {
    let a = BigInt::from_str("2");
    let b = BigInt::from_str("1234");
    let m = BigInt::from_str("10007");

    let _c = a.fast_pow(&b, &m);
}

#[test]
fn test5() {
    let a = BigInt::from_str("123456789012345678901234567890");

    if a.is_prime() {
        println!("a is prime");
    } else {
        println!("a is not prime");
    }
}

#[test]
fn test6() {
    let a = BigInt::from_str("123412341");
    let _b = a.sqrt();
}
fn main(){}