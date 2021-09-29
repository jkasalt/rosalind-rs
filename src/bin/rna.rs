use std::fs;

fn main() {
    let mut input = fs::read_to_string("inputs/rosalind_rna.txt").unwrap();
    input = input.replace("T", "U");
    println!("{}", input);
}
