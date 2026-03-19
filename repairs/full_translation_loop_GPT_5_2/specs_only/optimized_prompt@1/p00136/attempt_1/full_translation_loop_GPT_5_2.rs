use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Preconditions say the first token is a valid non-negative integer.
    let n: usize = match it.next() {
        Some(tok) => tok.parse::<i64>().unwrap().max(0) as usize,
        None => return, // No input; nothing to do.
    };

    // Six predefined ranges; the specification does not define boundaries.
    // Narrowest reasonable interpretation: use six contiguous 10-unit bins:
    // [0,10), [10,20), [20,30), [30,40), [40,50), [50, +inf)
    // Heights < 0 are counted in the first bin to keep total processed == n.
    let mut counts = [0usize; 6];

    for _ in 0..n {
        let h: f64 = it.next().unwrap().parse().unwrap();

        let idx = if h < 10.0 {
            0
        } else if h < 20.0 {
            1
        } else if h < 30.0 {
            2
        } else if h < 40.0 {
            3
        } else if h < 50.0 {
            4
        } else {
            5
        };
        counts[idx] += 1;
    }

    // Output format: six lines, each starting with a label then asterisks.
    // Labels are fixed; the specification does not prescribe exact text.
    let labels = [
        "0-10: ",
        "10-20: ",
        "20-30: ",
        "30-40: ",
        "40-50: ",
        "50+: ",
    ];

    let mut out = String::new();
    for i in 0..6 {
        out.push_str(labels[i]);
        out.extend(std::iter::repeat('*').take(counts[i]));
        out.push('\n');
    }

    print!("{out}");
}
