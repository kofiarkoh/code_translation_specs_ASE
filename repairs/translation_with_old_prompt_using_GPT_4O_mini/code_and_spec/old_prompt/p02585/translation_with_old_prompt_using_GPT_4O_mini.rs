use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read N and K
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Read P and C arrays
    let mut p = vec![0; n + 1];
    let mut c = vec![0; n + 1];
    
    let p_line = lines.next().unwrap().unwrap();
    for (i, value) in p_line.split_whitespace().enumerate() {
        p[i + 1] = value.parse().unwrap();
    }

    let c_line = lines.next().unwrap().unwrap();
    for (i, value) in c_line.split_whitespace().enumerate() {
        c[i + 1] = value.parse().unwrap();
    }

    let mut ans = i64::MIN;
    
    for i in 1..=n {
        let mut dist = vec![0; n + 1];
        let mut tmp = vec![0; n + 1];

        let mut max = i64::MIN;
        let mut j = i;
        let mut steps = 1;
        dist[j] = steps;

        while steps <= k && dist[p[j]] == 0 {
            dist[p[j]] = steps + 1;
            tmp[p[j]] = tmp[j] + c[p[j]];
            if tmp[p[j]] > max {
                max = tmp[p[j]];
            }
            steps += 1;
            j = p[j];
        }

        if steps > k || tmp[j] + c[p[j]] <= tmp[p[j]] {
            if max > ans {
                ans = max;
            }
            continue;
        }

        let period = dist[j] - dist[p[j]] + 1;
        let loop_value = tmp[j] + c[p[j]] - tmp[p[j]];
        j = p[j];
        tmp[j] += loop_value * ((k - dist[j] + 1) / period as i64);
        if tmp[j] > max {
            max = tmp[j];
        }

        let mut k_temp = j;
        while p[j] != k_temp {
            j = p[j];
            tmp[p[j]] += loop_value * ((k - dist[p[j]] + 1) / period as i64);
            if tmp[p[j]] > max {
                max = tmp[p[j]];
            }
        }

        if max > ans {
            ans = max;
        }
    }

    println!("{}", ans);
}
