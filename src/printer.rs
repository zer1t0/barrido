use reqwest::Url;
use log::warn;

pub struct Printer {
    show_status: bool,
    show_size: bool,
    show_progress: bool,
    expand_path: bool,
    cleaner_str: &'static str,
}

impl Printer {
    const TERMINAL_CLEANER_STRING: &'static str = "\r\x1b[2K";

    pub fn new(
        show_status: bool,
        show_size: bool,
        show_progress: bool,
        expand_path: bool,
    ) -> Self {
        let cleaner_str;
        if show_progress {
            cleaner_str = Self::TERMINAL_CLEANER_STRING;
        } else {
            cleaner_str = ""
        }

        return Self {
            show_status,
            show_size,
            show_progress,
            expand_path,
            cleaner_str,
        };
    }

    pub fn print_path(&self, url: &Url, status: u16, body_length: usize) {
        let path;
        if self.expand_path {
            path = url.as_str();
        } else {
            path = url.path();
        }

        let mut line = format!("{}", path);
        if self.show_status {
            line += format!(" {}", status).as_str();
        }
        if self.show_size {
            line += format!(" {}", body_length).as_str();
        }

        println!("{}{}", self.cleaner_str, line);
    }

    pub fn print_progress(&self, current_count: usize, max_count: usize) {
        if self.show_progress {
            let percentage = current_count as f32 / max_count as f32 * 100.0;

            print!("\r{}/{} {:.2}%", current_count, max_count, percentage);
        }
    }

    pub fn print_clean(&self) {
        print!("{}", Self::TERMINAL_CLEANER_STRING);
    }

    pub fn print_error(&self, error: reqwest::Error) {
            warn!("{}[x] {:?}", self.cleaner_str, error);
    }
}
