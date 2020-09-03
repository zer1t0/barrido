use std::fs::File;
use std::io::Lines;
use std::io::{BufRead, BufReader};

/// Function to read the inputs from files, strings or stdin
/// in a normalized iterator of strings.
pub fn read_inputs(inputs: Vec<String>) -> impl Iterator<Item = String> {
    let input_iter = Box::new(FileStringIter::new(inputs));
    return input_iter
        .map(|s| s.trim().to_string())
        .filter(|s| s.len() != 0 && !s.starts_with("#"));
}

/// Class to read a bunch of strings that could be filenames.
/// If string is a valid path, then the lines of the file are retrieved,
/// in other case, the string itself is retrieved.
pub struct FileStringIter {
    items: Vec<String>,
    lines: Option<Lines<BufReader<File>>>,
}

impl FileStringIter {
    pub fn new(mut items: Vec<String>) -> Self {
        items.reverse();
        return Self { items, lines: None };
    }
}

impl Iterator for FileStringIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            if let Some(lines) = &mut self.lines {
                if let Some(line) = lines.next() {
                    return Some(line.expect("Error reading input"));
                } else {
                    self.lines = None;
                }
            }

            let item = self.items.pop()?;

            match File::open(&item) {
                Ok(file) => {
                    self.lines = Some(BufReader::new(file).lines());
                }
                Err(_) => {
                    return Some(item);
                }
            }
        }
    }
}
