use std::io::{self, Read};

fn main() {
    // Read entire stdin. The SPEC assumes at least five integer tokens are present.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    // Split on whitespace and parse the first five tokens as i32.
    // We use unwrap() so that parse failures or missing tokens cause a panic,
    // which mirrors the C program's unspecified behavior when scanf fails.
    let mut parts = input.split_whitespace();
    let mut a: [i32; 5] = [
        parts.next().unwrap().parse::<i32>().unwrap(),
        parts.next().unwrap().parse::<i32>().unwrap(),
        parts.next().unwrap().parse::<i32>().unwrap(),
        parts.next().unwrap().parse::<i32>().unwrap(),
        parts.next().unwrap().parse::<i32>().unwrap(),
    ];

    // Sort in non-increasing (descending) order using the same nested-loop swap logic
    // as the original C implementation.
    for i in 0..5 {
        for j in i..5 {
            if a[i] < a[j] {
                a.swap(i, j);
            }
        }
    }

    // Print the five integers separated by single spaces with a trailing space,
    // and no trailing newline (matches SPEC exactly).
    for i in 0..5 {
        print!("{} ", a[i]);
    }
}
