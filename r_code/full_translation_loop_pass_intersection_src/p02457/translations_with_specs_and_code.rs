use std::io::{self, Read};

fn search(s: &[i32], key: i32) -> Option<usize> {
    // Binary search for exact match.
    let mut head: isize = 0;
    let mut tail: isize = s.len() as isize - 1;
    while head <= tail {
        let mid = (head + tail) / 2;
        let v = s[mid as usize];
        if v == key {
            return Some(mid as usize);
        } else if v > key {
            tail = mid - 1;
        } else {
            head = mid + 1;
        }
    }
    None
}

fn lower_bound(s: &[i32], key: i32) -> usize {
    // First index i such that s[i] >= key, or s.len() if none.
    // This is the safe/idiomatic equivalent of the C getLB, without relying on
    // unspecified behavior (like reading s[mid-1] when mid==0).
    let mut lo: usize = 0;
    let mut hi: usize = s.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if s[mid] < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn main() {
    // Read all input, parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let q: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // Storage sized to number of queries (as per spec's non-functional constraint).
    let mut s: Vec<i32> = Vec::with_capacity(q);
    let mut out = String::new();

    for _ in 0..q {
        let com: i32 = it.next().unwrap().parse().unwrap();
        match com {
            0 => {
                // insert
                let x: i32 = it.next().unwrap().parse().unwrap();
                if search(&s, x).is_none() {
                    let d = lower_bound(&s, x);
                    s.insert(d, x);
                }
                out.push_str(&format!("{}\n", s.len()));
            }
            1 => {
                // find
                let x: i32 = it.next().unwrap().parse().unwrap();
                if search(&s, x).is_some() {
                    out.push_str("1\n");
                } else {
                    out.push_str("0\n");
                }
            }
            2 => {
                // delete
                let x: i32 = it.next().unwrap().parse().unwrap();
                if let Some(idx) = search(&s, x) {
                    s.remove(idx);
                }
            }
            3 => {
                // dump/range output
                let l: i32 = it.next().unwrap().parse().unwrap();
                let r: i32 = it.next().unwrap().parse().unwrap();

                // Spec precondition: dump is only requested when set is non-empty.
                // We still handle empty safely (no output).
                if !s.is_empty() {
                    let mut i = lower_bound(&s, l);
                    while i < s.len() && s[i] <= r {
                        out.push_str(&format!("{}\n", s[i]));
                        i += 1;
                    }
                }
            }
            _ => {
                // Spec: behavior unspecified for invalid command codes; do nothing.
            }
        }
    }

    print!("{out}");
}
