package differenceofsquares

func SquareOfSum(n int) int {
	s := (n * (n + 1) / 2)
	return s * s
}

func SumOfSquares(n int) int {
	s := 0
	for i := 1; i <= n; i++ {
		s += i * i
	}
	return s
}

func Difference(n int) int {
	return SquareOfSum(n) - SumOfSquares(n)
}
