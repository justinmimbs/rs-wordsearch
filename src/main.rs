use wordsearch::Board;
use wordsearch::Dict;

fn main() {
    let mut dict = Dict::new();
    dict.add_word("and");
    dict.add_word("ant");

    println!("{:?}", dict);

    let board = "ab cd".parse::<Board>();

    println!("{:?}", board);
}
