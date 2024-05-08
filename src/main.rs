#![allow(unused)]
use std::process;
use std::fs::File;
use std::io::Read;
use std::io;
use std::env::{args, Args};
use std::env;

use scanner::Scanner;

pub mod scanner;
pub mod error_formatting;
pub mod token;


// static mut HAD_ERROR: bool = false;
// static mut HAD_RUNTIME_ERROR: bool = false;

struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    pub fn new() -> Self { Interpreter{ had_error: false, }}


    pub fn get_input(mut args: Args) -> Option<Vec<u8>> { 
        if args.len() == 1 {
            if let Some(file_path) = args.next() {
                if let Ok(mut file) = File::open(file_path) {
                    Some(file.bytes());
                }
            } 
        }

        None
    }

    fn run(&mut self, bytes: Vec<u8>) -> Result<(), io::Error> {
        // let tokens: Vec<Token> = Token::scan_tokens_from_src(bytes);

        // if self.had_error == true { process::exit(65) }

        Ok(())
    }
}

fn main() -> Result<(), io::Error>{
    // let mut interp = Interpreter::new();
    // interp.run(Interpreter::get_input(env::args()).unwrap());

    Ok(())
}
