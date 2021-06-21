use std::collections::BTreeMap;
use std::ops::Bound::Included;

use crate::symbol_table::ST;
use std::{usize};

pub struct BTree<KEY, VALUE>
where
    KEY: Ord + Clone,
    VALUE: Clone,
{
    root: BTreeMap<KEY, VALUE>,
}

impl<KEY, VALUE> ST<KEY, VALUE> for BTree<KEY, VALUE>
where
    KEY: Ord + Clone,
    VALUE: Clone,
{
    fn new() -> BTree<KEY, VALUE> {
        BTree { root: BTreeMap::new() }
    }

    fn put(&mut self, key: KEY, value: VALUE) {
        self.root.insert(key, value);
    }

    fn size(&self) -> usize {
        self.root.len()
    }

    fn get(&self, key: &KEY) -> Option<VALUE> {
        if let Some(v) = self.root.get(key) {
            return Some(v.clone());
        }

        return None
    }

    fn min(&self) -> Option<KEY> {
        if let Some((k, _)) = self.root.first_key_value() {
            return Some(k.clone());
        }

        return None
    }

    fn max(&self) -> Option<KEY> {
        if let Some((k, _)) = self.root.last_key_value() {
            return Some(k.clone());
        }

        return None
    }

    fn floor(&self, key: KEY) -> Option<KEY> {
        None
    }

    fn ceiling(&self, key: KEY) -> Option<KEY> {
        None
    }

    fn select(&self, position: usize) -> Option<KEY> {
        None
    }

    fn rank(&self, key: KEY) -> Option<usize> {
        None
    }

    fn keys_in_range(&self, min_key: &KEY, max_key: &KEY) -> Vec<KEY> {
        let mut keys = Vec::new();
        for (key, _) in self.root.range((Included(min_key), Included(max_key))) {
            keys.push(key.clone());
        }
        return keys;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol_table::test_client::symbol_table_integration::run_tests;

    #[test]
    fn run_integration_tests() {
        run_tests::<BTree<String, String>>();
    }
}
