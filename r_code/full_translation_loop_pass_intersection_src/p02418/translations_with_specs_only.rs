use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-delimited tokens.
    // Specification: no explicit error handling; missing inputs => undefined.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let s = it.next().unwrap();
    let t = it.next().unwrap();

    // Concatenate s with itself. Length bounds (<= 100) imply this is safe.
    let mut doubled = String::with_capacity(s.len() * 2);
    doubled.push_str(s);
    doubled.push_str(s);

    if doubled.contains(t) {
        println!("Yes");
    } else {
        println!("No");
    }
}
