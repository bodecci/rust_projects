use csv;
use std::{error::Error, path};

fn main() {
    if let Err(e) = read_from_file("./customers.csv") {
        eprintln!("{}", e);
    }
}
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    
}