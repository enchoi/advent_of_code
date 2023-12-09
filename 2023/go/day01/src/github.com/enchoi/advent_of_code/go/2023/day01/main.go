package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {

	data, err := os.ReadFile("input.txt")
	if err != nil {
		panic("no such file")
	}
	// fmt.Printf("%v", string(data))
	data_str := string(data)

	sum := 0
	for _, data := range strings.Split(data_str, "\n") {
		first := getFirstDigit(data)
		rev := reverseString(data)
		last := getFirstDigit(rev)
		sum += first*10 + last
	}

	fmt.Printf("The result is : %v\n", sum)
}

func getFirstDigit(line string) int {
	for _, r := range line {
		if unicode.IsDigit(r) {
			integer, _ := strconv.ParseInt(string(r), 10, 64)
			return int(integer)
		}
	}
	return 0
}

// it's a copy but I whant to rewrite it to check if a understand the logic
func reverseString(s string) string {
	// transform it into runes
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < len(runes)/2; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)

}
