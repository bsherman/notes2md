use std::io::{Error, ErrorKind};
use std::{fs, path::PathBuf};
use tempfile::tempfile_in;

pub mod processor;
use processor::applenotes;
use processor::simplenote;

#[derive(PartialEq)]
enum SourceType {
    File,
    Directory,
}

pub fn process_applenotes(source_dir: PathBuf, dest_dir: PathBuf) -> Result<(), Error> {
    let dv = verify_dest(&dest_dir);
    if dv.is_err() {
        dv
    } else {
        let sv = verify_source(&source_dir, SourceType::Directory);
        if sv.is_err() {
            sv
        } else {
            applenotes::process(source_dir, dest_dir)
        }
    }
}

pub fn process_simplenote(source_file: PathBuf, dest_dir: PathBuf) -> Result<(), Error> {
    let dv = verify_dest(&dest_dir);
    if dv.is_err() {
        dv
    } else {
        let sv = verify_source(&source_file, SourceType::File);
        if sv.is_err() {
            sv
        } else {
            simplenote::process(source_file, dest_dir)
        }
    }
}

fn verify_dest(dest_dir: &PathBuf) -> Result<(), Error> {
    let attr = fs::metadata(&dest_dir);
    match attr {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(Error::new(
                e.kind(),
                format!("dest_dir: '{}' not found", dest_dir.to_string_lossy()),
            )),
            _ => Err(e),
        },
        Ok(metadata) => match metadata.is_dir() {
            true => match tempfile_in(&dest_dir) {
                Err(e) => match e.kind() {
                    ErrorKind::PermissionDenied => Err(Error::new(
                        e.kind(),
                        format!("dest_dir: '{}' not writable", dest_dir.to_string_lossy()),
                    )),
                    _ => Err(e),
                },
                Ok(_) => Ok(()),
            },
            false => Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "dest_dir: '{}' must be a directory",
                    dest_dir.to_string_lossy()
                ),
            )),
        },
    }
}

fn verify_source(source_path: &PathBuf, source_type: SourceType) -> Result<(), Error> {
    let attr = fs::metadata(&source_path);
    match attr {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(Error::new(
                e.kind(),
                format!("source_path: '{}' not found", source_path.to_string_lossy()),
            )),
            _ => Err(e),
        },
        Ok(metadata) => {
            if metadata.is_dir() {
                if SourceType::Directory == source_type {
                    // read the directory to ensure it is permitted
                    match fs::read_dir(&source_path) {
                        Err(e) => match e.kind() {
                            ErrorKind::PermissionDenied => Err(Error::new(
                                e.kind(),
                                format!(
                                    "source_path: '{}' directory access denied",
                                    source_path.to_string_lossy()
                                ),
                            )),
                            _ => Err(e),
                        },
                        Ok(_) => Ok(()),
                    }
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "source_path: '{}' is a directory but file was required",
                            source_path.to_string_lossy()
                        ),
                    ))
                }
            } else if metadata.is_file() {
                if SourceType::File == source_type {
                    // open file to ensure it is permitted
                    let file = fs::File::open(&source_path);
                    match file {
                        Err(e) => match e.kind() {
                            ErrorKind::PermissionDenied => Err(Error::new(
                                e.kind(),
                                format!(
                                    "source_path: '{}' file access denied",
                                    source_path.to_string_lossy()
                                ),
                            )),
                            _ => Err(e),
                        },
                        Ok(_) => Ok(()),
                    }
                } else {
                    Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "source_path: '{}' is a file but directory was required",
                            source_path.to_string_lossy()
                        ),
                    ))
                }
            } else {
                // return error
                Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "source_path: '{}' is not a file or directory",
                        source_path.to_string_lossy()
                    ),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_dest_should_fail_when_not_found() {
        let non_existent_path = PathBuf::from("test_data/filename_which_does_not_exist");
        let error = verify_dest(&non_existent_path).unwrap_err();
        assert_eq!(ErrorKind::NotFound, error.kind());
        assert_eq!(
            "dest_dir: 'test_data/filename_which_does_not_exist' not found",
            format!("{}", error)
        );
    }

    #[test]
    fn verify_dest_should_fail_when_not_a_directory() {
        let non_existent_path = PathBuf::from("test_data/not_a_dir.txt");
        let error = verify_dest(&non_existent_path).unwrap_err();
        assert_eq!(ErrorKind::InvalidInput, error.kind());
        assert_eq!(
            "dest_dir: 'test_data/not_a_dir.txt' must be a directory",
            format!("{}", error)
        );
    }

    #[test]
    fn verify_dest_should_fail_when_directory_not_writable() {
        let restricted_path = PathBuf::from("test_data/dir_you_cant_write");
        let error = verify_dest(&restricted_path).unwrap_err();
        assert_eq!(ErrorKind::PermissionDenied, error.kind());
        assert_eq!(
            "dest_dir: 'test_data/dir_you_cant_write' not writable",
            format!("{}", error)
        );
    }

    #[test]
    fn verify_source_should_fail_when_source_does_not_exist() {
        let path = PathBuf::from("test_data/filename_which_does_not_exist");
        let error = verify_source(&path, SourceType::File).unwrap_err();
        assert_eq!(ErrorKind::NotFound, error.kind());
        assert_eq!(
            format!(
                "source_path: '{}' not found",
                String::from(path.to_string_lossy())
            ),
            format!("{}", error)
        );
    }

    #[test]
    fn verify_source_should_fail_when_source_is_not_file_or_dir() {
        let path = PathBuf::from("test_data/tty-device");
        let error = verify_source(&path, SourceType::File).unwrap_err();
        assert_eq!(ErrorKind::InvalidInput, error.kind());
        assert_eq!(
            format!(
                "source_path: '{}' is not a file or directory",
                String::from(path.to_string_lossy())
            ),
            format!("{}", error)
        );
    }

    #[test]
    fn verify_source_should_fail_when_source_is_denied_dir() {
        let path = PathBuf::from("test_data/dir_you_cant_read");
        let error = verify_source(&path, SourceType::Directory).unwrap_err();
        assert_eq!(ErrorKind::PermissionDenied, error.kind());
        assert_eq!(
            format!(
                "source_path: '{}' directory access denied",
                String::from(path.to_string_lossy())
            ),
            format!("{}", error)
        );
    }

    #[test]
    fn verify_source_should_fail_when_source_is_denied_file() {
        let path = PathBuf::from("test_data/file_you_cant_read.txt");
        let error = verify_source(&path, SourceType::File).unwrap_err();
        assert_eq!(ErrorKind::PermissionDenied, error.kind());
        assert_eq!(
            format!(
                "source_path: '{}' file access denied",
                String::from(path.to_string_lossy())
            ),
            format!("{}", error)
        );
    }
    #[test]
    fn verify_source_should_fail_when_want_file_but_is_dir() {
        let path = PathBuf::from("test_data/out");
        let error = verify_source(&path, SourceType::File).unwrap_err();
        assert_eq!(ErrorKind::InvalidInput, error.kind());
        assert_eq!(
            format!(
                "source_path: '{}' is a directory but file was required",
                String::from(path.to_string_lossy())
            ),
            format!("{}", error)
        );
    }

    #[test]
    fn verify_source_should_fail_when_want_dir_but_is_file() {
        let path = PathBuf::from("test_data/not_a_dir.txt");
        let error = verify_source(&path, SourceType::Directory).unwrap_err();
        assert_eq!(ErrorKind::InvalidInput, error.kind());
        assert_eq!(
            format!(
                "source_path: '{}' is a file but directory was required",
                String::from(path.to_string_lossy())
            ),
            format!("{}", error)
        );
    }
}
