use std::rc::Rc;
use std::cell::RefCell;

use std::collections::HashMap;
struct TrieNode<K, U> {
  child: HashMap<K, Rc<RefCell<TrieNode<K, U>>>>,
  data: U,
}
use std::hash::Hash;

impl<K: Eq + Hash, U: Default> TrieNode<K, U> {
  fn new() -> TrieNode<K, U> {
    TrieNode {
      child: HashMap::new(),
      data: Default::default(),
    }
  }
}

pub struct Trie<K, U> {
  root: Rc<RefCell<TrieNode<K, U>>>,
}

impl<K: Eq + Hash + Copy, U: Default + Clone> Trie<K, U> {
  pub fn new() -> Trie<K, U> {
    Trie {
      root: Rc::new(RefCell::new(TrieNode::new())),
    }
  }
  pub fn cursor(&self) -> TrieCursor<K, U> {
    TrieCursor(self.root.clone())
  }
  pub fn insert(&self, mut path: impl Iterator<Item = K>, data: U) {
    let mut cursor = self.cursor();
    while let Some(k) = path.next() {
      cursor = cursor.to_child_or_insert_default(k);
    }
    cursor.set_data(data);
  }
  pub fn get(&self, mut path: impl Iterator<Item = K>) -> Option<U> {
    let mut cursor = self.cursor();
    while let Some(k) = path.next() {
      match cursor.to_child(&k) {
        Some(next_cursor) => cursor = next_cursor,
        None => return None,
      }
    }
    Some(cursor.get_data())
  }
}

pub struct TrieCursor<K, U>(Rc<RefCell<TrieNode<K, U>>>);
impl<K: Eq + Hash + Copy, U: Default + Clone> TrieCursor<K, U> {
  fn to_child_or_insert_default(&self, k: K) -> TrieCursor<K, U> {
    self.init_child(k);
    self.to_child(&k).unwrap()
  }
  pub fn to_child(&self, k: &K) -> Option<TrieCursor<K, U>> {
    self.0.borrow().child.get(k).map(|c| TrieCursor(Rc::clone(c)))
  }
  fn init_child(&self, key: K) {
    self.0.borrow_mut().child.entry(key).or_insert(Rc::new(RefCell::new(TrieNode::new())));
  }
  pub fn set_data(&self, data: U) {
    self.0.borrow_mut().data = data;
  }
  pub fn get_data(&self) -> U {
    self.0.borrow().data.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let trie: Trie<char, bool> = Trie::new();
    trie.insert("to".chars(), true);
    trie.insert("A".chars(), true);
    trie.insert("tea".chars(), true);
    trie.insert("ted".chars(), true);
    trie.insert("ten".chars(), true);
    trie.insert("inn".chars(), true);

    let not_exist_words = vec!["t", "tod", "B", "todd", "te", "teab", "in", "innnn"];
    for word in not_exist_words {
      assert!(!trie.get(word.chars()).unwrap_or(false));
    }
    let exist_words = vec!["to", "A", "tea", "ted", "ten", "inn"];
    for word in exist_words {
      assert!(trie.get(word.chars()).unwrap_or(false));
    }
  }
}