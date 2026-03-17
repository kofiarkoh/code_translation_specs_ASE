// AOJ 2236 Rabbit Plays Games!
// 2018.2.10 bal4u

use std::io::{self, BufRead};

#[derive(Debug)]
struct T {
    a: i64,
    d: i32,
}

fn in_number() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut n = 0;
    let mut c = handle.bytes().next().unwrap().unwrap() as char;

    while c.is_digit(10) {
        n = (n << 3) + (n << 1) + (c as i32 - '0' as i32);
        c = handle.bytes().next().unwrap().unwrap() as char;
    }
    n
}

fn cmp(a: &T, b: &T) -> std::cmp::Ordering {
    let left = a.a * b.d;
    let right = b.a * a.d;
    left.cmp(&right)
}

fn main() {
    let n = in_number();
    let mut tbl: Vec<T> = Vec::new();
    let mut h = vec![0; 40002];
    let mut a = vec![0; 40002];
    let mut d = vec![0; 40002];
    let mut s = vec![0; 40002];

    for i in 0..=n {
        h[i] = in_number();
        a[i] = in_number();
        d[i] = in_number();
        s[i] = in_number();
    }

    let mut sz = 0;
    let mut turn = 0i64;
    let mut ans = 0i64;

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

    tbl.sort_by(|x, y| cmp(x, y));

    for i in 0..sz {
        ans += turn * tbl[i].a - tbl[i].d;
        if h[0] <= ans {
            println!("-1");
            return;
        }
        turn -= tbl[i].d;
    }
    println!("{}", ans);
}
