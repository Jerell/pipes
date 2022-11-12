use excel::bathymetry::Bathymetry;
// use physicalquantities::length::{Length, LengthUnits};

// use crate::pipeline::Pipeline;

pub mod excel;
pub mod physicalquantities;
pub mod pipeline;

fn main() {
    let _result = dbg!(Bathymetry::read_all());
}
