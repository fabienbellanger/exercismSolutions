package grains

import (
	"errors"
	"math"
)

func Square(input int) (uint64, error) {
	if input <= 0 || input > 64 {
		return 0, errors.New("Square must be between 1 and 64")
	}
	return uint64(math.Pow(2.0, float64(input-1))), nil
}

func Total() (total uint64) {
	for i := 1; i <= 64; i++ {
		if s, err := Square(i); err == nil {
			total += s
		}
	}
	return
}
