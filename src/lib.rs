mod utils;

use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::error::Error;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn convert(text_to_convert: &str, map_content: &str) -> String {
    let content = parse_content(&map_content).unwrap();
    let map = create_unicode_to_mltt_map(&content).unwrap();
    let converted_text = convert_to_mltt(&text_to_convert, &map).unwrap();
    converted_text
}

pub fn convert_to_mltt(
    text_to_convert: &str,
    map: &HashMap<String, String>,
) -> Result<String, Box<dyn Error>> {
    let mut text_to_convert = text_to_convert.to_owned();
    let mut keys = map.keys().map(String::from).collect::<Vec<_>>();
    keys.sort_by(|a, b| b.len().cmp(&a.len()));
    let to_string_vec = |x: Vec<&str>| x.into_iter().map(String::from).collect::<Vec<_>>();
    let _right_combinators = to_string_vec(vec!["ാ", "ി", "ീ", "ു", "ൂ", "ൃ", "ൄ", "ൌ", "്"]);
    let left_combinators = to_string_vec(vec!["െ", "േ", "ൈ"]);
    let combinators = to_string_vec(vec!["ൊ", "ോ"]);

    for key in combinators {
        if let Some(value) = map.get(&key) {
            while let Some(index) = text_to_convert.find(&key) {
                let middle_char = text_to_convert[index - 3..].chars().next().unwrap();
                let mid_val = map.get(&format!("{}", middle_char)).unwrap();
                let split_val = value.split("").collect::<Vec<_>>();
                let new_key = format!("{}{}", middle_char, key);
                let new_val = format!("{}{}{}", split_val[1], mid_val, split_val[2]);
                text_to_convert = text_to_convert.replace(&new_key, &new_val);
            }
        }
    }

    for key in left_combinators {
        if let Some(value) = map.get(&key) {
            while let Some(index) = text_to_convert.find(&key) {
                let right_char = text_to_convert[index - 3..].chars().next().unwrap();
                let right_val = map.get(&format!("{}", right_char)).unwrap();
                let new_key = format!("{}{}", right_char, key);
                let new_val = format!("{}{}", value, right_val);
                text_to_convert = text_to_convert.replace(&new_key, &new_val);
            }
        }else {
            eprintln!("{:#?} not found in the map!", key);
        }
    }

    let chillu_combinators = to_string_vec(vec!["ൺ", "ൻ", "ർ", "ൽ", "ൾ"]);

    for key in chillu_combinators {
        if let Some(value) = map.get(&key) {
            text_to_convert = text_to_convert.replace(&key, value);
        } else {
            eprintln!("{:#?} not found in the map!", key);
        }
    }

    for key in keys {
        if let Some(value) = map.get(&key) {
            // println!("{} –– {} : {}", text_to_convert, key, value);
            text_to_convert = text_to_convert.replace(&key, value);
        }else {
            eprintln!("{:#?} not found in the map!", key);
        }
    }
    
    Ok(text_to_convert)
}

pub fn parse_content(content: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let content = content
        .split("\n")
        .filter(|x| x.len() >= 1 && x.chars().nth(0) != Some('#') && x.contains("="))
        .map(|x| {
            x.split("=")
                .map(|s| s.trim())
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .filter(|x| x[1].len() >= 1 && x[0].len() >= 1)
        .collect::<Vec<_>>();
    Ok(content)
}

pub fn create_unicode_to_mltt_map(
    content: &Vec<Vec<String>>,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let map = content
        .into_iter()
        .map(|s| (s[1].to_owned(), s[0].to_owned()))
        .collect::<HashMap<_, _>>();
    Ok(map)
}


