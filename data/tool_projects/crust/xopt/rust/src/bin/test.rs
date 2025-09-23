use XOpt::xopt::{xopt_context, xopt_parse, XoptOption, XoptContext, XOPT_TYPE_INT, XOPT_TYPE_FLOAT, XOPT_TYPE_DOUBLE, XOPT_TYPE_BOOL, XOPT_CTX_POSIXMEHARDER, XOPT_CTX_STRICT, XOPT_NULLOPTION};
// A sample config type, matching the original SimpleConfig in your C code.
#[repr(C)]
#[derive(Debug)]
struct SimpleConfig {
    some_int: i32,
    some_float: f32,
    some_double: f64,
    help: bool,
}

// This macro is to emulate C's offsetof for our SimpleConfig.
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {{
        let tmp = std::mem::MaybeUninit::<$ty>::uninit();
        let base_ptr = tmp.as_ptr();
        let field_ptr = unsafe { std::ptr::addr_of!((*base_ptr).$field) };
        (field_ptr as usize) - (base_ptr as usize)
    }};
}

fn build_options() -> Vec<XoptOption> {
    vec![
        XoptOption {
            long_arg: Some("some-int".to_string()),
            short_arg: 'i',
            offset: offset_of!(SimpleConfig, some_int),
            callback: None,
            options: XOPT_TYPE_INT,
            arg_descrip: Some("n".to_string()),
            descrip: Some("Some integer value. Can set to whatever number you like.".to_string()),
        },
        XoptOption {
            long_arg: Some("some-float".to_string()),
            short_arg: 'f',
            offset: offset_of!(SimpleConfig, some_float),
            callback: None,
            options: XOPT_TYPE_FLOAT,
            arg_descrip: Some("n".to_string()),
            descrip: Some("Some float value.".to_string()),
        },
        XoptOption {
            long_arg: Some("some-double".to_string()),
            short_arg: 'd',
            offset: offset_of!(SimpleConfig, some_double),
            callback: None,
            options: XOPT_TYPE_DOUBLE,
            arg_descrip: Some("n".to_string()),
            descrip: Some("Some double value.".to_string()),
        },
        XoptOption {
            long_arg: Some("help".to_string()),
            short_arg: '?',
            offset: offset_of!(SimpleConfig, help),
            callback: None,
            options: XOPT_TYPE_BOOL,
            arg_descrip: None,
            descrip: Some("Shows this help message".to_string()),
        },
        XOPT_NULLOPTION(),
    ]
}

#[test]
fn test_parsing_some_args() {
    let mut config = SimpleConfig {
        some_int: 0,
        some_float: 0.0,
        some_double: 0.0,
        help: false,
    };

    // Suppose we want to parse the equivalent of:
    //   ./program --some-int=42 --some-float=3.14 file1.txt file2.txt
    // Notice that we do not supply the real binary name as argv[0] here,
    // but you could if you want. The first string in the slice acts like argv[0].
    let argv = vec![
        "program",
        "--some-int=42",
        "--some-float=3.14",
        "file1.txt",
        "file2.txt",
    ];
    let argc = argv.len() as i32;

    let mut err: Option<String> = None;
    let mut extras: Option<Vec<String>> = None;

    // Build the context using your stubs (unimplemented!).
    let mut ctx = xopt_context(
        Some("xopt-test"),
        &build_options(),
        XOPT_CTX_POSIXMEHARDER | XOPT_CTX_STRICT,
        &mut err,
    );
    // We expect no error from creating the context
    assert!(
        err.is_none(),
        "Expected no error when creating context, got: {:?}",
        err
    );
    assert!(
        ctx.is_some(),
        "Context should not be None after successful creation"
    );

    // Now call xopt_parse with our arguments. This is unimplemented, so it will panic!
    let extra_count = xopt_parse(
        ctx.as_mut().unwrap(),
        argc,
        &argv,
        &mut config as *mut SimpleConfig as *mut u8,
        &mut extras,
        &mut err,
    );

    // We expect no error from parsing
    assert!(
        err.is_none(),
        "Expected no error from parse, got: {:?}",
        err
    );

    // If the parse were implemented, config.some_int should be 42
    // and config.some_float should be 3.14. We'll assert that here:
    assert_eq!(config.some_int, 42, "some_int should be 42");
    // Float comparisons can use an epsilon or just direct eq if you trust the parse
    assert!(
        (config.some_float - 3.14).abs() < f32::EPSILON,
        "some_float should be 3.14, got: {}",
        config.some_float
    );
    // We didn't set `--some-double`, so it should remain 0.0
    assert_eq!(config.some_double, 0.0, "some_double should be 0.0 by default");
    // We didn't set `--help`, so it should remain false
    assert_eq!(config.help, false, "help should be false by default");

    // The final two arguments were not recognized as options. They should be "extras."
    assert_eq!(extra_count, 2, "Should have 2 extras (file1.txt, file2.txt)");
    let extras_vec = extras.unwrap_or_default();
    assert_eq!(extras_vec.len(), 2);
    assert_eq!(extras_vec[0], "file1.txt");
    assert_eq!(extras_vec[1], "file2.txt");
}

#[test]
fn test_help_arg() {
    let mut config = SimpleConfig {
        some_int: 0,
        some_float: 0.0,
        some_double: 0.0,
        help: false,
    };

    // This time, pretend the user invoked `program --help`.
    let argv = vec!["program", "--help"];
    let argc = argv.len() as i32;

    let mut err: Option<String> = None;
    let mut extras: Option<Vec<String>> = None;

    let mut ctx = xopt_context(
        Some("xopt-test"),
        &build_options(),
        XOPT_CTX_POSIXMEHARDER | XOPT_CTX_STRICT,
        &mut err,
    );
    assert!(
        err.is_none(),
        "Expected no error when creating context, got: {:?}",
        err
    );
    assert!(
        ctx.is_some(),
        "Context should not be None after successful creation"
    );

    let extra_count = xopt_parse(
        ctx.as_mut().unwrap(),
        argc,
        &argv,
        &mut config as *mut SimpleConfig as *mut u8,
        &mut extras,
        &mut err,
    );
    assert!(
        err.is_none(),
        "Expected no error from parse, got: {:?}",
        err
    );

    // Now, `--help` sets config.help to true
    assert_eq!(
        config.help, true,
        "Passing --help should set the `help` field to true"
    );
    // This command had no extras. So we should get 0.
    assert_eq!(extra_count, 0, "No non-option extras expected for `--help`");
    let extras_vec = extras.unwrap_or_default();
    assert_eq!(extras_vec.len(), 0, "No extras expected");
}



fn main(){}