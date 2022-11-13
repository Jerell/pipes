use calamine::{
    open_workbook, DataType, Error, RangeDeserializer, RangeDeserializerBuilder, Reader, Xlsx,
};

use crate::{
    physicalquantities::length::{Length, LengthUnits},
    pipeline::Pipeline,
};

use super::file_path;

#[derive(Debug)]
pub struct PipeBathymetry {
    pub name: String,
    coords: Vec<(f32, f32)>,
    insulation: String,
}

pub struct Insulation {
    pub name: String,
    pub inside_diameter: Length,
    pub r1: Length,
    pub u_wall: f32,
    pub ax: f32,
    pub ho: i32,
}

impl PipeBathymetry {
    fn new(name: String, coords: Vec<(f32, f32)>, insulation: String) -> PipeBathymetry {
        PipeBathymetry {
            name,
            coords,
            insulation,
        }
    }

    pub fn read_insulation(&self) -> Result<Insulation, Error> {
        let mut workbook: Xlsx<_> =
            open_workbook(file_path(String::from("pipeline insulations.xlsx")))?;

        let range = workbook
            .worksheet_range("Sheet 1")
            .ok_or(Error::Msg("Cannot find sheet"))??;

        let iter: RangeDeserializer<DataType, (String, f32, f32, f32, f32, i32)> =
            RangeDeserializerBuilder::new().from_range(&range)?;

        let relevant_row = iter
            .filter(|r| match r {
                Ok(values) => values.0 == self.insulation,
                Err(_) => false,
            })
            .next();

        if let Some(Ok(v)) = relevant_row {
            Ok(Insulation {
                name: v.0,
                inside_diameter: Length::new(v.1, LengthUnits::M),
                r1: Length::new(v.2, LengthUnits::M),
                u_wall: v.3,
                ax: v.4,
                ho: v.5,
            })
        } else {
            Err(From::from("no insulation values found"))
        }
    }

    pub fn elevations(&self) -> Vec<(f32, f32)> {
        let mut y: Vec<(f32, f32)> = Vec::new();

        let mut coords = self.coords.iter();

        let mut last_y = 0.0;
        if let Some(xy) = coords.next() {
            last_y = xy.1;
        }

        let mut update_last = || {
            let xy_next = coords.next();
            match xy_next {
                Some(xy) => {
                    let start_end_elevation = (last_y, xy.1);
                    last_y = xy.1;
                    y.push(start_end_elevation);
                }
                None => {}
            }
        };

        for _ in self.coords.iter() {
            update_last();
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
                    x_diffs.push(xy.0 - last_x);
                    last_x = xy.0;
                }
                None => {}
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

        let mut insulation = String::new();

        for row in iter {
            match Some(row) {
                Some(values) => {
                    let (x, y, insulation_name): (f32, f32, Option<String>) = values?;
                    coords.push((x, y));
                    let print_xy = || println!("  {x}, {y}");
                    match insulation_name {
                        Some(i) => {
                            println!("{i}");
                            insulation = i;
                            print_xy()
                        }
                        None => print_xy(),
                    }
                }
                _ => {}
            }
        }
        result = Ok(PipeBathymetry::new(
            String::from(sheet_name),
            coords,
            String::from(insulation),
        ));
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
