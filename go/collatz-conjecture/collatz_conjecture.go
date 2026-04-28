package collatzconjecture

import "errors"

func CollatzConjecture(n int) (int, error) {
	if n == 0 {
		return 0, errors.New("n is not a Collatz Conjecture")
	}

	steps := 0
	for n != 1 {
		if n%2 == 0 {
			n /= 2
		} else {
			n = 3*n + 1
		}

		if n <= 0 {
			return 0, errors.New("n is not a Collatz Conjecture")
		}

		steps++
	}

	return steps, nil
}
