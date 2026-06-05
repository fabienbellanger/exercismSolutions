package phonenumber

import (
	"fmt"
)

func Number(phoneNumber string) (string, error) {
	buf := make([]byte, 0, len(phoneNumber))
	for i := 0; i < len(phoneNumber); i++ {
		if c := phoneNumber[i]; c >= '0' && c <= '9' {
			buf = append(buf, c)
		}
	}

	s := string(buf)
	if len(s) == 11 && s[0] == '1' {
		s = s[1:]
	}
	if len(s) != 10 {
		return "", fmt.Errorf("invalid phone number length")
	}
	if s[0] < '2' || s[3] < '2' {
		return "", fmt.Errorf("invalid phone number")
	}

	return s, nil
}

func AreaCode(phoneNumber string) (string, error) {
	s, err := Number(phoneNumber)
	if err != nil {
		return "", err
	}

	return s[0:3], nil
}

func Format(phoneNumber string) (string, error) {
	s, err := Number(phoneNumber)
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("(%s) %s-%s", s[0:3], s[3:6], s[6:]), nil
}
