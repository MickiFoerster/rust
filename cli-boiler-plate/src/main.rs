fn main() {
    if let Err(e) = cli_boiler_plate::get_args().and_then(cli_boiler_plate::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
    println!("Work done");
}
