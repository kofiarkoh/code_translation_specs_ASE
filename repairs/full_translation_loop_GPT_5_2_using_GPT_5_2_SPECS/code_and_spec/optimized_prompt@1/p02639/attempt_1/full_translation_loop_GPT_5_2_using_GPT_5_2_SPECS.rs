use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace().filter_map(|s| s.parse::<i64>().ok());

    // Spec precondition: at least five integers are provided.
    // If fewer are present, behavior is unspecified; we simply do nothing.
    let a = match it.next() {
        Some(v) => v,
        None => return,
    };
    let b = match it.next() {
        Some(v) => v,
        None => return,
    };
    let c = match it.next() {
        Some(v) => v,
        None => return,
    };
    let d = match it.next() {
        Some(v) => v,
        None => return,
    };
    let e = match it.next() {
        Some(v) => v,
        None => return,
    };

    if a == 0 {
        print!("1");
    } else if b == 0 {
        print!("2");
    } else if c == 0 {
        print!("3");
    } else if d == 0 {
        print!("4");
    } else if e == 0 {
        print!("5");
    }
}
