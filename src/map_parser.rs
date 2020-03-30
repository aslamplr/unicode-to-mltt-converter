use std::collections::HashMap;
use std::error::Error;

pub fn parse_content(content: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let content = content
        .split('\n')
        .filter(|x| !x.is_empty() && !x.starts_with('#') && x.contains('='))
        .map(|x| {
            x.split('=')
                .map(|s| s.trim())
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .filter(|x| !x[1].is_empty() && !x[0].is_empty())
        .collect::<Vec<_>>();
    Ok(content)
}

pub fn create_unicode_to_mltt_map(
    content: &[Vec<String>],
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let map = content
        .iter()
        .map(|s| (s[1].to_owned(), s[0].to_owned()))
        .collect::<HashMap<_, _>>();
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parse_content() -> Result<(), Box<dyn Error>> {
        assert_eq!(parse_content("a=a")?, vec![vec!["a", "a"]]);
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
            vec![vec!["a", "a"], vec!["b", "c"], vec!["c", "d"],]
        );
        Ok(())
    }

    fn get_parsed_content(dummy: Vec<Vec<&str>>) -> Vec<Vec<String>> {
        dummy
            .into_iter()
            .map(|x| x.into_iter().map(String::from).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    #[test]
    fn simple_create_unicode_to_mltt_map() -> Result<(), Box<dyn Error>> {
        let content = get_parsed_content(vec![vec!["a", "a"], vec!["b", "c"], vec!["c", "d"]]);
        let mut expected_map: HashMap<String, String> = HashMap::new();
        expected_map.insert(String::from("a"), String::from("a"));
        expected_map.insert(String::from("c"), String::from("b"));
        expected_map.insert(String::from("d"), String::from("c"));
        assert_eq!(create_unicode_to_mltt_map(&content)?, expected_map);
        Ok(())
    }
}
