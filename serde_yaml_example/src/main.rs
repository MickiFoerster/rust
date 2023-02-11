use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Hash, PartialEq, Eq, Deserialize)]
enum Language {
    Deutsch,
    English,
}

fn main() -> Result<(), serde_yaml::Error> {
    let yaml = std::fs::read_to_string("input.yaml").expect("could not read file");
    let deserialized_map: BTreeMap<String, HashMap<Language, String>> =
        serde_yaml::from_str(&yaml)?;
    println!("{:#?}", deserialized_map);

    println!("{}", deserialized_map["label1"][&Language::English]);
    println!("{}", deserialized_map["label2"][&Language::English]);
    println!("{}", deserialized_map["label3"][&Language::English]);
    println!("{}", deserialized_map["label4"][&Language::English]);

    println!("{}", deserialized_map["label1"][&Language::Deutsch]);
    println!("{}", deserialized_map["label2"][&Language::Deutsch]);
    println!("{}", deserialized_map["label3"][&Language::Deutsch]);
    println!("{}", deserialized_map["label4"][&Language::Deutsch]);

    Ok(())
}
