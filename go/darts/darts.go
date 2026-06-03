package darts

func Score(x, y float64) int {
	z := x*x + y*y

	if z <= 1 {
		return 10
	} else if z <= 25 {
		return 5
	} else if z <= 100 {
		return 1
	} else {
		return 0
	}
}
