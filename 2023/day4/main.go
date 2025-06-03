package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Card struct {
	Id             int
	WinningNumbers map[int]bool
	YourNumbers    []int
}

func main() {
	file, err := os.Open("../input/day4.in")
	if err != nil {
		fmt.Println("file open error")
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	totalPoints := 0

	for scanner.Scan() {
		line := scanner.Text()
		card := parseCard(line)
		points := calculatePoints(card)
		totalPoints += points
		fmt.Printf("Card %d: %d points\n", card.Id, points)
	}

	fmt.Printf("Total points: %d\n", totalPoints)
}

func parseCard(line string) Card {
	parts := strings.Split(line, ": ")

	idPart := strings.TrimSpace(parts[0])
	idPart = strings.TrimPrefix(idPart, "Card ")
	id, _ := strconv.Atoi(idPart)

	/// number parts
	numberParts := strings.Split(parts[1], " | ")

	/// parse winning numbers
	winningNumberStr := strings.Fields(numberParts[0])
	winningNumber := make(map[int]bool)
	for _, numStr := range winningNumberStr {
		num, _ := strconv.Atoi(numStr)
		winningNumber[num] = true
	}

	/// parse your numbers
	yourNumberStr := strings.Fields(numberParts[1])
	yourNumber := make([]int, 0, len(yourNumberStr))
	for _, numStr := range yourNumberStr {
		num, _ := strconv.Atoi(numStr)
		yourNumber = append(yourNumber, num)
	}

	return Card{
		Id:             id,
		WinningNumbers: winningNumber,
		YourNumbers:    yourNumber,
	}
}

func calculatePoints(card Card) int {
	matches := 0

	for _, num := range card.YourNumbers {
		if card.WinningNumbers[num] {
			matches++
		}
	}

	if matches == 0 {
		return 0
	}

	points := 1
	for i := 1; i < matches; i++ {
		points *= 2
	}

	return points
}
