use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: i64 = it.next().unwrap().parse().unwrap();
    let w: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    for i in 0..=h {
        for j in 0..=w {
            // Specification assumes intermediate computations fit in the chosen integer type.
            let val = (w * i) + (h * j) - 2 * i * j;
            if val == k {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
