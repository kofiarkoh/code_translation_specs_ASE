use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // No more datasets.
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break; // Terminator, not a dataset.
        }

        let mut seq: Vec<usize> = Vec::with_capacity(n);
        for _ in 0..n {
            let v: usize = it.next().unwrap().parse().unwrap();
            seq.push(v);
        }

        let mut steps: usize = 0;

        loop {
            // Compute next sequence: at each position i, next[i] = frequency of seq[i] in seq.
            // Value range is assumed small (0..=100), but we implement counting via a map-like
            // array sized to max observed value (bounded by preconditions).
            let mut freq = vec![0usize; 101];
            for &v in &seq {
                // Preconditions say v in 0..=100; otherwise behavior is undefined.
                freq[v] += 1;
            }

            let mut next = Vec::with_capacity(n);
            for &v in &seq {
                next.push(freq[v]);
            }

            if next == seq {
                break; // Fixed point reached; do not count this non-changing application.
            }

            seq = next;
            steps += 1;
        }

        out.push_str(&format!("{}\n", steps));
        for (i, v) in seq.iter().enumerate() {
            if i != 0 {
                out.push(' ');
            }
            out.push_str(&v.to_string());
        }
        out.push('\n');
    }

    print!("{out}");
}
