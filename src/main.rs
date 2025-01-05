use json_parser::JsonParser;
mod handle_arguments;

fn main() {
    let (content, key, index) = match handle_arguments::get_arguments() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // Parser le contenu JSON
    let mut parser = JsonParser::new(&content);
    match parser.parse() {
        Ok(json_value) => json_value.handle_json_logic(key.as_ref(), index),
        Err(err) => eprintln!("Error parsing JSON: {}", err),
    }
}
