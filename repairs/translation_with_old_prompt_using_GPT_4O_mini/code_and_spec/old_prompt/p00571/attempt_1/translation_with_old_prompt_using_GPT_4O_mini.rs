use std::io::{self, BufRead};

#[derive(Debug)]
struct Artwork {
    value: i64,
    weight: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of artworks
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");
    assert!(n > 0, "N must be greater than 0");

    let mut artworks = Vec::with_capacity(n);
    
    // Read artworks' values and weights
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let value: i64 = parts.next().unwrap().parse().expect("Invalid input for value");
        let weight: i32 = parts.next().unwrap().parse().expect("Invalid input for weight");
        artworks.push(Artwork { value, weight });
    }

    // Sort artworks by value, and by weight if values are equal
    artworks.sort_by(|a, b| {
        if a.value == b.value {
            a.weight.cmp(&b.weight)
        } else {
            a.value.cmp(&b.value)
        }
    });

    const INF: i64 = 0x1010101010101010;
    let mut s = vec![0; n + 1];
    let mut min = INF;
    let mut max = i64::MIN;

    for i in 1..=n {
        let t = s[i - 1] - artworks[i - 1].value;
        if t < min {
            min = t;
        }
        s[i] = s[i - 1] + artworks[i - 1].weight;
        let t = s[i] - artworks[i - 1].value - min;
        if t > max {
            max = t;
        }
    }

    // Print the maximum difference
    println!("{}", max);
}
