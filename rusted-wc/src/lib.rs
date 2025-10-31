struct FileStream {
    flags: String,
    filenames: Vec<String>,
}

impl FileStream {
    fn build(args: Vec<String>) -> Result<FileStream, String> {
        // check for empty args
        if args.is_empty() {
            return Err(format!("rusted-wc: error: no arguments given"));
        }

        let mut arg_iter = args.iter();
    
        // process flags section of the args
        let flags: String;
        match args[0].starts_with("-") {
            true => flags = arg_iter.next().unwrap().to_string(),
            false => flags = "".to_string(),
        }

        // check for empty args before processing filenames
        if arg_iter.len() == 0 {
            return Err(format!("rusted-wc: error: no filename arguments given"));
        }

        let filenames: Vec<String> = arg_iter.map(|s| s.to_string()).collect();

        Ok(FileStream { flags, filenames })
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

    #[test]
    fn test_blank_flag() {
        let test = FileStream::build(vec![
            "README.md".to_string(),
        ]);

        assert_eq!(test.flags, "".to_string());
        assert_eq!(test.filenames, vec!["README.md".to_string()]);
    }

    #[test]
    fn test_multiple_files() {
        let test = FileStream::build(vec![
            "-w".to_string(),
            "README.md".to_string(),
            "README.md".to_string(),
        ]);

        assert_eq!(test.flags, "-w".to_string());
        assert_eq!(test.filenames, vec!["README.md".to_string(), "README.md".to_string()]);
    }
}
