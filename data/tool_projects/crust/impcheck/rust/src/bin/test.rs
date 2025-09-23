use impcheck::writer;
use impcheck::vec;
use impcheck::hash;
use impcheck::trusted_utils;
use impcheck::checker_interface;

use libc::*;
use std::{fs::File, io::{Read, Write}};
use crate::trusted_utils::*;
use std::process::Command;
use std::fs::OpenOptions;
use crate::checker_interface::*;
use std::thread;
use std::time::Duration;

fn do_assert(cond: bool) {
assert!(cond, "Assertion failed!");
}

#[test]    
fn test_small() {
println!("[TEST] --- begin test_small() ---");
let mut ht = hash::HashTable::<[i32; 128]>::new(7);
do_assert(ht.size == 0);
do_assert(ht.capacity == 1 << 7);
let mut arr = [0; 128];
let cap = ht.capacity;
for i in 1..=63 {
let ok = ht.hash_table_insert(i, Box::new(arr[i as usize]));
do_assert(ok);
do_assert(ht.size == i);
do_assert(ht.capacity == cap);
do_assert(ht.hash_table_find(i).is_some());
}
do_assert(ht.size == 63);
do_assert(ht.capacity == 128);
for i in 1..=63 {
do_assert(ht.hash_table_find(i).is_some());
}
let ok = ht.hash_table_insert(64, Box::new(arr[64]));
do_assert(ok);
do_assert(ht.size == 64);
do_assert(ht.capacity == 128);
let ok = ht.hash_table_insert(65, Box::new(arr[65]));
do_assert(ok);
do_assert(ht.size == 65);
do_assert(ht.capacity == 256);
for i in 1..=65 {
if i == 40 {
continue;
}
do_assert(ht.hash_table_find(i).is_some());
do_assert(ht.hash_table_delete(i));
do_assert(ht.hash_table_find(i).is_none());
}
do_assert(ht.size == 1);
do_assert(ht.hash_table_find(40).is_some());
do_assert(ht.hash_table_delete_last_found());
do_assert(ht.size == 0);
do_assert(ht.hash_table_find(40).is_none());
ht.hash_table_free();
println!("[TEST] ---  end  test_small() ---\n");
}

#[test]
fn test_big() {
println!("[TEST] --- begin test_big() ---");
let mut ht = hash::HashTable::<i32>::new(7);
do_assert(ht.size == 0);
let obj = 0;
let nb_elems = (1 << 20) + 3;
for i in 1..=nb_elems {
do_assert(ht.hash_table_find(i).is_none());
let ok = ht.hash_table_insert(i, Box::new(obj + i as i32));
do_assert(ok);
let val = ht.hash_table_find(i);
do_assert(val.is_some());
}
do_assert(ht.size == nb_elems);
for i in 1..=nb_elems {
do_assert(ht.size == nb_elems - i + 1);
let val = ht.hash_table_find(i);
do_assert(val.is_some());
let ok = ht.hash_table_delete_last_found();
do_assert(ok);
do_assert(ht.hash_table_find(i).is_none());
}
do_assert(ht.size == 0);
ht.hash_table_free();
println!("[TEST] ---  end  test_big() ---\n");
}

#[test]
fn test_alternate() {
println!("[TEST] --- begin test_alternate() ---");
let mut ht = hash::HashTable::<u64>::new(7);
do_assert(ht.size == 0);
let obj = 0;

const BLOCK_SIZE: u64 = 256;
const NB_ITERATIONS: i32 = 9999;
let mut counter = 1;
for _ in 0..NB_ITERATIONS {
for _ in 0..BLOCK_SIZE {
do_assert(ht.hash_table_find(counter).is_none());
let ok = ht.hash_table_insert(counter, Box::new(obj + counter));
do_assert(ok);
let val = ht.hash_table_find(counter);
do_assert(val.is_some());
counter += 1;
}
for delcounter in (counter - BLOCK_SIZE..counter).step_by(2) {
let val = ht.hash_table_find(delcounter);
do_assert(val.is_some());
let ok = ht.hash_table_delete_last_found();
do_assert(ok);
do_assert(ht.hash_table_find(delcounter).is_none());
}
}

println!("size={} counter={}", ht.size, counter);
for i in 1..=counter {
if i % 2 == 1 {
do_assert(ht.hash_table_find(i).is_none());
} else {
let val = ht.hash_table_find(i);
do_assert(val.is_some());
}
}
do_assert(ht.size == counter / 2);
println!("[TEST] ---  end  test_alternate() ---\n");
}



fn do_fork() -> bool {
match unsafe { libc::fork() } {
-1 => panic!("Fork failed"),
0 => true,  // Child process
_ => false, // Parent process
}
}

fn create_pipe(path: &str) {
let _ = std::fs::remove_file(path);
let res = unsafe { libc::mkfifo(path.as_ptr() as *const i8, 0o777) };
do_assert(res == 0);
}

fn await_ok(out: &mut File, input: &mut File) {
out.flush().unwrap();
let mut ok = [0u8; 1];
input.read_exact(&mut ok).unwrap();
do_assert(ok[0] != 0);
}

fn setup(cnf_input: &str) -> (u64, File, File) {
let checker_instance_id = 1;
let pipe_parsed = format!(".parsed.{}.pipe", checker_instance_id);
let pipe_directives = format!(".directives.{}.pipe", checker_instance_id);
let pipe_feedback = format!(".feedback.{}.pipe", checker_instance_id);

create_pipe(&pipe_parsed);
create_pipe(&pipe_directives);
create_pipe(&pipe_feedback);

if do_fork() {
let mut cmd = Command::new("build/impcheck_parse")
.arg(format!("-formula-input={}", cnf_input))
.arg(format!("-fifo-parsed-formula={}", pipe_parsed))
.spawn()
.expect("Failed to start impcheck_parse");
cmd.wait().expect("Failed to wait on child");
std::process::exit(0);
}

let mut in_parsed = File::open(&pipe_parsed).unwrap();
let nb_vars = trusted_utils_read_int(&mut in_parsed);
trusted_utils_read_int(&mut in_parsed);

let mut fvec = Vec::with_capacity(1 << 14);
loop {
let mut lit = [0u8; 4];
if in_parsed.read(&mut lit).unwrap() == 0 {
break;
}
fvec.push(i32::from_ne_bytes(lit));
}

let f = &fvec[..fvec.len() - (SIG_SIZE_BYTES / std::mem::size_of::<i32>())];
let mut fsig: Vec<u8> = vec![];
for i in &fvec[fvec.len() - (SIG_SIZE_BYTES / std::mem::size_of::<i32>())..] {
    fsig.push(*i as u8);
}
//let fsig = &fvec[fvec.len() - (SIG_SIZE_BYTES / std::mem::size_of::<i32>())..];

if do_fork() {
let mut cmd = Command::new("build/impcheck_check")
.arg(format!("-fifo-directives={}", pipe_directives))
.arg(format!("-fifo-feedback={}", pipe_feedback))
.arg("-check-model")
.spawn()
.expect("Failed to start impcheck_check");
cmd.wait().expect("Failed to wait on child");
std::process::exit(0);
}

let mut out_directives = OpenOptions::new()
.write(true)
.open(&pipe_directives)
.unwrap();
let mut in_feedback = File::open(&pipe_feedback).unwrap();

trusted_utils_write_char(TRUSTED_CHK_INIT, &mut out_directives);
trusted_utils_write_int(nb_vars, &mut out_directives);
trusted_utils_write_sig(&fsig, &mut out_directives);
await_ok(&mut out_directives, &mut in_feedback);

trusted_utils_write_char(TRUSTED_CHK_LOAD, &mut out_directives);
trusted_utils_write_int(f.len() as i32, &mut out_directives);
trusted_utils_write_ints(f, f.len() as u64, &mut out_directives);

trusted_utils_write_char(TRUSTED_CHK_END_LOAD, &mut out_directives);
await_ok(&mut out_directives, &mut in_feedback);

(checker_instance_id, out_directives, in_feedback)
}

fn confirm(cnf_input: &str, result: i32, sig: &[u8]) -> bool {
let sigstr: String = sig.iter().map(|b| format!("{:02x}", b)).collect();
let output = Command::new("build/impcheck_confirm")
.arg(format!("-formula-input={}", cnf_input))
.arg(format!("-result={}", result))
.arg(format!("-result-sig={}", sigstr))
.output()
.expect("Failed to execute impcheck_confirm");
output.status.success()
}

fn clean_up(checker_id: u64, mut out_directives: File, mut in_feedback: File) {
trusted_utils_write_char(TRUSTED_CHK_TERMINATE, &mut out_directives);
await_ok(&mut out_directives, &mut in_feedback);

drop(out_directives);
drop(in_feedback);

let pipe_parsed = format!(".parsed.{}.pipe", checker_id);
let pipe_directives = format!(".directives.{}.pipe", checker_id);
let pipe_feedback = format!(".feedback.{}.pipe", checker_id);
let _ = std::fs::remove_file(pipe_parsed);
let _ = std::fs::remove_file(pipe_directives);
let _ = std::fs::remove_file(pipe_feedback);

thread::sleep(Duration::from_millis(100)); // Wait for child processes to exit
}

fn produce_cls(
out_directives: &mut File,
in_feedback: &mut File,
id: u64,
clslen: i32,
lits: &[i32],
hintlen: i32,
hints: &[u64],
sig_or_null: Option<&mut [u8]>,
) {
trusted_utils_write_char(TRUSTED_CHK_CLS_PRODUCE, out_directives);
trusted_utils_write_ul(id, out_directives);
trusted_utils_write_int(clslen, out_directives);
trusted_utils_write_ints(lits, clslen as u64, out_directives);
trusted_utils_write_int(hintlen, out_directives);
trusted_utils_write_uls(hints, hintlen as u64, out_directives);
trusted_utils_write_bool(sig_or_null.is_some(), out_directives);
await_ok(out_directives, in_feedback);
if let Some(sig) = sig_or_null {
trusted_utils_read_sig(sig, in_feedback);
}
}

fn import_cls(
out_directives: &mut File,
in_feedback: &mut File,
id: u64,
clslen: i32,
lits: &[i32],
signature: &[u8],
) {
trusted_utils_write_char(TRUSTED_CHK_CLS_IMPORT, out_directives);
trusted_utils_write_ul(id, out_directives);
trusted_utils_write_int(clslen, out_directives);
trusted_utils_write_ints(lits, clslen as u64, out_directives);
trusted_utils_write_sig(signature, out_directives);
await_ok(out_directives, in_feedback);
}

fn delete_cls(out_directives: &mut File, in_feedback: &mut File, ids: &[u64], nb_ids: i32) {
trusted_utils_write_char(TRUSTED_CHK_CLS_DELETE, out_directives);
trusted_utils_write_int(nb_ids, out_directives);
trusted_utils_write_uls(ids, nb_ids as u64, out_directives);
await_ok(out_directives, in_feedback);
}

#[test]
fn test_trivial_sat() {
println!("[TEST] --- begin test_trivial_sat() ---");
let cnf = "cnf/trivial-sat.cnf";
let (chkid, mut out_directives, mut in_feedback) = setup(cnf);

trusted_utils_write_char(TRUSTED_CHK_VALIDATE_SAT, &mut out_directives);
trusted_utils_write_int(2, &mut out_directives);
let model = [1, -2];
trusted_utils_write_ints(&model, 2, &mut out_directives);
await_ok(&mut out_directives, &mut in_feedback);
let mut sat_sig = [0u8; SIG_SIZE_BYTES];
trusted_utils_read_sig(&mut sat_sig, &mut in_feedback);

let ok = confirm(cnf, 10, &sat_sig);
do_assert(ok);

clean_up(chkid, out_directives, in_feedback);
println!("[TEST] ---  end  test_trivial_sat() ---\n");
}

#[test]
fn test_trivial_unsat() {
println!("[TEST] --- begin test_trivial_unsat() ---");
let cnf = "cnf/trivial-unsat.cnf";
let (chkid, mut out_directives, mut in_feedback) = setup(cnf);

let cls_5 = [1];
let hints_5 = [1, 2];
produce_cls(&mut out_directives, &mut in_feedback, 5, 1, &cls_5, 2, &hints_5, None);
let cls_6 = [-1];
let hints_6 = [3, 4];
produce_cls(&mut out_directives, &mut in_feedback, 6, 1, &cls_6, 2, &hints_6, None);
let hints_7 = [5, 6];
produce_cls(&mut out_directives, &mut in_feedback, 7, 0, &[], 2, &hints_7, None);

trusted_utils_write_char(TRUSTED_CHK_VALIDATE_UNSAT, &mut out_directives);
await_ok(&mut out_directives, &mut in_feedback);
let mut unsat_sig = [0u8; SIG_SIZE_BYTES];
trusted_utils_read_sig(&mut unsat_sig, &mut in_feedback);

let ok = confirm(cnf, 20, &unsat_sig);
do_assert(ok);

clean_up(chkid, out_directives, in_feedback);
println!("[TEST] ---  end  test_trivial_unsat() ---\n");
}

#[test]
fn test_trivial_unsat_x2() {
println!("[TEST] --- begin test_trivial_unsat_x2() ---");
let cnf = "cnf/trivial-unsat.cnf";
let (chkid_1, mut out_directives_1, mut in_feedback_1) = setup(cnf);
let (chkid_2, mut out_directives_2, mut in_feedback_2) = setup(cnf);

let cls_5 = [1];
let hints_5 = [1, 2];
let mut sig_5 = [0u8; SIG_SIZE_BYTES];
produce_cls(&mut out_directives_1, &mut in_feedback_1, 5, 1, &cls_5, 2, &hints_5, Some(&mut sig_5));
let cls_6 = [-1];
let hints_6 = [3, 4];
let mut sig_6 = [0u8; SIG_SIZE_BYTES];
produce_cls(&mut out_directives_2, &mut in_feedback_2, 6, 1, &cls_6, 2, &hints_6, Some(&mut sig_6));

let del_ids = [3, 4];
delete_cls(&mut out_directives_2, &mut in_feedback_2, &del_ids, 2);

import_cls(&mut out_directives_1, &mut in_feedback_1, 6, 1, &cls_6, &sig_6);
import_cls(&mut out_directives_2, &mut in_feedback_2, 5, 1, &cls_5, &sig_5);

let hints_7 = [5, 6];
produce_cls(&mut out_directives_1, &mut in_feedback_1, 7, 0, &[], 2, &hints_7, None);

trusted_utils_write_char(TRUSTED_CHK_VALIDATE_UNSAT, &mut out_directives_1);
await_ok(&mut out_directives_1, &mut in_feedback_1);
let mut unsat_sig = [0u8; SIG_SIZE_BYTES];
trusted_utils_read_sig(&mut unsat_sig, &mut in_feedback_1);

let ok = confirm(cnf, 20, &unsat_sig);
do_assert(ok);

clean_up(chkid_1, out_directives_1, in_feedback_1);
clean_up(chkid_2, out_directives_2, in_feedback_2);
println!("[TEST] ---  end  test_trivial_unsat_x2() ---\n");
}

fn main() {
}

