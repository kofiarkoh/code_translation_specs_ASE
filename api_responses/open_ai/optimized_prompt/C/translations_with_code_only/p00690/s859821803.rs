// Aizu 1108: A Long Ride on a Railway
// 2017.9.16 bal4u@uu

use std::io::{self, BufRead};

#[derive(Clone, Copy)]
struct T {
    a: i32,
    b: i32,
    d: i32,
    f: i32,
}

const MAX_M: usize = 25;
const MAX_NUM: usize = 12;

fn combi(k: usize, a: i32, w: i32, tbl: &mut [T; MAX_M], ans: &mut [i32; MAX_NUM], tmp: &mut [i32; MAX_NUM], len: &mut i32, num: &mut i32, n: i32) {
    for b in 1..=n {
        if b == a {
            continue;
        }
        for i in 0..MAX_M {
            if (tbl[i].a == a && tbl[i].b == b || tbl[i].a == b && tbl[i].b == a) && tbl[i].f == 0 {
                tbl[i].f = 1;
                tmp[k] = b;
                combi(k + 1, b, w + tbl[i].d, tbl, ans, tmp, len, num, n);
                tbl[i].f = 0;
            }
        }
    }
    if w > *len {
        *num = k;
        *len = w;
        ans[..*num].copy_from_slice(&tmp[..*num]);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut tbl: [T; MAX_M] = Default::default();
    let mut ans: [i32; MAX_NUM] = Default::default();
    let mut tmp: [i32; MAX_NUM] = Default::default();
    let mut len = 0;
    let mut num = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let m: i32 = iter.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        for i in 0..m {
            let a: i32 = iter.next().unwrap().parse().unwrap();
            let b: i32 = iter.next().unwrap().parse().unwrap();
            let d: i32 = iter.next().unwrap().parse().unwrap();
            tbl[i as usize] = T { a, b, d, f: 0 };
        }

        for a in 1..=n {
            for b in 1..=n {
                if b == a {
                    continue;
                }
                for i in 0..m {
                    if (tbl[i as usize].a == a && tbl[i as usize].b == b || tbl[i as usize].b == a && tbl[i as usize].a == b) && tbl[i as usize].f == 0 {
                        tbl[i as usize].f = 1;
                        tmp[0] = a;
                        tmp[1] = b;
                        combi(2, b, tbl[i as usize].d, &mut tbl, &mut ans, &mut tmp, &mut len, &mut num, n);
                        tbl[i as usize].f = 0;
                    }
                }
            }
        }

        println!("{}", len);
        print!("{}", ans[0]);
        for i in 1..num {
            print!(" {}", ans[i]);
        }
        println!();
    }
}
