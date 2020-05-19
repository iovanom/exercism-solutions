// Package leap have the function to calculate the leap year
package leap

// IsLeapYear return if the year is leap
func IsLeapYear(year int) bool {
	return (year%4 == 0) && (year%100 != 0 || year%400 == 0)
}
