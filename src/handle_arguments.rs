pub use self::handle_arguments::get_arguments;

pub mod handle_arguments {
    use std::env;
    use std::fs;
    use std::io::{self, Read};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

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

    pub fn get_arguments() -> Result<(String, Option<String>, Option<usize>), String> {
       
        let stdin_content = read_stdin_with_timeout(Duration::from_millis(100));

        let args: Vec<String> = env::args().collect();
        let key = if args.len() >= 2 { Some(args[1].clone()) } else { None };
        let index = if args.len() == 3 {
            args[2].parse::<usize>().ok()
        } else {
            None
        };

        if let Some(input) = stdin_content {
            return Ok((input, key, index));
        } else if args.len() >= 2 {
            let file_path = &args[1];
            let content = fs::read_to_string(file_path)
                .map_err(|err| format!("Error reading file {}: {}", file_path, err))?;
            let key = if args.len() >= 3 { Some(args[2].clone()) } else { None };
            let index = if args.len() == 4 {
                args[3].parse::<usize>().ok()
            } else {
                None
            };
            return Ok((content, key, index));
        };

        Err(format!(
            "Usage: <file> [key] [index] or <standard input> [key] [index] \nExamples:\n./json_parser data.json\n./json_parser data.json grades\n./json_parser data.json grades 2\n./json_parser data.json details.city\n./json_parser data.json details.city 1\ncat data.json | ./json_parser\ncat data.json | ./json_parser grades\ncat data.json | ./json_parser grades 2\ncat data.json | ./json_parser details.city\ncat data.json | ./json_parser details.city 1"
        ))
    }
}
