package robotname

import (
	"fmt"
	"math/rand"
	"time"
)

var exists interface{}

var robots = make(map[string]interface{})

// MaxRobots show the maximum value of robots
const MaxRobots = 26 * 26 * 1000

// Robot representation
type Robot struct {
	name string
}

// Name return the name of the robot
func (r *Robot) Name() (string, error) {
	var err error
	if r.name == "" {
		r.name, err = r.generateName()
	}
	return r.name, err
}

// Reset the name to default value
func (r *Robot) Reset() {
	r.name = ""
}

func (r *Robot) generateName() (string, error) {
	rand.Seed(time.Now().UnixNano())
	generator := func() string {
		return fmt.Sprintf(
			"%c%c%03d",
			'A'+rand.Intn(int('Z'-'A')),
			'A'+rand.Intn(int('Z'-'A')),
			rand.Intn(999),
		)
	}
	if len(robots) < MaxRobots {
		for {
			name := generator()
			_, ok := robots[name]
			if !ok {
				robots[name] = exists
				return name, nil
			}
		}
	} else {
		return "", fmt.Errorf("error to generate new name. All names possible are taken")
	}
}
