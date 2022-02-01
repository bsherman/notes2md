use std::io::Error;
use std::path::PathBuf;

pub fn process(_source_file: PathBuf, _dest_dir: PathBuf) -> Result<(), Error> {
    println!("Simplenote conversion not yet implemented.");
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_dest_should_fail_when_not_found() {
        let non_existent_path = PathBuf::from("test_data/filename_which_does_not_exist");
        let error = verify_dest(&non_existent_path).unwrap_err();
    }