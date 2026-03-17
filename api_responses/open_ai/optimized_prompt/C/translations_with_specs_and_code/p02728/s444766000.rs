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

fn tdp(s: usize, tu: &mut Vec<i64>, tans: &mut Vec<i64>, t_s: &mut Vec<i64>, edges: &Vec<Vec<usize>>) {
    tu[s] = 1;
    for &t in &edges[s] {
        if tu[t] != 0 {
            continue;
        }
        tdp(t, tu, tans, t_s, edges);
        fc(s, t, tans);
        t_s[s] += t_s[t];
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

fn atdp(s: usize, tu: &mut Vec<i64>, tans: &mut Vec<i64>, edges: &Vec<Vec<usize>>) {
    tu[s] = 2;
    for &t in &edges[s] {
        if tu[t] == 2 {
            continue;
        }
        chrt(s, t, tans);
        atdp(t, tu, tans, edges);
        chrt(t, s, tans);
    }
}

fn at_dp(n: usize, a: &Vec<usize>, b: &Vec<usize>) -> Vec<i64> {
    let mut ta = vec![-1; n + 1];
    let mut t_s = vec![1; n + 1];
    let mut tans = vec![1; n + 1];
    let mut edges = vec![vec![]; n + 1];

    for i in 0..n - 1 {
        edges[a[i]].push(b[i]);
        edges[b[i]].push(a[i]);
    }

    let mut tu = vec![0; n + 1];
    tdp(1, &mut tu, &mut tans, &mut t_s, &edges);
    atdp(1, &mut tu, &mut tans, &edges);

    tans
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a = Vec::with_capacity(n - 1);
    let mut b = Vec::with_capacity(n - 1);

    for _ in 0..n - 1 {
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
    for i in 1..=n {
        println!("{}", (ans[i] * p) % M);
    }
}
