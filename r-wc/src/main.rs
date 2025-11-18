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

fn main() {
    let cli = Cli::parse();

    println!("{cli:?}");
}
