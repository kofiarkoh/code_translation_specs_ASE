use std::io::{self, Read};

fn main() {
    // Read all input (matches typical scanf behavior for whitespace-separated ints).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let a_bound: i32 = it.next().unwrap().parse().unwrap();
    let b_bound: i32 = it.next().unwrap().parse().unwrap();

    let mut total: i32 = 0;

    // Spec: if N < 1, output 0. This loop naturally does that.
    for i in 1..=n {
        // digitSum5(i): sum of five least significant decimal digits.
        // For N > 99,999, this still only considers the last five digits, per spec.
        let mut x = i;
        let mut sum = 0;
        for _ in 0..5 {
            sum += x % 10;
            x /= 10;
        }

        if a_bound <= sum && sum <= b_bound {
            total = total + i; // overflow behavior intentionally unspecified, matching spec.
        }
    }

    print!("{}", total);
}
