use std::io::{self, BufRead};

const INF: i32 = 1000;

fn search(tkrk: i32, kkt: usize, memo: &mut [[i32; 101]; 101], a: &[i32], e: &[i32], r: &[i32], n: usize) -> i32 {
    let mut ret = INF;
    if tkrk <= 0 {
        return 0;
    }
    if memo[tkrk as usize][kkt] > 0 {
        return memo[tkrk as usize][kkt] - 1;
    }
    memo[tkrk as usize][kkt] = INF + 1;

    for i in 0..n {
        if kkt >= r[i] {
            let mut next_kkt = kkt + e[i] as usize;
            if next_kkt > 100 {
                next_kkt = 100;
            }
            let candidate = search(tkrk - a[i], next_kkt, memo, a, e, r, n) + 1;
            if candidate < ret {
                ret = candidate;
            }
        }
    }
    memo[tkrk as usize][kkt] = ret + 1;
    ret
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let D = nums.next().unwrap();
        let N = nums.next().unwrap();

        if D == 0 && N == 0 {
            break;
        }

        let mut a = vec![0; 100];
        let mut e = vec![0; 100];
        let mut r = vec![0; 100];

        for i in 0..N {
            let line = lines.next().unwrap().unwrap();
            let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            a[i as usize] = nums.next().unwrap();
            e[i as usize] = nums.next().unwrap();
            r[i as usize] = nums.next().unwrap();
        }

        let mut memo = [[0; 101]; 101];
        let ret = search(D, 0, &mut memo, &a, &e, &r, N as usize);
        
        if ret == INF {
            println!("NA");
        } else {
            println!("{}", ret);
        }
    }
}
