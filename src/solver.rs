use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{self, BufRead},
};

use crate::trie::{TrieSet, SCORE};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct CardWord {
    cards: Vec<char>,
    score: u8,
}

impl CardWord {
    /// if all the cards in this card_word are contained in hand, remove them
    /// and return the remaining hand
    fn remove_from(&self, hand: &[char]) -> Option<Vec<char>> {
        let mut hand_remaining = hand.to_owned();
        for c in &self.cards {
            let index = hand_remaining.iter().position(|&x| x == *c);
            match index {
                Some(i) => hand_remaining.remove(i),
                None => return None,
            };
        }
        Some(hand_remaining)
    }

    fn new(hand: &[char]) -> CardWord {
        Self {
            cards: hand.to_owned(),
            score: {
                let mut s = 0u8;
                for c in hand {
                    s += SCORE[*c as usize - 65];
                }
                s
            },
        }
    }
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
pub struct Play {
    description: String,
    words: Vec<CardWord>,
    remaining: CardWord,
    score: u8,
}

impl Play {
    fn new(description: &str, card_words: &Vec<&CardWord>, remaining: &CardWord) -> Self {
        let mut words = vec![];
        let mut score = 0;
        for cw in card_words {
            let w = *cw;
            words.push(w.clone());
            score += cw.score;
        }
        Self {
            description: description.to_string(),
            words,
            remaining: remaining.clone(),
            score,
        }
    }
}

impl Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:3} {}", self.score, self.description).expect("Not written");
        for c in &self.words {
            write!(f, "{}  ", c).expect("Not written");
        }
        writeln!(f).expect("Not written");
        writeln!(f, "Remaining: {}", self.remaining)
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

    pub fn find(&self, cards: &Vec<char>) -> Vec<CardWord> {
        let mut collector = HashSet::new();
        for index in 0..cards.len() {
            let mut cards_used = vec![];
            let mut cards_remaining = cards.clone();
            cards_used.push(cards_remaining.remove(index));
            self.find_word(cards_used, cards_remaining, &mut collector);
        }
        collector.into_iter().collect()
    }

    fn find_combos<'a>(
        card_words: &'a Vec<CardWord>,
        cards_remaining: Vec<char>,
        choices: Vec<&'a CardWord>,
        collector: &mut Vec<(Vec<&'a CardWord>, CardWord)>,
    ) {
        let mut finished = true;
        for card_word in card_words.iter() {
            if card_word.cards.len() > cards_remaining.len() {
                break;
            }
            if let Some(hand) = card_word.remove_from(&cards_remaining) {
                finished = false;
                let mut new_choices = choices.clone();
                new_choices.push(card_word);
                Self::find_combos(card_words, hand.clone(), new_choices.clone(), collector);
            }
        }
        if finished {
            collector.push((choices, CardWord::new(&cards_remaining)));
        }
    }

    pub fn combos(&self, mut card_words: Vec<CardWord>, cards: &[char]) -> Vec<Play> {
        // Sort the words by length so we can exit early while recursing
        card_words.sort_by(|a, b| a.cards.len().cmp(&b.cards.len()));
        // Start from each word in words,
        //   at each recursion, ask the cardword if it can be made from the cards remaining
        //     if yes, add to combo and remove cards from cards_remaining
        //   stop when all the cardwords have said no or their len > cards_remaining or cards_remaining = 0
        let mut collector = vec![];
        for card_word in card_words.iter() {
            let cards_remaining = card_word.remove_from(cards).unwrap();
            let choices = vec![card_word];
            Self::find_combos(&card_words, cards_remaining, choices, &mut collector);
        }

        // find the best scoring hands with the most words and the longest word
        collector.sort_by(|a, b| a.1.score.cmp(&b.1.score));

        let top_score = &collector[0];
        let mut most_words = &collector[0];
        let mut max = 0;
        for w in collector[0].0.iter() {
            if w.cards.len() > max {
                max = w.cards.len();
            }
        }
        let mut longest_word = (&collector[0], max);
        let mut longest_word_top = (&collector[0], max);

        for play in &collector {
            if play.0.len() > most_words.0.len() {
                most_words = play;
            }
            let mut max = 0;
            for w in play.0.iter() {
                if w.cards.len() > max {
                    max = w.cards.len();
                }
            }
            if max > longest_word.1 {
                longest_word = (play, max);
            }
            if max > longest_word_top.1 && play.1.score <= longest_word_top.0 .1.score {
                longest_word_top = (play, max);
            }
        }

        vec![
            Play::new("Top Score", &top_score.0, &top_score.1),
            Play::new("Most Words", &most_words.0, &most_words.1),
            Play::new("Longest Word", &longest_word.0 .0, &longest_word.0 .1),
            Play::new(
                "Longest Word with Top Score",
                &longest_word_top.0 .0,
                &longest_word_top.0 .1,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dict() -> anyhow::Result<()> {
        let solver = Solver::new("dictionary-sowpods.txt".to_string())?;
        println!("{:?}", solver.dictionary.size());
        let key = "]QUIRINGXP".to_string();
        let cards = key.chars().collect::<Vec<char>>();
        let card_words = solver.find(&cards);
        for p in solver.combos(card_words, &cards) {
            println!("{}", p);
        }
        Ok(())
    }
}
