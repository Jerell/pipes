use excel::bathymetry::Bathymetry;

pub mod excel;
pub mod physicalquantities;
pub mod pipeline;

fn main() {
    let result = Bathymetry::read_all();

    match result {
        Ok(b) => {
            let ps = Bathymetry::to_pipelines(&b);
            for p in ps {
                println!("{p}")
            }
        }
        _ => {}
    }
}
