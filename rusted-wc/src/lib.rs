struct FileStream {
    flags: String,
    filenames: Vec<String>,
}
impl FileStream {
    fn build(args: Vec<String>) -> FileStream {
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
