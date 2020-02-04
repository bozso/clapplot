use std::error::Error;
use std::result::Result;

use hdf5file::{Hdf5File};

fn main() -> Result<(), Box<dyn Error>> {
    let f = Hdf5File::open_file("/home/istvan/test.hdf5")?;
    
    println!("Hello, world!");
    
    Ok(())
}
