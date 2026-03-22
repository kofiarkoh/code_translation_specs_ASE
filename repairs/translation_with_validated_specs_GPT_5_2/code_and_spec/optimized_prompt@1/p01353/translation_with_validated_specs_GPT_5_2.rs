use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Pair {
    t: i64, // corresponds to tbl[].a in C (long long)
    k: i64, // corresponds to tbl[].d in C (int), but we store as i64 for arithmetic
}

fn main() {
    // Read entire stdin, parse as nonnegative decimal integers (as in the C fast input).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse::<usize>().unwrap(),
        None => return,
    };

    // Store as i64 to match spec's 64-bit intermediate arithmetic for derived values.
    // (C uses int arrays, but computations promote; spec is source-of-truth.)
    let mut h = vec![0i64; n + 1];
    let mut a = vec![0i64; n + 1];
    let mut d = vec![0i64; n + 1];
    let mut s = vec![0i64; n + 1];

    for i in 0..=n {
        h[i] = it.next().unwrap().parse::<i64>().unwrap();
        a[i] = it.next().unwrap().parse::<i64>().unwrap();
        d[i] = it.next().unwrap().parse::<i64>().unwrap();
        s[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    let mut turn: i64 = 0;
    let mut ans: i64 = 0;
    let mut pairs: Vec<Pair> = Vec::new();

    // Phase 1: initial aggregation and collection
    for i in 1..=n {
        let k = a[i] - d[0];
        if k <= 0 {
            continue; // irrelevant entity
        }
        if a[0] <= d[i] {
            // Unsatisfiable condition for a relevant entity
            print!("-1\n");
            return;
        }

        turn += k;
        if s[i] > s[0] {
            ans += k;
        }

        // t(i) = ceil((h[i] - d[i]) / (a[0] - d[i])) using the given integer formula.
        let denom = a[0] - d[i];
        let t = (h[i] - d[i] + a[0] - 1) / denom;

        pairs.push(Pair { t, k });
    }

    // Phase 2: sort by decreasing ratio t/k using cross-multiplication:
    // p1 before p2 when t1*k2 > t2*k1.
    // Ties are left unspecified by the spec; we return Ordering::Equal for ties.
    pairs.sort_unstable_by(|p1, p2| {
        let left = p1.t * p2.k;
        let right = p2.t * p1.k;
        if left > right {
            Ordering::Less // "before" => earlier in ascending sort order
        } else if left < right {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    for p in pairs {
        ans += turn * p.t - p.k;
        if h[0] <= ans {
            print!("-1\n");
            return;
        }
        turn -= p.k;
    }

    print!("{ans}\n");
}
