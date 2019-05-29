use std::collections::BTreeMap;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct Dict {
    end: bool,
    cont: BTreeMap<char, Dict>,
}

impl Dict {
    pub fn new() -> Dict {
        Dict {
            end: false,
            cont: BTreeMap::new(),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dict(end: bool, list: Vec<(char, Dict)>) -> Dict {
        Dict {
            end,
            cont: list.into_iter().collect(),
        }
    }

    #[test]
    fn dict_add_words() {
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
