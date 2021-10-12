use std::mem::ManuallyDrop;
use std::slice;

pub fn my_app_simple_rust_func_called_from_go(arg1: u8, arg2: u16, arg3: u32) -> usize {
	arg1 as usize + arg2 as usize + arg3 as usize
}

pub fn my_app_receive_string_and_return_string(s: String) -> String {
	if s.len() > 15 {
		// this path has new memory alloc
		s[0..15].to_string()
	} else {
		// this path doesn't have new memory alloc
		s
	}
}

pub fn my_app_receive_str_and_return_string(s: &str) -> String {
	// both path alloc new memory
	if s.len() > 15 {
		s[0..15].to_string()
	} else {
		s.to_string()
	}
}

pub fn my_app_receive_str_and_return_str(s: &str) -> &str {
	// neither path alloc new memory
	if s.len() > 15 {
		&s[0..15]
	} else {
		s
	}
}

pub unsafe fn my_app_receive_string_and_return_str<'a>(s: String) -> (&'a str, *const u8, usize, usize) {
	// this function is only used as an example to show that we can use unsafe 
	// rust to turn an owned type to a reference, you should not write such code
	// in production code.

	

	// neither path alloc new memory
	let my_slice = if s.len() > 15 {
		&*(&s[0..15] as &str as *const str)
	} else {
		&*(&s as &str as *const str)
	};

	// you can replace the following two lines using s.into_raw_parts()
	// s.into_raw_parts() internally use ManuallyDrop too
	// I use ManuallyDrop explicit here to show you how memory is managed
	let s = ManuallyDrop::new(s);	
	(my_slice, s.as_ptr(), s.len(), s.capacity())
}


pub unsafe fn my_app_free_string_by_raw_parts(s: *mut u8, len: usize, cap: usize) {
	String::from_raw_parts(s, len, cap);
}