package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

var MAX_COLOR = map[string]int{"blue": 14, "green": 13, "red": 12}

const training_input string = `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`

func main() {
	start := time.Now()
	// open file
	data, err := os.ReadFile("input.txt")
	if err != nil {
		panic("Mais quess tu fé wesh")
	}
	data_str := string(data)
	// data_str := training_input
	// get games
	// get hands
	games := getGames(data_str)

	// checks hands
	sum := compute_ok_games(games)

	// return result
	total := time.Since(start)
	fmt.Printf("%v\n", sum)
	fmt.Printf("Done in %v\n", total)

	// compute minimum dices required
	dices := miniumDicesPerGames(games)

	// compute the "power"
	result := computePower(dices)

	fmt.Printf("Power : %v\n", result)

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

func compute_ok_games(games map[int][]map[string]int) int {
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
	return sum
}

func miniumDicesPerGames(games map[int][]map[string]int) []map[string]int {
	list_games := make([]map[string]int, 0)
	for _, hands := range games {
		required := make(map[string]int)
		for _, hand := range hands {
			for key, value := range hand {
				if set_value, ok := required[key]; ok {
					if set_value < value {
						required[key] = value
					}
				} else {
					required[key] = value
				}

			}
		}
		list_games = append(list_games, required)
	}
	return list_games
}

func computePower(dicesRequired []map[string]int) int {
	sum := 0
	for _, dices := range dicesRequired {
		power := 1
		for _, nbDices := range dices {
			power *= nbDices
		}
		sum += power
	}
	return sum
}
