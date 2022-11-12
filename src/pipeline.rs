use core::fmt;

use crate::physicalquantities::length::Length;

#[derive(Debug)]
pub struct Pipeline {
    name: String,
    elevation: Length,
    length: Length,
}

impl fmt::Display for Pipeline {
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

impl Pipeline {
    pub fn new(name: &str, elevation: Length, length: Length) -> Pipeline {
        Pipeline {
            name: String::from(name),
            elevation,
            length,
        }
    }
}

#[cfg(test)]
mod tests {

    // use crate::physicalquantities::length::LengthUnits;

    // use super::*;

    // #[test]
    // fn height() {
    //     let l0 = Length::new(1.0, LengthUnits::M);

    //     let e0 = Length::new(1.0, LengthUnits::M);
    //     let e1 = Length::new(3.0, LengthUnits::M);

    //     let p0 = Pipeline::new("pipe", l0, l0);
    //     let p1 = Pipeline::new("pipe", e1, l0);
    // }
}
