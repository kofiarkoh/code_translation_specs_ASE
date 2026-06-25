use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a0: i64 = it.next().unwrap().parse().unwrap();
    let b0: i64 = it.next().unwrap().parse().unwrap();
    let c0: i64 = it.next().unwrap().parse().unwrap();

    // Preconditions state no overflow occurs for arithmetic performed.
    let d = a0 - b0;

    let cout = if d <= 0 {
        c0
    } else if c0 <= 0 {
        // As specified: if D > 0 and C0 <= 0, then Cout = C0 - D.
        c0 - d
    } else {
        // For D > 0 and C0 > 0: reduce by up to D but not below zero.
        // Equivalent to max(0, C0 - D).
        let reduced = c0 - d;
        if reduced < 0 { 0 } else { reduced }
    };

    print!("{}", cout);
}
