use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    let t1: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let t2: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let r1: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let r2: i32 = match it.next().and_then(|s| s.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let a = "Alice";
    let b = "Bob";
    let d = "Draw";

    let out = if r1 < 0 || r2 < 0 {
        if t1 < t2 {
            a
        } else if t1 > t2 {
            b
        } else {
            d
        }
    } else if r1 > r2 {
        a
    } else if r1 < r2 {
        b
    } else {
        d
    };

    // `puts` prints the string followed by a newline.
    println!("{out}");
}
