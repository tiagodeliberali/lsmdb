use crate::symbol_table::ST;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct FrequencyCounter {}

impl FrequencyCounter {
    pub fn build(st: &mut impl ST<String, u64>, file_path: &str, min_length: usize) -> u64 {
        let file = File::open(file_path).expect("file not found");
        let reader = BufReader::new(file);
        let mut count: u64 = 0;

        for line in reader.lines() {
            for word in line.unwrap().split(" ") {
                let word = String::from(word);

                if &word.len() < &min_length {
                    continue;
                }
                if !st.contains(&word) {
                    st.put(word.clone(), 1);
                } else {
                    st.put(word.clone(), st.get(&word).unwrap() + 1);
                }

                count += 1;
            }
        }

        count
    }

    pub fn find_max(st: &mut impl ST<String, u64>) -> String {
        let mut max = String::new();

        st.put(max.clone(), 0);

        let keys = st.keys();
        for word in keys {
            if st.get(&word).unwrap() > st.get(&max).unwrap() {
                max = word.clone();
            }
        }

        max
    }
}
