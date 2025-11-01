use std::io::{BufRead, BufReader};
use std::fs::File;

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

    pub fn build(filename: String, reader: BufReader<File>) -> Result<WordCount, String> {
        let mut wc = WordCount::new();

        for line in reader.lines() {
            match line {
                Ok(parsed) => {
                    for word in parsed.split_whitespace() {
                        wc.letters.0 += word.len() + 1;
                        wc.letters.1 += word.chars().count() + 1;
                        wc.words += 1;
                    }
                    
                    if parsed.len() > wc.longest.0 {
                        wc.longest.0 = parsed.len();
                        wc.longest.1 = parsed.chars().count();
                    }
                    
                    wc.lines += 1;
                },
                Err(_) =>  {
                    return Err(
                        format!("rusted-wc: error: line was unparsable: {} in {}\n", 
                            wc.lines + 1, 
                            filename)
                    );
                },
            }
        }

        Ok(wc)
    }

    pub fn sum(&mut self, wc: &WordCount) -> &Self {
        self.lines += wc.lines;
        self.words += wc.words;
        self.letters.0 += wc.letters.0;
        self.letters.1 += wc.letters.1;
        self.longest.0 += wc.longest.0;
        self.longest.1 += wc.longest.1;

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::io::BufReader;
    use std::fs::File;

    #[test]
    fn test_build() {
        let _ = fs::write("test.txt", "This is a test file.");

        let buf = BufReader::new(File::open("test.txt").expect("Unable to open test.txt"));
        let wc: WordCount = WordCount::build("test.txt".to_string(), buf).unwrap();

        let _ = fs::remove_file("test.txt");

        assert_eq!(wc, WordCount {
            lines: 1,
            words: 5,
            letters: (21, 21),
            longest: (20, 20),
        });
    }

    #[test]
    fn test_add() {
        let _ = fs::write("test.txt", "This is a test file.");

        let buf = BufReader::new(File::open("test.txt").expect("Unable to open test.txt"));
        let wc: WordCount = WordCount::build("test.txt".to_string(), buf).unwrap();
        let mut wc_sum: WordCount = wc.clone();
        wc_sum.sum(&wc);

        let _ = fs::remove_file("test.txt");

        assert_eq!(wc_sum, WordCount {
            lines: 2,
            words: 10,
            letters: (42, 42),
            longest: (40, 40),
        });
    }
}
