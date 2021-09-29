use rosalind::pattern_separated_values;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_string = fs::read_to_string("inputs/rosalind_fib.txt")?;
    let input = pattern_separated_values(&input_string, ' ');
    let months = input[0];
    let litter = input[1];
    let mut result = vec![1, 1];
    for i in 2..months {
        result.push(result[i - 1] + litter * result[i - 2]);
    }
    println!("{:?}", result.last());
    Ok(())
}
