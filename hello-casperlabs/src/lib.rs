#![no_std]

extern crate alloc;
extern crate contract_ffi;

use alloc::string::String;
use contract_ffi::contract_api::{runtime, storage};

#[no_mangle]
pub extern "C" fn call() {
    let greeting = String::from("Hello, CasperLabs");
    let key = storage::new_turef(greeting);
    runtime::put_key("hello_casperlabs", &key.into());
}
