use std::io::{self, BufRead};

const INF: i64 = 0x1010101010101010;

#[derive(Debug)]
struct T {
    a: i64,
    b: i32,
}

fn in_int() -> i32 {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut n = 0;
    let mut c = handle.by_ref().bytes().next().unwrap().unwrap() as char;

    while c.is_digit(10) {
        n = 10 * n + (c as i32 - '0' as i32);
        c = handle.by_ref().bytes().next().unwrap().unwrap() as char;
    }
    n
}

fn big() -> i64 {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut n = 0;
    let mut c = handle.by_ref().bytes().next().unwrap().unwrap() as char;

    while c.is_digit(10) {
        n = 10 * n + (c as i64 - '0' as i64);
        c = handle.by_ref().bytes().next().unwrap().unwrap() as char;
    }
    n
}

fn cmp(a: &T, b: &T) -> std::cmp::Ordering {
    if a.a == b.a {
        return a.b.cmp(&b.b);
    }
    a.a.cmp(&b.a)
}

fn main() {
    let n = in_int();
    let mut tbl: Vec<T> = Vec::with_capacity(n as usize);
    let mut s: Vec<i64> = vec![0; (n + 1) as usize];

    for _ in 0..n {
        let a = big();
        let b = in_int();
        tbl.push(T { a, b });
    }

    tbl.sort_by(cmp);

    let mut min = INF;
    let mut max = -INF;

    for i in 1..=n {
        let t = s[i as usize - 1] - tbl[i as usize - 1].a;
        if t < min {
            min = t;
        }
        s[i as usize] = s[i as usize - 1] + tbl[i as usize - 1].b;
        let t = s[i as usize] - tbl[i as usize - 1].a - min;
        if t > max {
            max = t;
        }
    }
    println!("{}", max);
}
