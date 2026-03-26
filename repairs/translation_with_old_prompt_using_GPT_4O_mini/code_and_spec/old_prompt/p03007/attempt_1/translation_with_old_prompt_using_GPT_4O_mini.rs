use std::io::{self, BufRead};
use std::cmp::{max, min};

const MOD: i64 = 1_000_000_007; // 10^9 + 7

fn asc(a: &i64, b: &i64) -> std::cmp::Ordering {
    a.cmp(b)
}

fn dfs(f: usize, b: usize, l: usize, sign: i64, a: &[i64]) -> i64 {
    if f == l {
        let mut sum = a[l];
        if sign == 1 {
            for i in (l + 1)..(b) {
                println!("{} {}", sum, a[i]);
                sum -= a[i];
            }
            println!("{} {}", a[b], sum);
            sum = a[b] - sum;
            return sum;
        } else {
            for i in (l + 1)..=b {
                println!("{} {}", sum, a[i]);
                sum -= a[i];
            }
            return sum;
        }
    }
    if b == l + 1 {
        let mut sum = a[l + 1];
        if sign == 1 {
            for i in f..=l {
                println!("{} {}", sum, a[i]);
                sum -= a[i];
            }
            return sum;
        } else {
            for i in f..l {
                println!("{} {}", sum, a[i]);
                sum -= a[i];
            }
            println!("{} {}", a[l], sum);
            sum = a[l] - sum;
            return sum;
        }
    }
    if sign == 1 {
        println!("{} {}", a[b], a[f]);
        let x = dfs(f + 1, b - 1, l, -sign, a);
        println!("{} {}", a[b] - a[f], x);
        return a[b] - a[f] - x;
    } else {
        println!("{} {}", a[f], a[b]);
        let x = dfs(f + 1, b - 1, l, -sign, a);
        println!("{} {}", a[f] - a[b], x);
        return a[f] - a[b] - x;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n);
    let mut sum = 0;

    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
        sum += value.abs();
    }

    a.sort_by(asc);

    if a[0] >= 0 {
        println!("{}", sum - a[0] - a[0]);
        let mut x = a[0];
        for i in 1..(n - 1) {
            println!("{} {}", x, a[i]);
            x -= a[i];
        }
        println!("{} {}", a[n - 1], x);
    } else if a[n - 1] <= 0 {
        println!("{}", sum + a[n - 1] + a[n - 1]);
        let mut x = a[n - 1];
        for i in 0..(n - 1) {
            println!("{} {}", x, a[i]);
            x -= a[i];
        }
    } else {
        println!("{}", sum);
        let (mut l, mut r) = (0, n);
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
