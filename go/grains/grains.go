package grains

import "fmt"

// Square calculate the square of input number
func Square(input int) (uint64, error) {
	if input < 1 || input > 64 {
		return 0, fmt.Errorf("bad suqre: %d", input)
	}
	return 1 << uint(input-1), nil
}

// Total function calculate the number of grain of wheat on chessboard
func Total() (total uint64) {
	return 1<<64 - 1
}
