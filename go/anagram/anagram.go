package anagram

import (
	"slices"
	"strings"
)

func Detect(subject string, candidates []string) []string {
	lowerSubject := strings.ToLower(subject)
	sorted := sortRunes(lowerSubject)
	result := make([]string, 0, len(candidates))

	for _, word := range candidates {
		lower := strings.ToLower(word)
		if lower != lowerSubject && sortRunes(lower) == sorted {
			result = append(result, word)
		}
	}

	return result
}

func sortRunes(s string) string {
	r := []rune(s)
	slices.Sort(r)
	return string(r)
}
