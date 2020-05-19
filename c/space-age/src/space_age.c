#include "space_age.h"

static const unsigned long seconds_in_earth_year = 31557600;

static float get_earth_year(planet_t planet) {
    switch (planet) {
        case EARTH: return 1.0;
        case MERCURY: return 0.2408467;
        case VENUS: return 0.61519726;
        case MARS: return 1.8808158;
        case JUPITER: return 11.862615;
        case SATURN: return 29.447498;
        case URANUS: return 84.016846;
        case NEPTUNE: return 164.79132;
        default: return 0.0;
    }
}

float convert_planet_age(planet_t planet, unsigned long seconds) {
    float earth_years = seconds / seconds_in_earth_year;
    return earth_years / get_earth_year(planet);
}
