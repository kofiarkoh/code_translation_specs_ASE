use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn calc(x: usize, pw: &[i64]) -> i64 {
    // C macro: (1LL*((x&1)?0:pw[x>>1]))
    if (x & 1) == 1 {
        0
    } else {
        pw[x >> 1]
    }
}

fn init_dfs(u: usize, fa: isize, lst: &[i32], to: &[usize], pre: &[i32], sz: &mut [usize]) {
    sz[u] = 1;
    let mut i = lst[u];
    while i != -1 {
        let idx = i as usize;
        let v = to[idx];
        if v as isize != fa {
            init_dfs(v, u as isize, lst, to, pre, sz);
            sz[u] += sz[v];
        }
        i = pre[idx];
    }
}

fn dfs(
    u: usize,
    fa: isize,
    lst: &[i32],
    to: &[usize],
    pre: &[i32],
    sz: &[usize],
    dp_sz: &mut [usize],
    dp: &mut Vec<Vec<[i64; 2]>>,
    pw: &[i64],
) {
    dp[u][0][0] = 1;
    dp[u][0][1] = 0;

    let mut i = lst[u];
    while i != -1 {
        let idx = i as usize;
        let v = to[idx];
        if v as isize != fa {
            dfs(v, u as isize, lst, to, pre, sz, dp_sz, dp, pw);

            // tmp[5005][2] in C, but we only need up to dp_sz[u] + dp_sz[v]
            let new_sz = dp_sz[u] + dp_sz[v];
            let mut tmp = vec![[0i64; 2]; new_sz + 1];

            for x in (0..=dp_sz[u]).rev() {
                let du0 = dp[u][x][0];
                let du1 = dp[u][x][1];
                for y in (0..=dp_sz[v]).rev() {
                    let dv0 = dp[v][y][0];
                    let dv1 = dp[v][y][1];

                    let xy = x + y;

                    // tmp[xy][0] += du0*dv0 + du1*dv1
                    tmp[xy][0] = (tmp[xy][0] + (du0 * dv0) % MOD) % MOD;
                    tmp[xy][0] = (tmp[xy][0] + (du1 * dv1) % MOD) % MOD;

                    // tmp[xy][1] += du0*dv1 + du1*dv0
                    tmp[xy][1] = (tmp[xy][1] + (du0 * dv1) % MOD) % MOD;
                    tmp[xy][1] = (tmp[xy][1] + (du1 * dv0) % MOD) % MOD;
                }
            }

            // memcpy(dp[u], tmp, sizeof(tmp)) in C: overwrite first (new_sz+1) entries.
            for k in 0..=new_sz {
                dp[u][k] = tmp[k];
            }
            dp_sz[u] = new_sz;
        }
        i = pre[idx];
    }

    if u != 0 {
        // for x=dp_sz[u]..0:
        // dp[u][sz[u]][0] += dp[u][x][1]*calc(sz[u]-x)
        // dp[u][sz[u]][1] += dp[u][x][0]*calc(sz[u]-x)
        let target = sz[u];
        // Ensure dp[u] is large enough (it is sized n+1).
        for x in (0..=dp_sz[u]).rev() {
            let c = calc(target - x, pw);
            if c == 0 {
                continue;
            }
            let add0 = (dp[u][x][1] * c) % MOD;
            let add1 = (dp[u][x][0] * c) % MOD;
            dp[u][target][0] = (dp[u][target][0] + add0) % MOD;
            dp[u][target][1] = (dp[u][target][1] + add1) % MOD;
        }
    }

    dp_sz[u] += 1;
}

fn main() {
    // Read all input (scanf-like whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // pw[5005] with pw[0]=1; pw[i]=pw[i-1]*(i*2-1)%MOD
    let mut pw = vec![0i64; n + 1];
    pw[0] = 1;
    for i in 1..=n {
        let mul = (i as i64) * 2 - 1;
        pw[i] = (pw[i - 1] * (mul % MOD)) % MOD;
    }

    // Graph arrays like C: lst[5005], to[10005], pre[10005], tot
    let m = if n >= 1 { 2 * (n - 1) } else { 0 };
    let mut lst = vec![-1i32; n.max(1)];
    let mut to = vec![0usize; m];
    let mut pre = vec![0i32; m];
    let mut tot: usize = 0;

    let mut add_edge = |u: usize, v: usize, lst: &mut [i32], to: &mut [usize], pre: &mut [i32], tot: &mut usize| {
        to[*tot] = v;
        pre[*tot] = lst[u];
        lst[u] = *tot as i32;
        *tot += 1;
    };

    for _ in 1..n {
        let mut u: isize = it.next().unwrap().parse::<isize>().unwrap();
        let mut v: isize = it.next().unwrap().parse::<isize>().unwrap();
        u -= 1;
        v -= 1;
        let uu = u as usize;
        let vv = v as usize;
        add_edge(uu, vv, &mut lst, &mut to, &mut pre, &mut tot);
        add_edge(vv, uu, &mut lst, &mut to, &mut pre, &mut tot);
    }

    let mut sz = vec![0usize; n.max(1)];
    let mut dp_sz = vec![0usize; n.max(1)];

    // dp[u][k][2], sized (n x (n+1))
    let mut dp: Vec<Vec<[i64; 2]>> = vec![vec![[0i64; 2]; n + 1]; n.max(1)];

    if n > 0 {
        init_dfs(0, -1, &lst, &to, &pre, &mut sz);
        dfs(0, -1, &lst, &to, &pre, &sz, &mut dp_sz, &mut dp, &pw);
    }

    let mut ans: i64 = 0;
    for i in 0..=n {
        let c = calc(n - i, &pw);
        ans = (ans + (dp[0][i][0] * c) % MOD) % MOD;
        ans = (ans - (dp[0][i][1] * c) % MOD) % MOD;
    }

    // printf("%lld\n",(ans+mod)%mod);
    let out = (ans + MOD) % MOD;
    print!("{out}\n");
}
