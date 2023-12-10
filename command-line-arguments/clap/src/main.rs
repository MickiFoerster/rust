use clap::{value_parser, Arg};

fn main() {
    let matches = clap::Command::new("My Program")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Enable verbosity"),
        )
        .arg(
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .value_name("NUM")
                .value_parser(value_parser!(usize))
                .default_value("1")
                .help("Number of jobs"),
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Name argument")
                .default_value("myname"),
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");
    let jobs: usize = *matches.get_one("jobs").expect("'job' is invalid");
    let name = matches
        .get_one::<String>("name")
        .expect("'name' is invalid");
    dbg!(verbose, jobs, name);
}
