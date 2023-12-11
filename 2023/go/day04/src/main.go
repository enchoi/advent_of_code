package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

const training string = `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11`

type Pair struct {
	winner  []int64
	numbers []int64
}

func main() {
	data := training
	data = getFileContent("src/input.txt")
	// parse cards
	cards := getCards(data)

	// compute points
	points := computeCards(cards)

	// sum total points
	total := 0
	for _, point := range points {
		total += point
	}
	fmt.Printf("Total: %v\n", total)

}

func getFileContent(path string) string {
	data, err := os.ReadFile(path)
	if err != nil {
		panic("Nope")
	}
	return string(data)
}

func getCards(data string) map[int64]Pair {
	cards := make(map[int64]Pair)
	for _, line := range strings.Split(data, "\n") {
		card := parseCardNumber(&line)
		split := strings.Split(line, " | ")
		winner := parseNumbers(&split[0])
		numbers := parseNumbers(&split[1])
		cards[card] = Pair{winner, numbers}
	}
	return cards
}

func parseNumbers(str *string) []int64 {
	nums := make([]int64, 0)
	split := strings.Split(*str, " ")
	for _, chunk := range split {
		if num, err := strconv.ParseInt(chunk, 10, 32); err == nil {
			nums = append(nums, num)
		}
	}
	return nums
}

func parseCardNumber(str *string) int64 {
	split := strings.Split(*str, ":")
	split = strings.Split(split[0], " ")
	num, err := strconv.ParseInt(split[len(split)-1], 10, 32)
	if err != nil {
		panic("error parseing card number")
	}
	return num
}

func computePoint(winner, numbers []int64) int {
	sum := 0
numbers:
	for _, n := range numbers {
		for _, w := range winner {
			if n == w {
				sum += 1
				continue numbers
			}
		}
	}
	if sum == 0 {
		return sum
	}
	return 1 << (sum - 1)
}

func computeCards(cards map[int64]Pair) map[int64]int {
	points := make(map[int64]int)
	for card, pair := range cards {
		points[card] = computePoint(pair.winner, pair.numbers)
	}
	return points
}
