struct FileStream {
    flags: String,
    filenames: Vec<String>,
}

impl FileStream {
    fn build(args: Vec<String>) -> FileStream {
        // check for empty args
        if args.is_empty() {
            // TODO add in a process::exit routine and error print for empty argument
        }

        let mut arg_iter = args.iter();
    
        // process flags section of the args
        let flags: String;
        match args[0].starts_with("-") {
            true => flags = arg_iter.next().unwrap().to_string(),
            false => flags = "".to_string(),
        }

        // check for empty args before processing filenames
        if arg_iter.is_empty() {
            // TODO add in a process::exit routine and error print for no file argument
        }

        let filenames: Vec<String> = arg_iter.map(|s| s.to_string()).collect();

        FileStream { flags, filenames}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build() {
        let test = FileStream::build(vec![
            "-w".to_string(),
            "README.md".to_string(),
        ]);

        assert_eq!(test.flags, "-w".to_string());
        assert_eq!(test.filenames, vec!["README.md".to_string()]);
    }
}
