use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated i32 values.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // Unspecified behavior in spec for malformed/EOF; we just stop.
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut a: Vec<i32> = Vec::with_capacity(n);
        for _ in 0..n {
            let v: i32 = it.next().unwrap().parse().unwrap();
            a.push(v);
        }

        // Spec requires descending numeric order.
        // Note: C comparator uses subtraction which can overflow; spec says behavior unspecified then.
        // Rust uses a total order without overflow, which is well-defined; this only differs in those
        // unspecified-overflow cases.
        a.sort_unstable_by(|x, y| y.cmp(x));

        // Scan runs; update remembered value only on strictly longer run (ties do not replace).
        let mut best_len: usize = 0;
        let mut best_val: i32 = 0; // Only used if best_len > n/2.

        let mut i = 0usize;
        while i < n {
            let val = a[i];
            let mut j = i + 1;
            while j < n && a[j] == val {
                j += 1;
            }
            let run_len = j - i;
            if best_len < run_len {
                best_len = run_len;
                best_val = val;
            }
            i = j;
        }

        if best_len > n / 2 {
            out.push_str(&format!("{best_val}\n"));
        } else {
            out.push_str("NO COLOR\n");
        }
    }

    print!("{out}");
}
