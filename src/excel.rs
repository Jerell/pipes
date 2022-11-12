pub mod bathymetry;

fn file_path(filename: String) -> String {
    format!("{}/input/{}", env!("CARGO_MANIFEST_DIR"), filename)
}

#[cfg(test)]
mod tests {
    #[test]
    fn read() {}
}
