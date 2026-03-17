use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Failed to parse n");
    let k: usize = parts.next().unwrap().parse().expect("Failed to parse k");

    let mut p = vec![0; n + 1];
    let mut c = vec![0; n + 1];

    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        p[i] = line.trim().parse().expect("Failed to parse p[i]");
    }

    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        c[i] = line.trim().parse().expect("Failed to parse c[i]");
    }

    let mut ans: i64 = -(1 << 30);
    
    for i in 1..=n {
        let mut dist = vec![0; n + 1];
        let mut tmp = vec![0; n + 1];
        
        let mut j = i;
        let mut max: i64 = -(1 << 30);
        let mut k_count = 1; // Renamed to avoid shadowing
        dist[j] = k_count;

        while k_count <= k && dist[p[j]] == 0 {
            dist[p[j]] = k_count;
            tmp[p[j]] = tmp[j] + c[p[j]];
            if tmp[p[j]] > max {
                max = tmp[p[j]];
            }
            j = p[j];
            k_count += 1;
        }

        if k_count > k || tmp[j] + c[p[j]] <= tmp[p[j]] {
            if max > ans {
                ans = max;
            }
            continue;
        }

        let period = dist[j] - dist[p[j]] + 1;
        let loop_value = tmp[j] + c[p[j]] - tmp[p[j]];
        j = p[j];
        tmp[j] += loop_value * ((k - dist[j] + 1) as i64 / period as i64);
        if tmp[j] > max {
            max = tmp[j];
        }

        let mut start = j;
        while p[j] != start {
            j = p[j];
            tmp[p[j]] += loop_value * ((k - dist[p[j]] + 1) as i64 / period as i64);
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
