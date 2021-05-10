package raindrops

import "strconv"

func Convert(input int) (s string) {
	if input%3 == 0 {
		s += "Pling"
	}
	if input%5 == 0 {
		s += "Plang"
	}
	if input%7 == 0 {
		s += "Plong"
	}

	if len(s) == 0 {
		s = strconv.Itoa(input)
	}
	return
}
