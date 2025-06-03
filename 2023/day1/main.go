package main

import (
	"bufio"
	"fmt"
	"os"
	"unicode"
)

func main() {
	file, err := os.Open("../input/day1.in")
	if err != nil {
		fmt.Println("Error opening files")
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	sum := 0

	for scanner.Scan() {
		line := scanner.Text()

		// Find first digit
		var first, last int
		for _, r := range line {
			if unicode.IsDigit(r) {
				first = int(r - '0')
				break
			}
		}

		// Find last digit
		for i := len(line) - 1; i >= 0; i-- {
			if unicode.IsDigit(rune(line[i])) {
				last = int(line[i] - '0')
				break
			}
		}

		sum += first*10 + last
	}

	fmt.Println("Part 1: Sum of calibration values:", sum)
}
