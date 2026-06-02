package isbnverifier

import "strings"

func IsValidISBN(isbn string) bool {
	var s strings.Builder
	for _, c := range isbn {
		if strings.ContainsRune("0123456789X", c) {
			s.WriteString(string(c))
		} else if c != '-' {
			return false
		}
	}

	if len(s.String()) != 10 {
		return false
	}

	if strings.Contains(s.String(), "X") && !strings.HasSuffix(s.String(), "X") {
		return false
	}

	sum := 0
	for i, c := range s.String() {
		if c == 'X' {
			sum += 10 * (10 - i)
		} else {
			sum += int(c-'0') * (10 - i)
		}
	}

	return sum%11 == 0
}
