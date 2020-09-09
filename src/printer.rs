use log::warn;

use crate::communication::result_channel::{Answer};

pub struct Printer {
    show_status: bool,
    show_size: bool,
    show_progress: bool,
    show_headers: bool,
    expand_path: bool,
    cleaner_str: &'static str,
    newline: &'static str,
}

impl Printer {
    const TERMINAL_CLEANER_STRING: &'static str = "\r\x1b[2K";

    pub fn new(
        show_status: bool,
        show_size: bool,
        show_progress: bool,
        expand_path: bool,
        show_headers: bool
    ) -> Self {
        let cleaner_str;
        if show_progress {
            cleaner_str = Self::TERMINAL_CLEANER_STRING;
        } else {
            cleaner_str = ""
        }
        
        let newline = if show_headers {
            "\n"
        } else {
            ""
        };

        return Self {
            show_status,
            show_size,
            show_progress,
            show_headers,
            expand_path,
            cleaner_str,
            newline,
        };
    }

    pub fn print_answer(&self, answer: &Answer) {
        let path = if self.expand_path {
            answer.url.as_str()
        } else {
            answer.url.path()
        };

        let mut line = format!("{}", path);
        if self.show_status {
            line = format!("{} {}", line, answer.status);
        }
        
        if self.show_size {
            line = format!("{} {}", line, answer.size);
        }

        if self.show_headers {
            for (name, value) in answer.headers.iter() {
                let str_value = if let Ok(value) = value.to_str() {
                    value
                } else {
                    "---- No ASCII Header ----"
                };
                
                line = format!("{}\n{}: {}", line, name, str_value);
            }
        }

        eprint!("{}", self.cleaner_str);
        println!("{}{}", line, self.newline);
    }

    pub fn print_progress(&self, current_count: usize, max_count: usize) {
        if self.show_progress {
            let percentage = current_count as f32 / max_count as f32 * 100.0;
            eprint!("\r{}/{} {:.2}%", current_count, max_count, percentage);
        }
    }

    pub fn print_clean(&self) {
        eprint!("{}", Self::TERMINAL_CLEANER_STRING);
    }

    pub fn print_error(&self, error: reqwest::Error) {
            warn!("{}[x] {:?}", self.cleaner_str, error);
    }
}
