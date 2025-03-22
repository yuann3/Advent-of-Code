package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

type Hand struct {
	Cards string
	Bid   int
	Type  int
}

func main() {
	file, err := os.Open("../input/day7.in")
	if err != nil {
		fmt.Println("error opening file")
		return
	}
	defer file.Close()

	var hands []Hand

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		var cards string
		var bid int
		_, err := fmt.Sscanf(line, "%s %d", &cards, &bid)
		if err != nil {
			fmt.Println("Error parsing line:", err)
			continue
		}

		handType := getHandType(cards)

		hands = append(hands, Hand{Cards: cards, Bid: bid, Type: handType})
	}

	// Sort the hands
	sort.Slice(hands, func(i, j int) bool {
		if hands[i].Type != hands[j].Type {
			return hands[i].Type < hands[j].Type // Lower type value = weaker hand
		}

		return compareCardValues(hands[i].Cards, hands[j].Cards)
	})

	totalWinnings := 0
	for i, hand := range hands {
		rank := i + 1 // Rank starts at 1 for the weakest hand
		totalWinnings += hand.Bid * rank
	}

	fmt.Println("Total winnings:", totalWinnings)
}

func getHandType(cards string) int {
	counts := make(map[rune]int)
	for _, card := range cards {
		counts[card]++
	}

	// Five of a kind
	if len(counts) == 1 {
		return 7
	}

	// Four of a kind
	if len(counts) == 2 {
		for _, count := range counts {
			if count == 4 || count == 1 {
				return 6
			}
		}
		// If we're here, it must be a full house (3+2)
		return 5
	}

	// Three of a kind or two pair
	if len(counts) == 3 {
		for _, count := range counts {
			if count == 3 {
				return 4 // Three of a kind
			}
		}
		return 3 // Two pair
	}

	// One pair
	if len(counts) == 4 {
		return 2
	}

	// High card
	return 1
}

func compareCardValues(cards1, cards2 string) bool {
	cardValues := map[rune]int{
		'A': 14, 'K': 13, 'Q': 12, 'J': 11, 'T': 10,
		'9': 9, '8': 8, '7': 7, '6': 6, '5': 5, '4': 4, '3': 3, '2': 2,
	}

	for i := 0; i < len(cards1); i++ {
		value1 := cardValues[rune(cards1[i])]
		value2 := cardValues[rune(cards2[i])]

		if value1 != value2 {
			return value1 < value2
		}
	}

	return false
}
