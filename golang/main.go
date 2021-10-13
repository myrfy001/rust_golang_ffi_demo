package main

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L../rust/target/debug -lrust

#include "ffi_demo.h"
#include "stdlib.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)


func SimpleRustFuncCalledFromGo() {
	arg1 := 123
	arg2 := 1234
	arg3 := 1234567

	cArg1 := C.uint8_t(arg1)
	cArg2 := C.uint16_t(arg2);
	cArg3 := C.uint32_t(arg3);
	ret := C.simple_rust_func_called_from_go(cArg1, cArg2, cArg3)
	if int(ret) != arg1 + arg2 + arg3{
		panic("SimpleRustFuncCalledFromGo Error")	
	}
}

func PassStringBySinglePointer() {

	testProc := func(f int, x, y string) {
		goStr := x
		cStr := C.CString(goStr)  // Memory Alloc And String Copy
		defer C.free(unsafe.Pointer(cStr))
	
		var cStrRet *C.char
		switch f{
		case 1:
			cStrRet = C.receive_str_and_return_string(cStr)
		case 2:
			cStrRet = C.receive_string_and_return_string(cStr)
		case 3:
			cStrRet = C.receive_str_and_return_str(cStr)
		}
		
		goStrRet := C.GoString(cStrRet)  // Memory Alloc And String Copy
		C.free_cstring_alloc_by_rust(cStrRet)

		if goStrRet != y {
			panic(fmt.Sprintf("Error, expected %s, got %s", y, goStrRet))	
		}
	}

	testProc(1, "极客幼稚园是一个不错的微信公众号", "极客幼稚园")
	testProc(1, "Datafuse Lab", "Datafuse Lab")

	testProc(2, "极客幼稚园是一个不错的微信公众号", "极客幼稚园")
	testProc(2, "Datafuse Lab", "Datafuse Lab")

	testProc(3, "极客幼稚园是一个不错的微信公众号", "极客幼稚园")
	testProc(3, "Datafuse Lab", "Datafuse Lab")

}

func PassStringBySecondOrderPointer() {
	testProc := func(x, y string) {
		goStr := x
		cStr := C.CString(goStr)  // Memory Alloc And String Copy
		defer C.free(unsafe.Pointer(cStr))
	
		var cStrRet *C.char
		var cRawStr *C.char
		retCap := C.ulong(0)
		retLen := C.ulong(0)

		C.receive_string_and_return_str(cStr, &cStrRet, &cRawStr, &retLen, &retCap)
		
		
		goStrRet := C.GoString(cStrRet)  // Memory Alloc And String Copy
		C.free_string_alloc_by_rust_by_raw_parts(cStrRet, retLen, retCap)

		if goStrRet != y {
			panic(fmt.Sprintf("Error, expected %s, got %s", y, goStrRet))	
		}
	}

	testProc("极客幼稚园是一个不错的微信公众号", "极客幼稚园")
	testProc("Datafuse Lab", "Datafuse Lab")
}


func PassStringByReuseUnderlyingBuffer() {
	testProc := func(x, y string) {
		goStr := x
		cStr := C.CString(goStr)  // Memory Alloc And String Copy
		defer C.free(unsafe.Pointer(cStr))
	
		var cStrRet *C.char
		retLen := C.ulong(0)

		C.receive_str_and_return_str_no_copy(cStr, &cStrRet, &retLen)
		
		goStrRet := C.GoString(cStrRet)  // Memory Alloc And String Copy

		// Important: Not free the `cStrRet` here, the memory is allocated by Go at the above line:
		// `cStr := C.CString(goStr)``
		// and will be freed by the above `defer` statement

		if goStrRet != x {
			panic(fmt.Sprintf("Error, expected %s, got %s", x, goStrRet))	
		}
		goStrRetWithLengthLimit := goStrRet[:retLen]
		if goStrRetWithLengthLimit != y {
			panic(fmt.Sprintf("Error, expected %s, got %s", y, goStrRetWithLengthLimit))	
		}

		// Since we didn't change the start point of the string, the pointer returned by C library
		// is "useless"
		if goStr[:retLen] != y {
			panic(fmt.Sprintf("Error, expected %s, got %s", y, goStrRetWithLengthLimit))	
		}
	}

	testProc("极客幼稚园是一个不错的微信公众号", "极客幼稚园")
	testProc("Datafuse Lab", "Datafuse Lab")
}
