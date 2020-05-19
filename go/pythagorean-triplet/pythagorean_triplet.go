package pythagorean

type Triplet [3]int

// Range is the function that return and slice of Triplet in privided range
func Range(min, max int) []Triplet {
	return make([]Triplet, 0)
}

// Sum calculate the triplet from provided sum
func Sum(sum int) Triplet {
	return Triplet{}
}
