use Simple_Config::simple_config::{Cfg, CfgColor, CfgEntry, cfg_parse, CfgVal};

pub const TEST_CAPACITY: usize = 10;
#[test]
fn test() {
    let test_cases: &[(&'static str, Result<Cfg, &'static str>)] = &[
        (
            "",
            Ok(Cfg {
                entries: vec![],
                count: 0,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            " ",
            Ok(Cfg {
                entries: vec![],
                count: 0,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "#",
            Ok(Cfg {
                entries: vec![],
                count: 0,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "#\n",
            Ok(Cfg {
                entries: vec![],
                count: 0,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "x",
            Ok(Cfg {
                entries: vec![],
                count: 0,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "key: \"hello, world!\"",
            Ok(Cfg {
                entries: vec![CfgEntry {
                    key: "key".to_string(),
                    val: "hello, world!".into(),
                }],
                count: 1,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "key: 10 # Inline comment",
            Ok(Cfg {
                entries: vec![CfgEntry {
                    key: "key".to_string(),
                    val: 10.into(),
                }],
                count: 1,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "key: -1",
            Ok(Cfg {
                entries: vec![CfgEntry {
                    key: "key".to_string(),
                    val: (-1).into(),
                }],
                count: 1,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "key: true",
            Ok(Cfg {
                entries: vec![CfgEntry {
                    key: "key".to_string(),
                    val: true.into(),
                }],
                count: 1,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "key: false",
            Ok(Cfg {
                entries: vec![CfgEntry {
                    key: "key".to_string(),
                    val: false.into(),
                }],
                count: 1,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "key: rgba(255, 255, 255, 0.5)",
            Ok(Cfg {
                entries: vec![CfgEntry {
                    key: "key".to_string(),
                    val: CfgVal::Color(CfgColor {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 128,
                    }),
                }],
                count: 1,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "key: 0.5",
            Ok(Cfg {
                entries: vec![CfgEntry {
                    key: "key".to_string(),
                    val: 0.5.into(),
                }],
                count: 1,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "key_: true",
            Ok(Cfg {
                entries: vec![CfgEntry {
                    key: "key_".to_string(),
                    val: true.into(),
                }],
                count: 1,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "key.: true",
            Ok(Cfg {
                entries: vec![CfgEntry {
                    key: "key.".to_string(),
                    val: true.into(),
                }],
                count: 1,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "a: true\nb:true",
            Ok(Cfg {
                entries: vec![
                    CfgEntry {
                        key: "a".to_string(),
                        val: true.into(),
                    },
                    CfgEntry {
                        key: "b".to_string(),
                        val: true.into(),
                    },
                ],
                count: 2,
                capacity: TEST_CAPACITY,
            }),
        ),
        (
            "key: 1.",
            Ok(Cfg {
                entries: vec![CfgEntry {
                    key: "key".to_string(),
                    val: 1.0.into(),
                }],
                count: 1,
                capacity: TEST_CAPACITY,
            }),
        ),
        ("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", Err("key too long")),
        ("!", Err("invalid character")),
        ("key", Err("':' expected")),
        ("key!", Err("':' expected")),
        ("key  :", Err("missing value")),
        ("key ", Err("missing value")),
        ("key:  ", Err("missing value")),
        ("key:  \n", Err("missing value")),
        ("key: @\n", Err("invalid value")),
        ("key: \"", Err("closing '\"' expected")),
        ("key: \"hello", Err("closing '\"' expected")),
        (
            "key: \"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\"",
            Err("value too long"),
        ),
        ("key: 10x", Err("unexpected character 'x'")),
        ("key: -", Err("invalid value")),
        ("key: t", Err("invalid literal")),
        ("key: f", Err("invalid literal")),
        ("key: r", Err("invalid literal")),
        ("key: x", Err("invalid literal")),
        ("key: rgba(", Err("',' expected")),
        ("key: rgba", Err("'(' expected")),
        ("key: rgba x", Err("'(' expected")),
        (
            "key: rgba(-1",
            Err("red, blue and green must be integers in range [0, 255]"),
        ),
        (
            "key: rgba(0.5",
            Err("red, blue and green must be integers in range [0, 255]"),
        ),
        (
            "key: rgba(255, 255, 255, -1)",
            Err("alpha must be in range [0, 1]"),
        ),
        ("key: rgba(255, 255, 255, 1", Err("')' expected")),
        ("key: rgba(255, 255, 255, 1 x", Err("')' expected")),
        ("key: 2147483648", Err("number too large")),
        ("key: 0.0000000000", Err("number too large")),
        ("key: 1.33333333333333333", Err("number too large")),
        (
            "key: rgba(255, 255, 255, -0.5)",
            Err("alpha must be in range [0, 1]"),
        ),
        ("key: rgba(255, 255, 255, -)", Err("number expected")),
        (
            "key: rgba(255, 255, 255, 1.5)",
            Err("alpha must be in range [0, 1]"),
        ),
    ];

    for (src, expected) in test_cases {
        let cfg = cfg_parse(src);
        match expected {
            Ok(expected) => {
                assert!(cfg.is_ok());
                assert_eq!(cfg.unwrap(), *expected);
            }
            Err(expected) => {
                assert!(cfg.is_err());
                assert_eq!(cfg.unwrap_err().msg, *expected);
            }
        }
    }
}
fn main(){}
