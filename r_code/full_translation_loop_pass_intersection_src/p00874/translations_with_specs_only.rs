use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior on malformed/short input is unspecified by the spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n1: i64 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break, // Unspecified by spec; just stop if no more tokens.
        };
        let n2: i64 = match it.next() {
            Some(s) => s.parse().unwrap(),
            None => break, // Unspecified by spec.
        };

        if n1 == 0 {
            break; // Terminating case is not processed.
        }

        // Fixed, bounded storage for counts of values 1..20.
        // Index 0 unused; values are guaranteed (by preconditions) to be in 1..20.
        let mut c1 = [0i64; 21];
        let mut c2 = [0i64; 21];

        for _ in 0..n1 {
            let v: usize = it.next().unwrap().parse::<usize>().unwrap();
            c1[v] += 1;
        }
        for _ in 0..n2 {
            let v: usize = it.next().unwrap().parse::<usize>().unwrap();
            c2[v] += 1;
        }

        let mut total: i64 = 0;
        for v in 1..=20 {
            let m = if c1[v] > c2[v] { c1[v] } else { c2[v] };
            total += (v as i64) * m;
        }

        out.push_str(&total.to_string());
        out.push('\n');
    }

    print!("{out}");
}
