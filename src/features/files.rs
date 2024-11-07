use std::fs::{File, OpenOptions};
use std::{fs, io};
use std::io::{Read, Write};
use std::path::Path;

#[allow(dead_code)]
struct Files;
impl Files {
    /// Creates a new file, or overwrites an existing one, and writes the content to it.
    #[allow(dead_code)]
    pub fn write_all<P: AsRef<Path>>(path: P, content: &[u8]) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content)
    }

    /// Appends content to an existing file, or creates a new one if it doesn’t exist.
    #[allow(dead_code)]
    pub fn append_all<P: AsRef<Path>>(path: P, content: &[u8]) -> io::Result<()> {
        let mut file = OpenOptions::new().append(true).create(true).open(path)?;
        file.write_all(content)
    }

    /// Reads all content from a file into a `String`.
    #[allow(dead_code)]
    pub fn read_all<P: AsRef<Path>>(path: P) -> io::Result<String> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    /// Reads all content from a file into a `Vec<u8>`, for binary files.
    #[allow(dead_code)]
    pub fn read_all_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(content)
    }

    /// Deletes a file if it exists.
    #[allow(dead_code)]
    pub fn delete<P: AsRef<Path>>(path: P) -> io::Result<()> {
        if path.as_ref().exists() {
            fs::remove_file(path)
        } else {
            Ok(())
        }
    }

    /// Checks if a file exists at the specified path.
    #[allow(dead_code)]
    pub fn exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
    }

    /// Creates an empty file at the specified path.
    #[allow(dead_code)]
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<File> {
        File::create(path)
    }
}