use std::{cell::RefCell, rc::Rc};

const R: usize = 31; // A-Z, CL=[, ER=\, IN=], QU=^, TH=_
const CL: usize = 26;
const ER: usize = 27;
const IN: usize = 28;
const QU: usize = 29;
const TH: usize = 30;

const SCORE: [u8; R] = [
    2, 8, 8, 5, 2, 6, 6, 7, 2, 13, 8, 3, 5, 5, 2, 6, 15, 5, 3, 3, 4, 11, 10, 12, 4, 14, 10, 7, 7,
    9, 9,
];

#[derive(Debug)]
struct Node {
    next: [Option<Rc<RefCell<Node>>>; R],
    score: u8,
}
impl Node {
    fn new() -> Self {
        Self {
            next: Default::default(), // Default array initialization only works up to size 32
            score: 0,
        }
    }

    fn is_string(&self) -> bool {
        self.score > 0
    }

    fn add(&mut self, c: usize) -> &Option<Rc<RefCell<Node>>> {
        let is_none = self.next[c].is_none();
        if is_none {
            self.next[c] = Some(Rc::new(RefCell::new(Node::new())));
        }
        &self.next[c]
    }

    fn get(&self, c: char) -> &Option<Rc<RefCell<Node>>> {
        &self.next[c as usize - 65]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum KeyState {
    PREFIX,     // The Key is a valid prefix in the trie but not a stored string
    STRING(u8), // The Key is a string in the trie
    NEITHER,    // The Key is neither a prefix nor a string
}

#[derive(Debug)]
pub struct TrieSet {
    root: Option<Rc<RefCell<Node>>>,
    n: usize, //number of keys in the trie
}

impl TrieSet {
    pub fn new() -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(Node::new()))),
            n: 0,
        }
    }

    /// Does the set contain the given key?
    pub fn contains(&self, key: &Vec<char>) -> KeyState {
        let mut node = self.root.as_ref().unwrap().clone();
        let mut d = 0;
        loop {
            if let Some(n) = node.clone().borrow().get(key[d]) {
                if d == key.len() - 1 {
                    return match n.borrow().score {
                        0 => KeyState::PREFIX,
                        x => KeyState::STRING(x),
                    };
                }
                node = n.clone();
                d += 1;
            } else {
                return KeyState::NEITHER;
            }
        }
    }

    fn add_from_pos(
        &mut self,
        node: &Option<Rc<RefCell<Node>>>,
        key: &Vec<char>,
        pos: usize,
        score: u8,
    ) {
        if pos < key.len() - 1 {
            if let Some(double_card) = match key[pos..pos + 2] {
                ['C', 'L'] => Some(CL),
                ['E', 'R'] => Some(ER),
                ['I', 'N'] => Some(IN),
                ['Q', 'U'] => Some(QU),
                ['T', 'H'] => Some(TH),
                _ => None,
            } {
                let mut new_node = node.as_ref().unwrap().borrow_mut();
                self.add_from_pos(
                    new_node.add(double_card),
                    key,
                    pos + 2,
                    score + SCORE[double_card],
                );
            }
        }
        if pos < key.len() {
            let mut new_node = node.as_ref().unwrap().borrow_mut();
            let c = key[pos] as usize - 65;
            self.add_from_pos(new_node.add(c), key, pos + 1, score + SCORE[c]);
        } else if !node.as_ref().unwrap().borrow().is_string() {
            self.n += 1;
            node.as_ref().unwrap().borrow_mut().score = score;
        }
    }

    /// Adds all permutations of the string accounting for the double letter cards.
    pub fn add_perms(&mut self, key: String) {
        let chars = key.chars().collect();
        let node = self.root.as_ref().unwrap().clone();
        self.add_from_pos(&Some(node), &chars, 0, 0);
    }

    pub fn size(&self) -> usize {
        self.n
    }
}

impl Default for TrieSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perms() -> anyhow::Result<()> {
        let mut t = TrieSet::new();
        t.add_perms("INQUIRING".to_string());
        assert_eq!(t.n, 8);
        let key = "INQUIRING".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING(46));
        let key = "INQ".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::PREFIX);
        let key = "PINQ".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::NEITHER);
        let key = "INQP".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::NEITHER);
        // A-Z, CL=[, ER=\, IN=], QU=^, TH=_
        let key = "]^IR]G".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING(36));
        let key = "]^IRING".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING(36));
        let key = "]QUIR]G".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING(46));
        let key = "IN^IR]G".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING(36));
        let key = "IN^IRING".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING(36));
        let key = "]QUIRING".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING(46));
        let key = "INQUIR]G".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING(46));
        Ok(())
    }
}
