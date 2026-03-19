use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // No explicit error handling: if parsing fails, behavior is not defined by the spec.
    let n: i64 = it.next().unwrap().parse().unwrap();

    let mut counts = [0usize; 6];

    if n > 0 {
        for _ in 0..n {
            let h: f64 = it.next().unwrap().parse().unwrap();

            let idx = if h < 165.0 {
                0
            } else if h < 170.0 {
                1
            } else if h < 175.0 {
                2
            } else if h < 180.0 {
                3
            } else if h < 185.0 {
                4
            } else {
                5
            };
            counts[idx] += 1;
        }
    }
    // If n is negative, no heights are processed and all counts remain zero.

    let mut out = String::new();
    for (i, &c) in counts.iter().enumerate() {
        // Output format is "k:" immediately followed by markers, then newline.
        out.push_str(&(i + 1).to_string());
        out.push(':');
        out.extend(std::iter::repeat('*').take(c));
        out.push('\n');
    }

    print!("{out}");
}
