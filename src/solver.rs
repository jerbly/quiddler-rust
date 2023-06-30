use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{self, BufRead},
};

use crate::trie::TrieSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CardWord {
    cards: Vec<char>,
    score: u8,
}

impl Display for CardWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:3} ", self.score).expect("Not written");
        for c in self.cards.iter() {
            match c {
                '[' => {
                    write!(f, "cl").expect("Not written");
                }
                '\\' => {
                    write!(f, "er").expect("Not written");
                }
                ']' => {
                    write!(f, "in").expect("Not written");
                }
                '^' => {
                    write!(f, "qu").expect("Not written");
                }
                '_' => {
                    write!(f, "th").expect("Not written");
                }
                _ => {
                    write!(f, "{}", c).expect("Not written");
                }
            };
        }
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Solver {
    pub dictionary: TrieSet,
}

impl Solver {
    pub fn new(filename: String) -> anyhow::Result<Self> {
        let file = File::open(filename)?;
        Ok(Self {
            dictionary: {
                let mut trie = TrieSet::new();
                for key in io::BufReader::new(file).lines().flatten() {
                    trie.add_perms(key);
                }
                trie
            },
        })
    }

    fn find_word(
        &self,
        cards_used: Vec<char>,
        cards_remaining: Vec<char>,
        collector: &mut HashSet<CardWord>,
    ) {
        for index in 0..cards_remaining.len() {
            let mut new_cards_used = cards_used.clone();
            let mut new_cards_remaining = cards_remaining.clone();
            new_cards_used.push(new_cards_remaining.remove(index));
            match self.dictionary.contains(&new_cards_used) {
                crate::trie::KeyState::PREFIX => {
                    self.find_word(new_cards_used, new_cards_remaining, collector);
                }
                crate::trie::KeyState::STRING(score) => {
                    collector.insert(CardWord {
                        cards: new_cards_used.clone(),
                        score,
                    });
                    self.find_word(new_cards_used, new_cards_remaining, collector);
                }
                crate::trie::KeyState::NEITHER => {}
            }
        }
    }

    pub fn find(&self, cards: String) -> Vec<CardWord> {
        let mut collector = HashSet::new();
        let key: Vec<char> = cards.chars().collect();
        for index in 0..cards.len() {
            let mut cards_used = vec![];
            let mut cards_remaining = key.clone();
            cards_used.push(cards_remaining.remove(index));
            self.find_word(cards_used, cards_remaining, &mut collector);
        }
        collector.into_iter().collect()
    }

    pub fn combos(&self, words: Vec<CardWord>) {
        todo!();
        // Sort the words by length so we can exit early while recursing
        // Start from each word in words, store an index into the Vec to avoid copying
        //   this is the combo e.g. [1,7,19]
        //   at each recursion, ask the cardword if it can be made from the cards remaining
        //     if yes, add to combo and remove cards from cards_remaining (be careful with duplicates)
        //   skip those already in the combo,
        //   stop when all the cardwords have said no or their len > cards_remaining or cards_remaining = 0
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::KeyState;

    use super::*;

    #[test]
    fn test_dict() -> anyhow::Result<()> {
        let solver = Solver::new("dictionary-sowpods.txt".to_string())?;
        println!("{:?}", solver.dictionary.size());
        let key = "INQUIRING".to_string().chars().collect();
        assert_eq!(solver.dictionary.contains(&key), KeyState::STRING(46));
        for w in solver.find("]QUIRINGXP".to_string()) {
            println!("{}", w);
        }
        Ok(())
    }
}
