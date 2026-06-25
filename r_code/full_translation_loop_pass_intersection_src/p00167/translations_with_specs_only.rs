use std::io::{self, Read};

fn main() {
    // Read entire stdin; if malformed after a positive length, behavior is unspecified by spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut outputs = Vec::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            // If the dataset length cannot be read, stop with no further output.
            break;
        }
        let n: usize = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break, // length cannot be read as integer => stop
        };

        if n == 0 {
            // Terminating dataset: stop with no output for this dataset.
            break;
        }

        // Preconditions say n <= 100 and exactly n integers follow; otherwise unspecified.
        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            let tok = it.next().unwrap();
            let v: i64 = tok.parse().unwrap();
            a.push(v);
        }

        // Count inversions (adjacent swaps needed for stable bubble-sort-like process).
        // Quadratic time as per constraints.
        let mut inv: i64 = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if a[i] > a[j] {
                    inv += 1;
                }
            }
        }

        outputs.push(inv.to_string());
    }

    print!("{}", outputs.join("\n"));
    if !outputs.is_empty() {
        print!("\n");
    }
}
