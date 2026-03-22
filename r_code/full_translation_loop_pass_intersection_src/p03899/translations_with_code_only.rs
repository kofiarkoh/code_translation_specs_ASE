use std::io::{self, Read};

type I32 = i32;
type I64 = i64;

#[derive(Clone, Copy)]
struct Node {
    k: I32,
    v: I64,
}

fn run() {
    // Read entire stdin, parse as whitespace-separated integers (matches scanf behavior for %d-like).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: I32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // If no input, C would have undefined behavior; we just exit.
    };
    let _m: I32 = it.next().unwrap().parse().unwrap();
    let k: I32 = it.next().unwrap().parse().unwrap();
    let m = _m;

    // calloc semantics: zero-initialized.
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

    // Preserve C's signed overflow behavior as wrapping (best-effort emulation).
    for i in 2..=k {
        let mut f: I32 = 0;
        let mut b: I32 = 0;

        // d[0] = {i-2, dp[i-2]}
        let idx0 = (i - 2) as usize;
        d[0] = Node {
            k: i - 2,
            v: dp[idx0],
        };

        for j in (i - 1)..n {
            // if (d[f].k == j - m - 1) ++f;
            let f_usize = f as usize;
            if d[f_usize].k == j.wrapping_sub(m).wrapping_sub(1) {
                f = f.wrapping_add(1);
            }

            // v = d[f].v + (i64)i * a[j]
            let f_usize2 = f as usize;
            let j_usize = j as usize;
            let v = d[f_usize2]
                .v
                .wrapping_add((i as I64).wrapping_mul(a[j_usize] as I64));

            // while (f <= b && d[b].v <= dp[j]) --b;
            while f <= b {
                let b_usize = b as usize;
                if d[b_usize].v <= dp[j_usize] {
                    b = b.wrapping_sub(1);
                } else {
                    break;
                }
            }

            // d[++b] = {j, dp[j]};
            b = b.wrapping_add(1);
            let b_usize2 = b as usize;
            d[b_usize2] = Node { k: j, v: dp[j_usize] };

            // dp[j] = v;
            dp[j_usize] = v;
        }
    }

    let mut ans: I64 = 0;
    for i in (k - 1)..n {
        let val = dp[i as usize];
        if ans < val {
            ans = val;
        }
    }

    print!("{ans}\n");
}

fn main() {
    run();
}
