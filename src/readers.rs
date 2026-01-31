use std::{
    collections::VecDeque,
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

pub fn read_from_file(path: PathBuf, s: &mut String) -> io::Result<()> {
    let mut file = File::open(&path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("could not open file {}: {}", path.display(), e),
        )
    })?;

    file.read_to_string(s).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("could not read file {} to string: {}", path.display(), e),
        )
    })?;

    Ok(())
}

/// Recursively reads all files in a directory into a string.
/// Performs a breadth-first trversal of the dir tree.
pub fn read_from_dir(path: PathBuf, s: &mut String) -> io::Result<()> {
    let mut queue = VecDeque::from([path]);

    // visit a node
    while let Some(path) = queue.pop_front() {
        match visit_node(&mut queue, path, s) {
            Err(e) if e.kind() == io::ErrorKind::InvalidData => continue,
            Err(e) => return Err(e),
            Ok(_) => {}
        }
    }
    Ok(())
}

/// Processes a single directory, queuing subdirectories and reading files
fn visit_node(queue: &mut VecDeque<PathBuf>, current: PathBuf, s: &mut String) -> io::Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            queue.push_back(path);
            continue;
        }
        read_from_file(path, s)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{env::temp_dir, fs};

    use crate::readers::read_from_file;

    #[test]
    fn read_to_file_sucessfully() {
        // arrange
        let mut str = String::new();
        let path = temp_dir();
        let file = path.join("test.txt");
        let _ = fs::write(
            &file,
            "Hey, this is a test string for a test file for a test!",
        );

        // act
        let _ = read_from_file(path, &mut str);
        let x = fs::read_to_string(file);

        // assert
        assert!(x.unwrap().contains(&str));
    }
}
