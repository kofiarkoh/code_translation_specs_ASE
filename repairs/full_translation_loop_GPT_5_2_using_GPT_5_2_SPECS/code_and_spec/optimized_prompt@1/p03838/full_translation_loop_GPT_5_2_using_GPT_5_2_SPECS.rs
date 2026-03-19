use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // As in the C code: no explicit error handling if missing/invalid.
    let x: i64 = it.next().unwrap().parse().unwrap();
    let y: i64 = it.next().unwrap().parse().unwrap();

    // Solve (constant time), preserving the C logic exactly.
    let ans: i64;
    if x >= 0 {
        if y >= x {
            ans = -x + y;
        } else if y > 0 {
            ans = x - y + 2;
        } else if y >= -x {
            ans = x + y + 1;
        } else {
            ans = -y - x + 1;
        }
    } else {
        if y >= -x {
            ans = x + y + 1;
        } else if y > 0 {
            ans = -x - y + 1;
        } else if y >= x {
            ans = -x + y;
        } else {
            ans = x - y + 2;
        }
    }

    // Output: no trailing newline, matching the C program.
    print!("{ans}");
}
