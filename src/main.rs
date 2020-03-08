use std::env;
use std::error::Error;
use std::fs;

use unicode_to_mltt_converter::convert_text;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("Invalid arguments!");
        std::process::exit(1);
    }
    let map_file_path = &args[1];
    let map_content = fs::read_to_string(&map_file_path)?;
    let text_to_convert = &args[2];
    let converted_text = convert_text(&text_to_convert, &map_content)?;
    println!("{}", converted_text);

    Ok(())
}
