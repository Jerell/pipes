use std::{fs::File, io::Write};

use excel::bathymetry::Bathymetry;

pub mod excel;
pub mod physicalquantities;
pub mod pipeline;

fn main() -> std::io::Result<()> {
    if let Ok(b) = Bathymetry::read_all() {
        let mut file = File::create("foo.yml")?;

        let ps = Bathymetry::to_pipelines(&b);
        for p in ps {
            file.write_all(p.to_string().as_bytes())?;
            file.write_all(b"\n")?;
        }
        println!("done");
        Ok(())
    } else {
        panic!("cannot read bathymetry")
    }
}
