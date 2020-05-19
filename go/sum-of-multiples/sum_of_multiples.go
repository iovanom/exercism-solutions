package summultiples

// SumMultiples function calculate the sum of multiples
func SumMultiples(limit int, divisors ...int) (sum int) {
	multiplesMap := map[int]bool{}
	for _, d := range divisors {
		if d == 0 || d > limit {
			continue
		}
		for i := 1; i <= (limit-1)/d; i++ {
			if !multiplesMap[i*d] {
				sum += i * d
				multiplesMap[i*d] = true
			}
		}
	}
	return sum
}
