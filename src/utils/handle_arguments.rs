    use std::env;
    use std::fs;
    use std::io::{self, Read};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub enum Command{
        Parse(String, Option<String>, Option<usize>),
        Search(String, String),
    }

    fn read_stdin_with_timeout(timeout: Duration) -> Option<String> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            if handle.read_to_string(&mut buffer).is_ok() {
                let _ = tx.send(buffer);
            }
        });
    
        match rx.recv_timeout(timeout) {
            Ok(input) => Some(input),
            Err(_) => None,
        }
    }

    pub fn get_arguments() -> Result<Command, String> {
        let stdin_content = read_stdin_with_timeout(Duration::from_millis(100));
    
        let args: Vec<String> = env::args().collect();
    
        if let Some(input) = stdin_content {
            if args.len() >= 2 && (args[1] == "-s" || args[1] == "--search") {
                if args.len() < 3 {
                    return Err("Search mode requires a value to search for.".to_string());
                }
                let search_value = args[2].clone();
                return Ok(Command::Search(input, search_value));
            } else {
                let key = if args.len() >= 2 { Some(args[1].clone()) } else { None };
                let index = if args.len() >= 3 {
                    args[2].parse::<usize>().ok()
                } else {
                    None
                };
                return Ok(Command::Parse(input, key, index));
            }
        }
    
        if args.len() >= 2 {
            if args[1] == "-s" || args[1] == "--search" {
                if args.len() < 3 {
                    return Err("Search mode requires a value to search for.".to_string());
                }
                let search_value = args[2].clone();
                let file_path = if args.len() > 3 { Some(&args[3]) } else { None };
                let content = if let Some(path) = file_path {
                    fs::read_to_string(path)
                        .map_err(|err| format!("Error reading file {}: {}", path, err))?
                } else {
                    return Err("No file provided for search mode, and no data found on stdin.".to_string());
                };
                return Ok(Command::Search(content, search_value));
            }
    
            let file_path = &args[1];
            let content = fs::read_to_string(file_path)
                .map_err(|err| format!("Error reading file {}: {}", file_path, err))?;
            let key = if args.len() >= 3 { Some(args[2].clone()) } else { None };
            let index = if args.len() == 4 {
                args[3].parse::<usize>().ok()
            } else {
                None
            };
            return Ok(Command::Parse(content, key, index));
        }
    
        Err(format!(
            "Usage: <file> [key] [index] or <standard input> [key] [index] \nExamples:\n\
            ./json_parser data.json\n\
            ./json_parser data.json grades\n\
            ./json_parser data.json grades 2\n\
            ./json_parser data.json details.city\n\
            ./json_parser data.json details.city 1\n\
            ./json_parser -s \"search_value\" data.json\n\
            cat data.json | ./json_parser -s \"search_value\"\n\
            cat data.json | ./json_parser grades\n\
            cat data.json | ./json_parser grades 2\n\
            cat data.json | ./json_parser details.city\n\
            cat data.json | ./json_parser details.city 1"
        ))
    } 