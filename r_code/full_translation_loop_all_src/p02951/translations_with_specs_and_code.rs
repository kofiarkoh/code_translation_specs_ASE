use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let mut a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let mut c: i32 = it.next().unwrap().parse().unwrap();

    // Invariant per spec / C: first integer reduced by second before further operations.
    a -= b;

    // C behavior: decrement a and c while a > 0, stopping early if c reaches 0.
    // Note: If c starts negative, spec says undefined; we do not add checks.
    while a > 0 {
        a -= 1;
        c -= 1;
        if c == 0 {
            break;
        }
    }

    // Output matches C: print final c with newline.
    print!("{c}\n");
}
