use crate::solver::Solver;
use std::{env, time::Instant};

pub mod solver;
pub mod trie;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No cards given");
    }
    let now = Instant::now();
    let solver = Solver::new("dictionary-sowpods.txt".to_string())?;
    let elapsed = now.elapsed();
    println!("{} words in {:?}", solver.dictionary.size(), elapsed);

    let key = args[1].clone();
    let chars = key.chars().collect::<Vec<char>>();
    let mut pos = 0;
    let mut cards = vec![];
    while pos < chars.len() {
        if pos < chars.len() - 1 {
            if let Some(double_card) = match chars[pos..pos + 2] {
                ['c', 'l'] => Some('['),
                ['e', 'r'] => Some('\\'),
                ['i', 'n'] => Some(']'),
                ['q', 'u'] => Some('^'),
                ['t', 'h'] => Some('_'),
                _ => None,
            } {
                cards.push(double_card);
                pos += 1;
            } else {
                cards.push(chars[pos]);
            }
        } else {
            cards.push(chars[pos]);
        }
        pos += 1;
    }
    let now = Instant::now();
    let card_words = solver.find(&cards);
    let elapsed = now.elapsed();
    println!("{} words found in {:?}", &card_words.len(), elapsed);

    let now = Instant::now();
    let plays = solver.combos(card_words, &cards);
    let elapsed = now.elapsed();
    println!("combos found in {:?}", elapsed);
    println!("============================================================");
    for p in plays {
        println!("{}", p);
    }

    Ok(())
}
