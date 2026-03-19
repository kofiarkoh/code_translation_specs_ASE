use std::io::{self, Read};

fn main() {
    // Read entire stdin; if the judge provides no EOF (interactive), this could block.
    // However, the specification implies standard batch input with three integers.
    // Still, to avoid any potential blocking on some environments, we only need the first line/bytes.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    let c: i64 = it.next().unwrap().parse().unwrap();

    let d = a - b;

    // Postconditions:
    // - If D <= 0: C' = C
    // - If D > 0 and C <= 0: C' = C - D
    // - If D > 0 and C > 0: C' = max(0, C - D)
    let c_prime = if d <= 0 {
        c
    } else if c <= 0 {
        c - d
    } else {
        let r = c - d;
        if r < 0 { 0 } else { r }
    };

    // Match typical competitive programming output: newline-terminated.
    println!("{c_prime}");
}
