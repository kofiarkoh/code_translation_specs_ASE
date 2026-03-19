use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Node {
    k: i32,
    v: i64,
}

fn main() {
    // Read entire stdin
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

    // dp initialized to a[i] (as i64)
    let mut dp: Vec<i64> = a.iter().map(|&x| x as i64).collect();

    // Deque-like structure implemented with array + front/back indices.
    // C code allocates size n and uses indices f/b; we do the same.
    let mut d: Vec<Node> = vec![Node { k: 0, v: 0 }; n_usize];

    // Main DP transitions
    for i in 2..=k {
        let mut f: i32 = 0;
        let mut b: i32 = 0;

        // d[0] = { i-2, dp[i-2] }
        // Note: This assumes i-2 is a valid index into dp, which holds for the intended usage.
        let idx = (i - 2) as usize;
        d[0] = Node {
            k: i - 2,
            v: dp[idx],
        };

        // for (j = i-1; j < n; ++j)
        for j in (i - 1)..n {
            // if (d[f].k == j - m - 1) ++f;
            // (This is exactly as in C; it advances the front when the stored index expires.)
            if d[f as usize].k == j - m - 1 {
                f += 1;
            }

            // v = d[f].v + i * a[j]
            let v = d[f as usize].v + (i as i64) * (a[j as usize] as i64);

            // while (f <= b && d[b].v <= dp[j]) --b;
            while f <= b && d[b as usize].v <= dp[j as usize] {
                b -= 1;
            }

            // d[++b] = { j, dp[j] };
            b += 1;
            d[b as usize] = Node {
                k: j,
                v: dp[j as usize],
            };

            // dp[j] = v;
            dp[j as usize] = v;
        }
    }

    // ans = max_{i=k-1..n-1} dp[i]
    let mut ans: i64 = 0;
    for i in (k - 1)..n {
        let val = dp[i as usize];
        if ans < val {
            ans = val;
        }
    }

    print!("{ans}\n");
}
