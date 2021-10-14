use std::mem::ManuallyDrop;
use std::os::raw::c_char;
use std::ffi::{CStr, CString};

use crate::my_app;

#[no_mangle]
fn simple_rust_func_called_from_go(arg1: u8, arg2: u16, arg3: u32) -> usize {
    my_app::my_app_simple_rust_func_called_from_go(arg1, arg2, arg3) as usize
}


#[no_mangle]
pub fn receive_str_and_return_string(s: *const c_char) -> *const c_char {
	let cstr = {
        // ALWAYS check if a pointer is null !!!!
        assert!(!s.is_null());
        // what the following line do is iter over the memory address starts 
        // from s and find the first null byte, i.e.,get the length of the str.
        // compared to `*const c_char`, `&CStr` save length in itself.
        // since we read from a raw pointer, we need to use unsafe block.
        unsafe{CStr::from_ptr(s)}
    };

    // the following line will iter over the str again to check all the bytes 
    // are valid utf-8 encoding. if we pass the check, we only change the fat 
    // pointer type from `&CStr` to `&str` to tell the type system: This is a 
    // string that satisfy rust's requirement, you can use it in safe rust. The
    // underlying byte buffer is not touched.
    // since the `&str` is a reference, it now refs to the memory owned by the 
    // foreign system (for example, Golang), so, it's the caller's responsibility
    // to make sure the string is not freed during the call.
    let rstr = cstr.to_str().expect("not valid utf-8 string");

    // in the above code, we didn't alloc new memory space, but we went
    // through the whole string twice to make it safe. you need to think 
    // about the overhead if this is on your hot path.
    // Now, we got a rust str and can call safe rust functions!
    let ret = my_app::my_app_receive_str_and_return_string(rstr);

    // Now we got a String, there was a memory alloc in the above function call.
    // We will return a pointer to the golang caller, the pointer must point to
    // a Null terminated string, but a rust String is not ensured to be Null 
    // terminated, the std lib provide CString for help.
    // But there is overhead again :(
    // * First, it will check again whether the whole underlying bytes
    //   contain null in the middle, so it will go through the string again.
    // * Second, it will try to push a Null byte to the underlying buffer
    //   (because rust string does not end with null, it store the length of
    //   the string in the string header struct), if the underlying buffer 
    //   has enough free space, then no overhead, but if the underlying buffer
    //   is full, then a reallocation will happen...
    let c_ret = CString::new(ret).expect("null byte in the middle");

    // Finally, We need to return a pointer pointing to the heap memory of the
    // CString. The into_raw() will consume the c_ret, let the compiler 
    // forget the heap memory it owned, and return a raw pointer pointing to
    // the heap address.
    // But this cause another problem: who will free the memory later?
    // We will solve the problem later.
    c_ret.into_raw()

    // IMPORTNT NOTICE
    // Up to now, you should known that:
    // * the input of this ffi function is owned by the caller(memory is 
    //   allocated by golang) and should be free by the code in golang
    // * the return value is owned by rust (memory is allocated by rust) and
    //   should be freed by code in rust.
}


#[no_mangle]
pub fn receive_string_and_return_string(s: *const c_char) -> *const c_char {
    // the following lines which don't have comments is the same as previous
    // functions, you can refer to the previous comments if you can't understand
    // why we need those lines of code.
    
    let cstr = {
        assert!(!s.is_null());
        unsafe{CStr::from_ptr(s)}
    };

    // to_str() will go through the string and to_string() will alloc new memory
    // and copy the whole string, so the following line will do one allocation
    // and two pass of scan.
    let string = cstr.to_str().expect("not valid utf-8 string").to_string();

    // the following function call maybe alloc new memory, depending on the string length.
    let ret = my_app::my_app_receive_string_and_return_string(string);

    let c_ret = CString::new(ret).expect("null byte in the middle");
    c_ret.into_raw()
    
    // IMPORTNT NOTICE
    // the same as above, input memory is owned by golang and return value 
    // is owned by rust
}


#[no_mangle]
pub fn receive_str_and_return_str(s: *const c_char) -> *const c_char {
	// the following lines which don't have comments is the same as previous
    // functions, you can refer to the previous comments if you can't understand
    // why we need those lines of code.
    
    let cstr = {
        assert!(!s.is_null());
        unsafe{CStr::from_ptr(s)}
    };

    let rstr = cstr.to_str().expect("not valid utf-8 string");

    // the following function call won't alloc memory, it will reuse the 
    // underlying buffer, it seems good now ~
    let ret = my_app::my_app_receive_str_and_return_str(rstr);

    // But...,if the above function call return a sub slice of the input string,
    // there won't be a Null byte at the end of the sub string. So, we have to 
    // create a CString again, it has overhead!
    let c_ret = CString::new(ret).expect("null byte in the middle");
    c_ret.into_raw()

    // Important Notice:
    // This is an example to show the overhead when dealing with rust reference 
    // to strings. In rust, the `my_app::my_app_receive_str_and_return_str(str)`
    // function both take and return refernece to str, so we can avoid copy data
    // in the underlying buffer.
    // But depending on the logics in the wrapped function, the return value
    // may or maynot reuse the input data's underlying memory at ffi boundary.
    // In this example:
    // Although in rust, it's zero-copy, but after wrapped by the ffi interface,
    // The return value can't reuse the memory space.
    //
    // If you really want a zero-copy, then you should redesign the api interface
    // we will show it later.
    //
    // As the author of this library, it's your responsibility to write a clear
    // document telling the user what happened with the input and output data.
    //
    // Al last, the same as above, input memory is owned by golang and 
    // return value is owned by rust
}

#[no_mangle]
// the follow ffi interface is a very ugly api design, only used as example, never use in real code.
pub fn receive_string_and_return_str(s: *const c_char, new_ptr: *mut *const c_char, c_origin_ptr: *mut *const c_char, len: *mut usize, cap: *mut usize) {
	// the following lines which don't have comments is the same as previous
    // functions, you can refer to the previous comments if you can't understand
    // why we need those lines of code.
    
    let cstr = {
        assert!(!s.is_null());
        unsafe{CStr::from_ptr(s)}
    };

    let string = cstr.to_str().expect("not valid utf-8 string").to_string();

    
    let (ret, t_c_origin_ptr, t_len, t_cap) = unsafe{my_app::my_app_receive_string_and_return_str(string)};


    let c_ret = CString::new(ret).expect("null byte in the middle");
    unsafe {
        *new_ptr = c_ret.into_raw();
        *c_origin_ptr = t_c_origin_ptr as *const i8;
        *len = t_len;
        *cap = t_cap;
    }
}

#[no_mangle]
pub unsafe fn free_string_alloc_by_rust_by_raw_parts(s: *mut c_char, len: usize, cap: usize) {
	String::from_raw_parts(s as *mut u8, len, cap);
}

#[no_mangle]
pub unsafe fn free_cstring_alloc_by_rust(s: *mut c_char) {
	CString::from_raw(s);
}


#[no_mangle]
pub fn receive_str_and_return_str_no_copy(s: *const c_char, new_ptr: *mut *const c_char, len: *mut usize) {
	// the following lines which don't have comments is the same as previous
    // functions, you can refer to the previous comments if you can't understand
    // why we need those lines of code.
    
    let cstr = {
        assert!(!s.is_null());
        unsafe{CStr::from_ptr(s)}
    };

    let rstr = cstr.to_str().expect("not valid utf-8 string");
    let ret = my_app::my_app_receive_str_and_return_str(rstr);
    let c_ret = ret.as_ptr();

    unsafe {
        *new_ptr = c_ret as *const i8;
        *len = ret.len();
    }

    // Important Notice:
    // This function has no `to_owned()` nor `to_string()`, so it won't alloc
    // any new memory on heap.
    // To solve the problem that returned sub string not end with Null byte,
    // we redesigned the api so that it will return the length of the string.
    // If you write a library like this, you should write it clear in your
    // document that the returned pointer points to the caller's memory.
}