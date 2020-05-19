package accumulate

// Accumulate is a function for execute some converter function on all slice items
func Accumulate(items []string, converter func(string) string) []string {
	var result = []string{}
	for _, item := range items {
		result = append(result, converter(item))
	}
	return result
}
