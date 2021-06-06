use crate::stack::Stack;
use std::{io::Empty, ops::Deref, slice::Iter, usize};

struct Node<T: Ord> {
    pub key: T,
    pub value: String,
    pub size: usize,
    pub left: Box<Option<Node<T>>>,
    pub right: Box<Option<Node<T>>>,
}

impl<T: Ord> Node<T> {
    pub fn new<S: Ord>(key: S, value: String, size: usize) -> Node<S> {
        Node {
            key,
            value,
            size,
            left: Box::new(None),
            right: Box::new(None),
        }
    }
}

struct SymbolTable<T: Ord + Clone> {
    root: Option<Node<T>>,
    pub test: Vec<T>,
}

impl<T: Ord + Clone> SymbolTable<T> {
    pub fn new<S: Ord + Clone>() -> SymbolTable<S> {
        SymbolTable {
            root: None,
            test: Vec::new(),
        }
    }

    pub fn put(&mut self, key: T, value: String) {
        SymbolTable::put_node(&mut self.root, key, value);
    }

    fn put_node(node: &mut Option<Node<T>>, key: T, value: String) {
        if node.is_none() {
            node.replace(Node::<String>::new(key, value, 1));
            return;
        }

        match key.cmp(&node.as_ref().unwrap().key) {
            std::cmp::Ordering::Less => {
                SymbolTable::put_node(&mut node.as_mut().unwrap().left, key, value)
            }
            std::cmp::Ordering::Greater => {
                SymbolTable::put_node(&mut node.as_mut().unwrap().right, key, value)
            }
            std::cmp::Ordering::Equal => node.as_mut().unwrap().value = value,
        }

        node.as_mut().unwrap().size = SymbolTable::get_size(&node.as_ref().unwrap().left)
            + SymbolTable::get_size(&node.as_ref().unwrap().right)
            + 1;
    }

    fn get_size(node: &Box<Option<Node<T>>>) -> usize {
        match node.deref() {
            Some(n) => n.size,
            None => 0,
        }
    }

    pub fn size(&self) -> usize {
        match self.root.as_ref() {
            Some(node) => node.size,
            None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn get(&self, key: T) -> Option<String> {
        SymbolTable::get_node(&self.root, key)
    }

    fn get_node(node: &Option<Node<T>>, key: T) -> Option<String> {
        if node.is_none() {
            return None;
        }

        match key.cmp(&node.as_ref().unwrap().key) {
            std::cmp::Ordering::Less => {
                SymbolTable::get_node(node.as_ref().unwrap().left.deref(), key)
            }
            std::cmp::Ordering::Greater => {
                SymbolTable::get_node(node.as_ref().unwrap().right.deref(), key)
            }
            std::cmp::Ordering::Equal => Some(node.as_ref().unwrap().value.clone()),
        }
    }

    pub fn contains(&self, key: T) -> bool {
        self.get(key).is_some()
    }

    pub fn delete(&mut self, key: T) {
        // missing implementation
    }

    pub fn keys(&mut self) -> Iter<T> {
        self.test.clear();
        SymbolTable::build_recursive(&mut self.test, self.root.as_ref());
        return self.test.iter();
    }

    fn build_recursive(result: &mut Vec<T>, node: Option<&Node<T>>) {
        if node.is_none() {
            return;
        }

        SymbolTable::build_recursive(result, node.as_ref().unwrap().left.deref().as_ref());
        result.push(node.as_ref().unwrap().key.clone());
        SymbolTable::build_recursive(result, node.as_ref().unwrap().right.deref().as_ref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_keys() {
        // arrange
        let st = &mut SymbolTable::<String>::new::<String>();
        let keys = "S E A R C H E X A M P L E".split(" ");

        // act
        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // assert
        assert!(!st.is_empty());
        assert_eq!(st.size(), 10);

        let iter = &mut st.keys();
        assert_eq!(iter.next(), Some(&String::from("A")));
        assert_eq!(iter.next(), Some(&String::from("C")));
        assert_eq!(iter.next(), Some(&String::from("E")));
        assert_eq!(iter.next(), Some(&String::from("H")));
        assert_eq!(iter.next(), Some(&String::from("L")));
        assert_eq!(iter.next(), Some(&String::from("M")));
        assert_eq!(iter.next(), Some(&String::from("P")));
        assert_eq!(iter.next(), Some(&String::from("R")));
        assert_eq!(iter.next(), Some(&String::from("S")));
        assert_eq!(iter.next(), Some(&String::from("X")));
    }

    #[test]
    fn get_values() {
        // arrange
        let st = &mut SymbolTable::<String>::new::<String>();
        let keys = "S E A R C H E X A M P L E".split(" ");

        // act
        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // assert
        assert!(st.contains(String::from("S")));
        assert!(!st.contains(String::from("T")));

        assert_eq!(st.get(String::from("A")), Some(String::from("8")));
        assert_eq!(st.get(String::from("C")), Some(String::from("4")));
        assert_eq!(st.get(String::from("E")), Some(String::from("12")));
        assert_eq!(st.get(String::from("H")), Some(String::from("5")));
        assert_eq!(st.get(String::from("L")), Some(String::from("11")));
        assert_eq!(st.get(String::from("M")), Some(String::from("9")));
        assert_eq!(st.get(String::from("P")), Some(String::from("10")));
        assert_eq!(st.get(String::from("R")), Some(String::from("3")));
        assert_eq!(st.get(String::from("S")), Some(String::from("0")));
        assert_eq!(st.get(String::from("X")), Some(String::from("7")));
    }

    #[test]
    fn delete_values() {
        // arrange
        let st = &mut SymbolTable::<String>::new::<String>();
        st.put(String::from("test"), String::from("test"));

        // act
        st.delete(String::from("test"));

        // assert
        // assert!(st.is_empty());
        // assert!(!st.contains(String::from("test")));
    }
}
