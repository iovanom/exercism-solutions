package raindrops

import "fmt"

// Convert converting the input nr to rain drops
func Convert(input int) string {
	var msg string
	if input%3 == 0 {
		msg += "Pling"
	}
	if input%5 == 0 {
		msg += "Plang"
	}
	if input%7 == 0 {
		msg += "Plong"
	}
	if msg == "" {
		msg = fmt.Sprintf("%d", input)
	}
	return msg
}
