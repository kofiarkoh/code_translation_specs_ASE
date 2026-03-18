use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse three integers like scanf("%d%d%d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let x: i32 = it.next().unwrap().parse().unwrap();

    let ans: i32;
    if a >= 2 * b {
        // Preserve C's integer division semantics (trunc toward zero) for i32.
        ans = ((x - 1) / 500 + 1) * b;
    } else {
        let mut tmp = (x / 1000) * a;
        let c = x % 1000;
        if c != 0 {
            if c <= 500 && b < a {
                tmp += b;
            } else {
                tmp += a;
            }
        }
        ans = tmp;
    }

    print!("{}\n", ans);
}
