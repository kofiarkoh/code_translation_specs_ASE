use std::io::{self, BufRead};
use std::cmp::{max, min};

const MOD: i64 = 1_000_000_007; // 10^9 + 7
const MAX_N: usize = 100_010;

fn asc(a: &i64, b: &i64) -> std::cmp::Ordering {
    a.cmp(b)
}

fn dfs(f: usize, b: usize, l: usize, sign: i64, a: &[i64]) -> i64 {
    if f == l {
        let mut sum = a[l];
        if sign == 1 {
            for i in l + 1..=b {
                sum -= a[i];
            }
            return a[b] - sum;
        } else {
            for i in l + 1..=b {
                sum -= a[i];
            }
            return sum;
        }
    }
    if b == l + 1 {
        let mut sum = a[l + 1];
        if sign == 1 {
            for i in f..=l {
                sum -= a[i];
            }
            return sum;
        } else {
            for i in f..l {
                sum -= a[i];
            }
            return a[l] - sum;
        }
    }
    let x = dfs(f + 1, b - 1, l, -sign, a);
    if sign == 1 {
        return a[b] - a[f] - x;
    } else {
        return a[f] - a[b] - x;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse n");
    let mut a: Vec<i64> = Vec::with_capacity(n);
    let mut sum = 0;

    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().expect("Failed to parse value");
        a.push(value);
        sum += value.abs();
    }

    a.sort_by(asc);

    if a[0] >= 0 {
        println!("{}", sum - 2 * a[0]);
    } else if a[n - 1] <= 0 {
        println!("{}", sum + 2 * a[n - 1]);
    } else {
        println!("{}", sum);
        let mut l = 0;
        let mut r = n;
        while r - l > 1 {
            let med = (l + r) / 2;
            if a[med] < 0 {
                l = med;
            } else {
                r = med;
            }
        }
        dfs(0, n - 1, l, 1, &a);
    }
}
