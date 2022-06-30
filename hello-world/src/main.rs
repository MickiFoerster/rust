fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command as AssertCommand;
    use std::process::Command;
    use std::str;

    #[test]
    fn test_output() {
        let cargo_build = Command::new("cargo").arg("build").output();
        assert!(cargo_build.is_ok());
        let res = cargo_build.unwrap();
        let stdout = str::from_utf8(&res.stdout).unwrap();
        let stderr = str::from_utf8(&res.stderr).unwrap();
        println!("{}", stdout);
        println!("{}", stderr);

        let mut cmd = AssertCommand::cargo_bin("hello-world").unwrap();
        cmd.assert().success().stdout("Hello, world!\n");
    }
}
