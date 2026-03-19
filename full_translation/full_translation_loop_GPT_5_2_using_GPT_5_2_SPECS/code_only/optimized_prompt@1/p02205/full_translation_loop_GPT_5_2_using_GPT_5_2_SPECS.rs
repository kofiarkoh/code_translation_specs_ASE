use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut n: i64 = it.next().unwrap().parse().unwrap();
    let mut a: i32 = it.next().unwrap().parse().unwrap();
    let mut b: i32 = it.next().unwrap().parse().unwrap();

    // C: N %= 12; (for negative N, C keeps the sign of the dividend)
    n %= 12;

    // Loop structure mirrors the C code closely.
    let mut i: i64 = 0;
    while i < n {
        // C int arithmetic can overflow (UB). Use wrapping ops to emulate typical 2's complement wrap.
        a = a.wrapping_sub(b);
        i += 1;
        if i == n {
            break;
        }
        b = a.wrapping_add(b);
        i += 1;
    }

    print!("{} {}\n", a, b);
}
