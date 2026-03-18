use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Preconditions say N is positive and there are N integers; no extra error handling specified.
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // Sort in non-decreasing order.
    a.sort();

    // Compute (sum of even indices) - (sum of odd indices) on the sorted sequence.
    // Use i128 accumulator to avoid overflow in intermediate sums; specification does not define overflow.
    let mut even_sum: i128 = 0;
    let mut odd_sum: i128 = 0;

    for (i, &v) in a.iter().enumerate() {
        if i % 2 == 0 {
            even_sum += v as i128;
        } else {
            odd_sum += v as i128;
        }
    }

    let diff: i128 = even_sum - odd_sum;
    print!("{}", diff);
}
