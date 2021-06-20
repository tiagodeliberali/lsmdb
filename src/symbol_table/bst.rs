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
    #[allow(dead_code)]
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
    use crate::symbol_table::test_client::symbol_table_integration::run_tests;

    #[test]
    fn run_integration_tests() {
        run_tests::<BST<String, String>>();
    }

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
}
