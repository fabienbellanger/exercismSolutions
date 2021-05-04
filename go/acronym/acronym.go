package acronym

import (
	"strings"
)

// Abbreviate should have a comment documenting it.
func Abbreviate(s string) (result string) {
	words := strings.Fields(strings.ReplaceAll(s, "-", " "))
	for _, word := range words {
		for _, c := range word {
			if c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' {
				result += string(c)
				break
			}
		}
	}

	return strings.ToUpper(result)
}
