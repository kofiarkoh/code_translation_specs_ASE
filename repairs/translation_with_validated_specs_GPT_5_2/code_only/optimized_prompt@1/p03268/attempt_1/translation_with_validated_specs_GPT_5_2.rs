use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // In C, signed overflow is UB. We emulate typical two's-complement wraparound
    // using wrapping_* operations to preserve behavior as closely as possible.
    let ans: i64 = if k % 2 != 0 {
        let a = n / k;
        a.wrapping_mul(a).wrapping_mul(a)
    } else {
        let a = n / k;
        let b = (n.wrapping_add(k / 2)) / k;
        a.wrapping_mul(a)
            .wrapping_mul(a)
            .wrapping_add(b.wrapping_mul(b).wrapping_mul(b))
    };

    // Match C output formatting exactly: one integer followed by '\n'
    print!("{}\n", ans);

    // Rust stdout is flushed on program exit; explicit flush not required to match output.
}
