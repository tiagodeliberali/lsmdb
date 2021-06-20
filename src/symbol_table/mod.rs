pub mod bst;
pub mod red_black_bst;
pub mod test_client;

pub trait ST<KEY: Ord + Clone, VALUE: Clone> {
    fn new() -> Self;
    fn put(&mut self, key: KEY, value: VALUE);
    fn size(&self) -> usize;
    fn get(&self, key: &KEY) -> Option<VALUE>;
    fn min(&self) -> Option<KEY>;
    fn max(&self) -> Option<KEY>;
    fn floor(&self, key: KEY) -> Option<KEY>;
    fn ceiling(&self, key: KEY) -> Option<KEY>;
    fn select(&self, position: usize) -> Option<KEY>;
    fn rank(&self, key: KEY) -> Option<usize>;
    fn keys_in_range(&self, min_key: &KEY, max_key: &KEY) -> Vec<KEY>;

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn contains(&self, key: &KEY) -> bool {
        self.get(key).is_some()
    }

    fn delete(&mut self, _key: KEY) {
        // missing implementation
    }

    fn keys(&self) -> Vec<KEY> {
        let keys = Vec::new();

        let min_key = self.min();
        let max_key = self.max();

        if min_key.is_none() || max_key.is_none() {
            return keys;
        }

        let min_key = min_key.unwrap();
        let max_key = max_key.unwrap();

        return self.keys_in_range(&min_key, &max_key);
    }
}
