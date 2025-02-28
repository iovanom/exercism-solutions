package pangram

import "strings"

func IsPangram(input string) bool {
	c := make([]bool, 'z'-'a'+1)
	for _, r := range strings.ToLower(input) {
		if r >= 'a' && r <= 'z' {
			c['z'-r] = true
		}
	}
	for _, v := range c {
		if !v {
			return false
		}
	}
	return true
}
