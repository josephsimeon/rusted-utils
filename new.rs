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

#[derive(Debug)] // TODO delete when finished
struct WordCount {
    lines: usize,
    words: usize,
    letters: (usize, usize),
    longest: (usize, usize),
}

impl WordCount {
    fn new() -> WordCount {
        WordCount {
            lines: 0,
            words: 0,
            letters: (0, 0),
            longest: (0, 0),
        }
    }
}

fn process_file(name: String) -> WordCount {
    match File::open(name.clone()) {
        Ok(_) => {
            if metadata(name.clone()).unwrap().is_dir() {
                eprintln!("Directories cannot be used with rusted-wc\n");
                process::exit(1);
            }
        },
        Err(_) => {
            eprintln!("No file found for rusted-wc: {}\n", name.clone());
            process::exit(1);
        },
    }
    
    let reader = BufReader::new(File::open(name.clone()).expect("Unable to open file"));
    let mut wc = WordCount::new();
    
    for line in reader.lines() {
        match line {
            Ok(parsed) => {
                for word in parsed.split_whitespace() {
                    wc.letters.0 += word.len() + 1;
                    wc.letters.1 += word.chars().count() + 1;
                    wc.words += 1;
                }
                
                if parsed.len() > wc.longest.0 {
                    wc.longest.0 = parsed.len();
                    wc.longest.1 = parsed.chars().count();
                }
                
                wc.lines += 1;
            },
            Err(_) => {
                eprintln!("Line was unparsable: {} in {}\n", wc.lines + 1, name);
                process::exit(1);
            },
        }
    }
    
    wc
}

fn main() {
    // TODO let args: Vec<String> = env::args().skip(1).collect();
    let args: Vec<String> = vec!["-w".to_string(), "README.md".to_string(), "README.md".to_string()];

    let mut fs = FileStream::new();
    fs.update(args);
    fs.check();
    println!("fs = {fs:?}"); // TODO delete when finished
    
    let mut wc: Vec<WordCount> = Vec::new();
    for file in fs.names {
        wc.push(process_file(file));
    }
    println!("wc = {wc:?}");
}
