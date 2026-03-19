use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(tok) => tok.parse::<usize>().unwrap(),
        None => return,
    };

    // The expected output indicates 6 bins for human heights in cm:
    // 1: [0, 165)
    // 2: [165, 170)
    // 3: [170, 175)
    // 4: [175, 180)
    // 5: [180, 185)
    // 6: [185, +inf)
    // Values below 0 are treated as belonging to bin 1 to preserve total count == n.
    let mut counts = [0usize; 6];

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

    let mut out = String::new();
    for (i, &c) in counts.iter().enumerate() {
        out.push_str(&(i + 1).to_string());
        out.push(':');
        out.extend(std::iter::repeat('*').take(c));
        out.push('\n');
    }

    print!("{out}");
}
