use std::cmp::Ordering;
use std::io::{self, Read};

type Int64 = i64;

#[derive(Clone, Copy)]
struct Node {
    w: i32,
    s: i32,
    v: i32,
}

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // No input: C would have undefined behavior; we just exit.
    };

    let mut p: Vec<Node> = Vec::with_capacity(n);
    for _ in 0..n {
        let w: i32 = it.next().unwrap().parse().unwrap();
        let s: i32 = it.next().unwrap().parse().unwrap();
        let v: i32 = it.next().unwrap().parse().unwrap();
        p.push(Node { w, s, v });
    }

    // qsort with comparator: (p->s+p->w)-(q->s+q->w)
    // In C, this subtraction can overflow (UB). We emulate typical 32-bit wraparound
    // by using wrapping arithmetic and then ordering by the wrapped i32 result.
    p.sort_by(|a, b| {
        let sa = a.s.wrapping_add(a.w);
        let sb = b.s.wrapping_add(b.w);
        let diff = sa.wrapping_sub(sb);
        if diff < 0 {
            Ordering::Less
        } else if diff > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    const M: usize = 20000;
    let mut dp: Vec<Int64> = vec![0; M + 1];

    for item in p {
        // C loop: for (j = p[i].s; j >= 0; j--)
        // If s is negative, the loop doesn't execute.
        if item.s < 0 {
            continue;
        }
        let mut j = item.s as i32;
        while j >= 0 {
            let k_i32 = j.wrapping_add(item.w);
            // C would index dp[k] without bounds checks (UB if out of range).
            // We preserve defined behavior for in-range indices; out-of-range is ignored
            // to avoid panics (closest safe approximation).
            if k_i32 >= 0 {
                let k = k_i32 as usize;
                if k <= M {
                    let jj = j as usize;
                    let cand = dp[jj].wrapping_add(item.v as Int64);
                    if cand > dp[k] {
                        dp[k] = cand;
                    }
                }
            }
            j -= 1;
        }
    }

    let mut ans: Int64 = 0;
    for &x in &dp {
        if x > ans {
            ans = x;
        }
    }

    // Match C printf("%lld\n", ans);
    print!("{}\n", ans);
}
