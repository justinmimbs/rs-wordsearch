use wordsearch::Dict;
use wordsearch::Graph;

fn main() {
    let mut dict = Dict::new();
    dict.add_word("and");
    dict.add_word("ant");

    println!("{:?}", dict);

    let graph = Graph::grid(2, 2);

    println!("{:?}", graph);
}
