// @file    main.rs / rusted-wc
// @brief   Recreating `wc` based on the `man` page in Rust as a way to learn.
// @author  Joseph Simeon
//
use filestream::FileStream;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_filestream() {
        let flags = "-Lclmw".to_string();
        let vec: Vec<String> = vec!["-w".to_string(), "README.md".to_string()];
        let filestream = FileStream::build(flags, vec).unwrap();

        assert_eq!(filestream.get_flags(), vec!["-w".to_string()]);
        assert_eq!(filestream.get_filenames(), vec!["README.md".to_string()]);
    }
}
