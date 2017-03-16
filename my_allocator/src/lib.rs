#![feature(allocator)]
#![allocator]
#![no_std]
#![crate_type="rlib"]
#![crate_name="my_allocator"]

use core::ptr;

const ALIGN: usize = 8;

extern {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
    fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    if align > ALIGN {
        ptr::null_mut()
    } else {
        unsafe {
            malloc(size)
        }
    }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe {
        free(ptr);
    }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize, _align: usize) -> *mut u8 {
    unsafe {
        realloc(ptr, size)
    }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize) -> usize {
    old_size
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
