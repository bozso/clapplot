use std::error::Error;
use std::result::Result;
use std::path::PathBuf;

use structopt::StructOpt;

use hdf5file::{Hdf5File};

#[derive(Debug, StructOpt)]
struct Main {
    file_path: PathBuf,

    #[structopt(subcommand)]
    op: Op,
}

impl Main {
    fn main(&self) -> Result<(), Box<dyn Error>> {
        let mut file = Hdf5File::open_file(&self.file_path)?;

        match &self.op {
            Op::Get { object_path } => {
                let object = file.get_object(object_path)?;
                println!("{:?}", object);
            }
            Op::Ls => {
                for object_path in file.object_paths()? {
                    println!("{:?}", object_path?.to_str());
                }
            }
        }
        return Ok(());
    }
}

#[derive(Debug, StructOpt)]
enum Op {
    Get { object_path: PathBuf },
    Ls,
}

fn main() -> Result<(), Box<dyn Error>> {
    return Main::from_args().main();
}
