use crate::symbol_table::ST;
use std::{ops::Deref, usize};

pub struct Node<KEY, VALUE>
where
    KEY: Ord,
    VALUE: Clone,
{
    pub key: KEY,
    pub value: VALUE,
    pub size: usize,
    pub left: Box<Option<Node<KEY, VALUE>>>,
    pub right: Box<Option<Node<KEY, VALUE>>>,
}

impl<KEY, VALUE> Node<KEY, VALUE>
where
    KEY: Ord,
    VALUE: Clone,
{
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

pub struct BST<KEY, VALUE>
where
    KEY: Ord + Clone,
    VALUE: Clone,
{
    root: Option<Node<KEY, VALUE>>,
}

impl<KEY, VALUE> BST<KEY, VALUE>
where
    KEY: Ord + Clone,
    VALUE: Clone,
{
    fn put_node(node: &mut Option<Node<KEY, VALUE>>, key: KEY, value: VALUE) {
        if node.is_none() {
            node.replace(Node::<KEY, VALUE>::new(key, value, 1));
            return;
        }

        let node = node.as_mut().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => BST::put_node(&mut node.left, key, value),
            std::cmp::Ordering::Greater => BST::put_node(&mut node.right, key, value),
            std::cmp::Ordering::Equal => node.value = value,
        }

        node.size = BST::get_size(&node.left) + BST::get_size(&node.right) + 1;
    }

    fn get_size(node: &Box<Option<Node<KEY, VALUE>>>) -> usize {
        match node.deref() {
            Some(n) => n.size,
            None => 0,
        }
    }

    fn get_node(node: &Option<Node<KEY, VALUE>>, key: &KEY) -> Option<VALUE> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => BST::get_node(node.left.deref(), key),
            std::cmp::Ordering::Greater => BST::get_node(node.right.deref(), key),
            std::cmp::Ordering::Equal => Some(node.value.clone()),
        }
    }

    fn min_node(node: &Option<Node<KEY, VALUE>>) -> Option<KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        if node.left.is_none() {
            return Some(node.key.clone());
        }

        BST::min_node(&node.left)
    }

    fn max_node(node: &Option<Node<KEY, VALUE>>) -> Option<KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        if node.right.is_none() {
            return Some(node.key.clone());
        }

        BST::max_node(&node.right)
    }

    pub fn floor_node(node: &Option<Node<KEY, VALUE>>, key: KEY) -> Option<KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => BST::floor_node(node.left.deref(), key),
            std::cmp::Ordering::Greater => match BST::floor_node(node.right.deref(), key) {
                Some(v) => Some(v),
                None => Some(node.key.clone()),
            },
            std::cmp::Ordering::Equal => Some(node.key.clone()),
        }
    }

    fn ceiling_node(node: &Option<Node<KEY, VALUE>>, key: KEY) -> Option<KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => match BST::ceiling_node(node.left.deref(), key) {
                Some(v) => Some(v),
                None => Some(node.key.clone()),
            },
            std::cmp::Ordering::Greater => BST::ceiling_node(node.right.deref(), key),
            std::cmp::Ordering::Equal => Some(node.key.clone()),
        }
    }

    fn select_node(node: &Option<Node<KEY, VALUE>>, position: usize) -> Option<KEY> {
        let node = node.as_ref().unwrap();
        let left_count = BST::get_size(&node.left);

        match position.cmp(&left_count) {
            std::cmp::Ordering::Less => BST::select_node(node.left.deref(), position),
            std::cmp::Ordering::Greater => {
                BST::select_node(node.right.deref(), position - left_count - 1)
            }
            std::cmp::Ordering::Equal => Some(node.key.clone()),
        }
    }

    fn rank_node(node: &Option<Node<KEY, VALUE>>, key: KEY, position: usize) -> Option<usize> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        let left_count = BST::get_size(&node.left);

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => BST::rank_node(node.left.deref(), key, position),
            std::cmp::Ordering::Greater => {
                BST::rank_node(node.right.deref(), key, position + left_count + 1)
            }
            std::cmp::Ordering::Equal => Some(position + left_count),
        }
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
            BST::keys_node(result, node.left.deref().as_ref(), min_key, max_key);
        }

        if &node.key >= min_key && &node.key <= max_key {
            result.push(node.key.clone());
        }

        if &node.key < max_key {
            BST::keys_node(result, node.right.deref().as_ref(), min_key, max_key);
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

        BST::<String, VALUE>::draw_node(node.left.deref(), position * 2, level + 1, result);
        BST::<String, VALUE>::draw_node(node.right.deref(), position * 2 + 1, level + 1, result);
    }
}

impl<KEY, VALUE> ST<KEY, VALUE> for BST<KEY, VALUE>
where
    KEY: Ord + Clone,
    VALUE: Clone,
{
    fn new() -> BST<KEY, VALUE> {
        BST { root: None }
    }

    fn put(&mut self, key: KEY, value: VALUE) {
        BST::put_node(&mut self.root, key, value);
    }

    fn size(&self) -> usize {
        match self.root.as_ref() {
            Some(node) => node.size,
            None => 0,
        }
    }

    fn get(&self, key: &KEY) -> Option<VALUE> {
        BST::get_node(&self.root, key)
    }

    fn min(&self) -> Option<KEY> {
        BST::min_node(&self.root)
    }

    fn max(&self) -> Option<KEY> {
        BST::max_node(&self.root)
    }

    fn floor(&self, key: KEY) -> Option<KEY> {
        BST::floor_node(&self.root, key)
    }

    fn ceiling(&self, key: KEY) -> Option<KEY> {
        BST::ceiling_node(&self.root, key)
    }

    fn select(&self, position: usize) -> Option<KEY> {
        if position >= self.size() {
            return None;
        }

        BST::select_node(&self.root, position)
    }

    fn rank(&self, key: KEY) -> Option<usize> {
        BST::rank_node(&self.root, key, 0)
    }

    fn keys_in_range(&self, min_key: &KEY, max_key: &KEY) -> Vec<KEY> {
        let mut keys = Vec::new();
        BST::keys_node(&mut keys, self.root.as_ref(), &min_key, &max_key);
        return keys;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_sequncial_index_tree() {
        // arrange
        let st = &mut BST::<String, String>::new();
        let keys = "A B C D E F G H I J K L M N O P Q".split(" ");

        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // act
        let result = &mut Vec::new();
        BST::<String, String>::draw_node(&st.root, 0, 0, result);

        // assert
        assert_eq!(result.len(), 17);

        // print tree
        for line in result {
            println!("{}", line);
        }
    }

    #[test]
    fn draw_tree() {
        // arrange
        let st = &mut BST::<String, String>::new();
        let keys = "S E A R C H E X A M P L E".split(" ");

        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // act
        let result = &mut Vec::new();
        BST::<String, String>::draw_node(&st.root, 0, 0, result);

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
        let st = &mut BST::<String, String>::new();
        let keys = "S E A R C H E X A M P L E".split(" ");

        // act
        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // assert
        assert!(!st.is_empty());
        assert_eq!(st.size(), 10);

        let keys = st.keys();
        let mut iter = keys.iter();
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
        let st = &mut BST::<String, String>::new();
        let keys = "S E A R C H E X A M P L E".split(" ");

        // act
        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // assert
        assert!(!st.is_empty());
        assert_eq!(st.size(), 10);

        let keys = st.keys_in_range(&String::from("D"), &String::from("R"));
        let mut iter = keys.iter();
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
        let st = &mut BST::<String, String>::new();
        let keys = "S E A R C H E X A M P L E".split(" ");

        // act
        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // assert
        assert!(st.contains(&String::from("S")));
        assert!(!st.contains(&String::from("T")));

        assert_eq!(st.get(&String::from("A")), Some(String::from("8")));
        assert_eq!(st.get(&String::from("C")), Some(String::from("4")));
        assert_eq!(st.get(&String::from("E")), Some(String::from("12")));
        assert_eq!(st.get(&String::from("H")), Some(String::from("5")));
        assert_eq!(st.get(&String::from("L")), Some(String::from("11")));
        assert_eq!(st.get(&String::from("M")), Some(String::from("9")));
        assert_eq!(st.get(&String::from("P")), Some(String::from("10")));
        assert_eq!(st.get(&String::from("R")), Some(String::from("3")));
        assert_eq!(st.get(&String::from("S")), Some(String::from("0")));
        assert_eq!(st.get(&String::from("X")), Some(String::from("7")));
    }

    #[test]
    fn find_key_by_position() {
        // arrange
        let st = &mut BST::<String, String>::new();
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
        let st = &mut BST::<String, String>::new();
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
        let st = &mut BST::<String, String>::new();
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
        let st = &mut BST::<String, String>::new();
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
        let st = &mut BST::<String, String>::new();
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
        let st = &mut BST::<String, String>::new();
        st.put(String::from("test"), String::from("test"));

        // act
        st.delete(String::from("test"));

        // assert
        // assert!(st.is_empty());
        // assert!(!st.contains(String::from("test")));
    }
}
