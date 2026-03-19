use std::io::{self, Read};

const HASH: usize = 100_003;

#[derive(Clone, Copy)]
struct Node {
    next: Option<usize>,
    v: i64,
    l: i64,
    r: i64,
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    // Mirrors the C recursive gcd with swap via XOR (but safely/idiomatically).
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn main() {
    // Read all input, parse as i64 (C: long long).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let mut x: i64 = it.next().unwrap().parse().unwrap();
    let mut d: i64 = it.next().unwrap().parse().unwrap();

    if x == 0 || d == 0 {
        if x == 0 && d == 0 {
            print!("1\n");
        } else if d == 0 {
            print!("{}\n", n + 1);
        } else {
            // n*(n-1)/2 + 1
            print!("{}\n", n * (n - 1) / 2 + 1);
        }
        return;
    }

    // C uses llabs; i64::abs can overflow on MIN, but inputs are assumed to fit typical contest ranges.
    // To preserve behavior as closely as possible without unsafe, use wrapping_abs.
    let g = gcd(x.wrapping_abs(), d.wrapping_abs());
    x /= g;
    d /= g;

    let y: i64;
    if d < 0 {
        y = x;
        x = x + d * (n - 1);
        d *= -1;
    } else {
        y = x + d * (n - 1);
    }

    // Hash table of linked lists via indices into `nodes`.
    let mut head: Vec<Option<usize>> = vec![None; HASH];
    let mut nodes: Vec<Node> = vec![
        Node {
            next: None,
            v: 0,
            l: 0,
            r: d,
        };
        (n as usize) + 1
    ];

    head[0] = Some(0);

    for i in 1..=n {
        // Use i128 for intermediate products to avoid Rust overflow panics in debug;
        // then cast back to i64, matching typical C long long arithmetic intent.
        let ii = i as i128;
        let x2 = (x as i128) * 2;
        let y2 = (y as i128) * 2;
        let dd = d as i128;
        let im1 = (i - 1) as i128;

        let l = ((x2 + dd * im1) * ii / 2) as i64;
        let r = ((y2 - dd * im1) * ii / 2 + dd) as i64;

        // V = (L % D + D) % D; with D > 0 here.
        let mut v = l % d;
        v = (v + d) % d;

        let h = (v.rem_euclid(HASH as i64)) as usize;

        // Find existing interval to merge.
        let mut cur = head[h];
        let mut found: Option<usize> = None;
        while let Some(idx) = cur {
            let p = nodes[idx];
            if p.v == v && p.l <= r && p.r >= l {
                found = Some(idx);
                break;
            }
            cur = p.next;
        }

        if let Some(idx) = found {
            // Merge intervals.
            if l < nodes[idx].l {
                nodes[idx].l = l;
            }
            if r > nodes[idx].r {
                nodes[idx].r = r;
            }
        } else {
            let ui = i as usize;
            nodes[ui].v = v;
            nodes[ui].l = l;
            nodes[ui].r = r;
            nodes[ui].next = head[h];
            head[h] = Some(ui);
        }
    }

    let mut ans: i64 = 0;
    for h in 0..HASH {
        let mut cur = head[h];
        while let Some(idx) = cur {
            let p = nodes[idx];
            ans += (p.r - p.l) / d;
            cur = p.next;
        }
    }

    print!("{}\n", ans);
}
