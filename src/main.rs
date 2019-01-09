extern crate ethereum_bn128;
extern crate parity_bytes as bytes;
extern crate rustc_hex;

use std::ffi::CStr;
use std::os::raw::c_char;

use rustc_hex::FromHex;
use rustc_hex::ToHex;

use bytes::BytesRef;

#[no_mangle]
pub fn ec_mul(input_hex_ptr: *const c_char) -> *const c_char {
    let input_hex = unsafe { CStr::from_ptr(input_hex_ptr) };
    let input_str: &str = input_hex.to_str().unwrap();
    let input_parsed = FromHex::from_hex(input_str).unwrap();

    let mut output = vec![0u8; 64];
    match ethereum_bn128::bn128_mul(&input_parsed[..], &mut BytesRef::Fixed(&mut output[..])) {
        Ok(_) => {
            let mut output_hex = output.to_hex();
            output_hex.push_str("\0");
            return output_hex.as_ptr() as *const c_char;
        }
        Err(_) => return "\0".as_ptr() as *const c_char,
    }
}

#[no_mangle]
pub fn ec_add(input_hex_ptr: *const c_char) -> *const c_char {
    let input_hex = unsafe { CStr::from_ptr(input_hex_ptr) };
    let input_str: &str = input_hex.to_str().unwrap();
    let input_parsed = FromHex::from_hex(input_str).unwrap();

    let mut output = vec![0u8; 64];
    match ethereum_bn128::bn128_add(&input_parsed[..], &mut BytesRef::Fixed(&mut output[..])) {
        Ok(_) => {
            let mut output_hex = output.to_hex();
            output_hex.push_str("\0");
            return output_hex.as_ptr() as *const c_char;
        }
        Err(_) => return "\0".as_ptr() as *const c_char,
    }
}

#[no_mangle]
pub fn ec_pairing(input_hex_ptr: *const c_char) -> *const c_char {
    let input_hex = unsafe { CStr::from_ptr(input_hex_ptr) };
    let input_str: &str = input_hex.to_str().unwrap();
    let input_parsed = FromHex::from_hex(input_str).unwrap();

    let mut output = vec![0u8; 32];
    match ethereum_bn128::bn128_pairing(&input_parsed[..], &mut BytesRef::Fixed(&mut output[..])) {
        Ok(_) => {
            let mut output_hex = output.to_hex();
            output_hex.push_str("\0");
            return output_hex.as_ptr() as *const c_char;
        }
        Err(_) => return "\0".as_ptr() as *const c_char,
    }
}

extern "C" {
    fn emscripten_exit_with_live_runtime();
}

fn main() {
    unsafe {
        emscripten_exit_with_live_runtime();
    }
}
