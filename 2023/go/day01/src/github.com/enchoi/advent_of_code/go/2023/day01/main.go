package main

import (
	"fmt"
	"os"
	"regexp"
	"sort"
	"strings"
)

var (
	digits = map[string]int{"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9,
		"1": 1, "2": 2, "3": 3, "4": 4, "5": 5, "6": 6, "7": 7, "8": 8, "9": 9}
	digitsKeys = []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"}
)

func main() {

	data, err := os.ReadFile("input.txt")
	if err != nil {
		panic("no such file")
	}

	data_str := string(data)

	sum := 0
	for _, data := range strings.Split(data_str, "\n") {
		hashmap := getDigits(data)
		keys := getMapKey(hashmap)
		sum += hashmap[keys[0]]*10 + hashmap[keys[len(keys)-1]]
	}

	fmt.Printf("The result is : %v\n", sum)
}

func getDigits(line string) map[int]int {
	hashmap := make(map[int]int)
	for _, str := range digitsKeys {
		r := regexp.MustCompile(str)
		matches := r.FindAllIndex([]byte(line), -1)
		for _, match := range matches {
			hashmap[match[0]] = digits[str]
		}
	}
	return hashmap
}

func getMapKey(hashmap map[int]int) []int {
	keys := make([]int, 0, len(hashmap))
	for k := range hashmap {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	return keys
}
