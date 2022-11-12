use excel::bathymetry::Bathymetry;

pub mod excel;
pub mod physicalquantities;
pub mod pipeline;

fn main() {
    let result = dbg!(Bathymetry::read_all());

    match result {
        Ok(b) => {
            let _p = dbg!(Bathymetry::to_pipelines(&b));
        }
        _ => {}
    }
}
