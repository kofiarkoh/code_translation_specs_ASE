use std::cmp::Ordering;
use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let rows: i64 = match it.next() {
            Some(tok) => tok.parse().unwrap(),
            None => break, // No more input; unspecified behavior, but we stop.
        };
        let cols: i64 = match it.next() {
            Some(tok) => tok.parse().unwrap(),
            None => break, // No more input; unspecified behavior, but we stop.
        };

        if rows == 0 && cols == 0 {
            break;
        }

        // Preconditions state rows>0 and cols>0 for non-terminating cases.
        let r = rows as usize;
        let c = cols as usize;

        // Column sums fit in the integer type used by the program.
        // Use i64 as a reasonable "int" analogue; spec does not mandate width.
        let mut sums = vec![0i64; c];

        // Read matrix in row-major order; accumulate per-column sums.
        for _ in 0..r {
            for j in 0..c {
                let v: i64 = it.next().unwrap().parse().unwrap();
                sums[j] += v;
            }
        }

        // Produce permutation of 1-based column indices ordered by:
        // primary: non-increasing sum, secondary: increasing index.
        let mut idx: Vec<usize> = (0..c).collect();
        idx.sort_by(|&a, &b| {
            match sums[b].cmp(&sums[a]) {
                Ordering::Equal => a.cmp(&b),
                other => other,
            }
        });

        for (k, &col0) in idx.iter().enumerate() {
            if k > 0 {
                out.push(' ');
            }
            out.push_str(&(col0 + 1).to_string());
        }
        out.push('\n');
    }

    print!("{out}");
}
