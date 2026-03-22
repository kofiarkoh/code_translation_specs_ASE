use std::io::{self, Read};

const HASH: usize = 100_003;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    // Matches C semantics: gcd(|a|,|b|) with recursion; here iterative.
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }
    while a != 0 {
        let t = b % a;
        b = a;
        a = t;
    }
    b
}

#[derive(Clone, Copy)]
struct Node {
    next: Option<usize>,
    v: i64,
    l: i64,
    r: i64,
}

fn main() {
    // Read exactly three signed integers (behavior on parse failure is unspecified by spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let mut x: i64 = it.next().unwrap().parse().unwrap();
    let mut d: i64 = it.next().unwrap().parse().unwrap();

    // Special cases per specification (and C code).
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

    // Normalize by gcd(|X|,|D|) as in C.
    let g = gcd(x, d);
    x /= g;
    d /= g;

    // Compute Y and ensure D > 0, matching C logic.
    let y: i64;
    if d < 0 {
        y = x;
        x = x + d * (n - 1);
        d = -d;
    } else {
        y = x + d * (n - 1);
    }

    // Hash table of linked lists, but implemented safely with indices into a Vec<Node>.
    // C uses fixed arrays: d[200001] and hash[HASH]. We mirror the fixed capacity expectation.
    // If N exceeds 200000, C would overflow; spec says behavior undefined. We do not add checks.
    let mut nodes: Vec<Node> = Vec::with_capacity(200_001);
    let mut hash: Vec<Option<usize>> = vec![None; HASH];

    // Initial node (i = 0)
    nodes.push(Node {
        next: None,
        v: 0,
        l: 0,
        r: d,
    });
    hash[0] = Some(0);

    for i in 1..=n {
        // L = (X*2 + D*(i-1)) * i / 2
        // R = (Y*2 - D*(i-1)) * i / 2 + D
        let l = (x * 2 + d * (i - 1)) * i / 2;
        let r = (y * 2 - d * (i - 1)) * i / 2 + d;

        // V = ((L % D) + D) % D  (ensure non-negative)
        let mut v = l % d;
        if v < 0 {
            v += d;
        }
        // Now v in [0, d-1]
        let h = (v.rem_euclid(HASH as i64)) as usize;

        // Find existing interval with same v that overlaps [l, r]
        let mut cur = hash[h];
        let mut found: Option<usize> = None;
        while let Some(idx) = cur {
            let node = nodes[idx];
            if node.v == v && node.l <= r && node.r >= l {
                found = Some(idx);
                break;
            }
            cur = node.next;
        }

        if let Some(idx) = found {
            // Merge intervals: expand to cover both.
            let nl = if nodes[idx].l < l { nodes[idx].l } else { l };
            let nr = if nodes[idx].r > r { nodes[idx].r } else { r };
            nodes[idx].l = nl;
            nodes[idx].r = nr;
        } else {
            // Insert new node at head of bucket.
            let next = hash[h];
            let idx = nodes.len();
            nodes.push(Node {
                next,
                v,
                l,
                r,
            });
            hash[h] = Some(idx);
        }
    }

    // Sum over all buckets: ans += (r - l) / D
    let mut ans: i64 = 0;
    for &head in &hash {
        let mut cur = head;
        while let Some(idx) = cur {
            let node = nodes[idx];
            ans += (node.r - node.l) / d;
            cur = node.next;
        }
    }

    print!("{}\n", ans);
}
