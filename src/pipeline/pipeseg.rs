use core::fmt;

use crate::physicalquantities::length::Length;

#[derive(Debug)]
pub struct PipeSeg {
    name: String,
    elevation: Length,
    length: Length,
}

impl fmt::Display for PipeSeg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
  - pipeseg:
      name: {}
      key: false
      length: {}
      elevation: {}
      ambient: AMBIENT
      uValue: UVALUE
      diameters:
          - DIAMETER",
            self.name, self.length, self.elevation
        )
    }
}

impl PipeSeg {
    pub fn new(name: &str, elevation: Length, length: Length) -> PipeSeg {
        PipeSeg {
            name: String::from(name),
            elevation,
            length,
        }
    }
}
