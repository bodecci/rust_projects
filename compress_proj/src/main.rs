extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;


fn main() {
    // the program expects 3 args. if not, show user what args is needed
    if args().len() !=3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
}