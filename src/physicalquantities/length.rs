use core::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Length {
    m: f32,
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} m", self.m)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LengthUnits {
    M,
    Km,
    Mm,
    Inch,
}

impl PartialEq for Length {
    fn eq(&self, other: &Self) -> bool {
        self.m == other.m
    }
}

impl Add<Length> for Length {
    type Output = Length;
    fn add(self, other: Length) -> Length {
        Length::new(self.m + other.m, LengthUnits::M)
    }
}
impl Sub<Length> for Length {
    type Output = Length;
    fn sub(self, other: Length) -> Length {
        Length::new(self.m - other.m, LengthUnits::M)
    }
}

impl Length {
    pub fn new(n: f32, unit: LengthUnits) -> Length {
        match unit {
            LengthUnits::M => Length { m: n },
            LengthUnits::Km => Length { m: n * 1000.0 },
            LengthUnits::Mm => Length { m: n / 1000.0 },
            LengthUnits::Inch => Length { m: n * 0.0254 },
        }
    }

    pub fn pythag(a: Length, b: Length) -> Length {
        Length {
            m: (f32::powi(a.m(), 2) + f32::powi(b.m(), 2)).sqrt(),
        }
    }

    pub fn m(&self) -> f32 {
        self.m
    }

    pub fn km(&self) -> f32 {
        self.m / 1000.0
    }

    pub fn mm(&self) -> f32 {
        self.m * 1000.0
    }

    pub fn inch(&self) -> f32 {
        self.m / 0.0254
    }
}
