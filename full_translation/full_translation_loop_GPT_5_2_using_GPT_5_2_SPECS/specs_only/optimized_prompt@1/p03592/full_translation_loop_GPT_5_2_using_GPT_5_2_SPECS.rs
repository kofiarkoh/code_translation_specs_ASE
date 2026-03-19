use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: i64 = it.next().unwrap().parse().unwrap(); // height bound (nonnegative per spec)
    let w: i64 = it.next().unwrap().parse().unwrap(); // width bound (nonnegative per spec)
    let target: i64 = it.next().unwrap().parse().unwrap();

    // Brute-force search over all (i, j) in inclusive ranges, terminating immediately on success.
    for i in 0..=h {
        for j in 0..=w {
            // Spec precondition states no overflow occurs; use i64 arithmetic directly.
            let value = w * i + h * j - 2 * i * j;
            if value == target {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
