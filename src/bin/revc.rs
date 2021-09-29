use std::fs;

fn compute(input: &str) -> String {
    let mut res = String::new();
    input
        .chars()
        .rev()
        .filter(|c| c.is_ascii_alphabetic())
        .for_each(|c| {
            let to_push = match c {
                'A' => 'T',
                'C' => 'G',
                'T' => 'A',
                'G' => 'C',
                x => panic!("input has unexpected character {}", x),
            };
            res.push(to_push);
        });
    res
}

fn main() {
    let input = fs::read_to_string("inputs/rosalind_revc.txt").unwrap();
    let res = compute(&input);
    println!("{}", res);
}

#[cfg(test)]
mod revc_test {
    #[test]
    fn test() {
        let s = "AAAACCCGGT";
        assert_eq!(super::compute(s), "ACCGGGTTTT");
    }
}
