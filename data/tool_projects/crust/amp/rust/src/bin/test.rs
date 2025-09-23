use amp::amp::*;

#[test]
fn test() {
    let args = ["some", "stuff", "here"];

    let buf = amp_encode(&args);

    let mut msg = Amp {
        version: 0,
        argc: 0,
        buf: String::new(),
    };

    msg.decode(&buf);
    assert_eq!(AMP_VERSION, msg.version, "Protocol version mismatch");
    assert_eq!(3, msg.argc, "Argument count mismatch");

    let expected_args = ["some", "stuff", "here"];
    for expected in &expected_args {
        let arg = msg.decode_arg();
        assert_eq!(*expected, arg, "Decoded argument does not match expected");
    }

    println!("ok");
}
fn main(){}