use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::hash_map::Entry;

struct Node {
    nodes: HashMap<char, Rc<RefCell<Node>>>,
    is_final: bool,
}

impl Default for Node {
    fn default() -> Self {
        Node { nodes: HashMap::new(), is_final: false }
    }
}

impl Node {
    fn new() -> Self {
        Node::default()
    }
}

pub struct Trie {
    root: Rc<RefCell<Node>>
}

impl Default for Trie {
    fn default() -> Self {
        Trie { root: Rc::new(RefCell::new(Node::new())) }
    }
}

fn to_chars(str: &str) -> Vec<char> {
    let mut chars: Vec<char> = Vec::new();
    for char in str.chars() {
        chars.push(char);
    }
    chars
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    #[allow(dead_code)]
    pub fn add(&mut self, str: &str) {
        Trie::add_to_node(&to_chars(str), self.root.clone(), 0)
    }

    fn add_to_node(chars: &[char], node: Rc<RefCell<Node>>, idx: usize) {
        if idx >= chars.len() {
            return;
        }
        let char = chars[idx];

        let mut ref_mut_node = node.borrow_mut();
        let entry = ref_mut_node.nodes.entry(char);

        let next_node = match entry {
            Entry::Occupied(occupied_entry) => {
                occupied_entry.get().clone()
            }
            Entry::Vacant(vacant_entry) => {
                let new_node = Rc::new(RefCell::new(Node::new()));
                vacant_entry.insert(new_node.clone());
                new_node
            }
        };

        if idx == chars.len() - 1 {
            next_node.borrow_mut().is_final = true;
        }

        Trie::add_to_node(chars, next_node, idx + 1)
    }

    pub fn contains(&self, str: &str) -> bool {
        Trie::contains_from_node(&to_chars(str), self.root.clone(), 0)
    }

    fn contains_from_node(chars: &[char], node: Rc<RefCell<Node>>, idx: usize) -> bool {
        if idx >= chars.len() {
            return false;
        }

        let char = chars[idx];

        let ref_node = node.borrow();
        let next_node = ref_node.nodes.get(&char);

        if next_node.is_none() {
            return false;
        }
        let next_node = next_node.unwrap().clone();
        if idx == chars.len() - 1 && next_node.borrow().is_final {
            return true;
        }

        Trie::contains_from_node(chars, next_node, idx + 1)
    }
}

#[test]
fn test() {
    let mut trie = Trie::new();

    trie.add("cat");
    trie.add("can");
    trie.add("cast");
    trie.add("boost");

    assert_eq!(trie.contains("cat"), true);
    assert_eq!(trie.contains("can"), true);
    assert_eq!(trie.contains("cas"), false);
    assert_eq!(trie.contains("casting"), false);
    assert_eq!(trie.contains("cost"), false);
    assert_eq!(trie.contains("boost"), true);
    assert_eq!(trie.contains("but"), false);
    assert_eq!(trie.contains("bot"), false);
}