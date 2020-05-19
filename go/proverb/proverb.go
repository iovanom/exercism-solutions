// Package proverb implement the function for generate the proverb
package proverb

import "fmt"

const mainTemplate = "For want of a %s the %s was lost."
const endTemplate = "And all for the want of a %s."

// Proverb function generate the proverb
func Proverb(rhyme []string) []string {
	var result = []string{}
	for i := range rhyme {
		if i != 0 {
			result = append(result, fmt.Sprintf(mainTemplate, rhyme[i-1], rhyme[i]))
		}
		if i == len(rhyme)-1 {
			result = append(result, fmt.Sprintf(endTemplate, rhyme[0]))
		}
	}
	return result
}
