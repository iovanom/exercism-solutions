package diffsquares

// SquareOfSum callculating the squares of sum from 0 to numbers
func SquareOfSum(number int) int {
	var result = (number + 1) * number / 2
	return result * result
}

// SumOfSquares callculating the sum of squalres from 0 to number
func SumOfSquares(number int) int {
	return (number * (number + 1) * (2*number + 1)) / 6
}

// Difference calculate the difference for SumOfSquares and SquareOfSum
func Difference(number int) int {
	return SquareOfSum(number) - SumOfSquares(number)
}
