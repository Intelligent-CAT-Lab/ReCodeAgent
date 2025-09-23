use Simple_Config::simple_config::*;
#[test]
fn run_get_str_test() {
    let entries = vec![
        CfgEntry {
            key: "keyA".to_string(),
            val: "foobar".into(),
        },
        CfgEntry {
            key: "keyC".chars().collect(),
            val: true.into(),
        },
    ];

    let cfg = Cfg {
        entries,
        count: 2,
        capacity: 2,
    };

    assert!(cfg_get_string(&cfg, "keyA", "barfoo") == "foobar");
    assert!(cfg_get_string(&cfg, "keyB", "barfoo") == "barfoo");
    assert!(cfg_get_string(&cfg, "keyC", "barfoo") == "barfoo");
}
#[test]
fn run_get_int_test() {
    let entries = vec![CfgEntry {
        key: "key".to_string(),
        val: 16.into(),
    }];

    let cfg = Cfg {
        entries,
        count: 1,
        capacity: 1,
    };

    assert_eq!(cfg_get_int(&cfg, "key", 64), 16);
    assert_eq!(cfg_get_int_min(&cfg, "key", 64, 8), 16);
    assert_eq!(cfg_get_int_min(&cfg, "key", 64, 32), 64);

    assert_eq!(cfg_get_int_max(&cfg, "key", 64, 32), 16);
    assert_eq!(cfg_get_int_max(&cfg, "key", 4, 8), 4);

    assert_eq!(cfg_get_int_range(&cfg, "key", 64, 8, 32), 16);
    assert_eq!(cfg_get_int_range(&cfg, "key", 4, 4, 8), 4);
    assert_eq!(cfg_get_int_range(&cfg, "key", 32, 32, 64), 32);
}
#[test]
fn run_get_float_test() {
    let entries = vec![CfgEntry {
        key: "key".to_string(),
        val: 16.0.into(),
    }];

    let cfg = Cfg {
        entries,
        count: 1,
        capacity: 1,
    };

    assert!((cfg_get_float(&cfg, "key", 64.0) - 16.0).abs() < std::f32::EPSILON);
    assert!((cfg_get_float_min(&cfg, "key", 64.0, 8.0) - 16.0).abs() < std::f32::EPSILON);
    assert!((cfg_get_float_min(&cfg, "key", 64.0, 32.0) - 64.0).abs() < std::f32::EPSILON);

    assert!((cfg_get_float_max(&cfg, "key", 64.0, 32.0) - 16.0).abs() < std::f32::EPSILON);
    assert!((cfg_get_float_max(&cfg, "key", 4.0, 8.0) - 4.0).abs() < std::f32::EPSILON);

    assert!((cfg_get_float_range(&cfg, "key", 64.0, 8.0, 32.0) - 16.0).abs() < std::f32::EPSILON);
    assert!((cfg_get_float_range(&cfg, "key", 4.0, 4.0, 8.0) - 4.0).abs() < std::f32::EPSILON);
    assert!((cfg_get_float_range(&cfg, "key", 32.0, 32.0, 64.0) - 32.0).abs() < std::f32::EPSILON);
}
#[test]
fn run_get_color_test() {
    let c1 = CfgColor {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    let c2 = CfgColor {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    let entries = vec![CfgEntry {
        key: "key".to_string(),
        val: c1.into(),
    }];

    let cfg = Cfg {
        entries,
        count: 1,
        capacity: 1,
    };

    let actual = cfg_get_color(&cfg, "key", c2);
    assert_eq!(actual, c1);
}

fn main() {}
