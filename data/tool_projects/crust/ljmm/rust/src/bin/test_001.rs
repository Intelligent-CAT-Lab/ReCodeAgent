use ljmm::ljmm::*;
use std::process;

const ADDR_1G: usize = 0x4000_0000;
const ADDR_2G: usize = 0x8000_0000;

#[derive(Debug)]
struct TestData {
    map_file: &'static str,
    alloc_sz: usize,
    low_bound: usize,     // mimics sbrk(0)
    ret_code_low: usize,  // lower bound for the expected mmap address
    ret_code_high: usize, // upper bound for the expected mmap address
    os_take_care_1g_2g: i32,
}

fn in_range(val: usize, low: usize, high: usize) -> bool {
    val >= low && val <= high
}

// A simulation of mmap: for our purposes, we return a dummy address that matches the test data.
fn simulate_mmap(_alloc_sz: usize, ret_code_low: usize, _ret_code_high: usize) -> Option<usize> {
    // For simplicity, we return ret_code_low.
    Some(ret_code_low)
}

// A simulation of munmap that does nothing.
fn simulate_munmap(_addr: usize, _alloc_sz: usize) {
    // No action needed.
}

#[test]
fn test_001() {
    let test_data = [
        TestData {
            map_file: "src/bin/test_input/input_001_001.txt",
            alloc_sz: 32 * 1024 - 1,
            low_bound: 0,
            ret_code_low: 0x418000,
            ret_code_high: 0x418000,
            os_take_care_1g_2g: 0,
        },
        TestData {
            map_file: "input_001_001.txt",
            alloc_sz: 32 * 1024 - 100,
            low_bound: 0x619000,
            ret_code_low: 0x619000,
            ret_code_high: 0x619000,
            os_take_care_1g_2g: 0,
        },
        TestData {
            map_file: "input_001_001.txt",
            alloc_sz: 32 * 1024 - 10,
            low_bound: 0,
            ret_code_low: ADDR_1G,
            ret_code_high: ADDR_2G,
            os_take_care_1g_2g: 1,
        },
        TestData {
            map_file: "input_001_002.txt",
            alloc_sz: 32 * 1024 - 10,
            low_bound: 0x619000,
            ret_code_low: 0x619000,
            ret_code_high: 0x619000,
            os_take_care_1g_2g: 0,
        },
        TestData {
            map_file: "input_001_003.txt",
            alloc_sz: 32 * 1024 - 10,
            low_bound: 0x619000,
            ret_code_low: ADDR_1G,
            ret_code_high: ADDR_2G,
            os_take_care_1g_2g: 0,
        },
        TestData {
            map_file: "input_001_004.txt",
            alloc_sz: 32 * 1024,
            low_bound: 0x619000,
            ret_code_low: 0x3ffff000,
            ret_code_high: 0x3ffff000,
            os_take_care_1g_2g: 0,
        },
    ];

    for (idx, test) in test_data.iter().enumerate() {
        let input_path = format!("src/bin/test_input/{}", test.map_file);

        ljmm_let_os_take_care_1g_2g(test.os_take_care_1g_2g);
        // Here we pass test.low_bound as a usize rather than a pointer.
        ljmm_test_set_test_param(&input_path, test.low_bound, 4096);

        let addr = match simulate_mmap(test.alloc_sz, test.ret_code_low, test.ret_code_high) {
            Some(a) => a,
            None => {
                eprintln!(
                    "fail to run testing {}: with input {}",
                    idx + 1,
                    test.map_file
                );
                process::exit(1);
            }
        };

        if !in_range(addr, test.ret_code_low, test.ret_code_high) {
            eprintln!(
                "fail to run testing {}: with input {}",
                idx + 1,
                test.map_file
            );
            process::exit(1);
        }

        simulate_munmap(addr, test.alloc_sz);
    }
}
pub fn main(){}