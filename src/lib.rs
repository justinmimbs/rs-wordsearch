use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::str::FromStr;

type Map<T, U> = BTreeMap<T, U>;
type Set<T> = BTreeSet<T>;

// Dict

#[derive(Debug, PartialEq)]
pub struct Dict {
    end: bool,
    cont: Map<char, Dict>,
}

impl Dict {
    pub fn new() -> Dict {
        Dict {
            end: false,
            cont: Map::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) -> &Dict {
        let mut current: &mut Dict = self;
        for c in word.chars() {
            current = current.cont.entry(c).or_insert_with(|| Dict::new());
        }
        current.end = true;
        self
    }
}

impl<'a> FromIterator<&'a str> for Dict {
    fn from_iter<T>(words: T) -> Self
    where
        T: IntoIterator<Item = &'a str>,
    {
        let mut dict = Dict::new();
        for word in words {
            dict.add_word(word);
        }
        dict
    }
}

// Graph

#[derive(Debug, PartialEq)]
struct Graph(AdjMap);

type AdjMap = Map<u32, Set<u32>>;

fn add_edge(adj: &mut AdjMap, x: u32, y: u32) -> &AdjMap {
    adj.entry(x).or_insert_with(|| Set::new()).insert(y);
    adj
}

fn add_edges(adj: &mut AdjMap, x: u32, y: u32) -> &AdjMap {
    add_edge(adj, x, y);
    add_edge(adj, y, x);
    adj
}

impl Graph {
    fn grid(width: u32, height: u32) -> Graph {
        let mut adj = Map::new();

        for n in 1..(width * height) {
            let right = n % width != 0;
            let down = ((n - 1) / width) + 1 < height;
            let left = (n - 1) % width != 0;

            if right {
                add_edges(&mut adj, n, n + 1);
            }
            if right && down {
                add_edges(&mut adj, n, n + 1 + width);
            }
            if down {
                add_edges(&mut adj, n, n + width);
            }
            if down && left {
                add_edges(&mut adj, n, n - 1 + width);
            }
        }

        Graph(adj)
    }
}

// Board

#[derive(Debug, PartialEq)]
pub struct Board {
    grid: Graph,
    chars: Map<u32, char>,
}

impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<&str> = s.split_whitespace().collect();
        let height = rows.len();

        if height < 2 {
            return Err("must have at least two rows");
        }

        let width = rows[0].len();

        if !rows.iter().all(|row| row.len() == width) {
            return Err("all rows must be the same width");
        }
        if width < 2 {
            return Err("must have at least two columns");
        }

        let grid = Graph::grid(width as u32, height as u32);
        let chars: Map<u32, char> = rows
            .iter()
            .flat_map(|row| row.chars())
            .enumerate()
            .map(|(i, c)| (i as u32, c))
            .collect();

        Ok(Board { grid, chars })
    }
}

// test Board

#[cfg(test)]
mod test_board {
    use super::*;

    #[test]
    fn from_str_ok() {
        let board = "abc def ghi".parse::<Board>();

        let expected = Board {
            grid: Graph::grid(3, 3),
            chars: vec![
                (0, 'a'),
                (1, 'b'),
                (2, 'c'),
                (3, 'd'),
                (4, 'e'),
                (5, 'f'),
                (6, 'g'),
                (7, 'h'),
                (8, 'i'),
            ]
            .into_iter()
            .collect::<Map<u32, char>>(),
        };

        assert_eq!(board, Ok(expected));
    }

    #[test]
    fn from_str_err() {
        assert!("a b c".parse::<Board>().is_err());
        assert!("abc".parse::<Board>().is_err());
        assert!("abc de fghi".parse::<Board>().is_err());
    }
}

// test Graph

#[cfg(test)]
mod test_graph {
    use super::*;

    #[test]
    fn grid_3x3() {
        let graph = Graph::grid(3, 3);

        let expected: AdjMap = vec![
            (1, vec![2, 4, 5].into_iter().collect()),
            (2, vec![1, 3, 4, 5, 6].into_iter().collect()),
            (3, vec![2, 5, 6].into_iter().collect()),
            (4, vec![1, 2, 5, 7, 8].into_iter().collect()),
            (5, vec![1, 2, 3, 4, 6, 7, 8, 9].into_iter().collect()),
            (6, vec![2, 3, 5, 8, 9].into_iter().collect()),
            (7, vec![4, 5, 8].into_iter().collect()),
            (8, vec![4, 5, 6, 7, 9].into_iter().collect()),
            (9, vec![5, 6, 8].into_iter().collect()),
        ]
        .into_iter()
        .collect();

        assert_eq!(graph, Graph(expected));
    }
}

// test Dict

#[cfg(test)]
mod test_dict {
    use super::*;

    fn make_dict(end: bool, list: Vec<(char, Dict)>) -> Dict {
        Dict {
            end,
            cont: list.into_iter().collect(),
        }
    }

    #[test]
    fn add_words() {
        let dict: Dict = ["an", "and", "ant", "anti", "bad", "bat", "bot", "boy"]
            .iter()
            .map(|s| *s)
            .collect();

        let expected: Dict = make_dict(
            false,
            vec![
                (
                    'a',
                    make_dict(
                        false,
                        vec![(
                            'n',
                            make_dict(
                                true,
                                vec![
                                    ('d', make_dict(true, vec![])),
                                    ('t', make_dict(true, vec![('i', make_dict(true, vec![]))])),
                                ],
                            ),
                        )],
                    ),
                ),
                (
                    'b',
                    make_dict(
                        false,
                        vec![
                            (
                                'a',
                                make_dict(
                                    false,
                                    vec![
                                        ('d', make_dict(true, vec![])),
                                        ('t', make_dict(true, vec![])),
                                    ],
                                ),
                            ),
                            (
                                'o',
                                make_dict(
                                    false,
                                    vec![
                                        ('t', make_dict(true, vec![])),
                                        ('y', make_dict(true, vec![])),
                                    ],
                                ),
                            ),
                        ],
                    ),
                ),
            ],
        );

        assert_eq!(dict, expected);
    }
}
