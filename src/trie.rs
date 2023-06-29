use std::{cell::RefCell, rc::Rc};

const R: usize = 31; // A-Z, CL=[, ER=\, IN=], QU=^, TH=_
const CL: usize = 26;
const ER: usize = 27;
const IN: usize = 28;
const QU: usize = 29;
const TH: usize = 30;

#[derive(Debug)]
struct Node {
    next: [Option<Rc<RefCell<Node>>>; R],
    is_string: bool,
}
impl Node {
    fn new() -> Self {
        Self {
            next: Default::default(), // Default array initialization only works up to size 32
            is_string: false,
        }
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
    PREFIX,  // The Key is a valid prefix in the trie but not a stored string
    STRING,  // The Key is a string in the trie
    NEITHER, // The Key is neither a prefix nor a string
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
                    return match n.borrow().is_string {
                        true => KeyState::STRING,
                        false => KeyState::PREFIX,
                    };
                }
                node = n.clone();
                d += 1;
            } else {
                return KeyState::NEITHER;
            }
        }
    }

    fn add_from_pos(&mut self, node: &Option<Rc<RefCell<Node>>>, key: &Vec<char>, pos: usize) {
        if pos < key.len() - 1 {
            match key[pos..pos + 2] {
                ['C', 'L'] => {
                    let mut new_node = node.as_ref().unwrap().borrow_mut();
                    self.add_from_pos(new_node.add(CL), key, pos + 2);
                }
                ['E', 'R'] => {
                    let mut new_node = node.as_ref().unwrap().borrow_mut();
                    self.add_from_pos(new_node.add(ER), key, pos + 2);
                }
                ['I', 'N'] => {
                    let mut new_node = node.as_ref().unwrap().borrow_mut();
                    self.add_from_pos(new_node.add(IN), key, pos + 2);
                }
                ['Q', 'U'] => {
                    let mut new_node = node.as_ref().unwrap().borrow_mut();
                    self.add_from_pos(new_node.add(QU), key, pos + 2);
                }
                ['T', 'H'] => {
                    let mut new_node = node.as_ref().unwrap().borrow_mut();
                    self.add_from_pos(new_node.add(TH), key, pos + 2);
                }
                _ => {}
            }
        }
        if pos < key.len() {
            let mut new_node = node.as_ref().unwrap().borrow_mut();
            self.add_from_pos(new_node.add(key[pos] as usize - 65), key, pos + 1);
        } else if !node.as_ref().unwrap().borrow().is_string {
            self.n += 1;
            node.as_ref().unwrap().borrow_mut().is_string = true;
        }
    }

    /// Adds all permutations of the string accounting for the double letter cards.
    pub fn add_perms(&mut self, key: String) {
        let chars = key.chars().collect();
        let node = self.root.as_ref().unwrap().clone();
        self.add_from_pos(&Some(node), &chars, 0);
    }

    pub fn size(&self) -> usize {
        self.n
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
        assert_eq!(t.contains(&key), KeyState::STRING);
        let key = "INQ".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::PREFIX);
        let key = "PINQ".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::NEITHER);
        let key = "INQP".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::NEITHER);
        // A-Z, CL=[, ER=\, IN=], QU=^, TH=_
        let key = "]^IR]G".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING);
        let key = "]^IRING".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING);
        let key = "]QUIR]G".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING);
        let key = "IN^IR]G".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING);
        let key = "IN^IRING".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING);
        let key = "]QUIRING".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING);
        let key = "INQUIR]G".to_string().chars().collect();
        assert_eq!(t.contains(&key), KeyState::STRING);
        Ok(())
    }
}
