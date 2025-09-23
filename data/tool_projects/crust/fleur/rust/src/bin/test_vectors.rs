use fleur::fnv::{self, getDigest};
#[test]
fn test() {
    let str1 = "test";
    let h = fnv::fnv1(str1.as_bytes());

    let s = getDigest(h);
    let expected = "8c093f7e9fccbf69";
    assert_eq!(expected, s);
}

fn main(){}