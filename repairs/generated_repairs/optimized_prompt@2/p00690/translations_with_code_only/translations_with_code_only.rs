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

static mut TBL: [T; MAX_M] = [T { a: 0, b: 0, d: 0, f: 0 }; MAX_M];
static mut N: i32 = 0;
static mut M: i32 = 0;
static mut ANS: [i32; MAX_NUM] = [0; MAX_NUM];
static mut TMP: [i32; MAX_NUM] = [0; MAX_NUM];
static mut LEN: i32 = 0;
static mut NUM: i32 = 0;

fn combi(k: usize, a: i32, w: i32) {
    unsafe {
        for b in 1..=N {
            if b == a {
                continue;
            }
            for i in 0..M as usize {
                if (TBL[i].a == a && TBL[i].b == b || TBL[i].a == b && TBL[i].b == a) && TBL[i].f == 0 {
                    TBL[i].f = 1;
                    TMP[k] = b;
                    combi(k + 1, b, w + TBL[i].d);
                    TBL[i].f = 0;
                }
            }
        }
        if w > LEN {
            NUM = k as i32;
            LEN = w;
            ANS[..NUM as usize].copy_from_slice(&TMP[..NUM as usize]);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = nums.next().unwrap();
        let m = nums.next().unwrap();

        if n == 0 {
            break;
        }

        unsafe {
            N = n;
            M = m;
            for i in 0..M as usize {
                let line = reader.lines().next().unwrap().unwrap();
                let mut tbl_nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
                TBL[i].a = tbl_nums.next().unwrap();
                TBL[i].b = tbl_nums.next().unwrap();
                TBL[i].d = tbl_nums.next().unwrap();
                TBL[i].f = 0;
            }

            LEN = 0;
            NUM = 0;

            for a in 1..=N {
                for b in 1..=N {
                    if b == a {
                        continue;
                    }
                    for i in 0..M as usize {
                        if (TBL[i].a == a && TBL[i].b == b || TBL[i].b == a && TBL[i].a == b) && TBL[i].f == 0 {
                            TBL[i].f = 1;
                            TMP[0] = a;
                            TMP[1] = b;
                            combi(2, b, TBL[i].d);
                            TBL[i].f = 0;
                        }
                    }
                }
            }

            println!("{}", LEN);
            print!("{}", ANS[0]);
            for i in 1..NUM as usize {
                print!(" {}", ANS[i]);
            }
            println!();
        }
    }
}
