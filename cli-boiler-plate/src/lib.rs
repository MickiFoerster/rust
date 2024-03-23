use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pub files: Vec<String>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = clap::Command::new("cli-boiler-plate")
        .version("0.1.0")
        .author("Michael Förster <micki.foerster@gmail.com>")
        .about("Command-line boiler plate code")
        .arg(
            clap::Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(0..)
                .default_value("-"),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap()
        .map(|v| v.to_string())
        .collect();

    Ok(Config { files })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", &config);
    Ok(())
}
