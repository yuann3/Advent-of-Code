package main

import (
	"bufio"
	"fmt"
	"os"
)

type Number struct {
	value    int
	row      int
	startCol int
	endCol   int
}

func main() {
	file, err := os.Open("../input/day3.in")
	if err != nil {
		fmt.Println("file open error")
	}
	defer file.Close()

	var grid []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		grid = append(grid, scanner.Text())
	}

	numbers := findNumbers(grid)

	sum := 0
	for _, num := range numbers {
		if isPartNumber(grid, num) {
			sum += num.value
		}
	}

	fmt.Println("[P1] Day3 Result:", sum)

}

func findNumbers(grid []string) []Number {
	var numbers []Number

	for row, line := range grid {
		col := 0
		for col < len(line) {
			if line[col] < '0' || line[col] > '9' {
				col++
				continue
			}

			// extract number
			startCol := col
			value := 0
			for col < len(line) && line[col] >= '0' && line[col] <= '9' {
				value = value*10 + int(line[col]-'0')
				col++
			}
			endCol := col - 1

			numbers = append(numbers, Number{
				value:    value,
				row:      row,
				startCol: startCol,
				endCol:   endCol})
		}
	}

	return numbers
}

func isPartNumber(grid []string, num Number) bool {
	directions := []struct{ dr, dc int }{
		{-1, -1}, {-1, 0}, {-1, 1},
		{0, -1}, {0, 1},
		{1, -1}, {1, 0}, {1, 1},
	}

	for col := num.startCol; col <= num.endCol; col++ {
		for _, dir := range directions {
			newRow, newCol := num.row+dir.dr, col+dir.dc

			if newRow >= 0 && newRow < len(grid) && newCol >= 0 && newCol < len(grid[newRow]) {
				char := grid[newRow][newCol]

				if (char < '0' || char > '9') && char != '.' {
					return true
				}
			}
		}
	}

	return false
}
