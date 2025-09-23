use bigint::bigint::BigInt;

#[test]
fn test_basic_operations() {
    // Test from_int and from_string equality
    let x = BigInt::from_int(100);
    let y = BigInt::from_str("100");
    assert_eq!(x, y);

    // Test comparisons
    let x = BigInt::from_int(100);
    let y = BigInt::from_int(100);
    assert_eq!(x, y);
    assert!(x <= y);
    assert!(x >= y);
    assert!(!(x < y));
    assert!(!(x > y));

    let x = BigInt::from_int(100);
    let y = BigInt::from_int(101);
    assert_ne!(x, y);
    assert!(x <= y);
    assert!(!(x >= y));
    assert!(x < y);
    assert!(!(x > y));

    // Test arithmetic operations
    let x = BigInt::from_int(100);
    let y = BigInt::from_int(200);
    let z = x.add(&y);
    assert_eq!(z, BigInt::from_int(300));

    let x = BigInt::from_int(200);
    let y = BigInt::from_int(100);
    let z = x.sub(&y);
    assert_eq!(z, BigInt::from_int(100));

    let x = BigInt::from_int(100);
    let y = BigInt::from_int(200);
    let z = x.mul(&y);
    assert_eq!(z, BigInt::from_int(20000));

    // Test division and modulo
    let x = BigInt::from_int(200);
    let y = BigInt::from_int(100);
    let z = x.div(&y);
    assert_eq!(z, BigInt::from_int(2));

    let z = x.r#mod(&y);
    assert_eq!(z, BigInt::from_int(0));

    // Test power
    let x = BigInt::from_int(2);
    let y = BigInt::from_int(10);
    let z = x.pow(&y);
    assert_eq!(z, BigInt::from_int(1024));

    // Test modular inverse
    let a = BigInt::from_int(17);
    let m = BigInt::from_int(43);
    let z = a.modinv(&m);
    let t = a.mul(&z);
    let r = t.r#mod(&m);
    assert_eq!(r, BigInt::from_int(1));

    // Test square root
    let x = BigInt::from_int(100);
    let z = x.sqrt();
    assert_eq!(z, BigInt::from_int(10));

    // Test primality
    let x = BigInt::from_int(17);
    assert!(x.is_prime());
    let x = BigInt::from_int(18);
    assert!(!x.is_prime());

    // Test fast power
    let x = BigInt::from_int(745232);
    let y = BigInt::from_int(67121);
    let z = BigInt::from_int(1022117);
    let result = x.fast_pow(&y, &z);
    assert_eq!(result, BigInt::from_int(97));

    println!("Test passed");
}
fn main(){}