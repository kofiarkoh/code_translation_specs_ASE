use std::io::{self, Read};

fn lower_bound(a: &[i64], x: i64) -> usize {
    // First index i such that a[i] >= x, or a.len() if none.
    let mut lo: usize = 0;
    let mut hi: usize = a.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if a[mid] < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn main() {
    // Read entire stdin; behavior for malformed input is unspecified by the spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let q: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Fixed allocation sized to query count, as required.
    let mut data: Vec<i64> = Vec::with_capacity(q);

    let mut out = String::new();

    for _ in 0..q {
        let cmd: i32 = it.next().unwrap().parse().unwrap();
        match cmd {
            0 => {
                let x: i64 = it.next().unwrap().parse().unwrap();
                let pos = lower_bound(&data, x);
                if pos == data.len() || data[pos] != x {
                    data.insert(pos, x);
                }
                out.push_str(&format!("{}\n", data.len()));
            }
            1 => {
                let x: i64 = it.next().unwrap().parse().unwrap();
                let pos = lower_bound(&data, x);
                let found = pos < data.len() && data[pos] == x;
                out.push_str(if found { "1\n" } else { "0\n" });
            }
            2 => {
                let x: i64 = it.next().unwrap().parse().unwrap();
                let pos = lower_bound(&data, x);
                if pos < data.len() && data[pos] == x {
                    data.remove(pos);
                }
            }
            3 => {
                let l: i64 = it.next().unwrap().parse().unwrap();
                let r: i64 = it.next().unwrap().parse().unwrap();

                // Spec states behavior is unspecified if dump is invoked on empty set.
                // We still implement the natural behavior; preconditions guarantee non-empty.
                let mut i = lower_bound(&data, l);
                while i < data.len() {
                    let v = data[i];
                    if v > r {
                        break;
                    }
                    out.push_str(&format!("{}\n", v));
                    i += 1;
                }
            }
            _ => {
                // Unspecified behavior for invalid command codes; do nothing.
            }
        }
    }

    print!("{out}");
}
