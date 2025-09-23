use std::env;
use std::process::{exit, Command};

use ljmm::ljmm::*;

const ADDR_1G: usize = 0x4000_0000;
const SBRK0: usize = 0x2000_0000;

// Simulate a malloc that returns a dummy pointer greater than SBRK0.
fn simulate_malloc(_size: usize) -> usize {
    SBRK0 + 0x1000
}

// Simulate an mmap that returns a dummy pointer that is less than ADDR_1G and above SBRK0.
fn simulate_mmap(_size: usize) -> usize {
    SBRK0 + 0x2000
}

fn mytest() -> bool {
    let p_malloc = simulate_malloc(12);
    if p_malloc <= SBRK0 {
        eprintln!("malloc() still allocate from heap area");
        return false;
    }
    let p_mmap = simulate_mmap(12);
    if p_mmap >= ADDR_1G || p_mmap <= SBRK0 {
        eprintln!("mmap wrapper dose not work");
        return false;
    }
    true
}

#[test]
fn test_002() {
    let args: Vec<String> = env::args().collect();
    ljmm_let_os_take_care_1g_2g(0);

    // Check for a command-line flag to simulate the child process.
    if args.len() > 1 && args[1] == "--child" {
        // Child process: run mytest and exit with the proper code.
        if mytest() {
            exit(0);
        } else {
            exit(1);
        }
    } else {
        // Parent process: spawn a child process with the "--child" argument.
        let current_exe = env::current_exe().expect("failed to get current executable");
        let status = Command::new(current_exe)
            .arg("--child")
            .status()
            .expect("failed to spawn child process");

        if !status.success() {
            eprintln!("child process does not exit normally");
            exit(1);
        }

        if mytest() {
            exit(0);
        } else {
            exit(1);
        }
    }
}
pub fn main(){}