package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type MappingRange struct {
	DestStart int64
	SrcStart  int64
	Length    int64
}

type Mapping struct {
	Ranges []MappingRange
}

func applyMapping(num int64, mapping Mapping) int64 {
	for _, r := range mapping.Ranges {
		if num >= r.SrcStart && num < r.SrcStart+r.Length {
			offset := num - r.SrcStart
			return r.DestStart + offset
		}
	}
	return num
}

func main() {
	file, err := os.Open("../input/day5.in")
	if err != nil {
		fmt.Println("file open error")
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var seeds []int64
	if scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "seeds:") {
			parts := strings.Fields(line[7:]) // Skip "seeds: " prefix and split by whitespace
			for _, part := range parts {
				seed, err := strconv.ParseInt(part, 10, 64)
				if err != nil {
					fmt.Println("Error parsing seed:", err)
					continue
				}
				seeds = append(seeds, seed)
			}
		}
	}

	fmt.Println("Seeds:", seeds)

	// Parse mappings
	var mappings []Mapping
	var currentMapping Mapping

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			continue
		}

		// If line contains "map:", we're starting a new mapping
		if strings.Contains(line, "map:") {
			if len(currentMapping.Ranges) > 0 {
				mappings = append(mappings, currentMapping)
				currentMapping = Mapping{}
			}
			continue
		}

		parts := strings.Fields(line)
		if len(parts) == 3 {
			destStart, _ := strconv.ParseInt(parts[0], 10, 64)
			srcStart, _ := strconv.ParseInt(parts[1], 10, 64)
			length, _ := strconv.ParseInt(parts[2], 10, 64)

			currentMapping.Ranges = append(currentMapping.Ranges, MappingRange{
				DestStart: destStart,
				SrcStart:  srcStart,
				Length:    length,
			})
		}
	}

	// Add the last mapping if it has ranges
	if len(currentMapping.Ranges) > 0 {
		mappings = append(mappings, currentMapping)
	}

	fmt.Printf("Parsed %d mappings\n", len(mappings))

	var lowestLocation int64 = -1

	for _, seed := range seeds {
		current := seed

		for _, mapping := range mappings {
			current = applyMapping(current, mapping)
		}

		if lowestLocation == -1 || current < lowestLocation {
			lowestLocation = current
		}
	}

	fmt.Println("Lowest location:", lowestLocation)
}
