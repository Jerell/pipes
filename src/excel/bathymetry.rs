use std::vec;

use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};

use super::file_path;

#[derive(Debug)]
struct PipeBathymetry {
    name: String,
    coords: Vec<(f32, f32)>,
}

impl PipeBathymetry {
    fn new(name: String, coords: Vec<(f32, f32)>) -> PipeBathymetry {
        PipeBathymetry {
            name: name,
            coords: coords,
        }
    }
}

#[derive(Debug)]
pub struct Bathymetry {
    sections: Vec<PipeBathymetry>,
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

        let mut result: Result<PipeBathymetry, Error> =
            Ok(PipeBathymetry::new(String::from(sheet_name), Vec::new()));

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
                None => {
                    result = Err(From::from("expected at least one record but got none"));
                }
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
}
