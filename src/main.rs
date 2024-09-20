use clap::{Arg, Command};

fn text_to_hex(text: &str) -> String {
    text.chars()
        .map(|c| format!("{:X}", c as u32))
        .collect::<Vec<String>>()
        .join(" ")
}

fn hex_to_text(hex: &str) -> String {
    hex.split_whitespace()
        .map(|h| {
            let code = u32::from_str_radix(h, 16).unwrap();
            char::from_u32(code).unwrap_or('?')
        })
        .collect()
}

fn main() {
    let matches = Command::new("Text and Hex Converter")
        .arg(Arg::new("text")
            .help("Text to convert")
            .required(true)
            .index(1))
        .arg(Arg::new("mode")
            .help("Mode of conversion")
            .required(true)
            .index(2))
        .get_matches();

    let text = matches.get_one::<String>("text").unwrap();
    let mode = matches.get_one::<String>("mode").unwrap();

    match mode.as_str() {
        "Text2Hex" => {
            let hex = text_to_hex(text);
            println!("Hex: {}", hex);
        },
        "Hex2Text" => {
            let text = hex_to_text(text);
            println!("Text: {}", text);
        },
        _ => {
            eprintln!("Invalid mode. Use 'Text2Hex' or 'Hex2Text'.");
        }
    }
}
