package letter

// FreqMap records the frequency of each rune in a given text.
type FreqMap map[rune]int

// Frequency counts the frequency of each rune in a given text and returns this
// data as a FreqMap.
func Frequency(s string) FreqMap {
	m := FreqMap{}
	for _, r := range s {
		m[r]++
	}
	return m
}

// ConcurrentFrequency if a function for calculate the frequency count
// concurrently
func ConcurrentFrequency(s []string) FreqMap {
	result := FreqMap{}
	resultChan := make(chan FreqMap)
	for _, text := range s {
		go func(text string) {
			resultChan <- Frequency(text)
		}(text)
	}
	for range s {
		resMap := <-resultChan
		for key, value := range resMap {
			result[key] += value
		}
	}
	return result
}
