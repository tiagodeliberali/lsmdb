mod stack;
mod symbol_table;

use crate::symbol_table::bst::BST;
use crate::symbol_table::frequency_counter::FrequencyCounter;
use crate::symbol_table::red_black_bst::RedBlackBST;
use crate::symbol_table::ST;

use std::time::Instant;

fn main() {
    let mut red_black: RedBlackBST<String, u64> = RedBlackBST::new();
    instrument(&mut red_black, "Red Back BST", "data/leipzig1M.txt");

    let mut bst: BST<String, u64> = BST::new();
    instrument(&mut bst, "BST", "data/leipzig1M.txt");
}

fn instrument<T: ST<String, u64>>(st: &mut T, name: &str, path: &str) {
    println!("{}", name);
    println!("-----------\n");

    let main_instant = Instant::now();

    let count = FrequencyCounter::build(st, path, 1);

    let load_st = main_instant.elapsed();
    let read_instant = Instant::now();

    let max = FrequencyCounter::find_max(st);

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
    println!(
        "average read word/s: {:.2?}",
        (1000_u128 * (count as u128) / read_st.as_millis())
    );
    println!("count words: {:.2?}", count);
    println!("nodes (dinstinct words): {:.2?}", st.size());
    println!("Total time: {:.2?}", total);
    println!();
}
