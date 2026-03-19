use std::io::{self, Read};

fn main() {
    // Read entire stdin; no explicit error handling per specification.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Undefined behavior if parsing fails or n is negative; we unwrap and use i64 parsing.
    let n: i64 = it.next().unwrap().parse().unwrap();

    if n <= 1 {
        // Per postconditions: if number of elements is 0 or 1, output 0.
        println!("0");
        return;
    }

    // Additional memory usage is linear in n; store all elements.
    let n_usize: usize = n as usize;
    let mut a: Vec<i64> = Vec::with_capacity(n_usize);
    for _ in 0..n_usize {
        a.push(it.next().unwrap().parse().unwrap());
    }

    // Compute maximum number of consecutive adjacent steps where next <= previous.
    // Count is in "steps"/comparisons, so for n elements the maximum is n-1.
    let mut current: i64 = 0;
    let mut best: i64 = 0;

    for i in 0..(n_usize - 1) {
        if a[i + 1] <= a[i] {
            current += 1;
            if current > best {
                best = current; // invariant: best never decreases
            }
        } else {
            current = 0;
        }
    }

    println!("{}", best);
}
