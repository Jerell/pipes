use std::{fs::File, io::Write};

use excel::bathymetry::Bathymetry;

pub mod excel;
pub mod physicalquantities;
pub mod pipeline;

fn main() -> std::io::Result<()> {
    let result = Bathymetry::read_all();

    match result {
        Ok(b) => {
            let mut file = File::create("foo.yml")?;

            let ps = Bathymetry::to_pipelines(&b);
            for p in ps {
                file.write_all(p.to_string().as_bytes())?;
                file.write_all(b"\n")?;
            }
            println!("done");
            Ok(())
        }
        _ => panic!("cannot read bathymetry"),
    }
}
