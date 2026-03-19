use std::io::{self, Read};

fn main() {
    // Read entire stdin; no explicit error handling per specification.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let max_div: i64 = it.next().unwrap().parse().unwrap();

    // Fixed internal capacity: 1000 elements (precondition ensures n <= 1000).
    let mut a: Vec<i64> = Vec::with_capacity(1000);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // Apply remainder-based adjacent comparisons for each divisor from 1..=max_div.
    // For each divisor d, perform a single left-to-right pass over adjacent pairs,
    // swapping if (a[i] % d) > (a[i+1] % d).
    for d in 1..=max_div {
        let d = d as i64;
        if n >= 2 {
            for i in 0..(n - 1) {
                let left_rem = a[i] % d;
                let right_rem = a[i + 1] % d;
                if left_rem > right_rem {
                    a.swap(i, i + 1);
                }
            }
        }
    }

    // Output one value per line.
    let mut out = String::new();
    for v in a {
        out.push_str(&v.to_string());
        out.push('\n');
    }
    print!("{out}");
}
