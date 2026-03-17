use std::io::{self, BufRead};
use std::mem;

const MOD: i64 = 1_000_000_007;

fn calc(x: usize, pw: &[i64]) -> i64 {
    if x & 1 == 1 {
        0
    } else {
        pw[x >> 1]
    }
}

fn add_edge(u: usize, v: usize, lst: &mut Vec<i32>, pre: &mut Vec<i32>, tot: &mut usize) {
    let index = *tot;
    lst[index] = v as i32;
    pre[index] = lst[u];
    lst[u] = index as i32;
    *tot += 1;
}

fn init_dfs(u: usize, fa: i32, lst: &[i32], to: &[usize], sz: &mut Vec<usize>) {
    sz[u] = 1;
    let mut i = lst[u] as usize;
    while i != usize::MAX {
        if to[i] == fa as usize {
            i = pre[i] as usize;
            continue;
        }
        init_dfs(to[i], u as i32, lst, to, sz);
        sz[u] += sz[to[i]];
        i = pre[i] as usize;
    }
}

fn dfs(u: usize, fa: i32, lst: &[i32], to: &[usize], dp: &mut Vec<Vec<[i64; 2]>>, dp_sz: &mut Vec<usize>, sz: &mut Vec<usize>) {
    dp[u][0][0] = 1;
    let mut i = lst[u] as usize;
    while i != usize::MAX {
        let v = to[i];
        if v == fa as usize {
            i = pre[i] as usize;
            continue;
        }
        dfs(v, u as i32, lst, to, dp, dp_sz, sz);
        let mut tmp = vec![[0; 2]; 5005];
        for x in (0..=dp_sz[u]).rev() {
            for y in (0..=dp_sz[v]).rev() {
                tmp[x + y][0] = (tmp[x + y][0] + dp[u][x][0] * dp[v][y][0]) % MOD;
                tmp[x + y][0] = (tmp[x + y][0] + dp[u][x][1] * dp[v][y][1]) % MOD;
                tmp[x + y][1] = (tmp[x + y][1] + dp[u][x][0] * dp[v][y][1]) % MOD;
                tmp[x + y][1] = (tmp[x + y][1] + dp[u][x][1] * dp[v][y][0]) % MOD;
            }
        }
        for j in 0..=dp_sz[u] {
            dp[u][j] = tmp[j];
        }
        dp_sz[u] += dp_sz[v];
        i = pre[i] as usize;
    }
    if u != 0 {
        for x in (0..=dp_sz[u]).rev() {
            dp[u][sz[u]][0] = (dp[u][sz[u]][0] + dp[u][x][1] * calc(sz[u] - x, &pw)) % MOD;
            dp[u][sz[u]][1] = (dp[u][sz[u]][1] + dp[u][x][0] * calc(sz[u] - x, &pw)) % MOD;
        }
    }
    dp_sz[u] += 1;
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut lst = vec![-1; 5005];
    let mut to = vec![0; 10005];
    let mut pre = vec![0; 10005];
    let mut tot = 0;
    let mut sz = vec![0; 5005];
    let mut dp_sz = vec![0; 5005];
    let mut dp = vec![vec![[0; 2]; 5005]; 5005];
    let mut pw = vec![1; 5005];
    let mut ans = 0;

    for i in 1..=n {
        pw[i] = (pw[i - 1] * (i * 2 - 1) % MOD);
    }

    for _ in 1..n {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let u = nums.next().unwrap() - 1;
        let v = nums.next().unwrap() - 1;
        add_edge(u, v, &mut lst, &mut pre, &mut tot);
        add_edge(v, u, &mut lst, &mut pre, &mut tot);
    }

    init_dfs(0, -1, &lst, &to, &mut sz);
    dfs(0, -1, &lst, &to, &mut dp, &mut dp_sz, &mut sz);

    for i in 0..=n {
        ans = (ans + dp[0][i][0] * calc(n - i, &pw)) % MOD;
        ans = (ans - dp[0][i][1] * calc(n - i, &pw)) % MOD;
    }

    println!("{}", (ans + MOD) % MOD);
}
