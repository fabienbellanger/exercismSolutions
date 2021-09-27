const SECS_IN_EARTH_YEAR: f64 = 31_557_600.0; // 3600 * 24 * 365.25

#[derive(Debug)]
pub struct Duration {
    pub secs: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { secs: s }
    }
}

pub trait Planet {
    fn orbital() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.secs as f64 / SECS_IN_EARTH_YEAR / Self::orbital()
    }
}

macro_rules! planet {
    ($planet:ident, $orbital:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            fn orbital() -> f64 {
                $orbital
            }
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
