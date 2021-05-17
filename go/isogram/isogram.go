package isogram

import "strings"

func IsIsogram(input string) bool {
	input = strings.ReplaceAll(input, " ", "")
	input = strings.ReplaceAll(input, "-", "")
	input = strings.ToLower(input)

	letters := make(map[rune]struct{}, len(input))

	for _, r := range input {
		if _, ok := letters[r]; ok {
			return false
		} else {
			letters[r] = struct{}{}
		}
	}

	return true
}
