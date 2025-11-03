// @file    main.rs / rusted-wc
// @author  Joseph Simeon
// @brief   Recreating `wc` based on the `man` page in Rust as a way to learn.
//
use std::env;
use std::process;
use filestream::FileStream;
use rusted_wc::WordCount;
use std::fs::File;
use std::fs::metadata;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let filestream;
    match FileStream::build("-Lclmw".to_string(), args) {
        Ok(fs) => filestream = fs,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }

    let mut wordcount: Vec<WordCount> = Vec::new();
    for filename in filestream.get_filenames() {
        // Checking that the file exists and is not a directory
        match File::open(&filename) {
            Ok(_) => {
                if metadata(&filename).unwrap().is_dir() {
                    eprintln!("Directories cannot be used with rusted-wc\n");
                    process::exit(1);
                }
            },
            Err(_) => {
                eprintln!("No file found for rusted-wc: {}\n", filename.clone());
                process::exit(1);
            },
        }

        let buf = BufReader::new(File::open(&filename).expect("Unable to open file"));
        match WordCount::build(filename.clone(), buf) {
            Ok(count) => wordcount.push(count),
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            },
        }
    }

    let mut wc_sum: WordCount = WordCount::new();
    for (count, filename) in wordcount.iter().zip(filestream.get_filenames().iter()) {
        count.print(&filestream.get_flags().join(""), filename);
        wc_sum.sum(count);
    }

    if wordcount.len() > 1 {
        wc_sum.print(&filestream.get_flags().join(""), &"total".to_string());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filestream() {
        let flags = "-Lclmw".to_string();
        let vec: Vec<String> = vec!["-w".to_string(), "README.md".to_string()];
        let filestream = FileStream::build(flags, vec).unwrap();

        assert_eq!(filestream.get_flags(), vec!["-w".to_string()]);
        assert_eq!(filestream.get_filenames(), vec!["README.md".to_string()]);
    }
}
