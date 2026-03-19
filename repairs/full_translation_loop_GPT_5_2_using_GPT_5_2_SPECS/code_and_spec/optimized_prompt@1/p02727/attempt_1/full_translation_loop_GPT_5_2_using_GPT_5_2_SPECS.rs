use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    // Per spec: if any required input cannot be read, terminate with nonzero exit status.
    let x: usize = match it.next().and_then(|t| t.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let y: usize = match it.next().and_then(|t| t.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let a: usize = match it.next().and_then(|t| t.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let b: usize = match it.next().and_then(|t| t.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let c: usize = match it.next().and_then(|t| t.parse().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let mut p: Vec<i64> = Vec::with_capacity(a);
    let mut q: Vec<i64> = Vec::with_capacity(b);
    let mut r: Vec<i64> = Vec::with_capacity(c);

    for _ in 0..a {
        let v: i64 = match it.next().and_then(|t| t.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        p.push(v);
    }
    for _ in 0..b {
        let v: i64 = match it.next().and_then(|t| t.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        q.push(v);
    }
    for _ in 0..c {
        let v: i64 = match it.next().and_then(|t| t.parse().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        r.push(v);
    }

    // Sort descending (largest first), matching C comparator behavior.
    p.sort_unstable_by(|a, b| b.cmp(a));
    q.sort_unstable_by(|a, b| b.cmp(a));
    r.sort_unstable_by(|a, b| b.cmp(a));

    // Initial selection: top X from p and top Y from q.
    // Replacement process: consider r in descending order, replace the current smallest among
    // the selected elements (from p[0..X) and q[0..Y)) if r[i] is strictly larger.
    //
    // This matches the C code's logic using indices asumi=X-1 and iguti=Y-1 into the
    // selected prefixes, which are the current minima of each selected set.
    let mut asumi: isize = x as isize - 1;
    let mut iguti: isize = y as isize - 1;

    for &cand in &r {
        if asumi < 0 && iguti < 0 {
            break;
        } else if asumi < 0
            || (iguti >= 0
                && p[asumi as usize] > q[iguti as usize])
        {
            // Replace in q (its current minimum is >= p's current minimum, or p exhausted)
            let idx = iguti as usize;
            if cand > q[idx] {
                q[idx] = cand;
                iguti -= 1;
            } else {
                break;
            }
        } else {
            // Replace in p
            let idx = asumi as usize;
            if cand > p[idx] {
                p[idx] = cand;
                asumi -= 1;
            } else {
                break;
            }
        }
    }

    let mut ans: i64 = 0;
    for i in 0..x {
        ans += p[i];
    }
    for i in 0..y {
        ans += q[i];
    }

    print!("{ans}\n");
}
