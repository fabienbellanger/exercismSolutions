package microblog

func Truncate(phrase string) string {
	r := []rune(phrase)
	if len(r) > 5 {
		return string(r[:5])
	}
	return phrase
}
