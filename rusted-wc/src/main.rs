// @file    main.rs / rusted-wc
// @author  Joseph Simeon
// @brief   Recreating `wc` based on the `man` page in Rust as a way to learn.
//
use std::env;
use std::process;
use filestream::FileStream;
use rusted_wc::WordCount;
use std::fs::File;
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
        // TODO check that file exists

        let buf = BufReader::new(File::open(&filename).expect("Unable to open file"));
        match WordCount::build(filename.clone(), buf) {
            Ok(count) => wordcount.push(count),
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            },
        }
    }

    println!("{wordcount:?}"); // TODO debug

    let mut wc_sum: WordCount = WordCount::new();
    for count in &wordcount {
        // TODO print wc
        wc_sum.sum(count);
    }

    if wordcount.len() > 1 {
        // TODO print wc_sum
    }

    println!("{wc_sum:?}"); // TODO debug

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
