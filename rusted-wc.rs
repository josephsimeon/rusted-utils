// NAME
//      wc – word, line, character, and byte count
//
// SYNOPSIS
//      wc [--libxo] [-Lclmw] [file ...]
//
// DESCRIPTION
//      The wc utility displays the number of lines, words, and bytes contained in each input file, 
//      or standard input (if no file is specified) to the standard output.  A line is defined as a 
//      string of characters delimited by a ⟨newline⟩ character.  Characters beyond the final ⟨newline⟩ 
//      character will not be included in the line count.
//
//      A word is defined as a string of characters delimited by white space characters.  White space 
//      characters are the set of characters for which the iswspace(3) function returns true.  If 
//      more than one input file is specified, a line of cumulative counts for all the files is 
//      displayed on a separate line after the output for the last file.
//
//      The following options are available:
//
//      --libxo
//              Generate output via libxo(3) in a selection of different human and machine readable 
//              formats.  See xo_parse_args(3) for details on command line arguments.
//
//      -L      Write the length of the line containing the most bytes (default) or characters (when 
//              -m is provided) to standard output.  When more than one file argument is specified, 
//              the longest input line of all files is reported as the value of the final “total”.
//
//      -c      The number of bytes in each input file is written to the standard output.  This will 
//              cancel out any prior usage of the -m option.
//
//      -l      The number of lines in each input file is written to the standard output.
//
//      -m      The number of characters in each input file is written to the standard output.  If 
//              the current locale does not support multibyte characters, this is equivalent to the 
//              -c option. This will cancel out any prior usage of the -c option.
//
//      -w      The number of words in each input file is written to the standard output.
//
//      When an option is specified, wc only reports the information requested by that option. The 
//      order of output always takes the form of line, word, byte, and file name.  The default action 
//      is equivalent to specifying the -c, -l and -w options.
//
//      If no files are specified, the standard input is used and no file name is displayed. The 
//      prompt will accept input until receiving EOF, or [^D] in most environments.
//
//      If wc receives a SIGINFO (see the status argument for stty(1)) signal, the interim data will 
//      be written to the standard error output in the same format as the standard completion message.
//
// ENVIRONMENT
//      The LANG, LC_ALL and LC_CTYPE environment variables affect the execution of wc as described 
//      in environ(7).
use std::env;
use std::process;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs::metadata;

struct WordCount
{
    lines: u32,
    words: u32,
    characters: u32,
    bytes: u32,
    longest_line_in_characters: u32,
    longest_line_in_bytes: u32,
    filename: String,
}

impl WordCount {
    pub fn new(lines: u32, words: u32, characters: u32, bytes: u32, longest_line_in_characters: u32, longest_line_in_bytes: u32, filename: String) -> Self
    {
        Self {
            lines,
            words,
            characters,
            bytes,
            longest_line_in_characters,
            longest_line_in_bytes,
            filename,
        }
    }

    pub fn print(&self, flag: String)
    {
        if flag.is_empty() || flag.contains("l")
        {
            print!("\t{}", self.lines);
        }

        if flag.is_empty() || flag.contains("w")
        {
            print!("\t{}", self.words);
        }

        if flag.is_empty() || flag.contains("c") || flag.contains("m")
        {
            if flag.contains("m")
            {
                print!("\t{}", self.characters);
            }
            else
            {
                print!("\t{}", self.bytes);
            }
        }

        if flag.contains("L")
        {
            if flag.contains("m")
            {
                print!("\t{}", self.longest_line_in_characters)
            }
            else
            {
                print!("\t{}", self.longest_line_in_bytes);
            }
        }

        println!(" {}", self.filename);
    }
}

fn main ()
{
    let args: Vec<String> = env::args().skip(1).collect();

    let mut flag: String = "".to_string();
    let mut files: Vec<String> = Vec::new();

    for (i, a) in args.iter().enumerate()
    {
        match i
        {
            0 => {
                if a.as_str().starts_with("-")
                {
                    flag = a.to_string();
                }
                else
                {
                    files.push(a.to_string());
                }
            },
            _ => {
                files.push(a.to_string());
            },
        }
    }
    
    if !flag.is_empty()
    {
        for f in flag.chars()
        {
            match f
            {
                '-' | 'l' | 'w' | 'c' | 'm' | 'L' => {
                },
                _ => {
                    eprintln!("Illegal flag for rusted-wc: {}", f);
                    process::exit(1);
                },
            }
        }
    }

    if files.is_empty()
    {
        eprintln!("No file(s) to target with rusted-wc\n");
        process::exit(1);
    }

    let mut processed_files: Vec<WordCount> = Vec::new();
    for file in files
    {
        match File::open(file.clone())
        {
            Ok(_) => {
                if metadata(file.clone()).unwrap().is_dir()
                {
                    eprintln!("Directories cannot be used with rusted-wc\n");
                    process::exit(1);
                }
            },
            Err(_) => {
                eprintln!("No file found for rusted-wc: {}\n", file.clone());
                process::exit(1);
            },
        }

        processed_files.push(process_for_word_count(file));
    }

    print_out_word_count(processed_files, flag);
}

fn process_for_word_count (filename: String) -> WordCount
{
    let reader = BufReader::new(File::open(filename.clone()).expect("Unable to open file"));
    let mut sum: WordCount = WordCount::new(0, 0, 0, 0, 0, 0, filename);

    for line in reader.lines()
    {
        match line {
            Ok(parsed_line) => {
                for word in parsed_line.split_whitespace()
                {
                    sum.bytes = sum.bytes + word.len() as u32 + 1;
                    sum.characters = sum.characters + word.chars().count() as u32 + 1;
                    sum.words = sum.words + 1;
                }

                if parsed_line.len() > sum.longest_line_in_bytes as usize
                {
                    sum.longest_line_in_bytes = parsed_line.len() as u32;
                    sum.longest_line_in_characters = parsed_line.chars().count() as u32;
                }
                sum.lines = sum.lines + 1;
            },
            Err(_) => {
                eprintln!("Line was unparsable: {} in {}\n", sum.lines + 1, sum.filename);
                process::exit(1);
            },
        }
    }

    sum
}

fn print_out_word_count (vec: Vec<WordCount>, flag: String)
{
    let mut total: WordCount = WordCount::new(0, 0, 0, 0, 0, 0, "total".to_string());

    for v in &vec 
    {
        total.lines = total.lines + v.lines;
        total.words = total.words + v.words;
        total.characters = total.characters + v.characters;
        total.bytes = total.bytes + v.bytes;
        total.longest_line_in_characters = total.longest_line_in_characters + v.longest_line_in_characters;
        total.longest_line_in_bytes = total.longest_line_in_bytes + v.longest_line_in_bytes;
        v.print(flag.clone());
    }

    if vec.len() > 1
    {
        total.print(flag.clone());
    }
}
