use std::collections::HashMap;
use std::error::Error;

pub fn convert_to_mltt(
    text_to_convert: &str,
    map: &HashMap<String, String>,
) -> Result<String, Box<dyn Error>> {
    let mut text_to_convert = text_to_convert.to_owned();
    let mut keys = map.keys().map(String::from).collect::<Vec<_>>();
    keys.sort_by(|a, b| b.len().cmp(&a.len()));
    let to_string_vec = |x: Vec<&str>| x.into_iter().map(String::from).collect::<Vec<_>>();
    let _right_combinators = to_string_vec(vec!["ാ", "ി", "ീ", "ു", "ൂ", "ൃ", "ൄ", "ൌ", "്"]);
    let left_combinators = to_string_vec(vec!["െ", "േ", "ൈ", "്ര"]);
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
        } else {
            eprintln!("{:#?} not found in the map!", key);
        }
    }

    for key in left_combinators {
        if let Some(value) = map.get(&key) {
            while let Some(index) = text_to_convert.find(&key) {
                let right_char = text_to_convert[index - 3..].chars().next().unwrap();
                if index >= 9 {
                    let prev_char = text_to_convert[index - 9..].chars().next().unwrap();
                    if right_char == prev_char {
                        let right_char = text_to_convert[index - 9..index]
                            .chars()
                            .collect::<String>();
                        let right_val = map.get(&format!("{}", right_char)).unwrap();
                        let new_key = format!("{}{}", right_char, key);
                        let new_val = format!("{}{}", value, right_val);
                        text_to_convert = text_to_convert.replace(&new_key, &new_val);
                        continue;
                    }
                    if "്ര" == format!("്{}", right_char) {
                        let prev_right_char = format!("്{}", right_char);
                        let right_char = text_to_convert[index - 9..].chars().next().unwrap();
                        let prev_right_val = map.get(&format!("{}", prev_right_char)).unwrap();
                        let right_val = map.get(&format!("{}", right_char)).unwrap();
                        let new_key = format!("{}{}{}", right_char, prev_right_char, key);
                        let new_val = format!("{}{}{}", value, prev_right_val, right_val);
                        text_to_convert = text_to_convert.replace(&new_key, &new_val);
                    }
                }
                let right_val = map.get(&format!("{}", right_char)).unwrap();
                let new_key = format!("{}{}", right_char, key);
                let new_val = format!("{}{}", value, right_val);
                text_to_convert = text_to_convert.replace(&new_key, &new_val);
            }
        } else {
            eprintln!("{:#?} not found in the map!", key);
        }
    }

    for key in keys {
        if let Some(value) = map.get(&key) {
            text_to_convert = text_to_convert.replace(&key, value);
        }
    }
    Ok(text_to_convert)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map_parser::{create_unicode_to_mltt_map, parse_content};

    fn generate_map(content: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let content = parse_content(&content)?;
        create_unicode_to_mltt_map(&content)
    }

    #[test]
    fn simple_convert() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            convert_to_mltt("abc", &generate_map("a=a\nb=c\nc=d")?)?,
            "abb"
        );
        Ok(())
    }
}
