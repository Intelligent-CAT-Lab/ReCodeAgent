use murmurhash_c::murmurhash::murmurhash;
fn t(str_input: &str, seed: u32, expected: u32) {
    let hash = murmurhash(str_input.as_bytes(), seed);
    println!(
        "['{}'] '{:08x}' = '{:08x}' ...ok",
        str_input, expected, hash
    );
    assert_eq!(hash, expected);
}

#[test]
fn test_murmurhash() {
    let seed = 0;
    t("", seed, 0x00000000);
    t("0", seed, 0xd271c07f);
    t("01", seed, 0x61ec6600);
    t("012", seed, 0xec6cff8c);
    t("0123", seed, 0xd41994a0);
    t("01234", seed, 0x19d02170);
    t("2", seed, 0x0129e217);
    t("88", seed, 0x7a0040a5);
    t("asdfqwer", seed, 0xa46b5209);
    t("asdfqwerty", seed, 0xa3cfe04b);
    t("asd", seed, 0x14570c6f);
    t("Hello", seed, 0x12da77c8);
    t("Hello1", seed, 0x6357e0a6);
    t("Hello2", seed, 0xe5ce223e);
    t("hey", seed, 0x12f94418);
    t("dude", seed, 0xef0487f3);
    t("test", seed, 0xba6bd213);
    t("kinkajou", seed, 0xb6d99cf8);

    let seed = 1;
    t("", seed, 0x514e28b7);
}
fn main(){}