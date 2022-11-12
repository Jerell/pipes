use core::{fmt, panic};

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

            let max_length = Length::new(200.0, LengthUnits::M);
            let section_length = Length::new(length, LengthUnits::M);

            let mut sub_lengths = (1..)
                .map(|i| section_length / i)
                .filter(|l_vec| l_vec[0] <= max_length);

            let short_enough = sub_lengths.next();
            match short_enough {
                Some(l_vec) => {
                    for l in l_vec {
                        pipes.push(PipeSeg::new(
                            &format!("{}-{}", &pb.name, counter),
                            Length::new(elevation, LengthUnits::M),
                            l,
                        ))
                    }
                }
                None => panic!("cannot make a pipe segment short enough"),
            }
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
