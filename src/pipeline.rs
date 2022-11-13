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
        let max_length = Length::new(200.0, LengthUnits::M);

        let insulation = match pb.read_insulation() {
            Ok(ins) => ins,
            Err(_) => panic!("insulation properties not specified"),
        };

        let pipes: Vec<PipeSeg> = izip!(pb.lengths(), pb.elevations())
            .enumerate()
            .map(|(i, (length, (start_elevation, end_elevation)))| {
                let section_length = Length::new(length, LengthUnits::M);

                let start_elevation = Length::new(start_elevation, LengthUnits::M);
                let end_elevation = Length::new(end_elevation, LengthUnits::M);
                let section_height = end_elevation - start_elevation;

                let mut sub_lengths = (1..)
                    .map(|i| section_length / i)
                    .filter(|l_vec| l_vec[0] <= max_length);

                match sub_lengths.next() {
                    Some(l_vec) => l_vec
                        .iter()
                        .enumerate()
                        .map(|(j, l)| {
                            let step_height = (section_height / l_vec.len().try_into().unwrap())[0];
                            let elevation = start_elevation + step_height * j.try_into().unwrap();

                            PipeSeg::new(
                                &format!("{}-{}-{}", &pb.name, i, j),
                                elevation,
                                Length::new(l.m(), LengthUnits::M),
                                insulation.inside_diameter,
                                insulation.u_wall,
                            )
                        })
                        .collect::<Vec<_>>(),
                    None => panic!("cannot make a pipe segment short enough"),
                }
            })
            .flatten()
            .collect();

        println!("{}: {} pipesegs", pb.name, pipes.len());
        Pipeline(pipes)
    }
}

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pipe_strings: Vec<String> = self.0.iter().map(|ps| ps.to_string()).collect();

        write!(f, "{}", pipe_strings.join("\n"))
    }
}
