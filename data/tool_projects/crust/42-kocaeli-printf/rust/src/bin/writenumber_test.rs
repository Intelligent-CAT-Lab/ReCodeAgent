use proj_42_Kocaeli_Printf::ft_printf::{writeint, writeuint, writehex, writepoint, writechar, writestring, HEXALOW, DECIMAL, HEXAUP, LOCATION};

#[test]
fn test_writenumber() {
    let mut len = 0;

    len = 0;
    assert_eq!(writeint(42, &mut len), 1);
    assert!(len > 0);
    len = 0;
    assert_eq!(writeint(-42, &mut len), 1);
    assert!(len > 0);
    len = 0;
    assert_eq!(writeint(0, &mut len), 1);
    assert!(len > 0);
    len = 0;
    assert_eq!(writeint(-2147483648, &mut len), 1);
    assert!(len > 0);

    len = 0;
    assert_eq!(writeuint(42, &mut len), 1);
    assert!(len > 0);
    len = 0;
    assert_eq!(writeuint(0, &mut len), 1);
    assert!(len > 0);

    len = 0;
    assert_eq!(writehex(42, 'x', &mut len), 1);
    assert!(len > 0);
    len = 0;
    assert_eq!(writehex(42, 'X', &mut len), 1);
    assert!(len > 0);
    len = 0;
    assert_eq!(writehex(0, 'x', &mut len), 1);
    assert!(len > 0);

    len = 0;
    let ptr: *const std::ffi::c_void = 0x1234 as *const _;
    assert_eq!(writepoint(ptr, &mut len), 1);
    assert!(len > 0);
    len = 0;
    let ptr: *const std::ffi::c_void = std::ptr::null();
    assert_eq!(writepoint(ptr, &mut len), 1);
    assert!(len > 0);

    println!("All tests passed successfully.");
}

fn main() {
}