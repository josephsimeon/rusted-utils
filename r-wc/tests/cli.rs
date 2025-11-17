use assert_cmd::cargo;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn runs() {
        let cmd = cargo::cargo_bin_cmd!("r-wc").unwrap();
        let stdout = String::from_utf8(cmd.stdout).expect("invalid UTF-8");
        assert_eq!(stdout, "Hello, world!\n");
    }
}
