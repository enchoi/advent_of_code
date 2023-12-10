package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

const _training string = `467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..`

type Match struct {
	Start int
	End   int
	Line  int
	Value string
}

func main() {
	reg_numbers := regexp.MustCompile(`\d+`)
	reg_symbols := regexp.MustCompile(`(\*|\#|\+|\$|\/|\@|\%|\&|\=|\-)`)

	content := _training
	content = getFileContent("input.txt")
	numbers := getMatches(reg_numbers, &content)
	symbols := getMatches(reg_symbols, &content)

	// search valid number
	valid_numbers := getValidNumbers(numbers, symbols)

	result := computeResult(valid_numbers)
	fmt.Printf("result: %v\n", result)
}

func getMatches(pattern *regexp.Regexp, text *string) []Match {
	matches := make([]Match, 0)
	for nbLine, line := range strings.Split(*text, "\n") {
		for _, match := range pattern.FindAllStringIndex(line, -1) {
			matches = append(matches, Match{Start: match[0], End: match[1], Line: nbLine, Value: string(line[match[0]:match[1]])})
		}
	}
	// fmt.Printf("matches: %v\n", matches)
	return matches
}

func getFileContent(path string) string {
	data, err := os.ReadFile("src/input.txt")
	if err != nil {
		panic("Nope")
	}
	return string(data)
}

func getValidNumbers(numbers, symbols []Match) []Match {
	valid := make([]Match, 0)
	for _, number := range numbers {
		for _, symbol := range symbols {
			if symbol.End >= number.Start &&
				symbol.Start <= number.End &&
				symbol.Line >= number.Line-1 &&
				symbol.Line <= number.Line+1 {
				valid = append(valid, number)
			}
		}
	}
	return valid
}

func computeResult(valids []Match) int {
	sum := 0
	for _, v := range valids {
		if value, err := strconv.Atoi(v.Value); err == nil {
			sum += value
		}
	}
	return sum
}
