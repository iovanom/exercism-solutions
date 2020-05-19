// Package triangle implemented a function for determine what kind of triangle
// we have, equilateral, isoscele or scalene
package triangle

import "math"

// Kind is type for kind of triangle
type Kind int

const (
	// NaT is for not a triangle
	NaT Kind = iota // not a triangle
	// Equ is for equilateral
	Equ
	// Iso is for isosceles
	Iso
	// Sca is for scalene
	Sca
)

// KindFromSides return the kind of the triangle
func KindFromSides(a, b, c float64) Kind {
	switch {
	case a+b < c || a+c < b || c+a < b || b+c < a:
		return NaT
	case a == 0 || b == 0 || c == 0:
		return NaT
	case math.IsNaN(a+b+c) || math.IsInf(a+b+c, 1) || math.IsInf(a+b+c, -1):
		return NaT
	case a == b && a == c:
		return Equ
	case a == b || a == c || b == c:
		return Iso
	default:
		return Sca
	}
}
