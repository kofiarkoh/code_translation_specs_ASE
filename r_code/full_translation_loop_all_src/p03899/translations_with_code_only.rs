use std::io::{self, Read};

type I32 = i32;
type I64 = i64;

#[derive(Clone, Copy)]
struct Node {
    k: I32,
    v: I64,
}

fn run() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf with %d).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: I32 = it.next().unwrap().parse().unwrap();
    let _m: I32 = it.next().unwrap().parse().unwrap();
    let k: I32 = it.next().unwrap().parse().unwrap();
    let m = _m;

    // calloc semantics: zero-initialized; but we fully fill `a` and `dp` anyway.
    let n_usize = n as usize;
    let mut a: Vec<I32> = vec![0; n_usize];
    for i in 0..n_usize {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    let mut dp: Vec<I64> = vec![0; n_usize];
    for i in 0..n_usize {
        dp[i] = a[i] as I64;
    }

    let mut d: Vec<Node> = vec![Node { k: 0, v: 0 }; n_usize];

    // Preserve C's behavior as closely as possible, including potential panics
    // if the C code would have performed out-of-bounds access for bad inputs.
    for i in 2..=k {
        let mut f: I32 = 0;
        let mut b: I32 = 0;

        let idx0 = (i - 2) as usize;
        d[0] = Node {
            k: i - 2,
            v: dp[idx0],
        };

        for j in (i - 1)..n {
            // if (d[f].k == j - m - 1) ++f;
            if d[f as usize].k == j - m - 1 {
                f += 1;
            }

            // i64 v = d[f].v + (i64) i * a[j];
            // Use wrapping ops to emulate typical two's-complement overflow behavior.
            let v = d[f as usize]
                .v
                .wrapping_add((i as I64).wrapping_mul(a[j as usize] as I64));

            // while (f <= b && d[b].v <= dp[j]) --b;
            while f <= b && d[b as usize].v <= dp[j as usize] {
                b -= 1;
            }

            // d[++b] = (node) {j, dp[j]};
            b += 1;
            d[b as usize] = Node {
                k: j,
                v: dp[j as usize],
            };

            // dp[j] = v;
            dp[j as usize] = v;
        }
    }

    let mut ans: I64 = 0;
    for i in (k - 1)..n {
        if ans < dp[i as usize] {
            ans = dp[i as usize];
        }
    }

    print!("{}\n", ans);
}

fn main() {
    run();
}
