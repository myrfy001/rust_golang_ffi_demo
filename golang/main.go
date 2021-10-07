package main

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L../rust/target/debug -lrust

#include "ffi_demo.h"
*/
import "C"


func SimpleRustFuncCalledFromGo() {
	arg1 := 123
	arg2 := 1234
	arg3 := 1234567

	cArg1 := C.uchar(arg1)
	cArg2 := C.short(arg2);
	cArg3 := C.int(arg3);
	ret := C.simple_rust_func_called_from_go(cArg1, cArg2, cArg3)
	if int(ret) != arg1 + arg2 + arg3{
		panic("SimpleRustFuncCalledFromGo Error")	
	}
}