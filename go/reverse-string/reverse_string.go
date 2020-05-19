package reverse

// String reversing the input string
func String(input string) string {
	var charLen = len([]rune(input))
	if input == "" || charLen == 1 {
		return input
	}
	var revers = make([]rune, charLen)
	for index, r := range []rune(input) {
		revers[charLen-index-1] = r
	}
	return string(revers)
}
