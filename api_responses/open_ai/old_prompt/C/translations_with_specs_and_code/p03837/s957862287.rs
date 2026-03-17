use std::io::{self, BufRead};

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    let mut a = vec![0; m as usize];
    let mut b = vec![0; m as usize];
    let mut c = vec![0; m as usize];
    let mut dist = vec![vec![i32::MAX; n as usize]; n as usize];

    for i in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut edge_data = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        a[i as usize] = edge_data.next().unwrap() - 1; // Convert to 0-based index
        b[i as usize] = edge_data.next().unwrap() - 1; // Convert to 0-based index
        c[i as usize] = edge_data.next().unwrap();
    }

    for i in 0..n {
        dist[i as usize][i as usize] = 0;
    }

    for i in 0..m {
        dist[a[i as usize] as usize][b[i as usize] as usize] = min(dist[a[i as usize] as usize][b[i as usize] as usize], c[i as usize]);
        dist[b[i as usize] as usize][a[i as usize] as usize] = min(dist[b[i as usize] as usize][a[i as usize] as usize], c[i as usize]);
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i as usize][j as usize] = min(dist[i as usize][j as usize], dist[i as usize][k as usize] + dist[k as usize][j as usize]);
            }
        }
    }

    let mut ans = m;

    for i in 0..m {
        let mut f = true;
        for j in 0..n {
            if dist[j as usize][a[i as usize] as usize] + c[i as usize] == dist[j as usize][b[i as usize] as usize] {
                f = false;
                break;
            }
        }
        if !f {
            ans -= 1;
        }
    }

    println!("{}", ans);
}
