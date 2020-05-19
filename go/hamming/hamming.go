package hamming

import "errors"

// Distance function calculate the DNA distance
func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, errors.New("the lengths are different")
	}
	var distance = 0
	for i := range a {
		if a[i] != b[i] {
			distance++
		}
	}
	return distance, nil
}
