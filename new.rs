// @file    rusted-wc.rs
// @author  Joseph Simeon
// @brief   `wc` command-line rewritten with Rust

struct FileStream {
    flags: String,
    names: Vec<String>,
}

impl FileStream {
    fn new() -> FileStream {
        FileStream {
            flags: String::new(),
            names: Vec::new(),
        }
    }

    fn update(&mut self, args: Vec<String>) -> &Self {
    }

    fn update_flags(&mut self, new: String) -> &Self {
    }

    fn update_filenames(&mut self, new: String) -> &Self {
    }
}

fn main() {
    let mut fs = FileStream::new();
}
