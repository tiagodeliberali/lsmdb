use std::{cmp::max, ops::Deref, slice::Iter, usize};

pub struct Node<KEY: Ord, VALUE: Clone> {
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

pub struct SymbolTable<KEY: Ord + Clone, VALUE: Clone> {
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

    pub fn rank(&self, key: KEY) -> Option<usize> {
        SymbolTable::rank_node(&self.root, key, 0)
    }

    fn rank_node(node: &Option<Node<KEY, VALUE>>, key: KEY, position: usize) -> Option<usize> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        let left_count = SymbolTable::get_size(&node.left);

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => SymbolTable::rank_node(node.left.deref(), key, position),
            std::cmp::Ordering::Greater => {
                SymbolTable::rank_node(node.right.deref(), key, position + left_count + 1)
            }
            std::cmp::Ordering::Equal => Some(position + left_count),
        }
    }

    pub fn keys(&mut self) -> Iter<KEY> {
        self.test.clear();

        let min_key = self.min();
        let max_key = self.max();

        if min_key.is_none() || max_key.is_none() {
            return self.test.iter();
        }

        let min_key = min_key.unwrap();
        let max_key = max_key.unwrap();

        SymbolTable::keys_node(&mut self.test, self.root.as_ref(), &min_key, &max_key);
        return self.test.iter();
    }

    pub fn keys_in_range(&mut self, min_key: &KEY, max_key: &KEY) -> Iter<KEY> {
        self.test.clear();
        SymbolTable::keys_node(&mut self.test, self.root.as_ref(), &min_key, &max_key);
        return self.test.iter();
    }

    fn keys_node(
        result: &mut Vec<KEY>,
        node: Option<&Node<KEY, VALUE>>,
        min_key: &KEY,
        max_key: &KEY,
    ) {
        if node.is_none() {
            return;
        }

        let node = node.as_ref().unwrap();

        if &node.key > min_key {
            SymbolTable::keys_node(result, node.left.deref().as_ref(), min_key, max_key);
        }

        if &node.key >= min_key && &node.key <= max_key {
            result.push(node.key.clone());
        }

        if &node.key < max_key {
            SymbolTable::keys_node(result, node.right.deref().as_ref(), min_key, max_key);
        }
    }
    // don't take this too seriously
    pub fn draw_node(
        node: &Option<Node<String, VALUE>>,
        position: usize,
        level: usize,
        result: &mut Vec<String>,
    ) {
        if node.is_none() {
            return;
        }

        let node = node.as_ref().unwrap();
        let width = 200 / (2 as usize).pow(level as u32);
        let padding = width * position + width / 2;

        if result.len() <= level {
            result.push(format!(
                "|{:>width$}({})",
                node.key,
                node.size,
                width = padding
            ));
        } else {
            let value = result.get(level).unwrap().to_owned();
            let new_value = format!("|{:>width$}({})", node.key, node.size, width = padding);

            let size = usize::max(value.len(), new_value.len());

            let mut text = String::new();

            for i in 0..size {
                if let Some(v) = value.chars().nth(i) {
                    if v != ' ' {
                        text = format!("{}{}", text, value.chars().nth(i).unwrap());
                        continue;
                    }
                }
                if let Some(v) = new_value.chars().nth(i) {
                    if v != ' ' {
                        text = format!("{}{}", text, new_value.chars().nth(i).unwrap());
                        continue;
                    }
                }
                text = format!("{} ", text);
            }
            result[level] = text;
        }

        SymbolTable::<String, VALUE>::draw_node(node.left.deref(), position * 2, level + 1, result);
        SymbolTable::<String, VALUE>::draw_node(
            node.right.deref(),
            position * 2 + 1,
            level + 1,
            result,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_tree() {
        // arrange
        let st = &mut SymbolTable::<String, String>::new::<String, String>();
        let keys = "S E A R C H E X A M P L E".split(" ");

        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // act
        let result = &mut Vec::new();
        SymbolTable::<String, String>::draw_node(&st.root, 0, 0, result);

        // assert
        assert_eq!(result.len(), 6);

        // print tree
        for line in result {
            println!("{}", line);
        }
    }

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
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn symbol_table_iterate_keys_ordered_between_range() {
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

        let iter = &mut st.keys_in_range(&String::from("D"), &String::from("R"));
        assert_eq!(iter.next(), Some(&String::from("E")));
        assert_eq!(iter.next(), Some(&String::from("H")));
        assert_eq!(iter.next(), Some(&String::from("L")));
        assert_eq!(iter.next(), Some(&String::from("M")));
        assert_eq!(iter.next(), Some(&String::from("P")));
        assert_eq!(iter.next(), Some(&String::from("R")));
        assert_eq!(iter.next(), None);
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
    fn find_position_by_keys() {
        // arrange
        let st = &mut SymbolTable::<String, String>::new::<String, String>();
        let keys = "S E A R C H E X A M P L E".split(" ");

        // act
        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // assert
        assert_eq!(st.rank(String::from("A")), Some(0));
        assert_eq!(st.rank(String::from("H")), Some(3));
        assert_eq!(st.rank(String::from("S")), Some(8));
        assert_eq!(st.rank(String::from("G")), None);
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
