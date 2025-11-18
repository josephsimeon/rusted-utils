use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `wc`
///
/// Utility to display the number of lines, words, and bytes contained in each input file by
/// default.
struct Cli {
    /// File input to be processed by `r-wc`
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
}

fn main() {
    let cli = Cli::parse();

    println!("{cli:?}");
}

#[cfg(test)]
mod test {
    use super::*;

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
}
