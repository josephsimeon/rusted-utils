use assert_cmd::cargo;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn runs_version() {
        let mut cmd = cargo::cargo_bin_cmd!("r-echo");
        let out = cmd.arg("-V").unwrap();
        let stdout = String::from_utf8(out.stdout).expect("invalid UTF-8");

        let version = env!("CARGO_PKG_VERSION");

        assert_eq!(stdout, format!("r-echo {version}\n"));
    }
}

