use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();
    let x: i32 = it.next().unwrap().parse().unwrap();

    let mut count: i32 = 0;

    // Use i32 and wrapping arithmetic to emulate C int overflow behavior as closely as possible.
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                let sum = 500i32
                    .wrapping_mul(i)
                    .wrapping_add(100i32.wrapping_mul(j))
                    .wrapping_add(50i32.wrapping_mul(k));
                if sum == x {
                    count = count.wrapping_add(1);
                }
            }
        }
    }

    // Match C's printf("%d", count) (no trailing newline).
    print!("{}", count);
}
