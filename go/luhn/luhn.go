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
	j := 0
	for i := len(input) - 1; i >= 0; i-- {
		n, err := strconv.Atoi(string(input[i]))
		if err != nil {
			return false
		}

		if j%2 != 0 {
			d := 2 * n
			if d <= 9 {
				sum += d
			} else {
				sum += d - 9
			}
		} else {
			sum += n
		}
		j++
	}

	return sum%10 == 0
}
