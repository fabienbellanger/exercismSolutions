package luhn

import (
	"strconv"
	"strings"
)

func Valid(input string) bool {
	input = strings.ReplaceAll(input, " ", "")
	if len(input) <= 1 {
		return false
	}

	sum := 0
	for i, j := len(input)-1, 0; i >= 0; i-- {
		n, err := strconv.Atoi(string(input[i]))
		if err != nil {
			return false
		}

		if j%2 != 0 {
			d := 2 * n
			if d <= 9 {
				n = d
			} else {
				n = d - 9
			}
		}
		sum += n

		j++
	}

	return sum%10 == 0
}
