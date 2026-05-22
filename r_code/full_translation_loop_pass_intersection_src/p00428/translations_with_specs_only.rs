use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let r_opt = it.next();
        if r_opt.is_none() {
            break; // unspecified behavior per spec if input is missing; we just stop
        }
        let rows: usize = r_opt.unwrap().parse().unwrap();
        let cols: usize = it.next().unwrap().parse().unwrap();

        if rows == 0 && cols == 0 {
            break;
        }

        // Column sums; use i64 as a reasonable "execution environment integer" stand-in.
        // Overflow behavior is unspecified by the spec; Rust debug overflow would panic,
        // release would wrap for primitive ops only with wrapping_*; we keep normal +.
        let mut sums: Vec<i64> = vec![0; cols];

        for _ in 0..rows {
            for c in 0..cols {
                let v: i64 = it.next().unwrap().parse().unwrap();
                sums[c] += v;
            }
        }

        // Selection-based ordering to match the stated O(cols^2) additional factor.
        let mut used = vec![false; cols];
        let mut first = true;

        for _ in 0..cols {
            let mut best_idx: Option<usize> = None;
            for j in 0..cols {
                if used[j] {
                    continue;
                }
                match best_idx {
                    None => best_idx = Some(j),
                    Some(b) => {
                        if sums[j] > sums[b] || (sums[j] == sums[b] && j < b) {
                            best_idx = Some(j);
                        }
                    }
                }
            }

            let b = best_idx.unwrap();
            used[b] = true;

            if !first {
                out.push(' ');
            }
            first = false;
            out.push_str(&(b + 1).to_string());
        }

        out.push('\n');
    }

    print!("{out}");
}
