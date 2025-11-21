use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `echo`
/// 
/// Utility wriates any specified operands seperated by a single blank (' ') character and followed
/// by a newline ('\n') character to the standard output.
struct Cli {
    #[arg(required(true))]
    /// Input operand processed by `r-echo`.
    string: Vec<String>,

    #[arg(short('n'))]
    /// Do not print the trailing newline character.
    no_newline: bool,
}

fn main() {
    let cli = Cli::parse();

    // print all strings given
    print!("{}", cli.string.join(" "));

    // print newline
    if !cli.no_newline { print!("\n") };
}
