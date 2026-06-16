package main

import "fmt"

// AnotherConsumer is a second consumer of shared types and functions
func AnotherConsumer() {
	// Use helper function
	fmt.Println("Another message:", HelperFunction())

	// Create another SharedStruct instance
	s := &SharedStruct{
		ID:        2,
		Name:      "another test",
		Value:     99.9,
		Constants: []string{SharedConstant, "extra"},
	}

	// Use the struct methods
	if name := s.GetName(); name != "" {
		fmt.Println("Got name:", name)
	}

	// Implement the interface with a custom type
	type CustomImplementor struct {
		SharedStruct
	}

	custom := &CustomImplementor{
		SharedStruct: *s,
	}

	// Custom type implements SharedInterface through embedding
	var iface SharedInterface = custom
	iface.Process()

	// Use shared type as a slice type
	values := []SharedType{1, 2, 3}
	for _, v := range values {
		fmt.Println("Value:", v)
	}
}
