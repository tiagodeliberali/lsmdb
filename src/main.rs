mod stack;
mod symbol_table;

use crate::symbol_table::SymbolTable;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    println!("let's build a tree!");

    use std::time::Instant;
    let now = Instant::now();

    let file = File::open("data/leipzig1M.txt").expect("file not found");
    let reader = BufReader::new(file);
    let st = &mut SymbolTable::<String, String>::new::<String, String>();
    let mut count: u64 = 0;

    for line in reader.lines() {
        for word in line.unwrap().split(" ") {
            st.put(String::from(word), String::from(word));
            count += 1;
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed load file to symbol table: {:.2?}", elapsed);
    println!("count words: {:.2?}", count);
    println!("nodes: {:.2?}", st.size());
    println!("average word/s: {:.2?}", (count / elapsed.as_secs()));
}
