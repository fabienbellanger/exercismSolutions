package space

import (
	"errors"
)

// Planet represents the planet name.
type Planet string

const SECS_IN_EARTH_YEAR = 31557600.0 // 3600 * 24 * 365.25

func (p *Planet) orbital() (y float64, err error) {
	switch *p {
	case "Earth":
		y = 1.0
	case "Mercury":
		y = 0.2408467
	case "Venus":
		y = 0.61519726
	case "Mars":
		y = 1.8808158
	case "Jupiter":
		y = 11.862615
	case "Saturn":
		y = 29.447498
	case "Uranus":
		y = 84.016846
	case "Neptune":
		y = 164.79132
	default:
		err = errors.New("invalid planet")
	}

	return
}

// Age calculates age for planet form sec.
func Age(sec float64, planet Planet) (age float64) {
	orbibal, err := planet.orbital()
	if err != nil {
		return -1
	}
	return sec / SECS_IN_EARTH_YEAR / orbibal
}
