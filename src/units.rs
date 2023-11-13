/// [Solar radius][https://www.wikipedia.com/en/Solar_radius]
#[derive(Debug, Default)]
pub struct SolarRadius(pub f32);

impl SolarRadius {
    const METERS: f64 = 695_700_000_f64;

    pub fn from_meters(value: f64) -> Self {
        Self((value / Self::METERS) as f32)
    }

    pub fn in_meters(&self) -> f64 {
        self.0 as f64 * Self::METERS
    }
}

/// [Astronomical unit][https://www.wikipedia.com/en/Astronomical_unit]
#[derive(Debug, Default)]
pub struct AstronomicalUnit(pub f32);

impl AstronomicalUnit {
    const METERS: f64 = 149_597_870_700_000_000_000_f64;

    pub fn from_meters(value: f64) -> Self {
        Self((value / Self::METERS) as f32)
    }

    pub fn in_meters(&self) -> f64 {
        self.0 as f64 * Self::METERS
    }
}

/// [Light-year][https://www.wikipedia.com/en/Light-year]
#[derive(Debug, Default)]
pub struct LightYear(pub f32);

impl LightYear {
    const METERS: f64 = 9_460_730_472_580_800_f64;

    pub fn from_meters(value: f64) -> Self {
        Self((value / Self::METERS) as f32)
    }

    pub fn in_meters(&self) -> f64 {
        self.0 as f64 * Self::METERS
    }
}

/// [Parsec][https://www.wikipedia.com/en/Parsec]
#[derive(Debug, Default)]
pub struct Parsec(pub f32);

impl Parsec {
    const METERS: f64 = 30_856_775_814_913_673_f64;

    pub fn from_meters(value: f64) -> Self {
        Self((value / Self::METERS) as f32)
    }

    pub fn in_meters(&self) -> f64 {
        self.0 as f64 * Self::METERS
    }
}