package main

import "testing"

func TestSimpleRustFuncCalledFromGo(t *testing.T) {
	SimpleRustFuncCalledFromGo()
}

func TestPassStringBySinglePointer(t *testing.T) {
	PassStringBySinglePointer()
}

func TestPassStringBySecondOrderPointer(t *testing.T) {
	PassStringBySecondOrderPointer()
}

func TestPassStringByReuseUnderlyingBuffer(t *testing.T) {
	PassStringByReuseUnderlyingBuffer()
}