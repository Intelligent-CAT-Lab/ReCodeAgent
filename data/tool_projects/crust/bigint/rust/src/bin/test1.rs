use bigint::bigint::BigInt;

fn factorial(n: BigInt) -> BigInt {
    let mut result = BigInt::from_int(1);
    let mut i = BigInt::from_int(1);

    while i <= n {
        result = result.mul(&i);
        i = i.add(&BigInt::from_int(1));
    }

    result
}

#[test]
fn test_factorial() {
    let n = BigInt::from_int(100);
    let result = factorial(n.clone());

    println!("{}! = {}", n, result);

    let expected = BigInt::from_str("93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000");
    assert_eq!(result, expected);
    println!("Test passed");
}

fn main(){}
