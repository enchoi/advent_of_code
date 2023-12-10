package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

var MAX_COLOR = map[string]int{"blue": 14, "green": 13, "red": 12}

func main() {
	start := time.Now()
	// open file
	data, err := os.ReadFile("input.txt")
	if err != nil {
		panic("Mais quess tu fé wesh")
	}
	data_str := string(data)
	// get games
	// get hands
	games := getGames(data_str)

	// checks hands
	sum := 0
HAND:
	for num, hands := range games {
		for _, hand := range hands {
			for color, number := range hand {
				if number > MAX_COLOR[color] {
					continue HAND
				}
			}
		}
		sum += num
	}
	// return result
	total := time.Since(start)
	fmt.Printf("%v\n", sum)
	fmt.Printf("Done in %v\n", total)

}

func getGames(data string) map[int][]map[string]int {
	hash := make(map[int][]map[string]int)
	for _, line := range strings.Split(data, "\n") {
		split := strings.Split(line, ":")
		game_hands := split[0]
		game_number_str := strings.Split(game_hands, " ")[1]
		game_number, err := strconv.Atoi(game_number_str)
		if err != nil {
			panic("Error parsing game number")
		}
		part2 := split[1]
		hands := getHand(strings.Trim(part2, " "))
		hash[game_number] = hands
	}
	return hash
}

func getHand(hands string) []map[string]int {
	split := strings.Split(hands, ";")
	list := make([]map[string]int, 1)
	for _, part := range split {
		hash := make(map[string]int)
		for _, hand := range strings.Split(part, ",") {
			split := strings.Split(strings.Trim(hand, " "), " ")
			dice, _ := strconv.Atoi(split[0])
			color := split[1]

			hash[string(color)] = dice

		}
		list = append(list, hash)
	}
	return list
}
