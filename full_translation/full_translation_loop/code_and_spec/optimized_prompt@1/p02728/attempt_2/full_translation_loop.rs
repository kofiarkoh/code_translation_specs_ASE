use std::io::{self, BufRead};

const M: i64 = 1_000_000_007;

fn mbk(a: i64, n: i64, m: i64) -> i64 {
    if n == 0 {
        1
    } else {
        let half = mbk((a * a) % m, n / 2, m);
        let result = if n % 2 == 1 { (half * a) % m } else { half };
        result
    }
}

fn inv(a: i64, m: i64) -> i64 {
    mbk(a, m - 2, m)
}

fn fc(s: usize, t: usize, tans: &mut Vec<i64>) {
    tans[s] = (tans[s] * tans[t]) % M;
}

fn ic(s: usize, t: usize, tans: &mut Vec<i64>) {
    tans[s] = (tans[s] * inv(tans[t], M)) % M;
}

fn fp(s: usize, tans: &mut Vec<i64>, t_s: &mut Vec<i64>) {
    tans[s] = (tans[s] * inv(t_s[s], M)) % M;
}

fn ip(s: usize, tans: &mut Vec<i64>, t_s: &mut Vec<i64>) {
    tans[s] = (tans[s] * t_s[s]) % M;
}

fn tdp(s: usize, ta: &Vec<usize>, nt: &Vec<usize>, to: &Vec<usize>, tu: &mut Vec<i64>, tans: &mut Vec<i64>, t_s: &mut Vec<i64>) {
    tu[s] = 1;
    let mut i = ta[s];
    while i != usize::MAX {
        if tu[to[i]] != 0 {
            i = nt[i];
            continue;
        }
        tdp(to[i], ta, nt, to, tu, tans, t_s);
        fc(s, to[i], tans);
        t_s[s] += t_s[to[i]];
        i = nt[i];
    }
    fp(s, tans, t_s);
}

fn chrt(s: usize, t: usize, tans: &mut Vec<i64>, t_s: &mut Vec<i64>) {
    ip(s, tans, t_s);
    ic(s, t, tans);
    ip(t, tans, t_s);
    t_s[s] -= t_s[t];
    t_s[t] += t_s[s];
    fp(s, tans, t_s);
    fc(t, s, tans);
    fp(t, tans, t_s);
}

fn atdp(s: usize, ta: &Vec<usize>, nt: &Vec<usize>, to: &Vec<usize>, tu: &mut Vec<i64>, tans: &mut Vec<i64>) {
    tu[s] = 2;
    let _ans = tans[s]; // Store the answer for this node
    let mut i = ta[s];
    while i != usize::MAX {
        if tu[to[i]] == 2 {
            i = nt[i];
            continue;
        }
        chrt(s, to[i], tans, &mut vec![1; tans.len()]); // Pass t_s as a new vector
        atdp(to[i], ta, nt, to, tu, tans);
        chrt(to[i], s, tans, &mut vec![1; tans.len()]); // Pass t_s as a new vector
        i = nt[i];
    }
}

fn at_dp(n: usize, a: &Vec<usize>, b: &Vec<usize>) -> Vec<i64> {
    let mut ta = vec![usize::MAX; n + 1];
    let mut t_s = vec![1; n + 1];
    let mut tans = vec![1; n + 1];
    let mut nt = vec![usize::MAX; 2 * (n - 1)];
    let mut to = vec![0; 2 * (n - 1)];
    for i in 0..(n - 1) {
        nt[i] = ta[to[i + n - 1] = a[i]];
        nt[i + n - 1] = ta[to[i] = b[i]];
        ta[a[i]] = i;
        ta[b[i]] = i + n - 1;
    }
    let mut tu = vec![0; n + 1];
    tdp(1, &ta, &nt, &to, &mut tu, &mut tans, &mut t_s);
    let mut ans = vec![0; n + 1];
    atdp(1, &ta, &nt, &to, &mut tu, &mut tans);
    for i in 1..=n {
        ans[i - 1] = tans[i] % M;
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = Vec::with_capacity(n - 1);
    let mut b = Vec::with_capacity(n - 1);
    
    for _ in 0..(n - 1) {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<usize> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        a.push(nums[0]);
        b.push(nums[1]);
    }
    
    let mut p = 1;
    for i in 1..=n {
        p = (p * i as i64) % M;
    }
    
    let ans = at_dp(n, &a, &b);
    for i in 0..n {
        println!("{}", (ans[i] * p) % M);
    }
}
