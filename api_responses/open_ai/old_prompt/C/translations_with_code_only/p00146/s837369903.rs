use std::io::{self, BufRead};

const MAX_N: usize = 110;
const MAX_MASK: usize = 1 << 16;
const INF: f64 = 1e18;

fn f(i: usize, j: usize, k: usize, n: usize, b: &[i32], c: &[i32]) -> f64 {
    if j == 0 {
        return 0.0;
    }
    let mut w = 0;
    for l in 0..n {
        if j & (1 << l) != 0 {
            w += c[l] * 20;
        }
    }
    (b[k] - b[i]).abs() as f64 / 2000.0 * (w as f64 + 70.0)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = [0; MAX_N];
    let mut b = [0; MAX_N];
    let mut c = [0; MAX_N];
    let mut d = [[INF; MAX_MASK]; MAX_N];
    let mut fr = [[-1; MAX_MASK]; MAX_N];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        a[i] = nums[0];
        b[i] = nums[1];
        c[i] = nums[2];
    }

    for i in 0..n {
        d[i][1 << i] = 0.0;
    }

    for j in 0..(1 << n) {
        for i in 0..n {
            if (1 << i) & j == 0 {
                continue;
            }
            for k in 0..n {
                if j & (1 << k) != 0 {
                    continue;
                }
                let new_cost = d[i][j] + f(i, j, k, n, &b, &c);
                if d[k][j + (1 << k)] > new_cost {
                    d[k][j + (1 << k)] = new_cost;
                    fr[k][j + (1 << k)] = i as i32;
                }
            }
        }
    }

    let mut k = (1 << n) - 1;
    let mut j = 0;
    for i in 0..n {
        if d[i][k] < d[j][k] {
            j = i;
        }
    }

    let mut r = [0; MAX_N];
    r[n - 1] = j;
    for i in 1..n {
        r[n - i - 1] = fr[r[n - i] as usize][k];
        k -= 1 << r[n - i];
    }

    for i in 0..n - 1 {
        print!("{} ", a[r[i]]);
    }
    println!("{}", a[r[n - 1]]);
}
