use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break; // Unspecified behavior in spec; just stop if input ends.
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break; // terminating dataset indicator; no output
        }

        // Spec allows up to 1,000,001; no explicit handling required if exceeded.
        let mut a: Vec<i32> = Vec::with_capacity(n);
        for _ in 0..n {
            let v: i32 = it.next().unwrap().parse().unwrap();
            a.push(v);
        }

        // C code sorts descending using a comparator that can overflow:
        // return (*(int*)b) - (*(int*)s);
        // Spec says overflow does not occur, so we can safely sort by value.
        a.sort_unstable_by(|x, y| y.cmp(x)); // descending

        // Find maximum run length and its value.
        let mut best_len: usize = 0;
        let mut best_val: i32 = 0;

        let mut i = 0usize;
        while i < n {
            let val = a[i];
            let mut j = i + 1;
            while j < n && a[j] == val {
                j += 1;
            }
            let len = j - i;
            if best_len < len {
                best_len = len;
                best_val = val;
            }
            i = j;
        }

        if best_len > n / 2 {
            out.push_str(&format!("{best_val}\n"));
        } else {
            // Fixed message per C code / spec.
            out.push_str("NO COLOR\n");
        }
    }

    print!("{out}");
}
