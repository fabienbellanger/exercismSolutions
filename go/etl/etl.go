package etl

import "strings"

func Transform(in map[int][]string) map[string]int {
	result := make(map[string]int)

	for score, letters := range in {
		for _, letter := range letters {
			letter = strings.ToLower(letter)
			result[letter] = score
		}
	}

	return result
}
