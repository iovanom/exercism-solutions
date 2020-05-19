package etl

import "strings"

// Transform function transfroming the score map
func Transform(scores map[int][]string) map[string]int {
	var result = map[string]int{}
	for k, v := range scores {
		for _, r := range v {
			result[strings.ToLower(r)] = k
		}
	}
	return result
}
