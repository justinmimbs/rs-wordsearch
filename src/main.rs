mod dictionary;

use wordsearch::{Board, Dict};

fn main() {
    let dict: Dict = dictionary::WORDS.iter().map(|s| *s).collect();

    if let Ok(board) = "blu mar ten".parse::<Board>() {
        let paths = board.search(&dict);
        let words: Vec<String> = paths.iter().map(|path| board.path_to_word(path)).collect();

        println!("{:?}", words);
    }
}
