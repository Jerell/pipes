use core::fmt;

use crate::physicalquantities::length::Length;

#[derive(Debug)]
pub struct PipeSeg {
    name: String,
    elevation: Length,
    length: Length,
    diameter: Length,
    u_wall: f32,
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
      uValue: {}
      diameters:
          - {}",
            self.name, self.length, self.elevation, self.u_wall, self.diameter
        )
    }
}

impl PipeSeg {
    pub fn new(
        name: &str,
        elevation: Length,
        length: Length,
        diameter: Length,
        u_wall: f32,
    ) -> PipeSeg {
        PipeSeg {
            name: String::from(name),
            elevation,
            length,
            diameter,
            u_wall,
        }
    }
}
