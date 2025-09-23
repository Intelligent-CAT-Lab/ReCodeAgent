use libbeaufort::decrypt::beaufort_decrypt;
use libbeaufort::encrypt::beaufort_encrypt;
use libbeaufort::tableau::beaufort_tableau;
use libbeaufort::*;

#[test]
fn test_encrypt() {
    let monkey_s = b"kinkajous are awesome";
    let monkey_key = b"monkey";
    let monkey = beaufort_encrypt(monkey_s, monkey_key, &[]);
    assert_eq!(monkey, b"26004Fyuv AnK Cs9sqC8");

    let goodman_s = b"the \nbig \nlebowski";
    let goodman_key = b"goodman";
    let goodman = beaufort_encrypt(goodman_s, goodman_key, &[]);
    assert_eq!(goodman, b"n7A \n24u \n22D0huq5");

    let groove_s = b"d4nc3 t0 th3 mus!c :D";
    let groove_key = b"groove";
    let groove = beaufort_encrypt(groove_s, groove_key, &[]);
    assert_eq!(groove, b"3n1Cs lg y7l 9ko!F :b");
}

#[test]
fn test_decrypt() {
    let monkey_s = b"kinkajous are awesome";
    let monkey_key = b"monkey";
    let monkey = beaufort_encrypt(monkey_s, monkey_key, &[]);
    let decrypted = beaufort_decrypt(&monkey, monkey_key, &[]);
    assert_eq!(decrypted, monkey_s);

    let goodman_s = b"the \nbig \nlebowski";
    let goodman_key = b"goodman";
    let goodman = beaufort_encrypt(goodman_s, goodman_key, &[]);
    let decrypted = beaufort_decrypt(&goodman, goodman_key, &[]);
    assert_eq!(decrypted, goodman_s);

    let groove_s = b"d4nc3 t0 th3 mus!c :D";
    let groove_key = b"groove";
    let groove = beaufort_encrypt(groove_s, groove_key, &[]);
    let decrypted = beaufort_decrypt(&groove, groove_key, &[]);
    assert_eq!(decrypted, groove_s);
}

#[test]
fn test_tableau() {
    let mat = beaufort_tableau(std::str::from_utf8(BEAUFORT_ALPHA).unwrap());
    assert!(!mat.is_empty());

    let mat_small = beaufort_tableau("abc");
    assert_eq!(mat_small.len(), 3);
    assert_eq!(mat_small[0], b"acb");
    assert_eq!(mat_small[1], b"bac");
    assert_eq!(mat_small[2], b"cba");
}

fn main(){}