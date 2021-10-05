use core::ffi;

use crate::my_app;

#[no_mangle]
fn simple_rust_func_called_from_go(arg1: u8, arg2: u16, arg3: u32) -> usize {
    my_app::my_app_simple_rust_func_called_from_go(arg1, arg2, arg3) as usize
}
