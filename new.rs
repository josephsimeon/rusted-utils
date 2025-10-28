// @file    rusted-wc.rs
// @author  Joseph Simeon
// @brief   `wc` command-line rewritten with Rust
use std::process;

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
    
    fn filestream_error(errs: String) {
        eprintln!("{errs}");
        process::exit(1);
    }

    fn update(&mut self, args: Vec<String>) -> &Self {
        if args.is_empty() {
            FileStream::filestream_error(format!("Error in FileStream, no arguments given"));
        }
    
        let mut arg_iter = args.iter();
        if args[0].starts_with("-") {
            self.update_flags(arg_iter.next().unwrap());
        }

        for arg in arg_iter {
            self.update_filenames(arg);
        }
        self
    }

    fn update_flags(&mut self, new: &String) -> &Self {
        self.flags = String::from(new);
        self
    }

    fn update_filenames(&mut self, new: &String) -> &Self {
        self.names.push(String::from(new));
        self
    }
    
    fn check(&self) {
        for f in self.flags.chars() {
            match f {
                '-' | 'l' | 'w' | 'c' | 'm' | 'L' => {},
                _ => {
                    FileStream::filestream_error(format!("Error in FileStream illegal flag for rusted-wc: {}", f));
                    process::exit(1);
                },
            }
        }

        if self.names.is_empty() {
            FileStream::filestream_error(format!("Error in FileStream no file(s) to target with rusted-wc"));
            process::exit(1);
        }
    }
}

fn main() {
    // TODO let args: Vec<String> = env::args().skip(1).collect();
    let args: Vec<String> = vec!["-w".to_string(), "README.md".to_string(), "README.md".to_string()];

    let mut fs = FileStream::new();
    fs.update(args);
    fs.check();
    println!("fs = {fs:?}"); // TODO delete when finished
}
