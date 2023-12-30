package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {

	data := getFileContent("src/input.txt")
	secquence, nodes := parseData(data)
	computePart2(secquence, nodes)

}

func getFileContent(path string) string {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	return string(data)
}

func parseData(data string) ([]int, map[string][]string) {
	sequence := make([]int, 0)
	nodes := make(map[string][]string)
	for index, line := range strings.Split(data, "\n") {
		if index == 0 {
			sequence = append(sequence, parseSequence(line)...)
			continue
		}
		if line == "" {
			continue
		}
		splited := strings.SplitN(line, " = ", 2)
		name, rest := splited[0], splited[1]
		leftRight := strings.Split(rest, ", ")
		left := strings.Replace(leftRight[0], "(", "", -1)
		right := strings.Replace(leftRight[1], ")", "", -1)
		value := make([]string, 2)
		value[0] = left
		value[1] = right
		nodes[name] = value
	}
	return sequence, nodes
}

func parseSequence(str string) []int {
	parsed := make([]int, len(str))
	for idx, r := range str {
		var i int
		if r == 'L' {
			i = 0
		} else {
			i = 1
		}
		parsed[idx] = i
	}
	return parsed
}

func computePart1(sequence []int, nodes map[string][]string) {
	current := "AAA"
	index := 0
	for current != "ZZZ" {
		current = nodes[current][sequence[index%len(sequence)]]
		index++
	}
	fmt.Printf("Result: %v\n", index)
}

func computePart2(sequence []int, nodes map[string][]string) {
	starts := make([]string, 0)
	for key, _ := range nodes {
		if key[len(key)-1] == 'A' {
			starts = append(starts, key)
		}
	}
	cycles := make([]int, 0)
	for index := range starts {
		cur := starts[index]
		seq := 0
		for cur[len(cur)-1] != 'Z' {
			cur = nodes[cur][sequence[seq%len(sequence)]]
			seq++
		}
		cycles = append(cycles, seq)
	}
	fmt.Printf("Result: %v\n", LCM(cycles[0], cycles[1], cycles[2:]...))
}

// greatest common divisor (GCD) via Euclidean algorithm
func GCD(a, b int) int {
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}

// find Least Common Multiple (LCM) via GCD
func LCM(a, b int, integers ...int) int {
	result := a * b / GCD(a, b)

	for i := 0; i < len(integers); i++ {
		result = LCM(result, integers[i])
	}

	return result
}
