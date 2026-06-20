package armstrongnumbers

import (
	"strconv"
)

func IsNumber(n int) bool {
	s := strconv.Itoa(n)
	l := len(s)

	sum := 0
	for _, c := range s {
		d, err := strconv.Atoi(string(c))
		if err != nil {
			return false
		}
		sum += pow(d, l)
	}

	return sum == n
}

func pow(base, exp int) int {
	result := 1
	for range exp {
		result *= base
	}
	return result
}
