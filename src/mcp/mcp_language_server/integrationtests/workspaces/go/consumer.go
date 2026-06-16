package main

import "fmt"

// ConsumerFunction uses the helper function
func ConsumerFunction() {
	message := HelperFunction()
	fmt.Println(message)

	// Use shared struct
	s := &SharedStruct{
		ID:        1,
		Name:      "test",
		Value:     42.0,
		Constants: []string{SharedConstant},
	}

	// Call methods on the struct
	fmt.Println(s.Method())
	s.Process()

	// Use shared interface
	var iface SharedInterface = s
	fmt.Println(iface.GetName())

	// Use shared type
	var t SharedType = 100
	fmt.Println(t)
}
