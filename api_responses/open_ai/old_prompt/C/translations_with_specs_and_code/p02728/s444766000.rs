use std::io::{self, BufRead};

const M: i64 = 1_000_000_007;

fn mbk(a: i64, n: i64, m: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    let half = mbk((a * a) % m, n / 2, m);
    if n % 2 == 0 {
        half
    } else {
        (half * (a % m)) % m
    }
}

fn inv(a: i64, m: i64) -> i64 {
    mbk(a, m - 2, m)
}

fn tdp(s: usize, ta: &[i32], nt: &[i32], to: &[i32], tS: &mut Vec<i64>, tans: &mut Vec<i64>, tu: &mut Vec<i64>) {
    tu[s] = 1;
    let mut i = ta[s] as usize;
    while i != !0 {
        if tu[to[i]] != 0 {
            i = nt[i] as usize;
            continue;
        }
        tdp(to[i], ta, nt, to, tS, tans, tu);
        tans[s] = (tans[s] * tans[to[i]] % M);
        tS[s] += tS[to[i]];
        i = nt[i] as usize;
    }
    tans[s] = (tans[s] * inv(tS[s], M) % M);
}

fn chrt(s: usize, t: usize, ta: &[i32], nt: &[i32], to: &[i32], tS: &mut Vec<i64>, tans: &mut Vec<i64>, tu: &mut Vec<i64>) {
    tans[s] = (tans[s] * inv(tans[t], M) % M);
    tans[t] = (tans[t] * tans[s] % M);
    tS[s] -= tS[t];
    tS[t] += tS[s];
    tans[s] = (tans[s] * inv(tS[s], M) % M);
    tans[t] = (tans[t] * tS[s] % M);
}

fn atdp(s: usize, ta: &[i32], nt: &[i32], to: &[i32], tS: &mut Vec<i64>, tans: &mut Vec<i64>, tu: &mut Vec<i64>, ans: &mut Vec<i64>) {
    tu[s] = 2;
    ans[s] = tans[s];
    let mut i = ta[s] as usize;
    while i != !0 {
        if tu[to[i]] == 2 {
            i = nt[i] as usize;
            continue;
        }
        chrt(s, to[i], ta, nt, to, tS, tans, tu);
        atdp(to[i], ta, nt, to, tS, tans, tu, ans);
        chrt(to[i], s, ta, nt, to, tS, tans, tu);
        i = nt[i] as usize;
    }
}

fn at_dp(v: usize, a: &[i32], b: &[i32]) -> Vec<i64> {
    let mut ta = vec![-1; v + 1];
    let mut tS = vec![1; v + 1];
    let mut tans = vec![1; v + 1];
    let mut nt = vec![0; 2 * (v - 1)];
    let mut to = vec![0; 2 * (v - 1)];
    let mut tu = vec![0; v + 1];
    let mut ans = vec![0; v + 1];

    for i in 0..(v - 1) {
        nt[i] = ta[a[i] as usize];
        nt[i + (v - 1)] = ta[b[i] as usize];
        to[i] = b[i];
        to[i + (v - 1)] = a[i];
        ta[a[i] as usize] = i as i32;
        ta[b[i] as usize] = (i + (v - 1)) as i32;
    }

    tdp(1, &ta, &nt, &to, &mut tS, &mut tans, &mut tu);
    atdp(1, &ta, &nt, &to, &mut tS, &mut tans, &mut tu, &mut ans);

    ans
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = vec![0; n - 1];
    let mut b = vec![0; n - 1];

    for i in 0..(n - 1) {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        a[i] = nums[0];
        b[i] = nums[1];
    }

    let mut p = 1;
    for i in 1..=n {
        p = (p * i as i64) % M;
    }

    let ans = at_dp(n, &a, &b);
    for i in 1..=n {
        println!("{}", (ans[i] * p % M));
    }
}
