package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type CubeSet struct {
	Red   int
	Green int
	Blue  int
}

type Game struct {
	ID     int
	Revals []CubeSet
}

func main() {
	file, err := os.Open("../input/day2.in")
	if err != nil {
		fmt.Println("file open error")
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	limits := CubeSet{
		Red:   12,
		Green: 13,
		Blue:  14,
	}

	sum := 0

	for scanner.Scan() {
		line := scanner.Text()
		game := parseGame(line)

		if isGameValid(game, limits) {
			sum += game.ID
		}
	}

	fmt.Println("[P1] Day 2: Sum of possible game IDs:", sum)
}

func parseGame(line string) Game {
	parts := strings.Split(line, ": ")

	// remove the "Game " prefix
	idPart := strings.TrimPrefix(parts[0], "Game ")
	id, _ := strconv.Atoi(idPart)

	revalParts := strings.Split(parts[1], "; ")
	revals := make([]CubeSet, len(revalParts))

	for i, revalStr := range revalParts {
		revals[i] = parseRevals(revalStr)
	}

	return Game{
		ID:     id,
		Revals: revals,
	}
}

func parseRevals(revalStr string) CubeSet {
	cubes := CubeSet{}

	cubesParts := strings.Split(revalStr, ", ")

	for _, cubesPart := range cubesParts {
		parts := strings.Split(cubesPart, " ")
		count, _ := strconv.Atoi(parts[0])
		color := parts[1]

		switch color {
		case "red":
			cubes.Red = count
		case "green":
			cubes.Green = count
		case "blue":
			cubes.Blue = count
		}
	}

	return cubes
}

func isGameValid(game Game, limit CubeSet) bool {
	for _, reval := range game.Revals {
		if reval.Red > limit.Red ||
			reval.Green > limit.Green ||
			reval.Blue > limit.Blue {
			return false
		}
	}
	return true
}
