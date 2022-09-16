use std::{
    fs::{self, File},
    io::{self, ErrorKind},
    path::Path,
};

/// Given a path to a file create a file, recursively creating parent directories as needed.
pub fn create_file_all<P: AsRef<Path>>(path: P) -> io::Result<File> {
    let path = path.as_ref();
    if path.is_dir() {
        return Err(io::Error::new(
            ErrorKind::Other,
            "Path to file should be provided, but this path leads to directory.",
        ));
    }

    let dir = path.parent().unwrap();

    fs::create_dir_all(dir)?;
    File::create(path)
}
