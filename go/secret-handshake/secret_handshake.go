package secret

// Handshake is a function that decript the handshake
func Handshake(code uint) (result []string) {
	if code&1 != 0 {
		result = append(result, "wink")
	}
	if code&2 != 0 {
		result = append(result, "double blink")
	}
	if code&4 != 0 {
		result = append(result, "close your eyes")
	}
	if code&8 != 0 {
		result = append(result, "jump")
	}
	if code&16 != 0 {
		reverse(&result)
	}
	return result
}

func reverse(container *[]string) {
	s := *container
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
}
