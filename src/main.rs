extern crate libc;

#[link(name = "badmath", kind = "static")]
extern "C" {
    fn bad_add(v1: f32, v2: f32, v3: *mut i32) -> f32;
    fn change_val(v1: *mut i32) -> f32;
}

use std::mem;

fn main() {
    println!("Hello from RUST");
    unsafe {
        let my_num1: *mut i32 = libc::malloc(mem::size_of::<i32>()) as *mut i32;
        *my_num1 = 32;

        let my_num2: *mut i32 = libc::malloc(mem::size_of::<i32>()) as *mut i32;
        *my_num2 = 5;
        // if my_num1.is_null() {
        // panic!("failed to allocate memory-1");
        // }
        // libc::free(my_num1 as *mut libc::c_void);

        let my_num3: *mut i32 = libc::malloc(1000) as *mut i32;
        // if my_num3.is_null() {
        // panic!("failed to allocate memory-3");
        // }
        // libc::free(my_num3 as *mut libc::c_void);
        let res = bad_add(12., 10., my_num1);
        println!("Result is {}", &res);
        let res = change_val(my_num2);
        println!("Changed Value is {}", &res);
    };
}
