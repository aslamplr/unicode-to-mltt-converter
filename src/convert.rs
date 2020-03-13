use std::collections::HashMap;
use std::error::Error;

fn get_prev_char(s: &str, i: usize) -> Option<char> {
    assert!(
        i < s.len() && i > 0,
        "i should not be zero index and should not exceed the len on str"
    );
    get_nth_char_from(&s, i, -1)
}

fn get_next_char(s: &str, i: usize) -> Option<char> {
    assert!(i < s.len(), "i should not exceed the len on str");
    get_nth_char_from(&s, i, 0)
}

fn get_nth_char_from(s: &str, i: usize, n: isize) -> Option<char> {
    let char_indices = s
        .char_indices()
        .filter(|(idx, _)| (n >= 0 && idx > &i) || (n < 0 && idx < &i))
        .collect::<Vec<_>>();
    let idx: usize = if n >= 0 {
        n as usize
    } else {
        (char_indices.len() as isize + n) as usize
    };
    if idx < char_indices.len() {
        Some(char_indices[idx].1)
    } else {
        None
    }
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
    let left_combinators = to_string_vec(vec!["െ", "േ", "ൈ", "്ര"]);
    let combinators = to_string_vec(vec!["ൊ", "ോ"]);

    let mut temp_map: HashMap<String, String> = HashMap::new();
    if text_to_convert.len() >= 9 {
        for (index, ch) in text_to_convert.char_indices() {
            if index >= 3 && index + 3 < text_to_convert.len() && ch == '്' {
                let left_char = get_prev_char(&text_to_convert, index);
                let right_char = get_next_char(&text_to_convert, index);
                if let (Some(left_char), Some(right_char)) = (left_char, right_char) {
                    let key = format!("{}്{}", left_char, right_char);
                    if let Some(value) = map.get(&key) {
                        temp_map.insert(key, value.to_string());
                    }
                }
            }
        }
    }

    for (key, value) in temp_map {
        text_to_convert = text_to_convert.replace(&key, &value);
    }

    for key in left_combinators {
        if let Some(value) = map.get(&key) {
            while let Some(index) = text_to_convert.find(&key) {
                if let Some(right_char) = get_prev_char(&text_to_convert, index) {
                    let right_char = right_char.to_string();
                    if "്ര" == format!("്{}", right_char) {
                        let prev_right_char = format!("്{}", right_char);
                        if let Some(right_char) = get_nth_char_from(&text_to_convert, index, -3) {
                            let prev_right_val = map.get(&prev_right_char.to_string()).unwrap();
                            let right_val = map.get(&right_char.to_string()).unwrap();
                            let new_key = format!("{}{}{}", right_char, prev_right_char, key);
                            let new_val = format!("{}{}{}", value, prev_right_val, right_val);
                            text_to_convert = text_to_convert.replace(&new_key, &new_val);
                        }
                    }
                    let right_val = map.get(&right_char).unwrap_or(&right_char);
                    let new_key = format!("{}{}", right_char, key);
                    let new_val = format!("{}{}", value, right_val);
                    text_to_convert = text_to_convert.replace(&new_key, &new_val);
                }
            }
        } else {
            eprintln!("{:#?} not found in the map!", key);
        }
    }

    for key in combinators {
        if let Some(value) = map.get(&key) {
            while let Some(index) = text_to_convert.find(&key) {
                if let Some(middle_char) = get_prev_char(&text_to_convert, index) {
                    let middle_char = middle_char.to_string();
                    let prev_char =
                        if let Some(prev_char) = get_nth_char_from(&text_to_convert, index, -2) {
                            prev_char
                        } else {
                            ' '
                        };
                    let raa_val = map.get("്ര").unwrap();
                    let raa_val = if raa_val == &prev_char.to_string() {
                        raa_val
                    } else {
                        ""
                    };
                    let mid_val = map.get(&middle_char).unwrap_or(&middle_char);
                    let mid_val = format!("{}{}", raa_val, mid_val);
                    let middle_char = format!("{}{}", raa_val, middle_char);
                    let split_val = value.split("").collect::<Vec<_>>();
                    let new_key = format!("{}{}", middle_char, key);
                    let new_val = format!("{}{}{}", split_val[1], mid_val, split_val[2]);
                    text_to_convert = text_to_convert.replace(&new_key, &new_val);
                }
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
