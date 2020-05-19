package space

// Planet is the planet type extended from string
type Planet string

// EarthYearSeconds represent the how many seconds are in one Earth year
const EarthYearSeconds int64 = 31557600

// orbitalPeriods represent the orbital period or platents in Earth years
var orbitalPeriods = map[Planet]float64{
	"Mercury": 0.2408467,
	"Venus":   0.61519726,
	"Earth":   1,
	"Mars":    1.8808158,
	"Jupiter": 11.862615,
	"Saturn":  29.447498,
	"Uranus":  84.016846,
	"Neptune": 164.79132,
}

// Age function calculate the age for diferent planets
func Age(seconds float64, planet Planet) float64 {
	planetOrbitalPeriod, ok := orbitalPeriods[planet]
	if !ok {
		return 0.0
	}
	return seconds / float64(EarthYearSeconds) / planetOrbitalPeriod
}
