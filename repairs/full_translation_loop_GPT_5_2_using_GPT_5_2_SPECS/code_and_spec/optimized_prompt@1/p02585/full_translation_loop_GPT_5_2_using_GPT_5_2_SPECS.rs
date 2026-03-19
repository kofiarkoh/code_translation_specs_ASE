use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let k_limit: i64 = it.next().unwrap().parse().unwrap();

    // 1-based arrays to match C semantics
    let mut p = vec![0usize; n + 1];
    let mut c = vec![0i64; n + 1];

    for i in 1..=n {
        p[i] = it.next().unwrap().parse::<usize>().unwrap();
    }
    for i in 1..=n {
        c[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    // Spec note: C initializes ans/max to -(1<<30) (about -1e9), not i64::MIN.
    // The spec explicitly includes the assumption that the true maximum is not less than ~-2^30.
    let neg_init: i64 = -(1i64 << 30);
    let mut ans: i64 = neg_init;

    let mut dist = vec![0i64; n + 1];
    let mut tmp = vec![0i64; n + 1];

    for start in 1..=n {
        // Reset per start (O(N) like the C code)
        for j in 1..=n {
            dist[j] = 0;
            tmp[j] = 0;
        }

        let mut j: usize = start;
        let mut k: i64 = 1;
        dist[j] = k;
        let mut maxv: i64 = neg_init;

        // Follow path until K moves reached or next would revisit a seen node
        while k <= k_limit && dist[p[j]] == 0 {
            k += 1;
            let nxt = p[j];
            dist[nxt] = k;
            tmp[nxt] = tmp[j] + c[nxt];
            if tmp[nxt] > maxv {
                maxv = tmp[nxt];
            }
            j = nxt;
        }

        // If we already exceeded K, or the cycle sum is non-positive, just use maxv.
        // This matches the C condition:
        //   if (k > K || tmp[j] + C[P[j]] <= tmp[P[j]]) { ... }
        if k > k_limit || tmp[j] + c[p[j]] <= tmp[p[j]] {
            if maxv > ans {
                ans = maxv;
            }
            continue;
        }

        // We found a beneficial cycle; compute its period and total loop gain.
        let period: i64 = dist[j] - dist[p[j]] + 1;
        let loop_gain: i64 = tmp[j] + c[p[j]] - tmp[p[j]];

        // Move j into the cycle start as in C: j = P[j];
        j = p[j];

        // Add as many full loops as possible from each cycle node, respecting remaining moves.
        // C uses: tmp[node] += loop * ((K - dist[node] + 1) / period)
        let loops_here = (k_limit - dist[j] + 1) / period;
        tmp[j] += loop_gain * loops_here;
        if tmp[j] > maxv {
            maxv = tmp[j];
        }

        let cycle_start = j;
        while p[j] != cycle_start {
            let nxt = p[j];
            let loops_nxt = (k_limit - dist[nxt] + 1) / period;
            tmp[nxt] += loop_gain * loops_nxt;
            if tmp[nxt] > maxv {
                maxv = tmp[nxt];
            }
            j = nxt;
        }

        if maxv > ans {
            ans = maxv;
        }
    }

    // Exact I/O: one line with the answer
    println!("{ans}");
}
