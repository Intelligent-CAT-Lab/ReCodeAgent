use std::ptr::copy_nonoverlapping;
use serde::{Deserialize, Serialize};

extern "C" {
    fn context_alloc(_: usize) -> *const core::ffi::c_void;
    pub fn context_reset();
}

#[derive(Serialize, Deserialize)]
pub struct ForeignExecution {
    pub return_value: serde_json::Value,
    pub execution_success: bool,
    pub input_modifications: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct ExecutionData {
    pub inputs: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub result: ForeignExecution,
}

pub struct TestExecutor<F: FnMut()>(pub F);
impl<F: FnMut()> TestExecutor<F> {
    pub fn run(&mut self) {
        (self.0)()
    }
}
impl<F: FnMut()> Drop for TestExecutor<F> {
    fn drop(&mut self) {
        unsafe {
            context_reset();
        }
    }
}

pub fn alloc_raw_bytes(bytes: &[u8]) -> *const core::ffi::c_char {
    let data = unsafe { context_alloc(bytes.len()) };
    unsafe {
        copy_nonoverlapping(bytes.as_ptr(), data as *mut _, bytes.len());
    }
    return data as *const _;
}

pub type JSONObject = *mut Opaque;

#[repr(C)]
pub struct Opaque {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub fn ser<T: Serialize>(data: &T) -> *mut Opaque {
    let mut bytes = serde_json::to_vec(data).unwrap();
    bytes.reserve_exact(1);
    bytes.push(0);
    alloc_raw_bytes(&bytes) as *mut core::ffi::c_char as *mut Opaque
}

pub fn de<'a, T: Deserialize<'a>>(object: *mut Opaque) -> T {
    let c_string =
        unsafe { core::ffi::CStr::from_ptr(object as *mut core::ffi::c_char as *const _) };
    serde_json::from_str(c_string.to_str().unwrap()).unwrap()
}

