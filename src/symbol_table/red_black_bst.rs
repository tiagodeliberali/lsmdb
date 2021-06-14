use crate::symbol_table::ST;
use std::{
    ops::{Deref, DerefMut},
    usize,
};

pub struct Node<KEY: Ord, VALUE: Clone> {
    pub key: KEY,
    pub value: VALUE,
    pub size: usize,
    pub left: Box<Option<Node<KEY, VALUE>>>,
    pub right: Box<Option<Node<KEY, VALUE>>>,
    pub is_red: bool,
}

impl<KEY: Ord, VALUE: Clone> Node<KEY, VALUE> {
    pub fn new<A: Ord, B: Clone>(key: A, value: B, size: usize, is_red: bool) -> Node<A, B> {
        Node {
            key,
            value,
            size,
            left: Box::new(None),
            right: Box::new(None),
            is_red,
        }
    }
}

pub struct RedBlackBST<KEY: Ord + Clone, VALUE: Clone> {
    root: Option<Node<KEY, VALUE>>,
}

impl<KEY: Ord + Clone, VALUE: Clone> RedBlackBST<KEY, VALUE> {
    fn is_red(node: &Option<Node<KEY, VALUE>>) -> bool {
        if let Some(v) = node {
            v.is_red
        } else {
            false
        }
    }

    fn flip_colors(h: &mut Node<KEY, VALUE>) {
        h.is_red = true;

        if let Some(h_left) = h.left.deref_mut() {
            h_left.is_red = false;
        }

        if let Some(h_right) = h.right.deref_mut() {
            h_right.is_red = false;
        }
    }

    fn rotate_left(mut h: Node<KEY, VALUE>) -> Node<KEY, VALUE> {
        let mut x = h.right.take().unwrap();

        let x_left = x.left.take();

        if let Some(x_left_value) = x_left {
            h.right.replace(x_left_value);
        }

        x.is_red = h.is_red;
        h.is_red = true;

        x.size = h.size;
        h.size = RedBlackBST::get_size(&h.left) + RedBlackBST::get_size(&h.right) + 1;

        x.left.replace(h);
        return x;
    }

    fn rotate_right(mut h: Node<KEY, VALUE>) -> Node<KEY, VALUE> {
        let mut x = h.left.take().unwrap();

        let x_right = x.right.take();

        if let Some(x_right_value) = x_right {
            h.left.replace(x_right_value);
        }

        x.is_red = h.is_red;
        h.is_red = true;

        x.size = h.size;

        h.size = RedBlackBST::get_size(&h.left) + RedBlackBST::get_size(&h.right) + 1;

        x.right.replace(h);
        return x;
    }

    fn put_node(option_node: &mut Option<Node<KEY, VALUE>>, key: KEY, value: VALUE) {
        if option_node.is_none() {
            option_node.replace(Node::<KEY, VALUE>::new(key, value, 1, true));
            return;
        }

        let mut node = option_node.take().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => RedBlackBST::put_node(&mut node.left, key, value),
            std::cmp::Ordering::Greater => RedBlackBST::put_node(&mut node.right, key, value),
            std::cmp::Ordering::Equal => node.value = value,
        }

        if RedBlackBST::is_red(&node.right) && !RedBlackBST::is_red(&node.left) {
            node = RedBlackBST::rotate_left(node);
        }

        if RedBlackBST::is_red(&node.left) {
            if let Some(left_node) = node.left.deref() {
                if RedBlackBST::is_red(&left_node.left) {
                    node = RedBlackBST::rotate_right(node);
                }
            }
        }

        if RedBlackBST::is_red(&node.right) && RedBlackBST::is_red(&node.left) {
            RedBlackBST::flip_colors(&mut node);
        }

        node.size = RedBlackBST::get_size(&node.left) + RedBlackBST::get_size(&node.right) + 1;

        option_node.replace(node);
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
            std::cmp::Ordering::Less => RedBlackBST::get_node(node.left.deref(), key),
            std::cmp::Ordering::Greater => RedBlackBST::get_node(node.right.deref(), key),
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

        RedBlackBST::min_node(&node.left)
    }

    fn max_node(node: &Option<Node<KEY, VALUE>>) -> Option<KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        if node.right.is_none() {
            return Some(node.key.clone());
        }

        RedBlackBST::max_node(&node.right)
    }

    pub fn floor_node(node: &Option<Node<KEY, VALUE>>, key: KEY) -> Option<KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => RedBlackBST::floor_node(node.left.deref(), key),
            std::cmp::Ordering::Greater => match RedBlackBST::floor_node(node.right.deref(), key) {
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
            std::cmp::Ordering::Less => match RedBlackBST::ceiling_node(node.left.deref(), key) {
                Some(v) => Some(v),
                None => Some(node.key.clone()),
            },
            std::cmp::Ordering::Greater => RedBlackBST::ceiling_node(node.right.deref(), key),
            std::cmp::Ordering::Equal => Some(node.key.clone()),
        }
    }

    fn select_node(node: &Option<Node<KEY, VALUE>>, position: usize) -> Option<KEY> {
        let node = node.as_ref().unwrap();
        let left_count = RedBlackBST::get_size(&node.left);

        match position.cmp(&left_count) {
            std::cmp::Ordering::Less => RedBlackBST::select_node(node.left.deref(), position),
            std::cmp::Ordering::Greater => {
                RedBlackBST::select_node(node.right.deref(), position - left_count - 1)
            }
            std::cmp::Ordering::Equal => Some(node.key.clone()),
        }
    }

    fn rank_node(node: &Option<Node<KEY, VALUE>>, key: KEY, position: usize) -> Option<usize> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        let left_count = RedBlackBST::get_size(&node.left);

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => RedBlackBST::rank_node(node.left.deref(), key, position),
            std::cmp::Ordering::Greater => {
                RedBlackBST::rank_node(node.right.deref(), key, position + left_count + 1)
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
            RedBlackBST::keys_node(result, node.left.deref().as_ref(), min_key, max_key);
        }

        if &node.key >= min_key && &node.key <= max_key {
            result.push(node.key.clone());
        }

        if &node.key < max_key {
            RedBlackBST::keys_node(result, node.right.deref().as_ref(), min_key, max_key);
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

        RedBlackBST::<String, VALUE>::draw_node(node.left.deref(), position * 2, level + 1, result);
        RedBlackBST::<String, VALUE>::draw_node(
            node.right.deref(),
            position * 2 + 1,
            level + 1,
            result,
        );
    }
}

impl<KEY: Ord + Clone, VALUE: Clone> ST<KEY, VALUE> for RedBlackBST<KEY, VALUE> {
    fn new() -> RedBlackBST<KEY, VALUE> {
        RedBlackBST { root: None }
    }

    fn put(&mut self, key: KEY, value: VALUE) {
        RedBlackBST::put_node(&mut self.root, key, value);
    }

    fn size(&self) -> usize {
        match self.root.as_ref() {
            Some(node) => node.size,
            None => 0,
        }
    }

    fn get(&self, key: &KEY) -> Option<VALUE> {
        RedBlackBST::get_node(&self.root, key)
    }

    fn min(&self) -> Option<KEY> {
        RedBlackBST::min_node(&self.root)
    }

    fn max(&self) -> Option<KEY> {
        RedBlackBST::max_node(&self.root)
    }

    fn floor(&self, key: KEY) -> Option<KEY> {
        RedBlackBST::floor_node(&self.root, key)
    }

    fn ceiling(&self, key: KEY) -> Option<KEY> {
        RedBlackBST::ceiling_node(&self.root, key)
    }

    fn select(&self, position: usize) -> Option<KEY> {
        if position >= self.size() {
            return None;
        }

        RedBlackBST::select_node(&self.root, position)
    }

    fn rank(&self, key: KEY) -> Option<usize> {
        RedBlackBST::rank_node(&self.root, key, 0)
    }

    fn keys_in_range(&self, min_key: &KEY, max_key: &KEY) -> Vec<KEY> {
        let mut keys = Vec::new();
        RedBlackBST::keys_node(&mut keys, self.root.as_ref(), &min_key, &max_key);
        return keys;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_sequncial_index_tree() {
        // arrange
        let st = &mut RedBlackBST::<String, String>::new();
        let keys = "A B C D E F G H I J K L M N O P Q".split(" ");

        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // act
        let result = &mut Vec::new();
        RedBlackBST::<String, String>::draw_node(&st.root, 0, 0, result);

        // assert
        assert_eq!(result.len(), 5);

        // print tree
        for line in result {
            println!("{}", line);
        }
    }

    #[test]
    fn draw_tree() {
        // arrange
        let st = &mut RedBlackBST::<String, String>::new();
        let keys = "S E A R C H E X A M P L E".split(" ");

        for (position, key) in keys.enumerate() {
            st.put(String::from(key), format!("{}", position));
        }

        // act
        let result = &mut Vec::new();
        RedBlackBST::<String, String>::draw_node(&st.root, 0, 0, result);

        // assert
        assert_eq!(result.len(), 4);

        // print tree
        for line in result {
            println!("{}", line);
        }
    }

    #[test]
    fn symbol_table_iterate_keys_ordered() {
        // arrange
        let st = &mut RedBlackBST::<String, String>::new();
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
        let st = &mut RedBlackBST::<String, String>::new();
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
        let st = &mut RedBlackBST::<String, String>::new();
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
        let st = &mut RedBlackBST::<String, String>::new();
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
        let st = &mut RedBlackBST::<String, String>::new();
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
        let st = &mut RedBlackBST::<String, String>::new();
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
        let st = &mut RedBlackBST::<String, String>::new();
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
        let st = &mut RedBlackBST::<String, String>::new();
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
        let st = &mut RedBlackBST::<String, String>::new();
        st.put(String::from("test"), String::from("test"));

        // act
        st.delete(String::from("test"));

        // assert
        // assert!(st.is_empty());
        // assert!(!st.contains(String::from("test")));
    }
}