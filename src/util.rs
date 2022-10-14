use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(path: &Path, contents: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_files() {
        assert_eq!(
            read_file(&PathBuf::from("tests/data/empty.txt")).unwrap(),
            String::from("")
        );
        assert_eq!(
            read_file(&PathBuf::from("tests/data/lines.txt")).unwrap(),
            String::from("foo\nbar baz\n")
        );
    }

    #[test]
    fn write_files() {
        assert_eq!(
            write_file(&PathBuf::from("tests/data/empty.txt")),
            String::from("")
        );
        assert_eq!(
            write_file(&PathBuf::from("tests/data/lines.txt")),
            String::from("foo\nbar baz\n")
        );
    }
}
