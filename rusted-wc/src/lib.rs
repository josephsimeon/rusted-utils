#[derive(Debug)]
struct FileStream {
    flags: String,
    filenames: Vec<String>,
}

impl FileStream {
    fn build(flags_ok: String, args: Vec<String>) -> Result<FileStream, String> {
        // check for empty args
        if args.is_empty() {
            return Err(format!("rusted-wc: error: no arguments given"));
        }

        let mut arg_iter = args.iter();
    
        // process flags section of the args
        let flags: String;
        match args[0].starts_with('-') {
            true => flags = arg_iter.next().unwrap().to_string(),
            false => flags = "".to_string(),
        }

        if !flags.is_empty() {
            // check that flags contain legal options for rusted-wc
            if flags == "-".to_string() {
                return Err(format!("rusted-wc: error: no flag information '-'"));
            }

            for f in flags.chars() {
                if !flags_ok.contains(f) {
                    return Err(format!("rusted-wc: error: illegal option '{}'", f));
                }
            }
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
        let vec: Vec<String> = vec![
            "-w".to_string(), 
            "README.md".to_string(),
        ];
        let flags: String = "-lwcmL".to_string();
        let test = FileStream::build(flags, vec).unwrap();

        assert_eq!(test.flags, "-w".to_string());
        assert_eq!(test.filenames, vec!["README.md".to_string()]);
    }

    #[test]
    fn test_blank_flag() {
        let vec: Vec<String> = vec![
            "README.md".to_string(),
        ];
        let flags: String = "-lwcmL".to_string();
        let test = FileStream::build(flags, vec).unwrap();

        assert_eq!(test.flags, "".to_string());
        assert_eq!(test.filenames, vec!["README.md".to_string()]);
    }

    #[test]
    fn test_multiple_files() {
        let vec: Vec<String> = vec![
            "-w".to_string(), 
            "README.md".to_string(), 
            "README.md".to_string(),
        ];
        let flags: String = "-lwcmL".to_string();
        let test = FileStream::build(flags, vec).unwrap();

        assert_eq!(test.flags, "-w".to_string());
        assert_eq!(test.filenames, vec!["README.md".to_string(), "README.md".to_string()]);
    }

    #[test]
    fn test_no_arguments() {
        let vec: Vec<String> = vec![];
        let flags: String = "-lwcmL".to_string();
        let test = FileStream::build(flags, vec).unwrap_err();

        assert_eq!(test, format!("rusted-wc: error: no arguments given"));
    }

    #[test]
    fn test_no_files() {
        let vec: Vec<String> = vec![
            "-w".to_string(), 
        ];
        let flags: String = "-lwcmL".to_string();
        let test = FileStream::build(flags, vec).unwrap_err();

        assert_eq!(test, format!("rusted-wc: error: no filename arguments given"));
    }

    #[test]
    fn test_incomplete_flag() {
        let vec: Vec<String> = vec![
            "-".to_string(), 
            "README.md".to_string(),
        ];
        let flags: String = "-lwcmL".to_string();
        let test = FileStream::build(flags, vec).unwrap_err();

        assert_eq!(test, format!("rusted-wc: error: no flag information '-'"));
    }

    #[test]
    fn test_illegal_flag() {
        let vec: Vec<String> = vec![
            "-b".to_string(), 
            "README.md".to_string(),
        ];
        let flags: String = "-lwcmL".to_string();
        let test = FileStream::build(flags, vec).unwrap_err();

        assert_eq!(test, format!("rusted-wc: error: illegal option 'b'"));
    }
}
