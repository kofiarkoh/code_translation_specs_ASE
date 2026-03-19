use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Spec notes K must be non-zero; like the C code, we do not explicitly handle K == 0.
    let q1 = n / k;
    let cube1 = q1 * q1 * q1;

    let ans = if k % 2 != 0 {
        cube1
    } else {
        let q2 = (n + k / 2) / k;
        let cube2 = q2 * q2 * q2;
        cube1 + cube2
    };

    // Match C output formatting: one integer followed by newline.
    print!("{ans}\n");
}
