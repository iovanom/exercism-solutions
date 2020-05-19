// Package twofer is a implementation of two-fer
package twofer

import "fmt"

// ShareWith is function that receive the name and return the message
func ShareWith(name string) string {
	if name == "" {
		name = "you"
	}
	return fmt.Sprintf("One for %s, one for me.", name)
}
