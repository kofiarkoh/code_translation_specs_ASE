use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i64 then cast as needed.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse::<usize>().unwrap(),
        None => return,
    };
    let k_in: i64 = it.next().unwrap().parse::<i64>().unwrap();
    let k_usize: usize = if k_in < 0 { 0 } else { k_in as usize };

    // 1-based arrays sized 5001 as in C.
    let mut p = vec![0usize; 5001];
    let mut c = vec![0i64; 5001];

    for i in 1..=n {
        p[i] = it.next().unwrap().parse::<usize>().unwrap();
    }
    for i in 1..=n {
        c[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    let mut dist = vec![0i64; 5001];
    let mut tmp = vec![0i64; 5001];

    // C: long long ans = -(1 << 30)
    let mut ans: i64 = -((1i64) << 30);

    for i in 1..=n {
        // Reset only 1..=N as in C.
        for j in 1..=n {
            dist[j] = 0;
            tmp[j] = 0;
        }

        let mut j: usize = i;
        let mut kk: i64 = 1;
        dist[j] = kk;
        let mut maxv: i64 = -((1i64) << 30);

        // for (k = 1, j = i, dist[j] = k, max = ...; k <= K && dist[P[j]] == 0; j = P[j]) { ... }
        while (kk as i64) <= k_in && dist[p[j]] == 0 {
            kk += 1;
            let pj = p[j];
            dist[pj] = kk;
            tmp[pj] = tmp[j] + c[pj];
            if tmp[pj] > maxv {
                maxv = tmp[pj];
            }
            j = pj;
        }

        // if (k > K || tmp[j] + C[P[j]] <= tmp[P[j]]) { ... }
        // Note: kk is i64, K is i64.
        let pj = p[j];
        if kk > k_in || tmp[j] + c[pj] <= tmp[pj] {
            if maxv > ans {
                ans = maxv;
            }
            continue;
        }

        // period = dist[j] - dist[P[j]] + 1;
        let period: i64 = dist[j] - dist[pj] + 1;
        // loop = tmp[j] + C[P[j]] - tmp[P[j]];
        let loop_sum: i64 = tmp[j] + c[pj] - tmp[pj];

        // j = P[j];
        j = pj;

        // tmp[j] += loop * ((K - dist[j] + 1) / period);
        // Use i64 arithmetic; division truncates toward zero like C for positive values.
        let add0 = loop_sum * ((k_in - dist[j] + 1) / period);
        tmp[j] += add0;
        if tmp[j] > maxv {
            maxv = tmp[j];
        }

        // for (k = j; P[j] != k; j = P[j]) { ... }
        let start = j;
        while p[j] != start {
            let pj2 = p[j];
            let add = loop_sum * ((k_in - dist[pj2] + 1) / period);
            tmp[pj2] += add;
            if tmp[pj2] > maxv {
                maxv = tmp[pj2];
            }
            j = pj2;
        }

        if maxv > ans {
            ans = maxv;
        }
    }

    // Exact output formatting: "%lld\n"
    print!("{ans}\n");
    // stdout flush is implicit on program exit; C calls fflush(stdout) but not required here.
}
