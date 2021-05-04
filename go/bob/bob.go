package bob

import (
	"regexp"
	"strings"
)

// Hey should have a comment documenting it.
func Hey(remark string) string {
	re := regexp.MustCompile(`[\t\n\r ]+`)
	remark = re.ReplaceAllString(remark, "")

	re = regexp.MustCompile(`[a-zA-Z]+`)

	if strings.HasSuffix(remark, "?") {
		if remark == strings.ToUpper(remark) && re.MatchString(remark) {
			return "Calm down, I know what I'm doing!"
		}
		return "Sure."
	} else if remark == strings.ToUpper(remark) && re.MatchString(remark) {
		return "Whoa, chill out!"
	} else if remark == "" {
		return "Fine. Be that way!"
	} else {
		return "Whatever."
	}
}
