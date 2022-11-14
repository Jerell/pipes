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
    pub ambient: f32,
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
        let iter: RangeDeserializer<DataType, (String, f32, f32, f32, f32, i32, f32)> =
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
                ambient: v.6,
            })
        } else {
            Err(From::from("no insulation values found"))
        }
    }

    pub fn elevations(&self) -> Vec<(f32, f32)> {
        let mut coords = self.coords.iter();
        let mut ys: Vec<(f32, f32)> = Vec::new();
        let mut last_y = if let Some((_, y)) = coords.next() {
            *y
        } else {
            0.0
        };

        for (_x, y) in coords {
            let start_end_elevation = (last_y, *y);
            last_y = *y;
            ys.push(start_end_elevation);
        }

        ys
    }

    pub fn lengths(&self) -> Vec<f32> {
        let mut coords = self.coords.iter();
        let mut last_x = if let Some((x, _)) = coords.next() {
            *x
        } else {
            0.0
        };
        let mut x_diffs: Vec<f32> = Vec::new();

        for (x, _y) in coords {
            x_diffs.push(x - last_x);
            last_x = *x;
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
        let iter: RangeDeserializer<DataType, (f32, f32, Option<String>)> =
            RangeDeserializerBuilder::new().from_range(&range)?;

        let mut coords: Vec<(f32, f32)> = Vec::new();
        let mut insulation = String::new();
        let result: Result<PipeBathymetry, Error>;

        for row in iter {
            if let Ok((x, y, insulation_name)) = row {
                if let Some(i) = insulation_name {
                    println!("{i}");
                    insulation = i;
                }
                coords.push((x, y));
                println!("  {x}, {y}");
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
