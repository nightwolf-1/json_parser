pub use self::json_parser::JsonParser;
pub use self::json_parser::JsonValue;
pub mod handle_arguments;

pub mod json_parser {
    use std::fmt;

    pub enum JsonValue {
        Object(Vec<(String, JsonValue)>),
        Array(Vec<JsonValue>),
        String(String),
        Number(f64),
        Boolean(bool),
        Null,
    }

    pub struct JsonParser<'a> {
        pub input: &'a str,
        pub position: usize,
    }

    impl<'a> JsonParser<'a> {
        pub fn new(input: &'a str) -> Self {
            Self { input, position: 0 }
        }

        pub fn parse(&mut self) -> Result<JsonValue, String> {
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

        pub fn parse_object(&mut self) -> Result<JsonValue, String> {
            self.consume_char();
            let mut object = Vec::new();
            loop {
                self.skip_whitespace();
                if self.peek_char() == Some('}') {
                    self.consume_char();
                    break;
                }

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

        pub fn parse_array(&mut self) -> Result<JsonValue, String> {
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

        pub fn parse_string(&mut self) -> Result<JsonValue, String> {
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

        pub fn parse_number(&mut self) -> Result<JsonValue, String> {
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

        pub fn parse_boolean(&mut self) -> Result<JsonValue, String> {
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

        pub fn parse_null(&mut self) -> Result<JsonValue, String> {
            if self.input[self.position..].starts_with("null") {
                self.position += 4;
                Ok(JsonValue::Null)
            } else {
                Err("Invalid null".to_string())
            }
        }

        pub fn skip_whitespace(&mut self) {
            while let Some(c) = self.peek_char() {
                if c.is_whitespace() {
                    self.consume_char();
                } else {
                    break;
                }
            }
        }

        pub fn peek_char(&self) -> Option<char> {
            self.input[self.position..].chars().next()
        }

        pub fn consume_char(&mut self) -> Option<char> {
            let mut iter = self.input[self.position..].char_indices();
            let (_, c) = iter.next()?;
            let (next_index, _) = iter.next().unwrap_or((1, ' '));
            self.position += next_index;
            Some(c)
        }
    }

    impl JsonValue {
        pub fn to_lowercase(&self) -> JsonValue {
            match self {
                JsonValue::Object(obj) => {
                    let lowercased_obj = obj
                        .iter()
                        .map(|(key, value)| (key.clone(), value.to_lowercase()))
                        .collect();
                    JsonValue::Object(lowercased_obj)
                }
                JsonValue::Array(arr) => {
                    let lowercased_arr: Vec<JsonValue> =
                        arr.iter().map(|v| v.to_lowercase()).collect();
                    JsonValue::Array(lowercased_arr)
                }
                JsonValue::String(s) => JsonValue::String(s.to_lowercase()),
                JsonValue::Number(n) => JsonValue::Number(*n),
                JsonValue::Boolean(b) => JsonValue::Boolean(*b),
                JsonValue::Null => JsonValue::Null,
            }
        }

        pub fn get_all_occurrences(&self, target: &JsonValue) -> (usize, Vec<String>, Vec<Option<usize>>) {
            let mut count: usize = 0;
            let mut keys: Vec<String> = Vec::new();
            let mut indexes: Vec<Option<usize>> = Vec::new();

            fn search(
                json: &JsonValue,
                target: &JsonValue,
                path: String,
                count: &mut usize,
                keys: &mut Vec<String>,
                indexes: & mut Vec<Option<usize>>,
            ) {
                match json {
                    JsonValue::Object(obj) => {
                        for (key, value) in obj {
                            let new_path = if path.is_empty() {
                                key.clone()
                            } else {
                                format!("{}.{}", path, key)
                            };

                            if value.to_lowercase() == target.to_lowercase() {
                                *count += 1;
                                keys.push(new_path.clone());
                                indexes.push(None);
                            }

                            search(value, target, new_path, count, keys, indexes);
                        }
                    }
                    JsonValue::Array(arr) => {
                        for (i, value) in arr.iter().enumerate() {
                            let new_path = format!("{}", path);
                            if value.to_lowercase() == target.to_lowercase() {
                                *count += 1;
                                keys.push(new_path.clone());
                                indexes.push(Some(i));
                            }
                            search(value, target, new_path, count, keys, indexes);
                        }
                    }
                    _ => {}
                }
            }
            search(self, &target, String::new(), &mut count, &mut keys, &mut indexes);

            (count, keys, indexes)
        }

        pub fn handle_json_logic(&self, key: Option<&String>, index: Option<usize>) {
            if let Some(k) = key {
                if let Some(v) = self.get(k) {
                    if let Some(idx) = index {
                        if let Some(item) = v.get_index(idx - 1) {
                            println!("{}", item);
                        } else {
                            println!(
                                "Element not found in '{}' at index {}.\nThe JSON :\n {}",
                                k, idx, self
                            );
                        }
                    } else {
                        println!("{}", v);
                    }
                } else {
                    println!("Key '{}' not found.\nThe JSON file: \n{}", k, self);
                }
            } else {
                println!("{}", self);
            }
        }

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
                let (
                    reset,
                    color_key,
                    color_string,
                    color_number,
                    color_boolean,
                    color_null,
                    color_bracket,
                ) = colors;
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
                            .map(|v| {
                                format!("{}{}", inner_padding, format_json(v, indent + 4, colors))
                            })
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

            let colors = (
                reset,
                color_key,
                color_string,
                color_number,
                color_boolean,
                color_null,
                color_bracket,
            );
            write!(f, "{}", format_json(self, 0, colors))
        }
    }

    impl PartialEq for JsonValue {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (JsonValue::Object(a), JsonValue::Object(b)) => a == b,
                (JsonValue::Array(a), JsonValue::Array(b)) => a == b,
                (JsonValue::String(a), JsonValue::String(b)) => a == b,
                (JsonValue::Number(a), JsonValue::Number(b)) => (a - b).abs() < f64::EPSILON,
                (JsonValue::Boolean(a), JsonValue::Boolean(b)) => a == b,
                (JsonValue::Null, JsonValue::Null) => true,
                _ => false,
            }
        }
    }
}
