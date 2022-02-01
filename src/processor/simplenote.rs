use std::io::Error;
use std::path::PathBuf;

pub fn process(_source_file: PathBuf, _dest_dir: PathBuf) -> Result<(), Error> {
    println!("Simplenote conversion not yet implemented.");
    Ok(())
}
