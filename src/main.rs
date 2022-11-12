use excel::bathymetry::{self, Bathymetry};
// use physicalquantities::length::{Length, LengthUnits};

// use crate::pipeline::Pipeline;

pub mod excel;
pub mod physicalquantities;
pub mod pipeline;

fn main() {
    let result = dbg!(Bathymetry::read_all());

    match result {
        Ok(b) => {
            let lengths_0 = dbg!(b.sections[0].lengths());
        }
        _ => {}
    }
}
