use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = nums.next().unwrap();
    let k = nums.next().unwrap();

    let mut p = vec![0; (n + 1) as usize];
    let mut c = vec![0; (n + 1) as usize];

    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        p[i as usize] = line.trim().parse::<i32>().unwrap();
    }

    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        c[i as usize] = line.trim().parse::<i32>().unwrap();
    }

    let mut ans: i64 = -(1 << 30) as i64;

    for i in 1..=n {
        let mut dist = vec![0; (n + 1) as usize];
        let mut tmp = vec![0; (n + 1) as usize];
        let mut max: i64 = -(1 << 30) as i64;

        let mut j = i;
        let mut k = 1;
        dist[j as usize] = k;
        
        while k <= k && dist[p[j as usize]] == 0 {
            dist[p[j as usize]] = k;
            tmp[p[j as usize]] = tmp[j as usize] + c[p[j as usize]];
            if tmp[p[j as usize]] > max {
                max = tmp[p[j as usize]];
            }
            j = p[j as usize];
            k += 1;
        }

        if k > k || tmp[j as usize] + c[p[j as usize]] <= tmp[p[j as usize]] {
            if max > ans {
                ans = max;
            }
            continue;
        }

        let period = dist[j as usize] - dist[p[j as usize]] + 1;
        let loop_value = tmp[j as usize] + c[p[j as usize]] - tmp[p[j as usize]];
        j = p[j as usize];
        tmp[j as usize] += loop_value * ((k - dist[j as usize] + 1) / period);

        if tmp[j as usize] > max {
            max = tmp[j as usize];
        }

        let mut k = j;
        while p[j as usize] != k {
            j = p[j as usize];
            tmp[p[j as usize]] += loop_value * ((k - dist[p[j as usize]] + 1) / period);
            if tmp[p[j as usize]] > max {
                max = tmp[p[j as usize]];
            }
        }

        if max > ans {
            ans = max;
        }
    }

    println!("{}", ans);
}
