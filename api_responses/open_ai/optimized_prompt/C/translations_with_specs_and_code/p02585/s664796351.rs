use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read N and K
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Preconditions
    assert!(n > 0, "N must be a positive integer.");
    assert!(k >= 0, "K must be a non-negative integer.");

    // Read positions P and costs C
    let mut p = vec![0; n + 1]; // Using 1-based indexing
    let mut c = vec![0; n + 1]; // Using 1-based indexing

    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        p[i] = line.trim().parse().unwrap();
    }

    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        c[i] = line.trim().parse().unwrap();
    }

    let mut ans: i64 = i64::MIN; // Initialize to minimum possible value

    for i in 1..=n {
        let mut dist = vec![0; n + 1];
        let mut tmp = vec![0; n + 1];
        
        let mut max: i64 = i64::MIN;
        let mut j = i;
        let mut k = 1;

        dist[j] = k;

        while k <= k && dist[p[j]] == 0 {
            dist[p[j]] = k;
            tmp[p[j]] = tmp[j] + c[p[j]];
            if tmp[p[j]] > max {
                max = tmp[p[j]];
            }
            j = p[j];
            k += 1;
        }

        if k > k || tmp[j] + c[p[j]] <= tmp[p[j]] {
            if max > ans {
                ans = max;
            }
            continue;
        }

        let period = dist[j] - dist[p[j]] + 1;
        let loop_value = tmp[j] + c[p[j]] - tmp[p[j]];
        j = p[j];
        tmp[j] += loop_value * ((k - dist[j] + 1) / period) as i64;

        if tmp[j] > max {
            max = tmp[j];
        }

        let mut k_iter = j;
        while p[j] != k_iter {
            tmp[p[j]] += loop_value * ((k - dist[p[j]] + 1) / period) as i64;
            if tmp[p[j]] > max {
                max = tmp[p[j]];
            }
            j = p[j];
        }

        if max > ans {
            ans = max;
        }
    }

    println!("{}", ans);
}
