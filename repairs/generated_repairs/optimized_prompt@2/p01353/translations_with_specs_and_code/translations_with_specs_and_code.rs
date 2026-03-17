// AOJ 2236 Rabbit Plays Games!
// 2018.2.10 bal4u

use std::io::{self, BufRead};

#[derive(Debug)]
struct T {
    a: i64,
    d: i64, // Change d to i64
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of game parameters
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut tbl: Vec<T> = Vec::new();
    let mut h = vec![0; n + 1];
    let mut a = vec![0; n + 1];
    let mut d = vec![0; n + 1];
    let mut s = vec![0; n + 1];

    // Read the game parameters
    for i in 0..=n {
        let params: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        h[i] = params[0];
        a[i] = params[1];
        d[i] = params[2];
        s[i] = params[3];
    }

    let mut sz = 0;
    let mut turn: i64 = 0;
    let mut ans: i64 = 0;

    for i in 1..=n {
        let k = a[i] - d[0];
        if k <= 0 {
            continue;
        }
        if a[0] <= d[i] {
            println!("-1");
            return;
        }

        turn += k as i64; // Cast k to i64
        if s[i] > s[0] {
            ans += k as i64; // Cast k to i64
        }
        tbl.push(T {
            d: k as i64, // Cast k to i64
            a: (h[i] - d[i] + a[0] - 1) / (a[0] - d[i]) as i64, // Cast to i64
        });
        sz += 1;
    }

    // Sort the table based on the custom comparator
    tbl.sort_by(|x, y| {
        let left = x.a * y.d; // No need to cast y.d to i64
        let right = y.a * x.d; // No need to cast x.d to i64
        if left > right {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });

    for i in 0..sz {
        ans += turn * tbl[i].a - tbl[i].d; // No need to cast tbl[i].d to i64
        if h[0] <= ans {
            println!("-1");
            return;
        }
        turn -= tbl[i].d; // No need to cast tbl[i].d to i64
    }

    println!("{}", ans);
}
