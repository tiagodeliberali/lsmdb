use crate::symbol_table::ST;
use std::{
    ops::{Deref, DerefMut},
    usize,
};

pub struct Node<KEY, VALUE>
where
    KEY: Ord,
{
    pub key: KEY,
    pub value: VALUE,
    pub size: usize,
    pub left: Box<Option<Node<KEY, VALUE>>>,
    pub right: Box<Option<Node<KEY, VALUE>>>,
    pub is_red: bool,
}

impl<KEY, VALUE> Node<KEY, VALUE>
where
    KEY: Ord,
{
    pub fn new(key: KEY, value: VALUE, size: usize, is_red: bool) -> Node<KEY, VALUE> {
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

pub struct RedBlackBST<KEY, VALUE>
where
    KEY: Ord,
{
    root: Option<Node<KEY, VALUE>>,
}

impl<KEY, VALUE> RedBlackBST<KEY, VALUE>
where
    KEY: Ord,
{
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
            option_node.replace(Node::new(key, value, 1, true));
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

    fn get_node<'a>(node: &'a Option<Node<KEY, VALUE>>, key: &KEY) -> Option<&'a VALUE> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => RedBlackBST::get_node(node.left.deref(), key),
            std::cmp::Ordering::Greater => RedBlackBST::get_node(node.right.deref(), key),
            std::cmp::Ordering::Equal => Some(&node.value),
        }
    }

    fn min_node(node: &Option<Node<KEY, VALUE>>) -> Option<&KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        if node.left.is_none() {
            return Some(&node.key);
        }

        RedBlackBST::min_node(&node.left)
    }

    fn max_node(node: &Option<Node<KEY, VALUE>>) -> Option<&KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        if node.right.is_none() {
            return Some(&node.key);
        }

        RedBlackBST::max_node(&node.right)
    }

    pub fn floor_node<'a>(node: &'a Option<Node<KEY, VALUE>>, key: &KEY) -> Option<&'a KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => RedBlackBST::floor_node(node.left.deref(), key),
            std::cmp::Ordering::Greater => match RedBlackBST::floor_node(node.right.deref(), key) {
                Some(v) => Some(v),
                None => Some(&node.key),
            },
            std::cmp::Ordering::Equal => Some(&node.key),
        }
    }

    fn ceiling_node<'a>(node: &'a Option<Node<KEY, VALUE>>, key: &KEY) -> Option<&'a KEY> {
        if node.is_none() {
            return None;
        }

        let node = node.as_ref().unwrap();

        match key.cmp(&node.key) {
            std::cmp::Ordering::Less => match RedBlackBST::ceiling_node(node.left.deref(), key) {
                Some(v) => Some(v),
                None => Some(&node.key),
            },
            std::cmp::Ordering::Greater => RedBlackBST::ceiling_node(node.right.deref(), key),
            std::cmp::Ordering::Equal => Some(&node.key),
        }
    }

    fn select_node(node: &Option<Node<KEY, VALUE>>, position: usize) -> Option<&KEY> {
        let node = node.as_ref().unwrap();
        let left_count = RedBlackBST::get_size(&node.left);

        match position.cmp(&left_count) {
            std::cmp::Ordering::Less => RedBlackBST::select_node(node.left.deref(), position),
            std::cmp::Ordering::Greater => {
                RedBlackBST::select_node(node.right.deref(), position - left_count - 1)
            }
            std::cmp::Ordering::Equal => Some(&node.key),
        }
    }

    fn rank_node(node: &Option<Node<KEY, VALUE>>, key: &KEY, position: usize) -> Option<usize> {
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

    fn keys_node<'a>(
        result: &mut Vec<&'a KEY>,
        node: &'a Option<Node<KEY, VALUE>>,
        min_key: &KEY,
        max_key: &KEY,
    ) {
        if node.is_none() {
            return;
        }

        let node = node.as_ref().unwrap();

        if &node.key > min_key {
            RedBlackBST::keys_node(result, &node.left.as_ref(), min_key, max_key);
        }

        if &node.key >= min_key && &node.key <= max_key {
            result.push(&node.key);
        }

        if &node.key < max_key {
            RedBlackBST::keys_node(result, &node.right.as_ref(), min_key, max_key);
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

        RedBlackBST::<String, VALUE>::draw_node(node.left.deref(), position * 2, level + 1, result);
        RedBlackBST::<String, VALUE>::draw_node(
            node.right.deref(),
            position * 2 + 1,
            level + 1,
            result,
        );
    }
}

impl<KEY, VALUE> ST<KEY, VALUE> for RedBlackBST<KEY, VALUE>
where
    KEY: Ord + Clone,
    VALUE: Clone,
{
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

    fn get(&self, key: &KEY) -> Option<&VALUE> {
        RedBlackBST::get_node(&self.root, key)
    }

    fn min(&self) -> Option<&KEY> {
        RedBlackBST::min_node(&self.root)
    }

    fn max(&self) -> Option<&KEY> {
        RedBlackBST::max_node(&self.root)
    }

    fn floor(&self, key: &KEY) -> Option<&KEY> {
        RedBlackBST::floor_node(&self.root, key)
    }

    fn ceiling(&self, key: &KEY) -> Option<&KEY> {
        RedBlackBST::ceiling_node(&self.root, key)
    }

    fn select(&self, position: usize) -> Option<&KEY> {
        if position >= self.size() {
            return None;
        }

        RedBlackBST::select_node(&self.root, position)
    }

    fn rank(&self, key: &KEY) -> Option<usize> {
        RedBlackBST::rank_node(&self.root, key, 0)
    }

    fn keys_in_range(&self, min_key: &KEY, max_key: &KEY) -> Vec<&KEY> {
        let mut keys = Vec::new();
        RedBlackBST::keys_node(&mut keys, &self.root, &min_key, &max_key);
        return keys;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol_table::test_client::symbol_table_integration::run_tests;

    #[test]
    fn run_integration_tests() {
        run_tests::<RedBlackBST<String, String>>();
    }

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
}
