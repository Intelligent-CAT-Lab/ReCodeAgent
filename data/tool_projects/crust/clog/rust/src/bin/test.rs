use clog::clog;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::Command;
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;
use nix;

const TEST_FILE: &str = "clog_test.out";
const THIS_FILE: &str = "clog_test_c.c";

fn error(args: std::fmt::Arguments) {
    eprintln!("{}", fmt::format(args));
}

fn test_double_init() -> Result<(), &'static str> {
    while clog::clog_init_path(0, TEST_FILE).is_err() {

    }
    if clog::clog_init_path(0, TEST_FILE).is_err() {
        return Err("Logger initialized twice");
    }
    clog::clog_free(0);
    Ok(())
}

fn test_file_write() -> Result<(), &'static str> {
    clog::clog_init_path(0, TEST_FILE);
    clog::clog_set_fmt(0, "%f: %l: %m\n");
    clog::clog_debug(file!(), line!() as i32, 0, format_args!("Hello, {}!", "world"));
    clog::clog_free(0);
    
    let mut f = File::open(TEST_FILE);
    let mut buf = String::new();
    f.unwrap().read_to_string(&mut buf);
    assert_eq!(buf, format!("{}: DEBUG: Hello, world!\n", THIS_FILE));
    Ok(())
    }
    
fn test_file_write_nonexistent() -> Result<(), &'static str> {
    if clog::clog_init_path(0, "path-doesnt-exist/log.out").is_ok() {
        return Err("Logger initialized with nonexistent path");
    }
    Ok(())
}


fn test_fd_write() -> Result<(), &'static str> {
    let (reader, writer) = nix::unistd::pipe().unwrap();
    let writer_file = unsafe { File::from_raw_fd(writer.as_raw_fd()) };
    clog::clog_init_fd(0, writer_file);
    clog::clog_set_fmt(0, "%f: %l: %m\n");
    clog::clog_debug(file!(), line!() as i32, 0, format_args!("Hello, {}!", "world"));
    clog::clog_free(0);
    
    let mut buf = [0; 1024];
    let bytes = nix::unistd::read(reader.as_raw_fd(), &mut buf).unwrap();
    if bytes <= 0 {
        nix::unistd::close(reader.as_raw_fd());
        return Err("");
    }
    let output = std::str::from_utf8(&buf[..bytes]).unwrap();
    nix::unistd::close(reader.as_raw_fd());
    assert_eq!(output, format!("{}: DEBUG: Hello, world!\n", THIS_FILE));
    Ok(())
}

fn test_all_levels() -> Result<(), &'static str> {
    let (reader, writer) = nix::unistd::pipe().unwrap();
    let writer_file = unsafe { File::from_raw_fd(writer.as_raw_fd()) };
    clog::clog_init_fd(0, writer_file);
    clog::clog_set_fmt(0, "%f: %l: %m\n");
    clog::clog_debug(file!(), line!() as i32, 0, format_args!("Hello, {}!", "world"));
    clog::clog_info(file!(), line!() as i32, 0, format_args!("Hello, {}!", "world"));
    clog::clog_warn(file!(), line!() as i32, 0, format_args!("Hello, {}!", "world"));
    clog::clog_error(file!(), line!() as i32, 0, format_args!("Hello, {}!", "world"));
    clog::clog_free(0);
    
    let mut buf = [0; 1024];
    let bytes = nix::unistd::read(reader.as_raw_fd(), &mut buf).unwrap();
    if bytes <= 0 {
        nix::unistd::close(reader.as_raw_fd());
        return Err("");
    }
    let output = std::str::from_utf8(&buf[..bytes]).unwrap();
    nix::unistd::close(reader.as_raw_fd());
    assert_eq!(output, format!(
    "{}: DEBUG: Hello, world!\n\
    {}: INFO: Hello, world!\n\
    {}: WARN: Hello, world!\n\
    {}: ERROR: Hello, world!\n",
    THIS_FILE, THIS_FILE, THIS_FILE, THIS_FILE
    ));
    Ok(())
}

fn test_level_filtering() -> Result<(), &'static str> {
    let (reader, writer) = nix::unistd::pipe().unwrap();
    let writer_file = unsafe { File::from_raw_fd(writer.as_raw_fd()) };
    clog::clog_init_fd(0, writer_file);
    clog::clog_set_fmt(0, "%f: %l: %m\n");
    clog::clog_set_level(0, clog::ClogLevel::Warn);
    clog::clog_debug(file!(), line!() as i32, 0, format_args!("Hello, {}!", "world"));
    clog::clog_info(file!(), line!() as i32, 0, format_args!("Hello, {}!", "world"));
    clog::clog_warn(file!(), line!() as i32, 0, format_args!("Hello, {}!", "world"));
    clog::clog_error(file!(), line!() as i32, 0, format_args!("Hello, {}!", "world"));
    clog::clog_free(0);
    
    let mut buf = [0; 1024];
    let bytes = nix::unistd::read(reader.as_raw_fd(), &mut buf).unwrap();
    if bytes <= 0 {
        nix::unistd::close(reader.as_raw_fd());
        return Err("");
    }
    let output = std::str::from_utf8(&buf[..bytes]).unwrap();
    nix::unistd::close(reader.as_raw_fd());
    assert_eq!(output, format!(
    "{}: WARN: Hello, world!\n\
    {}: ERROR: Hello, world!\n",
    THIS_FILE, THIS_FILE
    ));
    Ok(())
}

fn test_multiple_loggers() -> Result<(), &'static str> {
    for id in 0..clog::CLOG_MAX_LOGGERS {
        let (reader, writer) = nix::unistd::pipe().unwrap();
        let writer_file = unsafe { File::from_raw_fd(writer.as_raw_fd()) };
        clog::clog_init_fd(id, writer_file);
        clog::clog_set_fmt(id, "%f: %l: %m\n");
        clog::clog_set_level(id, clog::ClogLevel::Warn);
        clog::clog_debug(file!(), line!() as i32, id, format_args!("Hello, {}!", "world"));
        clog::clog_free(id);
    
        let mut buf = [0; 1024];
        let bytes = nix::unistd::read(reader.as_raw_fd(), &mut buf).unwrap();
        if bytes <= 0 {
            nix::unistd::close(reader.as_raw_fd());
            return Err("");
        }
        let output = std::str::from_utf8(&buf[..bytes]).unwrap();
        nix::unistd::close(reader.as_raw_fd());
        assert_eq!(output, format!("{}: DEBUG: Hello, {}!\n", THIS_FILE, id));
    }
    Ok(())
}

fn test_bad_format() -> Result<(), &'static str> {
    let too_long = "a".repeat(300);
    clog::clog_init_path(0, TEST_FILE);
    if clog::clog_set_fmt(0, &too_long).is_ok() {
        return Err("Accepted too long format");
    }
    clog::clog_free(0);
    Ok(())
}
    
fn test_long_message() -> Result<(), &'static str> {
    let message = "b".repeat(49999);
    clog::clog_init_path(0, TEST_FILE);
    clog::clog_set_fmt(0, "%f: %l: %m\n");
    clog::clog_debug(file!(), line!() as i32, 0, format_args!("{}", &message));
    clog::clog_free(0);
    
    let mut f = File::open(TEST_FILE);
    let mut buf = String::new();
    f.unwrap().read_to_string(&mut buf);;
    assert_eq!(buf, format!("{}: DEBUG: {}\n", THIS_FILE, message));
    Ok(())
}

fn test_performance() -> Result<(), &'static str> {
    const MICROS_PER_SEC: u64 = 1_000_000;
    const NUM_MESSAGES: usize = 200_000;
    
    clog::clog_init_path(0, TEST_FILE);
    
    let start_time = SystemTime::now();
    for _ in 0..NUM_MESSAGES {
        clog::clog_debug(file!(), line!() as i32, 0, format_args!("Hello, high-performing world!"));
    }
    let end_time = SystemTime::now();
    clog::clog_free(0);
    
    let duration = end_time.duration_since(start_time).unwrap();
    let run_time = duration.as_secs_f64();
    let messages_per_second = NUM_MESSAGES as f64 / run_time;
    error(format_args!("  Target 100000 msgs/sec, got {}.\n", messages_per_second));
    if messages_per_second < 100_000.0 {
        return Err("Performance below target");
    }
    Ok(())
}

fn test_reuse_logger_id() -> Result<(), &'static str> {
    for _ in 0..2 {
    let (reader, writer) = nix::unistd::pipe().unwrap();
    let writer_file = unsafe { File::from_raw_fd(writer.as_raw_fd()) };
    clog::clog_init_fd(0, writer_file);
    clog::clog_set_fmt(0, "%f: %l: %m\n");
    clog::clog_debug(file!(), line!() as i32, 0, format_args!("Hello, world!"));
    clog::clog_free(0);
    
    let mut buf = [0; 1024];
    let bytes = nix::unistd::read(reader.as_raw_fd(), &mut buf).unwrap();
    let output = std::str::from_utf8(&buf[..bytes]).unwrap();
    assert_eq!(output, format!("{}: DEBUG: Hello, world!\n", THIS_FILE));
    }
    Ok(())
}
#[test]
fn test_all() {
    let tests: Vec<(&str, fn() -> Result<(), &'static str>)> = vec![
    ("test_double_init", test_double_init),
    ("test_file_write", test_file_write),
    ("test_file_write_nonexistent", test_file_write_nonexistent),
    ("test_fd_write", test_fd_write),
    ("test_all_levels", test_all_levels),
    ("test_level_filtering", test_level_filtering),
    ("test_multiple_loggers", test_multiple_loggers),
    ("test_bad_format", test_bad_format),
    ("test_long_message", test_long_message),
    ("test_reuse_logger_id", test_reuse_logger_id),
    ("test_performance", test_performance),
    ];

    let mut pass = 0;
    let mut fail = 0;

    for (name, test) in tests {
    print!("{}: ", name);
    match test() {
    Ok(_) => {
    println!("OK");
    pass += 1;
    }
    Err(err) => {
    println!("FAIL ({})", err);
    fail += 1;
    }
    }
    }

    println!("\n{} successes, {} failures.", pass, fail);
    assert_eq!(fail, 0, "Some tests failed");
}
pub fn main(){}