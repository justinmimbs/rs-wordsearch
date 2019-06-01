use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;
use std::iter::FromIterator;
use std::str::FromStr;

// Dict

#[derive(Debug, PartialEq)]
pub struct Dict {
    end: bool,
    next: Map<char, Dict>,
}

impl Dict {
    pub fn new() -> Dict {
        Dict {
            end: false,
            next: Map::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) -> &Dict {
        let mut dict: &mut Dict = self;
        for c in word.chars() {
            dict = dict.next.entry(c).or_insert_with(|| Dict::new());
        }
        dict.end = true;
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
struct Graph(Map<u32, Set<u32>>);

impl Graph {
    fn add_directed_edge(&mut self, x: u32, y: u32) -> &Graph {
        self.0.entry(x).or_insert_with(|| Set::new()).insert(y);
        self
    }

    fn add_edge(&mut self, x: u32, y: u32) -> &Graph {
        self.add_directed_edge(x, y);
        self.add_directed_edge(y, x);
        self
    }

    fn grid(width: u32, height: u32) -> Graph {
        let mut graph = Graph(Map::new());

        for n in 0..(width * height - 1) {
            let right = (n + 1) % width != 0;
            let down = (n / width) + 1 < height;
            let left = n % width != 0;

            if right {
                graph.add_edge(n, n + 1);
            }
            if right && down {
                graph.add_edge(n, n + 1 + width);
            }
            if down {
                graph.add_edge(n, n + width);
            }
            if down && left {
                graph.add_edge(n, n - 1 + width);
            }
        }

        graph
    }
}

// Board

#[derive(Debug, PartialEq)]
pub struct Board {
    grid: Graph,
    chars: Map<u32, char>,
}

pub type Path = Vec<u32>;

impl Board {
    pub fn search(&self, dict: &Dict) -> Vec<Path> {
        let path = vec![];
        self.grid
            .0
            .keys()
            .flat_map(|&pos| self.search_step(dict, &path, pos))
            .collect()
    }

    fn search_step(&self, dict: &Dict, path: &Path, pos: u32) -> Vec<Path> {
        match dict.next.get(self.chars.get(&pos).unwrap()) {
            Some(dict_here) => {
                let mut path_here = path.clone();
                path_here.push(pos);

                let mut moves = self.grid.0.get(&pos).unwrap().clone();
                for visited in path_here.iter() {
                    moves.remove(visited);
                }

                // TODO compare performance
                // let moves: Vec<u32> = self
                //     .grid
                //     .0
                //     .get(&pos)
                //     .unwrap()
                //     .iter()
                //     .filter(|next| !path_here.contains(next))
                //     .cloned()
                //     .collect();

                let mut results: Vec<Path> = moves
                    .iter()
                    .flat_map(|&next| self.search_step(&dict_here, &path_here, next))
                    .collect();

                if dict_here.end {
                    results.push(path_here);
                }
                results
            }
            None => vec![],
        }
    }

    pub fn path_to_word(&self, path: &Path) -> String {
        path.iter()
            .map(|pos| self.chars.get(pos).unwrap_or(&'?'))
            .collect()
    }
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
            .collect(),
        };

        assert_eq!(board, Ok(expected));
    }

    #[test]
    fn from_str_err() {
        assert!("a b c".parse::<Board>().is_err());
        assert!("abc".parse::<Board>().is_err());
        assert!("abc de fghi".parse::<Board>().is_err());
    }

    #[test]
    fn search() {
        let dict: Dict = [
            "an", "and", "ant", "anti", "bad", "banana", "bat", "bot", "boy",
        ]
        .iter()
        .map(|s| *s)
        .collect();

        let board = Board {
            grid: Graph::grid(2, 2),
            chars: vec![(0, 'b'), (1, 'a'), (2, 't'), (3, 'n')]
                .into_iter()
                .collect(),
        };

        let mut paths = board.search(&dict);
        paths.sort();

        let expected = vec![vec![0, 1, 2], vec![1, 3], vec![1, 3, 2]];

        assert_eq!(paths, expected);
    }
}

// test Graph

#[cfg(test)]
mod test_graph {
    use super::*;

    #[test]
    fn grid_3x3() {
        let graph = Graph::grid(3, 3);

        let expected = Graph(
            vec![
                (0, vec![1, 3, 4].into_iter().collect()),
                (1, vec![0, 2, 3, 4, 5].into_iter().collect()),
                (2, vec![1, 4, 5].into_iter().collect()),
                (3, vec![0, 1, 4, 6, 7].into_iter().collect()),
                (4, vec![0, 1, 2, 3, 5, 6, 7, 8].into_iter().collect()),
                (5, vec![1, 2, 4, 7, 8].into_iter().collect()),
                (6, vec![3, 4, 7].into_iter().collect()),
                (7, vec![3, 4, 5, 6, 8].into_iter().collect()),
                (8, vec![4, 5, 7].into_iter().collect()),
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(graph, expected);
    }
}

// test Dict

#[cfg(test)]
mod test_dict {
    use super::*;

    fn make_dict(end: bool, list: Vec<(char, Dict)>) -> Dict {
        Dict {
            end,
            next: list.into_iter().collect(),
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
