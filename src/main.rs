mod dictionary;

use std::env;
use std::process;
use wordsearch::{Board, Dict};

fn main() {
    let args = env::args();

    let board_str = args.skip(1).next().unwrap_or_else(|| {
        eprintln!("Argument error: missing argument");
        process::exit(1);
    });

    let board: Board = match board_str.parse() {
        Ok(board) => board,
        Err(message) => {
            eprintln!("Argument error: {}", message);
            process::exit(1);
        }
    };

    let dict: Dict = dictionary::WORDS.iter().map(|s| *s).collect();
    let paths = board.search(&dict);
    let words: Vec<String> = paths.iter().map(|path| board.path_to_word(path)).collect();

    println!("{:?}", words);
}
