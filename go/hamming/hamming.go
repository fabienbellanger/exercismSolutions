package hamming

import "errors"

// Distance returns Hamming distance if strings are corrects, else it returns an error.
func Distance(a, b string) (d int, err error) {
	if len(a) != len(b) {
		err = errors.New("strings with different length")
		return
	}

	if a == "" {
		return
	}

	for i := range a {
		if a[i] != b[i] {
			d++
		}
	}
	return
}
