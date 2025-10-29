// @file    rusted-wc.rs
// @author  Joseph Simeon
// @brief   `wc` command-line rewritten with Rust
use std::process;
use std::fs::File;
use std::fs::metadata;
use std::io::{BufRead, BufReader};
use std::env;

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

#[derive(Clone)]
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

    fn print(&self, flag: String,  name: String) {
        if flag.is_empty() || flag.contains("l") {
            print!("\t{}", self.lines);
        }

        if flag.is_empty() || flag.contains("w") {
            print!("\t{}", self.words);
        }

        if flag.is_empty() || flag.contains("c") || flag.contains("m") {
            if flag.contains("m") {
                print!("\t{}", self.letters.0);
            } else {
                print!("\t{}", self.letters.1);
            }
        }

        if flag.contains("L") {
            if flag.contains("m") {
                print!("\t{}", self.longest.0);
            } else {
                print!("\t{}", self.longest.1);
            }
        }

        println!(" {}", name);
    }

    fn add(&mut self, wc: WordCount) -> &Self {
        self.lines += wc.lines;
        self.words += wc.words;
        self.letters.0 += wc.letters.0;
        self.letters.1 += wc.letters.1;
        self.longest.0 += wc.longest.0;
        self.longest.1 += wc.longest.1;

        self
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
    let args: Vec<String> = env::args().skip(1).collect();

    let mut fs = FileStream::new();
    fs.update(args);
    fs.check();
    
    let mut wc: Vec<WordCount> = Vec::new();
    for file in &fs.names {
        wc.push(process_file(file.to_string()));
    }

    let mut total: WordCount = WordCount::new();
    for (count, file) in wc.iter().zip(fs.names.iter()) {
        count.print(fs.flags.clone(), file.to_string());
        total.add(count.clone());
    }

    if wc.len() > 1 {
        total.print(fs.flags.clone(), "total".to_string());
    }
}
