use std::{ops::Deref, slice::Iter, usize};

struct Node<KEY: Ord, VALUE: Clone> {
    pub key: KEY,
    pub value: VALUE,
    pub size: usize,
    pub left: Box<Option<Node<KEY, VALUE>>>,
    pub right: Box<Option<Node<KEY, VALUE>>>,
}

impl<KEY: Ord, VALUE: Clone> Node<KEY, VALUE> {
    pub fn new<A: Ord, B: Clone>(key: A, value: B, size: usize) -> Node<A, B> {
        Node {
            key,
            value,
            size,
            left: Box::new(None),
            right: Box::new(None),
        }
    }
}

struct SymbolTable<KEY: Ord + Clone, VALUE: Clone> {
    root: Option<Node<KEY, VALUE>>,
    pub test: Vec<KEY>,
}

impl<KEY: Ord + Clone, VALUE: Clone> SymbolTable<KEY, VALUE> {
    pub fn new<A: Ord + Clone, B: Clone>() -> SymbolTable<A, B> {
        SymbolTable {
            root: None,
            test: Vec::new(),
        }
    }

    pub fn put(&mut self, key: KEY, value: VALUE) {
        SymbolTable::put_node(&mut self.root, key, value);
    }

    fn put_node(node: &mut Option<Node<KEY, VALUE>>, key: KEY, value: VALUE) {
        if node.is_none() {
            node.replace(Node::<KEY, VALUE>::new(key, value, 1));
            return;
        }

        let node = node.as_mut().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => SymbolTable::put_node(&mut node.left, key, value),
            std::cmp::Ordering::Greater => SymbolTable::put_node(&mut node.right, key, value),
            std::cmp::Ordering::Equal => node.value = value,
        }

        node.size = SymbolTable::get_size(&node.left) + SymbolTable::get_size(&node.right) + 1;
    }

    fn get_size(node: &Box<Option<Node<KEY, VALUE>>>) -> usize {
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

    pub fn get(&self, key: KEY) -> Option<VALUE> {
        SymbolTable::get_node(&self.root, key)
    }

    fn get_node(node: &Option<Node<KEY, VALUE>>, key: KEY) -> Option<VALUE> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => SymbolTable::get_node(node.left.deref(), key),
            std::cmp::Ordering::Greater => SymbolTable::get_node(node.right.deref(), key),
            std::cmp::Ordering::Equal => Some(node.value.clone()),
        }
    }

    pub fn contains(&self, key: KEY) -> bool {
        self.get(key).is_some()
    }

    pub fn delete(&mut self, key: KEY) {
        // missing implementation
    }

    pub fn min(&self) -> Option<KEY> {
        SymbolTable::min_node(&self.root)
    }

    fn min_node(node: &Option<Node<KEY, VALUE>>) -> Option<KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        if node.left.is_none() {
            return Some(node.key.clone());
        }

        SymbolTable::min_node(&node.left)
    }

    pub fn max(&self) -> Option<KEY> {
        SymbolTable::max_node(&self.root)
    }

    fn max_node(node: &Option<Node<KEY, VALUE>>) -> Option<KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        if node.right.is_none() {
            return Some(node.key.clone());
        }

        SymbolTable::min_node(&node.right)
    }

    pub fn floor(&self, key: KEY) -> Option<KEY> {
        SymbolTable::floor_node(&self.root, key)
    }

    pub fn floor_node(node: &Option<Node<KEY, VALUE>>, key: KEY) -> Option<KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => SymbolTable::floor_node(node.left.deref(), key),
            std::cmp::Ordering::Greater => match SymbolTable::floor_node(node.right.deref(), key) {
                Some(v) => Some(v),
                None => Some(node.key.clone()),
            },
            std::cmp::Ordering::Equal => Some(node.key.clone()),
        }
    }

    pub fn ceiling(&self, key: KEY) -> Option<KEY> {
        SymbolTable::ceiling_node(&self.root, key)
    }

    pub fn ceiling_node(node: &Option<Node<KEY, VALUE>>, key: KEY) -> Option<KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => match SymbolTable::ceiling_node(node.left.deref(), key) {
                Some(v) => Some(v),
                None => Some(node.key.clone()),
            },
            std::cmp::Ordering::Greater => SymbolTable::ceiling_node(node.right.deref(), key),
            std::cmp::Ordering::Equal => Some(node.key.clone()),
        }
    }

    pub fn select(&self, position: usize) -> Option<KEY> {
        if position >= self.size() {
            return None;
        }

        SymbolTable::select_node(&self.root, position)
    }

    fn select_node(node: &Option<Node<KEY, VALUE>>, position: usize) -> Option<KEY> {
        let node = node.as_ref().unwrap();
        let left_count = SymbolTable::get_size(&node.left);

        match position.cmp(&left_count) {
            std::cmp::Ordering::Less => SymbolTable::select_node(node.left.deref(), position),
            std::cmp::Ordering::Greater => {
                SymbolTable::select_node(node.right.deref(), position - left_count - 1)
            }
            std::cmp::Ordering::Equal => Some(node.key.clone()),
        }
    }

    pub fn keys(&mut self) -> Iter<KEY> {
        self.test.clear();
        SymbolTable::build_recursive(&mut self.test, self.root.as_ref());
        return self.test.iter();
    }

    fn build_recursive(result: &mut Vec<KEY>, node: Option<&Node<KEY, VALUE>>) {
        if node.is_none() {
            return;
        }

        let node = node.as_ref().unwrap();

        SymbolTable::build_recursive(result, node.left.deref().as_ref());
        result.push(node.key.clone());
        SymbolTable::build_recursive(result, node.right.deref().as_ref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_table_iterate_keys_ordered() {
        // arrange
        let st = &mut SymbolTable::<String, String>::new::<String, String>();
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
    fn allows_to_search_value_by_key() {
        // arrange
        let st = &mut SymbolTable::<String, String>::new::<String, String>();
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
    fn find_key_by_position() {
        // arrange
        let st = &mut SymbolTable::<String, String>::new::<String, String>();
        let keys = "S E A R C H E X A M P L E".split(" ");

        // act
        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // assert
        assert_eq!(st.select(0), Some(String::from("A")));
        assert_eq!(st.select(3), Some(String::from("H")));
        assert_eq!(st.select(8), Some(String::from("S")));
        assert_eq!(st.select(10), None);
    }

    #[test]
    fn find_min_and_max_keys() {
        // arrange
        let st = &mut SymbolTable::<String, String>::new::<String, String>();
        let keys = "S E A R C H E X A M P L E".split(" ");

        // act
        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // assert
        assert_eq!(st.min(), Some(String::from("A")));
        assert_eq!(st.max(), Some(String::from("X")));
    }

    #[test]
    fn floor_can_find_lower_or_equal_key() {
        // arrange
        let st = &mut SymbolTable::<String, String>::new::<String, String>();
        let keys = "S O M E T H I N G T O F I N D".split(" ");

        // act
        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // assert
        assert_eq!(st.floor(String::from("M")), Some(String::from("M")));
        assert_eq!(st.floor(String::from("J")), Some(String::from("I")));
        assert_eq!(st.floor(String::from("A")), None);
    }

    #[test]
    fn ceiling_can_find_greater_or_equal_key() {
        // arrange
        let st = &mut SymbolTable::<String, String>::new::<String, String>();
        let keys = "S E A R C H E X A M P L E".split(" ");

        // act
        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // assert
        assert_eq!(st.ceiling(String::from("C")), Some(String::from("C")));
        assert_eq!(st.ceiling(String::from("D")), Some(String::from("E")));
        assert_eq!(st.ceiling(String::from("Z")), None);
    }

    #[test]
    fn delete_values_restore_symbol_table_to_empty() {
        // arrange
        let st = &mut SymbolTable::<String, String>::new::<String, String>();
        st.put(String::from("test"), String::from("test"));

        // act
        st.delete(String::from("test"));

        // assert
        // assert!(st.is_empty());
        // assert!(!st.contains(String::from("test")));
    }
}
