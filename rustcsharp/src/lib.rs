//external crates
extern crate time;
use time::{PreciseTime};

use std::os::raw::c_char;
use std::ffi::CString;

static mut STRING_POINTER: *mut c_char = 0 as *mut c_char;

#[no_mangle]
pub extern "C" fn hello_world() -> *mut c_char {
    let hw: &'static str = "Hello from rust!";
    store_string_on_heap(hw)
}

fn store_string_on_heap(string_to_store: &'static str) -> *mut c_char {
    //create a new raw pointer
    let pntr = CString::new(string_to_store).unwrap().into_raw();
    //store it in our static variable (REQUIRES UNSAFE)
    unsafe {
        STRING_POINTER = pntr;
    }
    //return the c_char
    return pntr;
}

#[no_mangle]
pub extern "C" fn free_string() {
    unsafe {
        let _ = CString::from_raw(STRING_POINTER);
        STRING_POINTER = 0 as *mut c_char;
    }
}

#[no_mangle]
pub extern "C" fn add_numbers(number1: i32, number2: i32) -> i32 {
    number1 + number2
}

#[no_mangle]
pub fn test_loop() -> i32 {
    let mut sum = 0;
    for i in 0..1000000000 {
        sum = i;
    }
    sum
}

#[repr(C)]
pub struct SampleStruct {    
    pub iterations: i32,
    pub duration: i64,
}

#[no_mangle]
pub fn time() -> SampleStruct {
    let start = PreciseTime::now();
    let k = test_loop();
    let end = PreciseTime::now();
    let duration = start.to(end).num_milliseconds();
    SampleStruct { iterations: k, duration: duration }
}

/*
#[cfg(test)]
mod tests {

    extern crate lib;

    use lib::hello_world;

    #[test]
    fn it_returns_hello_world() {
        assert_eq!(hello_world(), "hello world");
    }

    #[test]
    fn it_adds_numbers() {
        assert_eq!(add_numbers(1,2), 3);
    }
}
*/
