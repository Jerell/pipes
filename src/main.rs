use physicalquantities::length::{Length, LengthUnits};

use crate::pipeline::Pipeline;

pub mod physicalquantities;
pub mod pipeline;

fn main() {
    let l1 = Length::new(1.0, LengthUnits::Km);
    let l2 = l1;
    let p = Pipeline::new("pipe", l1, l2);

    println!("{}", p);
}
