package main

import (
	"fmt"

	"github.com/hbollon/go-edlib"
)

func main() {
	strList := []string{"test", "tester", "tests", "testers", "testing", "tsting", "sting"}
	res, err := edlib.FuzzySearchThreshold("testnig", strList, 0.7, edlib.Levenshtein)
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Printf("Result for 'testnig': %s", res)
	}

	res, err = edlib.FuzzySearchThreshold("hello", strList, 0.7, edlib.Levenshtein)
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Printf("Result for 'hello': %s", res)
	}
}
