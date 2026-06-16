package main

import "fmt"

// SharedStruct is a struct used across multiple files
type SharedStruct struct {
	ID        int
	Name      string
	Value     float64
	Constants []string
}

// Method is a method of SharedStruct
func (s *SharedStruct) Method() string {
	return s.Name
}

// SharedInterface defines behavior implemented across files
type SharedInterface interface {
	Process() error
	GetName() string
}

// SharedConstant is used in multiple files
const SharedConstant = "shared value"

// SharedType is a custom type used across files
type SharedType int

// Process implements SharedInterface for SharedStruct
func (s *SharedStruct) Process() error {
	fmt.Printf("Processing %s with ID %d\n", s.Name, s.ID)
	return nil
}

// GetName implements SharedInterface for SharedStruct
func (s *SharedStruct) GetName() string {
	return s.Name
}
