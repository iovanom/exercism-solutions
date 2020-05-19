package luhn

import (
	"strings"
	"unicode"
)

// Valid function validate the luhn number
func Valid(number string) bool {
	number = strings.TrimSpace(number)
	if len(number) < 2 {
		return false
	}
	var sum int
	var dCount int
	numbers := []rune(number)
	for i := len(numbers) - 1; i >= 0; i-- {
		var n = numbers[i]
		if unicode.IsSpace(n) {
			continue
		}
		if !unicode.IsDigit(n) {
			return false
		}
		d := int(n) - '0'
		dCount++
		if dCount%2 == 0 {
			d *= d
			if d > 9 {
				d -= 9
			}
		}
		sum += d
	}
	return sum%10 == 0
}
