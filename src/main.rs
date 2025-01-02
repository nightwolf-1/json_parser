use std::fs;
use std::fmt;
use std::env;

enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}                            

struct JsonParser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> JsonParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    fn parse(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        match self.peek_char() {
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('"') => self.parse_string(),
            Some(c) if c.is_digit(10) || c == '-' => self.parse_number(),
            Some('t') | Some('f') => self.parse_boolean(),
            Some('n') => self.parse_null(),
            _ => Err("Invalid JSON format".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.consume_char(); // Consume '{'
        let mut object = Vec::new();
        loop {
            self.skip_whitespace();
            if self.peek_char() == Some('}') {
                self.consume_char(); // Consume '}'
                break;
            }
    
            // Parse key as a String
            let key = if let JsonValue::String(key) = self.parse_string()? {
                key
            } else {
                return Err("Expected a string key".to_string());
            };
    
            self.skip_whitespace();
            if self.consume_char() != Some(':') {
                return Err("Expected ':' after key".to_string());
            }
            self.skip_whitespace();
            
            let value = self.parse()?;
            object.push((key, value));
            
            self.skip_whitespace();
            match self.peek_char() {
                Some(',') => {
                    self.consume_char();
                }
                Some('}') => {
                    self.consume_char();
                    break;
                }
                _ => return Err("Expected ',' or '}'".to_string()),
            }
        }
        Ok(JsonValue::Object(object))
    }
    
    fn parse_array(&mut self) -> Result<JsonValue, String> {
        self.consume_char();
        let mut array = Vec::new();
        loop {
            self.skip_whitespace();
            if self.peek_char() == Some(']') {
                self.consume_char();
                break;
            }
            let value = self.parse()?;
            array.push(value);
            self.skip_whitespace();
            match self.peek_char() {
                Some(',') => {
                    self.consume_char();
                }
                Some(']') => {
                    self.consume_char();
                    break;
                }
                _ => return Err("Expected ',' or ']'".to_string()),
            }
        }
        Ok(JsonValue::Array(array))
    }

    fn parse_string(&mut self) -> Result<JsonValue, String> {
        self.consume_char();
        let start = self.position;
        while let Some(c) = self.peek_char() {
            if c == '"' {
                let s = &self.input[start..self.position];
                self.consume_char();
                return Ok(JsonValue::String(s.to_string()));
            }
            self.consume_char();
        }
        Err("Unterminated string".to_string())
    }

    fn parse_number(&mut self) -> Result<JsonValue, String> {
        let start = self.position;
        while let Some(c) = self.peek_char() {
            if !c.is_digit(10) && c != '.' && c != '-' {
                break;
            }
            self.consume_char();
        }
        let number: f64 = self.input[start..self.position]
            .parse()
            .map_err(|_| "Invalid number format".to_string())?;
        Ok(JsonValue::Number(number))
    }

    fn parse_boolean(&mut self) -> Result<JsonValue, String> {
        if self.input[self.position..].starts_with("true") {
            self.position += 4;
            Ok(JsonValue::Boolean(true))
        } else if self.input[self.position..].starts_with("false") {
            self.position += 5;
            Ok(JsonValue::Boolean(false))
        } else {
            Err("Invalid boolean".to_string())
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, String> {
        if self.input[self.position..].starts_with("null") {
            self.position += 4;
            Ok(JsonValue::Null)
        } else {
            Err("Invalid null".to_string())
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.consume_char();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn consume_char(&mut self) -> Option<char> {
        let mut iter = self.input[self.position..].char_indices();
        let (_, c) = iter.next()?;
        let (next_index, _) = iter.next().unwrap_or((1, ' '));
        self.position += next_index;
        Some(c)
    }
}

impl JsonValue {
    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        if let JsonValue::Array(arr) = self {
            arr.get(index)
        } else {
            None
        }
    }

    pub fn get(&self, key_path: &str) -> Option<&JsonValue> {
        let keys: Vec<&str> = key_path.split('.').collect();
        let mut current_value = self;

        for key in keys {
            if let JsonValue::Object(obj) = current_value {
                if let Some((_, value)) = obj.iter().find(|(k, _)| k == key) {
                    current_value = value;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        Some(current_value)
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let reset = "\x1b[0m";
        let color_key = "\x1b[32m";    
        let color_string = "\x1b[33m";
        let color_number = "\x1b[34m";
        let color_boolean = "\x1b[35m";
        let color_null = "\x1b[31m";
        let color_bracket = "\x1b[36m";

        fn format_json(
            value: &JsonValue,
            indent: usize,
            colors: (&str, &str, &str, &str, &str, &str, &str),
        ) -> String {
            let (reset, color_key, color_string, color_number, color_boolean, color_null, color_bracket) = colors;
            let padding = " ".repeat(indent);
            let inner_padding = " ".repeat(indent + 4);

            match value {
                JsonValue::Object(obj) => {
                    let mut entries = Vec::new();
                    for (key, value) in obj {
                        let entry = format!(
                            "{}{}\"{}\"{}: {}",
                            inner_padding,
                            color_key,
                            key,
                            reset,
                            format_json(value, indent + 4, colors)
                        );
                        entries.push(entry);
                    }
                    format!(
                        "{}{{\n{}\n{}{}}}{}{}",
                        color_bracket,
                        entries.join(",\n"),
                        padding,
                        color_bracket,
                        color_bracket,
                        reset
                    )
                }
                JsonValue::Array(arr) => {
                    let entries: Vec<String> = arr
                        .iter()
                        .map(|v| format!("{}{}", inner_padding, format_json(v, indent + 4, colors)))
                        .collect();
                    format!(
                        "{}[\n{}\n{}{}{}]{}",
                        color_bracket,
                        entries.join(",\n"),
                        padding,
                        reset,
                        color_bracket,
                        reset
                    )
                }
                JsonValue::String(s) => format!("{}\"{}\"{}", color_string, s, reset),
                JsonValue::Number(n) => format!("{}{}{}", color_number, n, reset),
                JsonValue::Boolean(b) => format!("{}{}{}", color_boolean, b, reset),
                JsonValue::Null => format!("{}null{}", color_null, reset),
            }
        }

        let colors = (reset, color_key, color_string, color_number, color_boolean, color_null, color_bracket);
        write!(f, "{}", format_json(self, 0, colors))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} 'file' or {} 'file' 'key' or {} 'file' 'key' of array, 'index' of array or {} 'file' 'key.subkey'", &args[0], &args[0], &args[0], &args[0] );
    } else {
        // Lecture du fichier
        let file_path = &args[1];
        let mut key = None;
        let mut index = None;
        if args.len() == 3{
            key = Some(&args[2]);    
        }
        if args.len() == 4 {
            key = Some(&args[2]);
            index = Some(args[3].parse::<usize>().ok());
        }

        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Can't read file {}: {}", file_path, err);
                return;
            }
        };

        let mut parser = JsonParser::new(&content);
        match parser.parse() {
            Ok(value) => {
                if let Some(k) = key {
                    if let Some(v) = value.get(k) {
                        // Si un index est fourni
                        if let Some(idx) = index {
                            if let Some(item) = v.get_index(idx.unwrap() - 1) {
                                println!("{}", item);
                            } else {
                                println!("Element not found in '{}' in index {}.\nThe json file:\n {}",k, idx.unwrap(), value);
                            }
                        } else {
                            println!("{}", v);
                        }
                    } else {
                        println!("Key '{}' not found.\nThe json file: \n{}", k, value);
                    }
                } else {
                    println!("{}", value);
                }
            }
            Err(err) => {
                eprintln!("Error parsing JSON: {}", err);
            }
        }        
    }
}