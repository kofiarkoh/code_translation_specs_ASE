use std::io::{self, BufRead};

#[derive(Debug)]
struct T {
    a: i64,
    d: i32,
}

fn in_value() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn cmp(a: &T, b: &T) -> std::cmp::Ordering {
    // Ensure both sides are i64 for multiplication
    let left = a.a * b.d as i64;
    let right = b.a * a.d as i64;
    left.cmp(&right)
}

fn main() {
    let n = in_value();
    let mut tbl: Vec<T> = Vec::new();
    let mut h = vec![0; n as usize + 1];
    let mut a = vec![0; n as usize + 1];
    let mut d = vec![0; n as usize + 1];
    let mut s = vec![0; n as usize + 1];

    for i in 0..=n {
        h[i] = in_value();
        a[i] = in_value();
        d[i] = in_value();
        s[i] = in_value();
    }

    let mut sz = 0;
    let mut turn = 0;
    let mut ans = 0;

    for i in 1..=n {
        let k = a[i] - d[0];
        if k <= 0 {
            continue;
        }
        if a[0] <= d[i] {
            println!("-1");
            return;
        }

        turn += k;
        if s[i] > s[0] {
            ans += k;
        }
        tbl.push(T {
            d: k,
            a: (h[i] - d[i] + a[0] - 1) / (a[0] - d[i]),
        });
        sz += 1;
    }

    tbl.sort_by(cmp);

    for i in 0..sz {
        ans += turn * tbl[i].a - tbl[i].d as i64; // Cast d to i64 for safe subtraction
        if h[0] <= ans {
            println!("-1");
            return;
        }
        turn -= tbl[i].d;
    }
    println!("{}", ans);
}
