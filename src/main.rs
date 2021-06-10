mod bst;
mod stack;
mod symbol_table;

use crate::bst::BST;
use crate::symbol_table::SymbolTable;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

fn main() {
    println!("let's build a trees!");
    println!("--------------------\n");

    let min_length = 1;

    let main_instant = Instant::now();
    let st = &mut SymbolTable::<String, u32>::new::<String, u32>();

    let file = File::open("data/leipzig1M.txt").expect("file not found");
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

    let load_st = main_instant.elapsed();
    let read_instant = Instant::now();

    let mut max = String::new();

    st.put(max.clone(), 0);

    let keys = st.keys();
    for word in keys {
        if st.get(&word).unwrap() > st.get(&max).unwrap() {
            max = word.clone();
        }
    }

    println!("most used word is '{}': {}\n", max, st.get(&max).unwrap());

    let read_st = read_instant.elapsed();
    let total = main_instant.elapsed();

    println!(
        "Elapsed time to write file words to symbol table: {:.2?}",
        load_st
    );
    println!("average write word/s: {:.2?}", (count / load_st.as_secs()));
    println!(
        "Elapsed time to read frequencies from symbol table: {:.2?}",
        read_st
    );
    println!("average read word/s: {:.2?}", (1000_u128 * (count as u128) / read_st.as_millis()));
    println!("count words: {:.2?}", count);
    println!("nodes (dinsti nct words): {:.2?}", st.size());
    println!("Total time: {:.2?}", total);
}
