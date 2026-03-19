use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Node {
    k: i32,
    v: i64,
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let m: i32 = it.next().unwrap().parse().unwrap();
    let k: i32 = it.next().unwrap().parse().unwrap();

    let n_usize = n as usize;

    let mut a: Vec<i32> = vec![0; n_usize];
    for i in 0..n_usize {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    // dp[j] = best value for selecting exactly current t elements ending at index j.
    // Initialize for t = 1: dp[j] = 1 * a[j] = a[j].
    let mut dp: Vec<i64> = a.iter().map(|&x| x as i64).collect();

    // Monotone deque storage (implemented with array + head/tail indices).
    let mut d: Vec<Node> = vec![Node { k: 0, v: 0 }; n_usize];

    // For t = 2..k, update dp in-place.
    for t in 2..=k {
        let mut f: usize = 0;
        let mut b: usize = 0;

        // Seed with index (t-2) and value dp[t-2], matching the C code.
        let seed_idx = t - 2; // i32
        d[0] = Node {
            k: seed_idx,
            v: dp[seed_idx as usize],
        };

        // j ranges from (t-1) to n-1 inclusive.
        let start_j = (t - 1) as usize;
        for j in start_j..n_usize {
            // Pop front if it is exactly out of window: k == j - m - 1
            // (This matches the C code's equality check.)
            let out_k = (j as i32) - m - 1;
            if d[f].k == out_k {
                f += 1;
            }

            // Best previous + t * a[j]
            let v = d[f].v + (t as i64) * (a[j] as i64);

            // Maintain deque monotonicity based on dp[j] (old value before overwrite).
            // While back value <= dp[j], pop back.
            while f <= b && d[b].v <= dp[j] {
                if b == 0 {
                    // After this, f <= b would be false (since b would underflow),
                    // so we can break safely.
                    b = 0;
                    // Mark as empty by setting f > b.
                    f = 1;
                    break;
                } else {
                    b -= 1;
                }
            }

            // Push current (j, old dp[j]) to back.
            if f > b {
                // deque is empty; reset to single element at position 0
                f = 0;
                b = 0;
                d[0] = Node {
                    k: j as i32,
                    v: dp[j],
                };
            } else {
                b += 1;
                d[b] = Node {
                    k: j as i32,
                    v: dp[j],
                };
            }

            // Overwrite dp[j] with new value for this t.
            dp[j] = v;
        }
    }

    // Answer is max dp[i] for i in [k-1, n-1].
    let mut ans: i64 = 0;
    let start = (k - 1) as usize;
    for i in start..n_usize {
        if ans < dp[i] {
            ans = dp[i];
        }
    }

    print!("{ans}\n");
}
