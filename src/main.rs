mod libs;
mod utils;

use crate::libs::json_parser::{JsonParser,JsonValue};
use crate::utils::handle_arguments::{get_arguments,Command};

fn main() {
    match get_arguments() {
        Ok(command) => match command {
            Command::Parse(content, key, index) => {
                let mut parser = JsonParser::new(&content);
                match parser.parse() {
                    Ok(json_value) => {
                        if let Some(key) = key {
                            json_value.handle_json_logic(Some(&key), index);
                        } else {
                            println!("{}", json_value);
                        }
                    }
                    Err(e) => eprintln!("Error parsing JSON: {}", e),
                }
            }
            Command::Search(content, search_value) => {
                let mut parser = JsonParser::new(&content);
                match parser.parse() {
                    Ok(json_value) => {
                        let target_value = JsonValue::String(search_value.clone());
                        let (count, keys, indexes) = json_value.get_all_occurrences(&target_value);

                        println!("Found {} occurrences of '{}':", count, search_value);
                        for (i, key) in keys.iter().enumerate() {
                            match json_value.get(&key) {
                                Some(value) => {
                                    if let Some(index) = indexes[i] {
                                            println!(" - {} at index {} : {}", key, index, value);
                                    } else {
                                        println!(" - {} : {}", key, value);
                                    }
                                }
                                None => println!(" - {}: <not found>", key),
                            }
                        }
                    }
                    Err(e) => eprintln!("Error parsing JSON: {}", e),
                }
            }
        },
        Err(e) => eprintln!("{}", e),
    }
}
