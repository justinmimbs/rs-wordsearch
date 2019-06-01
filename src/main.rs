use wordsearch::Board;
use wordsearch::Dict;

fn main() {
    let dict: Dict = ["an", "and", "ant", "anti", "bad", "bat", "bot", "boy"]
        .iter()
        .map(|s| *s)
        .collect();

    println!("{:?}", dict);

    if let Ok(board) = "ba tn".parse::<Board>() {
        println!("{:?}", board);

        let paths = board.search(&dict);
        println!("{:?}", paths);
    }
}
