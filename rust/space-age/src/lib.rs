// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_SECONDS: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    const PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        let Duration(seconds) = d;
        ((*seconds as f64 / EARTH_YEAR_SECONDS / Self::PERIOD) * 100.0).round() / 100.0
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const PERIOD: f64 = 0.2408467;
}
impl Planet for Venus {
    const PERIOD: f64 = 0.61519726;
}
impl Planet for Earth {
    const PERIOD: f64 = 1.0;
}
impl Planet for Mars {
    const PERIOD: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const PERIOD: f64 = 11.862615;
}
impl Planet for Saturn {
    const PERIOD: f64 = 29.447498;
}
impl Planet for Uranus {
    const PERIOD: f64 = 84.016846;
}
impl Planet for Neptune {
    const PERIOD: f64 = 164.79132;
}
