mod convert;
mod map_parser;
mod utils;

use wasm_bindgen::prelude::*;

use std::error::Error;

use crate::convert::convert_to_mltt;
use crate::map_parser::{create_unicode_to_mltt_map, parse_content};
use crate::utils::set_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn convert(text_to_convert: &str, map_content: &str) -> String {
    set_panic_hook();
    let converted_text =
        convert_text(text_to_convert, map_content).unwrap_or_else(|_| String::from(""));
    converted_text
}

pub fn convert_text(text_to_convert: &str, map_content: &str) -> Result<String, Box<dyn Error>> {
    let content = parse_content(&map_content)?;
    let map = create_unicode_to_mltt_map(&content)?;
    let converted_text = convert_to_mltt(&text_to_convert, &map)?;
    Ok(converted_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_convert() -> Result<(), Box<dyn Error>> {
        assert_eq!(convert_text("abc", "a=a\nb=c\nc=d")?, "abb");
        Ok(())
    }
}
