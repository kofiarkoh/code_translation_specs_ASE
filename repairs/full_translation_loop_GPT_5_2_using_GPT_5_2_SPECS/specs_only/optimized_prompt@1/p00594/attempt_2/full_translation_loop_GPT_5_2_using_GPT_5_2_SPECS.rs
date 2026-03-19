use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let Some(tok) = it.next() else { break };
        let n: usize = tok.parse().unwrap();

        if n == 0 {
            break;
        }

        let mut values: Vec<i64> = Vec::with_capacity(n);
        for _ in 0..n {
            let v: i64 = it.next().unwrap().parse().unwrap();
            values.push(v);
        }

        values.sort_unstable();

        let threshold = n / 2;
        let mut majority: Option<i64> = None;

        let mut i = 0usize;
        while i < n {
            let v = values[i];
            let mut j = i + 1;
            while j < n && values[j] == v {
                j += 1;
            }
            if j - i > threshold {
                majority = Some(v);
                break;
            }
            i = j;
        }

        if let Some(v) = majority {
            out.push_str(&format!("{v}\n"));
        } else {
            // Fixed message indicating absence of a strict majority.
            out.push_str("NO COLOR\n");
        }
    }

    print!("{out}");
}
