package main

import "testing"

func TestSimpleRustFuncCalledFromGo(t *testing.T) {
	SimpleRustFuncCalledFromGo()
}

func TestPassStringBySinglePointer(t *testing.T) {
	PassStringBySinglePointer()
}