package robotname

import (
	"fmt"
	"math/rand"
	"time"
)

var exists interface{}

var robots = make(map[string]interface{})

const MaxRobots = 26 * 26 * 1000

type Robot struct {
	name string
}

func (r *Robot) Name() (string, error) {
	var err error
	if r.name == "" {
		r.name, err = r.generateName()
	}
	return r.name, err
}

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
		return "", fmt.Errorf("Error to generate new name. All names possible are taken.")
	}
}
