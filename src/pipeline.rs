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
        for (length, elevation) in izip!(pb.lengths(), pb.elevations()) {
            pipes.push(PipeSeg::new(
                &pb.name,
                Length::new(elevation, LengthUnits::M),
                Length::new(length, LengthUnits::M),
            ))
        }
        Pipeline(pipes)
    }
}

// impl fmt::Display for Pipeline {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "\
//     - pipeseg:
//         name: {}
//         key: false
//         length: {}
//         elevation: {}
//         ambient: AMBIENT
//         uValue: UVALUE
//         diameters:
//             - DIAMETER",
//             self.name, self.length, self.elevation
//         )
//     }
// }

// impl Pipeline {
//     pub fn new(name: &str, elevation: Length, length: Length) -> Pipeline {
//         Pipeline {
//             name: String::from(name),
//             elevation,
//             length,
//         }
//     }

//     // pub from_pipe_bathymetry(pb: PipeBathymetry) -> Vec<Pipeline> {

//     // }
// }
