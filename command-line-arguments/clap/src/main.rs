use clap::{App, Arg};

fn main() {
    let matches = App::new("My Program")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enable verbosity"),
        )
        .arg(
            Arg::with_name("jobs")
                .short("j")
                .long("jobs")
                .value_name("NUM")
                .help("Number of jobs")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("NAME")
                .help("Name argument")
                .takes_value(true)
                .default_value("myname"),
        )
        .get_matches();

    let verbose = matches.is_present("verbose");
    let jobs = matches.value_of("jobs").map(|v| v.parse::<u32>().unwrap());
    let name = matches.value_of("name").unwrap();

    println!("Verbose: {}", verbose);
    if let Some(jobs) = jobs {
        println!("Jobs: {}", jobs);
    }
    println!("Name: {}", name);
}
