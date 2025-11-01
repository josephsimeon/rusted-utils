#[derive(Debug)]
struct FileStream {
    flags: Vec<String>,
    filenames: Vec<String>,
}

impl FileStream {
    fn build(flags_ok: String, args: Vec<String>) -> Result<FileStream, String> {
        // check for empty args
        if args.is_empty() {
            return Err(format!("rusted-wc: error: no arguments given"));
        }

        // shadow args with a mut reference
        let mut args: Vec<String> = args.into_iter().collect();

        // create a vector that holds all flags
        let flags: Vec<String> = args
            .iter()                                     // iterate through
            .filter(|&word| word.starts_with('-'))      // filter viable flags
            .map(|word| word.to_string())               // translate from &String to String
            .collect();                                 // create the new collection of String

        // retain non flags
        args.retain(|word| !word.starts_with('-'));

        if flags.len() > 0 {
            // check that flags contain at least an option for rusted-wc
            for flag in &flags {
                if *flag == "-".to_string() {
                    return Err(format!("rusted-wc: error: no flag information '-'"));
                }

                for f in flag.chars() {
                    if !flags_ok.contains(f) {
                        return Err(format!("rusted-wc: error: illegal option '{}', usage [-Lclmw]", f));
                    }
                }
            }
        }

        // check for empty args before processing filenames
        if args.len() == 0 {
            return Err(format!("rusted-wc: error: no filename arguments given"));
        }

        let filenames: Vec<String> = args.into_iter().collect();

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

        assert_eq!(test.flags, vec!["-w".to_string()]);
        assert_eq!(test.filenames, vec!["README.md".to_string()]);
    }

    #[test]
    fn test_blank_flag() {
        let vec: Vec<String> = vec![
            "README.md".to_string(),
        ];
        let flags: String = "-lwcmL".to_string();
        let test = FileStream::build(flags, vec).unwrap();

        assert_eq!(test.flags, Vec::<String>::new());
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

        assert_eq!(test.flags, vec!["-w".to_string()]);
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
