// Package weather has logic for weather forecast.
package weather

// CurrentCondition represent the weather condition.
var CurrentCondition string

// CurrentLocation represent the location.
var CurrentLocation string

// Forecast return the weather for provided city.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
