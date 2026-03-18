use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // C code uses a VLA `int s[n]` but only needs it to temporarily store/modify values.
    // We can compute the same result in one pass without storing all values.
    let mut sum: i32 = 0;
    let mut min_val: i32 = 101; // sentinel as in C

    for _ in 0..n {
        let mut x: i32 = it.next().unwrap().parse().unwrap();
        sum += x;

        // If divisible by 10, treat as 101 for min tracking (matches C behavior).
        if x % 10 == 0 {
            x = 101;
        }
        if x < min_val {
            min_val = x;
        }
    }

    let mut ans = sum;
    if sum % 10 == 0 {
        if min_val == 101 {
            ans = 0;
        } else {
            ans = sum - min_val;
        }
    }

    print!("{ans}\n");
}
