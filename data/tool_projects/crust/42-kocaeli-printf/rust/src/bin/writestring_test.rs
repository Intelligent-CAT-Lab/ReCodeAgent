use proj_42_Kocaeli_Printf::ft_printf::{writechar, writestring};

#[test]
fn write_string() {
    let mut len = 0;

    len = 0;
    assert_eq!(writechar('A', &mut len), 1);
    assert_eq!(len, 1);
    len = 0;
    assert_eq!(writechar('B', &mut len), 1);
    assert_eq!(len, 1);
    len = 0;
    assert_eq!(writechar('z', &mut len), 1);
    assert_eq!(len, 1);

    len = 0;
    assert_eq!(writestring("Hello", &mut len), 1);
    assert_eq!(len, 5);
    len = 0;
    assert_eq!(writestring("World!", &mut len), 1);
    assert_eq!(len, 6);
    len = 0;
    assert_eq!(writestring("1234567890", &mut len), 1);
    assert_eq!(len, 10);

    println!("All tests passed successfully.");
}
fn main() {
}