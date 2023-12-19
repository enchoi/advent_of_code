package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

const training string = `Time:      7  15   30
Distance:  9  40  200`

func main() {
	data := training
	data = getFileContent("src/input.txt")
	times, distances := parseData(data)
	fmt.Printf("times: %v, distances: %v\n", times, distances)
	computePart1(times, distances)
}

func parseData(data string) ([]int, []int) {
	split := strings.Split(data, "\n")
	times := getInt(split[0])
	distances := getInt(split[1])
	return times, distances
}

func getInt(line string) []int {
	ints := make([]int, 0)
	line = strings.Replace(line, " ", "", -1)
	for _, chunk := range strings.Split(line, ":") {
		if integer, err := strconv.Atoi(chunk); err == nil {
			ints = append(ints, integer)
		}
	}
	return ints
}

func computePart1(times, distances []int) {
	product := 1
	for i := 0; i < len(times); i++ {
		count := 0
		for speed := 0; speed < times[i]; speed++ {
			if speed*(times[i]-speed) > distances[i] {
				count++
			}
		}
		product *= count
	}
	fmt.Printf("Product: %v\n", product)
}

func getFileContent(path string) string {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	return string(data)
}
