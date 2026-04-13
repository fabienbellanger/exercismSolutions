package parsinglogfiles

import (
	"fmt"
	"regexp"
)

func IsValidLine(text string) bool {
	r := regexp.MustCompile(`^\[(TRC|DBG|INF|WRN|ERR|FTL)\].*`)
	return r.MatchString(text)
}

func SplitLogLine(text string) []string {
	r := regexp.MustCompile(`<[~*=-]*>`)
	return r.Split(text, -1)
}

func CountQuotedPasswords(lines []string) int {
	r := regexp.MustCompile(`(?i)(\".*password.*\")+`)
	count := 0

	for _, line := range lines {
		if r.MatchString(line) {
			count++
		}
	}

	return count
}

func RemoveEndOfLineText(text string) string {
	r := regexp.MustCompile(`(end-of-line\d+)+`)
	if !r.MatchString(text) {
		return text
	}
	return r.ReplaceAllString(text, "")
}

func TagWithUserName(lines []string) []string {
	r := regexp.MustCompile(`(?i)user[ ]+([a-zA-Z0-9]{6,})`)

	result := make([]string, 0, len(lines))
	for _, line := range lines {
		match := r.FindStringSubmatch(line)
		if match != nil {
			line = fmt.Sprintf("[USR] %s %s", match[1], line)
		}
		result = append(result, line)
	}
	return result
}
