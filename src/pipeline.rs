use core::fmt;

use itertools::izip;

use crate::{
    excel::bathymetry::PipeBathymetry,
    physicalquantities::length::{Length, LengthUnits},
};

use self::pipeseg::PipeSeg;

pub mod pipeseg;

#[derive(Debug)]
pub struct Pipeline(Vec<PipeSeg>);

impl Pipeline {
    pub fn new(pb: &PipeBathymetry) -> Pipeline {
        let mut pipes: Vec<PipeSeg> = Vec::new();

        let mut counter = -1;
        for (length, elevation) in izip!(pb.lengths(), pb.elevations()) {
            counter += 1;
            pipes.push(PipeSeg::new(
                &format!("{}-{}", &pb.name, counter),
                Length::new(elevation, LengthUnits::M),
                Length::new(length, LengthUnits::M),
            ))
        }
        Pipeline(pipes)
    }
}

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pipe_strings: Vec<String> = self.0.iter().map(|ps| ps.to_string()).collect();

        write!(f, "{}", pipe_strings.join("\n"))
    }
}
