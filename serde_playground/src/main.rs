use serde_json::value::RawValue;

fn main() {
    let data = std::fs::read("json/deployment.json").unwrap();
    let str_data = String::from_utf8_lossy(&data);
    let raw: Box<RawValue> = serde_json::from_str(&str_data).unwrap();

    let val = serde_json::to_value(raw).unwrap();
    println!("{}", val["description"]);
    println!();
    println!("{}", val["properties"]["apiVersion"]);
    println!();
    println!(
        "description: {}",
        val["properties"]["apiVersion"]["description"]
    );
    println!();
    println!("type: {}", val["properties"]["apiVersion"]["type"]);
}
