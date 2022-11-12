use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};

use crate::pipeline::Pipeline;

use super::file_path;

#[derive(Debug)]
pub struct PipeBathymetry {
    pub name: String,
    coords: Vec<(f32, f32)>,
}

impl PipeBathymetry {
    fn new(name: String, coords: Vec<(f32, f32)>) -> PipeBathymetry {
        PipeBathymetry {
            name: name,
            coords: coords,
        }
    }

    pub fn elevations(&self) -> Vec<f32> {
        let mut y: Vec<f32> = Vec::new();
        for xy in self.coords.iter() {
            y.push(xy.1);
        }
        y
    }

    pub fn lengths(&self) -> Vec<f32> {
        let mut coords = self.coords.iter();

        let mut last_x = 0.0;
        let mut x_diffs: Vec<f32> = Vec::new();

        let mut update_last = || {
            let xy_next = coords.next();
            match xy_next {
                Some(xy) => {
                    println!("{}", xy.0);
                    x_diffs.push(xy.0 - last_x);
                    last_x = xy.0;
                }
                None => {
                    println!("end")
                }
            }
        };

        for _ in self.coords.iter() {
            update_last();
        }
        x_diffs
    }
}

#[derive(Debug)]
pub struct Bathymetry {
    pub sections: Vec<PipeBathymetry>,
}

impl Bathymetry {
    fn read_sheet(sheet_name: &str) -> Result<PipeBathymetry, Error> {
        let mut workbook: Xlsx<_> =
            open_workbook(file_path(String::from("pipeline bathymetry.xlsx")))?;

        let range = workbook
            .worksheet_range(sheet_name)
            .ok_or(Error::Msg("Cannot find sheet"))??;

        let iter = RangeDeserializerBuilder::new().from_range(&range)?;

        let mut coords: Vec<(f32, f32)> = Vec::new();

        let result: Result<PipeBathymetry, Error>;

        for row in iter {
            match Some(row) {
                Some(values) => {
                    let (x, y, insulation): (f32, f32, Option<String>) = values?;
                    coords.push((x, y));
                    let print_xy = || println!("  {x}, {y}");
                    match insulation {
                        Some(i) => {
                            println!("{i}");
                            print_xy()
                        }
                        None => print_xy(),
                    }
                }
                _ => {}
            }
        }
        result = Ok(PipeBathymetry::new(String::from(sheet_name), coords));
        result
    }

    pub fn read_all() -> Result<Bathymetry, Error> {
        let workbook: Xlsx<_> = open_workbook(file_path(String::from("pipeline bathymetry.xlsx")))?;

        let sheets = workbook.sheet_names().iter();

        let mut sections: Vec<PipeBathymetry> = Vec::new();

        for sheet in sheets {
            println!("{sheet}");

            match Bathymetry::read_sheet(sheet) {
                Ok(section) => sections.push(section),
                Err(error) => panic!("{}", error),
            }
        }

        let result: Result<Bathymetry, Error> = Ok(Bathymetry { sections });
        result
    }

    pub fn to_pipelines(b: &Bathymetry) -> Vec<Pipeline> {
        let mut sections: Vec<Pipeline> = Vec::new();
        for section in b.sections.iter() {
            sections.push(Pipeline::new(section));
        }
        sections
    }
}
