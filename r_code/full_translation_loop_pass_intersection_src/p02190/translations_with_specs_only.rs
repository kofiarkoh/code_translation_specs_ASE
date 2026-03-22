use std::io::{self, Read};

fn main() {
    // Read entire stdin; specification provides no explicit error handling.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // If input is missing/invalid, behavior is unspecified; unwrap() matches that spirit.
    let n: usize = it.next().unwrap().parse().unwrap();

    // Store up to 100001 integers as per constraints.
    let mut v: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        v.push(x);
    }

    // Ordering-based distinct counting: sort then count transitions.
    v.sort_unstable();

    let mut distinct: usize = 0;
    let mut prev: Option<i64> = None;
    for &x in &v {
        if prev.map_or(true, |p| p != x) {
            distinct += 1;
            prev = Some(x);
        }
    }

    println!("{}", distinct);
}
