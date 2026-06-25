use std::io::{self, Read};

fn main() {
    // Read all input (behavior unspecified on invalid/missing input, matching C).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let a_bound: i32 = it.next().unwrap().parse().unwrap();
    let b_bound: i32 = it.next().unwrap().parse().unwrap();

    let mut total: i32 = 0;

    for i in 1..=n {
        // digitSum5: sum of five least significant decimal digits
        let e = i % 10;
        let d = ((i - e) / 10) % 10;
        let c = ((i - 10 * d - e) / 100) % 10;
        let b = ((i - 100 * c - 10 * d - e) / 1000) % 10;
        let a = ((i - 1000 * b - 100 * c - 10 * d - e) / 10000) % 10;

        let sum = a + b + c + d + e;
        if a_bound <= sum && sum <= b_bound {
            total += i;
        }
    }

    // Match C output formatting: no trailing newline.
    print!("{total}");
}
