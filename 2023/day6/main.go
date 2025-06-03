package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("../input/day6.in")
	if err != nil {
		fmt.Println("error opening file")
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var times []int
	if scanner.Scan() {
		times = parseNumbers(scanner.Text())
	}

	var distances []int
	if scanner.Scan() {
		distances = parseNumbers(scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("error reading file: ", err)
		return
	}

	if len(times) != len(distances) {
		fmt.Println("Mismatch between times and distances")
		return
	}

	if len(times) > 0 {
		visualizeRace(times[0], distances[0])
	}

	result := 1
	for i := 0; i < len(times); i++ {
		ways := countWaysToWin(times[i], distances[i])
		fmt.Printf("Race %d: %d ways to win\n", i+1, ways)
		result *= ways
	}

	fmt.Printf("Result: %d\n", result)
}

func parseNumbers(line string) []int {
	parts := strings.Split(line, ":")
	if len(parts) < 2 {
		return nil
	}

	numberStr := strings.TrimSpace(parts[1])
	fields := strings.Fields(numberStr)

	numbers := make([]int, 0, len(fields))
	for _, field := range fields {
		var num int
		_, err := fmt.Sscanf(field, "%d", &num)
		if err == nil { // Only add if there's NO error
			numbers = append(numbers, num)
		}
	}

	return numbers
}

func countWaysToWin(time int, recordDistance int) int {
	winCount := 0
	for holdTime := 0; holdTime <= time; holdTime++ {
		distance := holdTime * (time - holdTime)
		if distance > recordDistance {
			winCount++
		}
	}
	return winCount
}

func visualizeRace(time int, recordDistance int) {
	fmt.Printf("\nRace with time %d ms and record distance %d mm:\n", time, recordDistance)
	fmt.Println("Hold Time | Speed | Travel Time | Distance | Beats Record?")
	fmt.Println("---------+-------+-------------+----------+--------------")

	for holdTime := 0; holdTime <= time; holdTime++ {
		speed := holdTime
		travelTime := time - holdTime
		distance := speed * travelTime
		beatsRecord := distance > recordDistance

		fmt.Printf("%9d | %5d | %11d | %8d | %t\n",
			holdTime, speed, travelTime, distance, beatsRecord)
	}
	fmt.Println()
}
