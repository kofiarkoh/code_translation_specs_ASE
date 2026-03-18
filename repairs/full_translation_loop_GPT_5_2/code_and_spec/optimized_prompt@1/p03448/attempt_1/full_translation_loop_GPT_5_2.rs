use std::io::{self, Read};

fn main() {
    // Read all input (whitespace-separated integers).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();
    let x: i32 = it.next().unwrap().parse().unwrap();

    let mut count: i32 = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if 500 * i + 100 * j + 50 * k == x {
                    count += 1;
                }
            }
        }
    }

    // Match C output formatting: no trailing newline.
    print!("{}", count);
}
