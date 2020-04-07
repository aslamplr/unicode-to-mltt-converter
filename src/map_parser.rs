use std::collections::HashMap;
use std::error::Error;

pub fn parse_content(content: &str) -> Result<Vec<(&str, &str)>, Box<dyn Error>> {
    let content = content
        .split('\n')
        .filter(|x| !x.is_empty() && !x.starts_with('#') && x.contains('='))
        .map(|x| x.split('=').map(|s| s.trim()).fold(("", ""), |a, b| (a.1, b)))
        .filter(|x| !x.1.is_empty() && !x.0.is_empty())
        .collect::<Vec<_>>();
    Ok(content)
}

pub fn create_unicode_to_mltt_map(
    content: &[(&str, &str)],
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let map = content
        .iter()
        .map(|s| (s.1.to_owned(), s.0.to_owned()))
        .collect::<HashMap<_, _>>();
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parse_content() -> Result<(), Box<dyn Error>> {
        assert_eq!(parse_content("a=a")?, vec![("a", "a")]);
        Ok(())
    }

    #[test]
    fn complex_parse_content() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            parse_content(
                r#"# This is a comment string should not be parsed.
a=a
# b = b 
b = c
c = d
a
b
c
."#
            )?,
            vec![("a", "a"), ("b", "c"), ("c", "d")]
        );
        Ok(())
    }

    #[test]
    fn simple_create_unicode_to_mltt_map() -> Result<(), Box<dyn Error>> {
        let content = vec![("a", "a"), ("b", "c"), ("c", "d")];
        let mut expected_map: HashMap<String, String> = HashMap::new();
        expected_map.insert(String::from("a"), String::from("a"));
        expected_map.insert(String::from("c"), String::from("b"));
        expected_map.insert(String::from("d"), String::from("c"));
        assert_eq!(create_unicode_to_mltt_map(&content)?, expected_map);
        Ok(())
    }
}
