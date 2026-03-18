use std::io::{self, Read};

fn lower_bound(a: &[i64], x: i64) -> usize {
    let mut l = 0usize;
    let mut r = a.len();
    while l < r {
        let m = (l + r) / 2;
        if a[m] < x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}

fn upper_bound(a: &[i64], x: i64) -> usize {
    let mut l = 0usize;
    let mut r = a.len();
    while l < r {
        let m = (l + r) / 2;
        if a[m] <= x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_ops: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // Initialize capacity based on number of operations; start empty.
    let mut a: Vec<i64> = Vec::with_capacity(n_ops);
    let mut out = String::new();

    for _ in 0..n_ops {
        let cmd: i32 = match it.next() {
            Some(t) => t.parse().unwrap(),
            None => break,
        };

        match cmd {
            // Insert key; if not present, insert; output count after insertion attempt.
            0 => {
                let key: i64 = it.next().unwrap().parse().unwrap();
                let pos = lower_bound(&a, key);
                if pos == a.len() || a[pos] != key {
                    a.insert(pos, key);
                }
                out.push_str(&format!("{}\n", a.len()));
            }

            // Delete key if present.
            1 => {
                let key: i64 = it.next().unwrap().parse().unwrap();
                let pos = lower_bound(&a, key);
                if pos < a.len() && a[pos] == key {
                    a.remove(pos);
                }
            }

            // Search key; output 1 if present else 0.
            2 => {
                let key: i64 = it.next().unwrap().parse().unwrap();
                let pos = lower_bound(&a, key);
                let exists = pos < a.len() && a[pos] == key;
                out.push_str(if exists { "1\n" } else { "0\n" });
            }

            // Dump by index range [l, r] inclusive (0-based), as implied by sample.
            // If invalid range, output nothing.
            3 => {
                let l: i64 = it.next().unwrap().parse().unwrap();
                let r: i64 = it.next().unwrap().parse().unwrap();

                if l < 0 || r < 0 || l > r {
                    continue;
                }
                let l = l as usize;
                let r = r as usize;
                if l >= a.len() {
                    continue;
                }
                let end = r.saturating_add(1).min(a.len());
                if l >= end {
                    continue;
                }
                for &v in &a[l..end] {
                    out.push_str(&format!("{}\n", v));
                }
            }

            // Unspecified command: do nothing.
            _ => {}
        }
    }

    print!("{out}");
}
