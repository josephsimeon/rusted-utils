// @file    rusted-wc.rs
// @author  Joseph Simeon
// @brief   `wc` command-line rewritten with Rust

#[derive(Debug)] // TODO delete when finsihed
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
        self.update_flags(args[0].clone());
        self.update_filenames(args[1].clone());
        self
    }

    fn update_flags(&mut self, new: String) -> &Self {
        self.flags = new;
        self
    }

    fn update_filenames(&mut self, new: String) -> &Self {
        self.names.push(new);
        self
    }
}

fn main() {
    // TODO let args: Vec<String> = env::args().skip(1).collect();

    let mut fs = FileStream::new();
    fs.update(vec!["-w".to_string(), "README.md".to_string()]);
    println!("fs = {fs:?}"); // TODO delete when finished
}
