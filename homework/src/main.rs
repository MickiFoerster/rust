use std::num::ParseIntError;

fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(9)
        .map(|i| u8::from_str_radix(&s[i..i + 8], 2))
        .collect()
}

fn main() -> Result<(), ParseIntError> {
    let fs_content = std::fs::read_to_string("PLATTE05.TXT").expect("cannot load file");
    for line in fs_content.lines() {
        if line.trim().len() > 0 {
            let bytes = decode(line)?;
            for i in bytes {
                let code: u32 = i.into();
                print!("{}", char::from_u32(code).expect("could not convert"));
            }
        }
        println!("");
    }
    //println!("file content: {fs_content}");
    Ok(())
}
