use std::env::args;
use std::fs::read;
use std::process::exit;
use std::collections::VecDeque;

use console::Term;

fn main() {
    let args: Vec<String> = args().collect();

    let path: &String = args.get(1).unwrap_or_else(|| {
        eprintln!("please provide filename");
        exit(1);
    });

    let contents_bytes: Vec<u8> = read(path).unwrap_or_else(|err| {
        eprintln!("failed to read file: {err}");
        exit(2);
    });

    let contents: String = String::from_utf8(contents_bytes).unwrap_or_else(|_e| {
        eprintln!("failed to decode {path}: is it utf-8?");
        exit(3);
    });

    let tokens: Vec<&str> = contents.split_whitespace().collect();

    let mut data: [u8; 65535] = [0u8; 65535];
    let mut ptr: u16 = 0;
    let mut curtok_i: usize = 0;
    let mut loop_stack: VecDeque<usize> = VecDeque::new();
    let term: Term = Term::stdout();

    while let Some(token) = tokens.get(curtok_i) {
        match token.to_lowercase().as_str() {
            "lohk" => { // <
                ptr -= 1;
            }
            "vome" => { // >
                ptr += 1;
            }
            "netra" => { // -
                data[ptr as usize] -= 1;
            }
            "ris" => {  // +
                data[ptr as usize] += 1;
            }
            "xata" => { // .
                print!("{}", data[ptr as usize] as char)
            }
            "fass" => { // ,
                data[ptr as usize] = term.read_char().unwrap_or('?') as u8;
            }
            "jahu" => { // [
                loop_stack.push_front(curtok_i);
            }
            "khra" => { // ]
                if data[ptr as usize] == 0 {
                    loop_stack.pop_front();
                } else {
                    curtok_i = *loop_stack.front().unwrap();
                }
            }
            &_ => {}
        }
        curtok_i += 1;
    }

    exit(0);
}
