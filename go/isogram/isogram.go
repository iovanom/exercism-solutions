package isogram

import "unicode"

// IsIsogram function check if the word is a isogram or not
func IsIsogram(word string) bool {
	var wordMap = map[rune]bool{}
	for _, c := range word {
		c = unicode.ToUpper(c)
		if !unicode.IsLetter(c) {
			continue
		}
		if wordMap[c] {
			return false
		}
		wordMap[c] = true
	}
	return true
}
