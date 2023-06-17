use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Language {
    English,
    Ukrainian,
}

#[derive(Serialize, Deserialize)]
pub struct KeyChar {
    language: Language,
    symbol: char,
}

#[derive(Serialize, Deserialize)]
pub struct Keys {
    key: char,
    translations: Vec<KeyChar>,
}

static DICTIONARY: Lazy<Vec<Keys>> = Lazy::new(|| {
    // const DICTIONARY_PATH: &str = "~/dictionary.json";
    // let file = File::open(DICTIONARY_PATH);
    // let var: Vec<Keys> = serde_json::from_reader(file.unwrap()).unwrap();

    let var: Vec<Keys> = serde_json::from_str(include_str!("dictionary.json")).unwrap();

    var
});

pub fn translate(text: String, from: Language, to: Language) -> String {
    let mut str = String::with_capacity(text.capacity());

    text.to_lowercase().chars().for_each(|c| {
        if let Some(key) = DICTIONARY.iter().find(|x| {
            x.translations
                .iter()
                .find(|&t| t.language == from && t.symbol == c)
                .is_some()
        }) {
            if let Some(a) = key.translations.iter().find(|x| x.language == to) {
                str.push(a.symbol)
            } else {
                str.push(c);
            }
        } else {
            str.push(c);
        }
    });

    str
}
