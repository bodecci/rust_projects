use csv;
use std::{error::Error, path, result};

fn main() {
    if let Err(e) = read_from_file("./customers.csv") {
        eprintln!("{}", e);
    }
}
// In the context of Result<(), Box<dyn Error>>, the () in the Ok(()) 
//case just means "the function succeeded, but there's no value to return."
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}