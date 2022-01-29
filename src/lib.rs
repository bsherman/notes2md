use std::{fs, path::PathBuf};
use std::io::{Error, ErrorKind};

pub fn process_source(source_path: PathBuf) -> Result<(), Error> {
    let attr = fs::metadata(source_path);
    match attr {
        Err(e) => Err(e),
        Ok(_metadata) => Ok(()),
    }
}

fn verify_dest(dest_dir: PathBuf) -> Result<(), Error> {
    let attr = fs::metadata(&dest_dir);
    match attr {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(Error::new(e.kind(),
                                    format!("dest_dir: '{}' not found",
                                               dest_dir.to_str().unwrap()))),
            _ => Err(e),
        }
        Ok(_metadata) => Ok(()),
    }
}
//fn process_apple_notes(source_dir: PathBuf) -> Result<(), ()> {
//    panic!("apple_notes conversion not yet implemented")
//}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn verify_dest_should_fail_when_not_found() {
        let non_existent_path = PathBuf::from("test_data/filename_which_does_not_exist");
        let error = verify_dest(non_existent_path).unwrap_err();
        assert_eq!(ErrorKind::NotFound, error.kind());
    }

    #[test]
    fn verify_dest_should_fail_with_desired_msg_when_not_found() {
        let non_existent_path = PathBuf::from("test_data/filename_which_does_not_exist");
        let error = verify_dest(non_existent_path).unwrap_err();
        assert_eq!("dest_dir: 'test_data/filename_which_does_not_exist' not found", format!("{}", error));
    }

    #[test]
    fn verify_dest_should_fail_when_not_directory() {
        let non_existent_path = PathBuf::from("test_data/not_a_dir.txt");
        let kind = verify_dest(non_existent_path).unwrap_err().kind();
        assert_eq!(ErrorKind::InvalidInput, kind);
    }

    #[test]
    fn process_source_should_fail_with_notfound_when_source_does_not_exist() {
        let non_existent_path = PathBuf::from("test_data/filename_which_does_not_exist");
        let kind = process_source(non_existent_path).unwrap_err().kind();
        assert_eq!(ErrorKind::NotFound, kind);
    }

    // #[test]
    fn process_source_should_fail_with_perm_denied_when_source_is_denied() {
        let non_existent_path = PathBuf::from("/root");
        let kind = process_source(non_existent_path).unwrap_err().kind();
        assert_eq!(ErrorKind::PermissionDenied, kind);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
