package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

const training string = `32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483`

type CamelCard int
type CamelCardHandType int

const (
	HighCard CamelCardHandType = iota + 1
	OnePair
	TwoPair
	ThreeOfAKind
	FullHouse
	FourOfAKind
	FiveOfAKind
)

const (
	O1 CamelCard = iota + 1
	T2
	T3
	F4
	F5
	S6
	S7
	E8
	N9
	T
	J
	Q
	K
	A
)

func CamelCardFromRune(r rune) CamelCard {
	switch r {
	case '1':
		return O1
	case '2':
		return T2
	case '3':
		return T3
	case '4':
		return F4
	case '5':
		return F5
	case '6':
		return S6
	case '7':
		return S7
	case '8':
		return E8
	case '9':
		return N9
	case 'T':
		return T
	case 'J':
		return J
	case 'Q':
		return Q
	case 'K':
		return K
	case 'A':
		return A
	}
	panic("should not arrive here")
}

func main() {
	// fmt.Printf("counter: %v\n", getCounter("23253"))
	// fmt.Printf("23253 => %v\n", getHandType(getCounter("23253")))
	// panic("")
	data := training
	data = getFileContent("src/input.txt")
	hands_scores := parseData(data)
	// fmt.Printf("hands: %v\n", hands_scores)
	hands := getHands(hands_scores)
	sortHands(hands)
	// fmt.Printf("hands: %v\n", hands)
	computeScore(hands_scores, hands)

}

//	func getTypes(hands_keys []string) map[string]CamelCardHandType {
//		counters := make(map[string]CamelCardHandType)
//		for _, hand := range hands_keys {
//			counter := getCounter(hand)
//			counters[hand] = getHandType(counter)
//		}
//		return counters
//	}
func getCounter(str string) map[CamelCard]int {
	cards := make(map[CamelCard]int)
	for _, r := range str {
		card := CamelCardFromRune(r)
		cards[card] += 1
	}
	return cards
}

func getHandType(cards map[CamelCard]int) CamelCardHandType {
	tmp := make(map[int]int)
	for _, value := range cards {
		tmp[value] += 1
	}
	if tmp[5] == 1 {
		return FiveOfAKind
	}
	if tmp[4] == 1 {
		return FourOfAKind
	}
	if tmp[3] == 1 && tmp[2] == 1 {
		return FullHouse
	}
	if tmp[3] == 1 {
		return ThreeOfAKind
	}
	if tmp[2] == 2 {
		return TwoPair
	}
	if tmp[2] == 1 {
		return OnePair
	}
	return HighCard
}

func parseData(data string) map[string]int {
	hands := make(map[string]int)
	lines := strings.Split(data, "\n")
	for _, line := range lines {
		split := strings.Split(line, " ")
		hand := split[0]
		if score, err := strconv.Atoi(split[1]); err == nil {
			hands[hand] = score
		}
	}
	return hands
}

func getHands(hands_scores map[string]int) []string {
	hands := make([]string, len(hands_scores))
	i := 0
	for k := range hands_scores {
		hands[i] = k
		i++
	}
	return hands
}

func sortHands(hands []string) {
	sort.Slice(hands, func(i, j int) bool {
		type_i := getHandType(getCounter(hands[i]))
		type_j := getHandType(getCounter(hands[j]))
		if type_i != type_j {
			return type_i < type_j
		}
		for k := 0; k < len(hands[i]); k++ {
			card_i := CamelCardFromRune(rune(hands[i][k]))
			card_j := CamelCardFromRune(rune(hands[j][k]))
			if card_i != card_j {
				return card_i < card_j
			}
		}
		panic("should not arrive here")
	})
}

func computeScore(hands_scores map[string]int, keys []string) {
	score := 0
	for index, key := range keys {
		score += (hands_scores[key] * (index + 1))
	}
	fmt.Printf("Score: %v\n", score)
}

func getFileContent(path string) string {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	return string(data)
}
