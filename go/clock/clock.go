package clock

import "fmt"

// Clock is the main type of clock package
type Clock int

const (
	minInHr  = 60
	minInDay = 60 * 24
)

// New function create the new clock instance
func New(hour, minute int) Clock {
	minute += hour * minInHr
	clock := Clock(0).Add(minute)
	return clock
}

// Add method add minutes to the clock
func (clock Clock) Add(minute int) Clock {
	clock = Clock((int(clock) + minute) % minInDay)
	if clock < 0 {
		clock += minInDay
	}
	return clock
}

// Subtract method substract minutes from clock
func (clock Clock) Subtract(minute int) Clock {
	return clock.Add(-minute)
}

func (clock Clock) String() string {
	return fmt.Sprintf("%02d:%02d", clock/minInHr, clock%minInHr)
}
