package main

import "fmt"

// TestStruct is a test struct with fields and methods
type TestStruct struct {
	Name string
	Age  int
}

// TestMethod is a method on TestStruct
func (t *TestStruct) Method() string {
	return t.Name
}

// TestInterface defines a simple interface
type TestInterface interface {
	DoSomething() error
}

// TestType is a type alias
type TestType string

// TestConstant is a constant
const TestConstant = "constant value"

// TestVariable is a package variable
var TestVariable = 42

// TestFunction is a function for testing
func TestFunction() {
	fmt.Println("This is a test function")
}

// CleanFunction is a clean function without errors
func CleanFunction() {
	fmt.Println("This is a clean function without errors")
}
