use std::cmp::Ordering;
use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior for malformed input/EOF is unspecified by spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // unspecified; choose to stop quietly
        }
        let n: usize = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break, // unspecified; choose to stop quietly
        };

        if n == 0 {
            break;
        }

        let mut a: Vec<i32> = Vec::with_capacity(n);
        for _ in 0..n {
            // Spec says exactly n integers follow; if not, behavior unspecified.
            let tok = match it.next() {
                Some(t) => t,
                None => break,
            };
            let v: i32 = match tok.parse() {
                Ok(v) => v,
                Err(_) => break,
            };
            a.push(v);
        }

        // Sort in descending numeric order.
        // Spec mentions subtraction-based comparison in original; overflow makes behavior unspecified.
        // We use a total ordering comparator without overflow; this is within the specified behavior
        // for all inputs where subtraction would not overflow.
        a.sort_unstable_by(|x, y| y.cmp(x));

        // Scan to find maximum run length; update remembered value only on strictly longer run.
        let mut best_len: usize = 0;
        let mut best_val: i32 = 0;

        let mut i = 0usize;
        while i < a.len() {
            let v = a[i];
            let mut j = i + 1;
            while j < a.len() && a[j] == v {
                j += 1;
            }
            let run_len = j - i;
            if run_len > best_len {
                best_len = run_len;
                best_val = v;
            }
            i = j;
        }

        if best_len > n / 2 {
            out.push_str(&format!("{}\n", best_val));
        } else {
            out.push_str("NO COLOR\n");
        }
    }

    print!("{out}");
}
