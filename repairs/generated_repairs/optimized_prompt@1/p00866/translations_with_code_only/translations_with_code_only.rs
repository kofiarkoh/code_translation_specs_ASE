// AOJ 1287: Stopped Watches
// 2017.10.28 bal4u@uu

use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
struct T {
    t: i32,
    id: usize,
}

const MAX_SIZE: usize = 10000;
const MAX_HOURS: usize = 12;
const MAX_MINUTES: usize = 60;

static mut TBL: [T; MAX_SIZE] = [T { t: 0, id: 0 }; MAX_SIZE];
static mut SZ: usize = 0;
static mut MIN: i32 = 0;
static mut MAX: i32 = 0;
static mut A: [[i32; MAX_MINUTES]; MAX_HOURS] = [[0; MAX_MINUTES]; MAX_HOURS];
static C: [[usize; 3]; 6] = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];

fn check(id: usize, t: &[i32; 3]) {
    unsafe {
        for j in 0..60 {
            for i in 0..6 {
                let h = (t[C[i][0]] + j) % 60;
                let hh = h / 5;
                let m = (t[C[i][1]] + j) % 60;
                let s = (t[C[i][2]] + j) % 60;
                if A[hh][m] == h {
                    TBL[SZ].t = ((hh * 60) + m) * 60 + s;
                    TBL[SZ].id = id;
                    SZ += 1;
                }
            }
        }
    }
}

fn cmp(a: &T, b: &T) -> Ordering {
    a.t.cmp(&b.t)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    unsafe {
        for i in 0..MAX_HOURS {
            for j in 0..MAX_MINUTES {
                A[i][j] = (60 * i + j) / 12;
            }
        }
    }

    while let Some(Ok(line)) = lines.next() {
        let n: i32 = line.trim().parse().unwrap();
        if n <= 0 {
            break;
        }

        unsafe {
            SZ = 0;
            for i in 0..n {
                let t: Vec<i32> = lines.next().unwrap().trim().split_whitespace()
                    .map(|x| x.parse().unwrap()).collect();
                check(i as usize, &[t[0], t[1], t[2]]);
            }

            let mut tbl_slice = &mut TBL[..SZ];
            tbl_slice.sort_by(cmp);

            let mut d = i32::MAX;
            for i in 0..SZ {
                let mut f = [0; 11]; // 11 because IDs are 0 to 10
                let mut k = 0;
                for j in i..SZ {
                    if f[tbl_slice[j].id] != 0 {
                        continue;
                    }
                    f[tbl_slice[j].id] = 1;
                    k += 1;
                    if k == n {
                        if (tbl_slice[j].t - tbl_slice[i].t) < d {
                            d = tbl_slice[j].t - tbl_slice[i].t;
                            MIN = tbl_slice[i].t;
                            MAX = tbl_slice[j].t;
                        }
                        break;
                    }
                }
            }

            println!("{:02}:{:02}:{:02} {:02}:{:02}:{:02}",
                MIN / 3600, (MIN / 60) % 60, MIN % 60,
                MAX / 3600, (MAX / 60) % 60, MAX % 60);
        }
    }
}
