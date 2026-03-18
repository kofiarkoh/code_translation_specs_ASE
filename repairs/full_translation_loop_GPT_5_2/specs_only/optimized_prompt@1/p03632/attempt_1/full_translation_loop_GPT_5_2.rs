use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: usize = it.next().unwrap().parse().unwrap();
    let b: usize = it.next().unwrap().parse().unwrap();
    let c: usize = it.next().unwrap().parse().unwrap();
    let d: usize = it.next().unwrap().parse().unwrap();

    // Arrays are initialized to zero and then updated to reflect the ranges.
    // Size 101 matches the stated input domain 0..=100.
    let mut x = [0i32; 101];
    let mut y = [0i32; 101];

    // Mark indices in each range as present.
    for i in a..=b {
        x[i] = 1;
    }
    for i in c..=d {
        y[i] = 1;
    }

    // Count overlaps.
    let mut count: i32 = 0;
    for i in 0..=100 {
        if x[i] == 1 && y[i] == 1 {
            count += 1;
        }
    }

    // Adjust by one if there is at least one overlap.
    if count > 0 {
        count -= 1;
    }

    println!("{}", count);
}
