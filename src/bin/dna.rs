use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/rosalind_dna.txt").unwrap();
    let mut res = HashMap::new();
    for c in input.chars() {
        *res.entry(c).or_insert(0) += 1;
    }
    for (key, entry) in res {
        println!(" {}: {} ", key, entry);
    }
}
