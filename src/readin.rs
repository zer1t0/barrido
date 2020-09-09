use log::warn;
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
    current_path: String,
}

impl FileStringIter {
    pub fn new(mut items: Vec<String>) -> Self {
        items.reverse();
        return Self {
            items,
            lines: None,
            current_path: "".to_string(),
        };
    }
}

impl Iterator for FileStringIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            if let Some(lines) = &mut self.lines {
                if let Some(line) = lines.next() {
                    match line {
                        Ok(line) => {
                            return Some(line);
                        }
                        Err(err) => {
                            warn!("Error reading lines in {}: {}. '{}' is taken as URL path ",
                                  self.current_path, err, self.current_path);
                            self.lines = None;
                            let current_path = self.current_path.clone();
                            self.current_path = "".to_string();
                            self.lines = None;
                            return Some(current_path);
                        }
                    }
                } else {
                    self.lines = None;
                }
            }

            let item = self.items.pop()?;

            match File::open(&item) {
                Ok(file) => {
                    self.current_path = item;
                    self.lines = Some(BufReader::new(file).lines());
                }
                Err(_) => {
                    return Some(item);
                }
            }
        }
    }
}
