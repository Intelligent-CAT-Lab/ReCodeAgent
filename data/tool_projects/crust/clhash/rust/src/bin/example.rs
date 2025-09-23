use clhash::clhash::{clhash, get_random_key_for_clhash};

#[test]
fn test() {
let random = get_random_key_for_clhash(0x23a23cf5033c3c81, 0xb3816f6a2c68e530);
let hashvalue1 = clhash(&random, b"my dog");
let hashvalue2 = clhash(&random, b"my cat");
let hashvalue3 = clhash(&random, b"my dog");
assert_eq!(hashvalue1, hashvalue3);
assert_ne!(hashvalue1, hashvalue2);
}
fn main(){}
