use CircularBuffer::circular_buffer;
#[test]
fn test_circular_buffer() {
    let mut cb = circular_buffer::CircularBuffer::new(8);
    let a = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
    let mut b = [0u8; 128];
    let mut len;
    let mut offset = 0;
    let mut out_len;

    len = 3;
    cb.push(&a[offset..offset + len], len);
    offset += len;
    assert_eq!(cb.get_data_size(), len);

    len = 7;
    cb.push(&a[offset..offset + len], len);
    offset += len;
    assert_eq!(cb.get_data_size(), 8);

    len = 3;
    b.fill(0);
    out_len = cb.pop(len, &mut b);
    assert_eq!(out_len, len);
    assert_eq!(&b[..len], b"234");

    len = 2;
    b.fill(0);
    out_len = cb.read(len, &mut b);
    assert_eq!(out_len, len);
    assert_eq!(&b[..len], b"56");

    len = 10;
    cb.push(&a[offset..offset + len], len);
    offset += len;
    assert_eq!(cb.get_data_size(), 8);

    len = 3;
    b.fill(0);
    out_len = cb.pop(len, &mut b);
    assert_eq!(out_len, len);
    assert_eq!(&b[..len], b"cde");

    len = 30;
    b.fill(0);
    out_len = cb.pop(len, &mut b);
    assert!(out_len <= 8);

    len = 5;
    cb.push(&a[offset..offset + len], len);
    offset += len;
    assert_eq!(cb.get_data_size(), 5);

    len = 2;
    b.fill(0);
    out_len = cb.pop(len, &mut b);
    assert_eq!(out_len, len);

    len = 10;
    cb.push(&a[offset..offset + len], len);
    offset += len;
    assert_eq!(cb.get_data_size(), 8);

    len = 6;
    b.fill(0);
    out_len = cb.pop(len, &mut b);
    assert_eq!(out_len, len);

    len = 4;
    cb.push(&a[offset..offset + len], len);
    offset += len;
    assert_eq!(cb.get_data_size(), 6);

    println!("All tests passed successfully.");
}

fn main() {}