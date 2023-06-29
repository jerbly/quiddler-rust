use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::trie::TrieSet;

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
        Ok(())
    }
}
