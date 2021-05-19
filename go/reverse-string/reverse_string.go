package reverse

func Reverse(s string) (r string) {
	runes := []rune(s)
	n := len(runes)
	for i := range runes {
		r += string(runes[n-i-1])
	}
	return
}
