package scrabble

import (
	"strings"
)

var points map[int][]rune = map[int][]rune{
	1:  {'A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'},
	2:  {'D', 'G'},
	3:  {'B', 'C', 'M', 'P'},
	4:  {'F', 'H', 'V', 'W', 'Y'},
	5:  {'K'},
	8:  {'J', 'X'},
	10: {'Q', 'Z'},
}

func Score(input string) (score int) {
	s := strings.ToUpper(input)
	for _, c := range s {
		if c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U' || c == 'L' || c == 'N' || c == 'R' || c == 'S' || c == 'T' {
			score += 1
		} else if c == 'D' || c == 'G' {
			score += 2
		} else if c == 'B' || c == 'C' || c == 'M' || c == 'P' {
			score += 3
		} else if c == 'F' || c == 'H' || c == 'V' || c == 'W' || c == 'Y' {
			score += 4
		} else if c == 'K' {
			score += 5
		} else if c == 'J' || c == 'X' {
			score += 8
		} else if c == 'Q' || c == 'Z' {
			score += 10
		}
	}
	return
}
