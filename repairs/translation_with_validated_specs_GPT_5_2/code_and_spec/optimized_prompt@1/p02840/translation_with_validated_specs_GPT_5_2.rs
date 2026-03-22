use std::io::{self, Read};

fn gcd(mut a: i64, mut b: i64) -> i64 {
    // Euclidean algorithm; works for non-negative inputs.
    while a != 0 {
        let t = b % a;
        b = a;
        a = t;
    }
    b
}

fn main() {
    const HASH: usize = 100_003;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let mut x: i64 = it.next().unwrap().parse().unwrap();
    let mut d: i64 = it.next().unwrap().parse().unwrap();

    // Special cases per specification.
    if x == 0 || d == 0 {
        if x == 0 && d == 0 {
            print!("1\n");
        } else if d == 0 {
            print!("{}\n", n + 1);
        } else {
            // x == 0, d != 0
            print!("{}\n", n * (n - 1) / 2 + 1);
        }
        return;
    }

    // Normalize by gcd(|X|, |D|) as in C code.
    let g = gcd(x.abs(), d.abs());
    x /= g;
    d /= g;

    // Ensure D > 0 by possibly reversing the progression endpoints.
    let y: i64;
    if d < 0 {
        y = x;
        x = x + d * (n - 1);
        d = -d;
    } else {
        y = x + d * (n - 1);
    }

    // Each bucket stores disjoint/merged intervals for a given residue v (mod D).
    // C code uses a custom hash table with linked lists and merges intervals if overlapping.
    // We replicate that behavior with Vec buckets and linear scan within a bucket.
    #[derive(Clone, Copy)]
    struct Entry {
        v: i64,
        l: i64,
        r: i64,
    }

    let mut buckets: Vec<Vec<Entry>> = vec![Vec::new(); HASH];

    // Initial entry: v=0, interval [0, D] (half-open in effect; see final counting).
    buckets[0].push(Entry { v: 0, l: 0, r: d });

    for i in 1..=n {
        // L = (X*2 + D*(i-1)) * i / 2
        // R = (Y*2 - D*(i-1)) * i / 2 + D
        let ii = i;
        let l = (x * 2 + d * (ii - 1)) * ii / 2;
        let r = (y * 2 - d * (ii - 1)) * ii / 2 + d;

        // V = ((L % D) + D) % D
        let mut v = l % d;
        if v < 0 {
            v += d;
        }
        let h = (v as i128).rem_euclid(HASH as i128) as usize;

        // Find an existing entry with same v and overlapping interval, else insert.
        let mut found_idx: Option<usize> = None;
        for (idx, e) in buckets[h].iter().enumerate() {
            if e.v == v && e.l <= r && e.r >= l {
                found_idx = Some(idx);
                break;
            }
        }

        if let Some(idx) = found_idx {
            let e = &mut buckets[h][idx];
            if l < e.l {
                e.l = l;
            }
            if r > e.r {
                e.r = r;
            }
        } else {
            buckets[h].push(Entry { v, l, r });
        }
    }

    // Sum counts: for each merged interval, add (r - l) / D.
    let mut ans: i64 = 0;
    for b in &buckets {
        for e in b {
            ans += (e.r - e.l) / d;
        }
    }

    print!("{}\n", ans);
}
