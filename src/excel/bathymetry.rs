use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};

use super::file_path;

pub struct Bathymetry {}

impl Bathymetry {
    fn read_sheet(sheet_name: &str) -> Result<(), Error> {
        let mut workbook: Xlsx<_> =
            open_workbook(file_path(String::from("pipeline bathymetry.xlsx")))?;

        let range = workbook
            .worksheet_range(sheet_name)
            .ok_or(Error::Msg("Cannot find sheet"))??;

        let iter = RangeDeserializerBuilder::new().from_range(&range)?;

        let mut result: Result<(), Error> = Ok(());

        for row in iter {
            match Some(row) {
                Some(values) => {
                    let (x, y, insulation): (String, f64, Option<String>) = values?;

                    let print_xy = || println!("  {x}, {y}");

                    match insulation {
                        Some(i) => {
                            println!("{i}");
                            print_xy()
                        }
                        None => print_xy(),
                    }

                    result = Ok(());
                }
                None => {
                    result = Err(From::from("expected at least one record but got none"));
                }
            }
        }
        result
    }

    pub fn read_all() -> Result<(), Error> {
        let workbook: Xlsx<_> = open_workbook(file_path(String::from("pipeline bathymetry.xlsx")))?;

        let sheets = workbook.sheet_names().iter();

        let mut result: Result<(), Error> = Ok(());

        for sheet in sheets {
            println!("{sheet}");

            result = Bathymetry::read_sheet(sheet);

            if let Err(error) = result {
                panic!("{}", error)
            }
        }

        result
    }
}
