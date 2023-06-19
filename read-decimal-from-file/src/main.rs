use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write};

fn read_decimal_from_file(file_path: &str) -> Result<f64> {
    let file = File::open(file_path).expect("could not open file");
    let reader = BufReader::new(file);
    let line = reader.lines().next().expect("could not read line")?;
    let decimal = line
        .trim()
        .parse::<f64>()
        .expect("could not parse file content");
    Ok(decimal)
}

fn main() {
    let filename = "number.txt";
    let decimal = 3.14;
    let mut file = File::create(filename).expect("could not create file");
    write!(file, "{}", decimal).expect("could not write to file");

    match read_decimal_from_file(filename) {
        Ok(decimal) => println!("Decimal read from file: {}", decimal),
        Err(err) => eprintln!("Error reading decimal from file: {}", err),
    }
}
