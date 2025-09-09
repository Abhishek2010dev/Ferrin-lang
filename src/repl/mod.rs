use std::io::{self, Write};

use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("{PROMPT}");
        stdout.flush().unwrap();

        let mut line = String::new();
        let bytes_read = stdin.read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        }

        let l = Lexer::new(line.trim().to_string());

        for token in l {
            println!("{token:?}")
        }
    }
}
