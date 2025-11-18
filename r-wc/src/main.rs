use clap::Parser;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `wc`
///
/// Utility to display the number of lines, words, and bytes contained in each input file by
/// default.
struct Cli {
    /// File input to be processed by `r-wc`
    #[arg(required(true))]
    file: Vec<String>,

    /// Print longest length in the input file in characters or bytes
    #[arg(short('L'))]
    length: bool,

    /// Print number of characters in the input file
    #[arg(short('c'))]
    bytes: bool,

    /// Print number of lines in the input file
    #[arg(short('l'))]
    lines: bool,

    /// Print number of bytes in the input file
    #[arg(short('m'))]
    chars: bool,

    /// Print number of words in the input file
    #[arg(short('w'))]
    words: bool,
}

#[derive(Debug, PartialEq, Clone)]
struct Flags {
    length: bool,
    bytes: bool,
    lines: bool,
    chars: bool,
    words: bool,
}

impl Flags {
    fn build(cli: &Cli) -> Flags {
        Flags {
            length: cli.length,
            bytes: cli.bytes,
            lines: cli.lines,
            chars: cli.chars,
            words: cli.words,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct WordCount {
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

    fn build(file: &String) -> Result<WordCount, String> {
        let mut wc = WordCount::new();

        let buf = BufReader::new(match File::open(file) {
                Ok(f) => f,
                Err(_) => return Err(format!("r-wc: error: unable to open {file}")),
            }
        );

        for line in buf.lines() {
            match line {
                Ok(parsed) => {
                    wc.letters.0 += parsed.len() + 1;
                    wc.letters.1 += parsed.chars().count() + 1;

                    for _ in parsed.split_whitespace() {
                        wc.words += 1;
                    }
                    
                    if parsed.len() > wc.longest.0 {
                        wc.longest.0 = parsed.len();
                        wc.longest.1 = parsed.chars().count();
                    }
                    
                    wc.lines += 1;
                },
                Err(_) => {
                    return Err(
                        format!("r-wc: error: line was unparsable: {} in {}", wc.lines + 1, file)
                    );
                }
            }
        }

        Ok(wc)
    }

    fn add(&mut self, other: &WordCount) -> &Self {
        self.lines += other.lines;
        self.words += other.words;
        self.letters.0 += other.letters.0;
        self.letters.1 += other.letters.1;
        self.longest.0 += other.longest.0;
        self.longest.1 += other.longest.1;

        self
    }
}

fn main() {
    let cli = Cli::parse();

    println!("{cli:?}");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_wc_new() {
        let test = WordCount {
            lines: 0,
            words: 0,
            letters: (0, 0),
            longest: (0, 0),
        };

        assert_eq!(WordCount::new(), test);
    }

    #[test]
    fn test_wc_add() {
        let mut test = WordCount::new();
        let other = WordCount {
            lines: 1,
            words: 2,
            letters: (3, 4),
            longest: (5, 6),
        };

        test.add(&other);

        assert_eq!(test, WordCount {
            lines: 1,
            words: 2,
            letters: (3, 4),
            longest: (5, 6),
        });
    }

    #[test]
    fn test_wc_build() {
        let _ = fs::write("test.txt", "This is a test file.");
        
        let wc = WordCount::build(&"test.txt".to_string()).unwrap();

        let _ = fs::remove_file("test.txt");

        assert_eq!(wc, WordCount {
            lines: 1,
            words: 5,
            letters: (21, 21),
            longest: (20, 20),
        });
    }

    #[test]
    fn test_wc_no_file() {
        let wc = WordCount::build(&"no_file.txt".to_string()).unwrap_err();

        assert_eq!(wc, "r-wc: error: unable to open no_file.txt");
    }

    #[test]
    fn test_flags_build() {
        let cli = Cli {
            file: Vec::new(),
            length: false,
            bytes: true,
            lines: true,
            chars: false,
            words: true,
        };
        let test = Flags::build(&cli);

        assert_eq!(test, Flags {
            length: false,
            bytes: true,
            lines: true,
            chars: false,
            words: true,
        });
    }
}
