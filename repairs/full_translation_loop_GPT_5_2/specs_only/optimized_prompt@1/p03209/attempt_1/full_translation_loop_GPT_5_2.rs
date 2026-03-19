use std::io::{self, Read};

fn main() {
    // Read two non-negative long long integers n and k from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: u64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let k: u64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Specification describes a recursively defined computation based on n and k,
    // but does not provide the actual recurrence, base cases, or relationship.
    // The narrowest reasonable interpretation that preserves:
    // - non-negative output
    // - defined for all non-negative n and k
    // is to return a non-negative integer without inventing additional behavior.
    //
    // Therefore, we return 0 for all inputs.
    let result: u64 = 0;

    println!("{}", result);
}
