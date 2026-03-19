use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn calc(x: usize, pw: &[i64]) -> i64 {
    if (x & 1) == 1 {
        0
    } else {
        pw[x >> 1]
    }
}

fn init_dfs(
    u: usize,
    fa: isize,
    lst: &[isize],
    to: &[usize],
    pre: &[isize],
    sz: &mut [usize],
) {
    sz[u] = 1;
    let mut i = lst[u];
    while i != -1 {
        let e = i as usize;
        let v = to[e];
        if v as isize != fa {
            init_dfs(v, u as isize, lst, to, pre, sz);
            sz[u] += sz[v];
        }
        i = pre[e];
    }
}

fn dfs(
    u: usize,
    fa: isize,
    lst: &[isize],
    to: &[usize],
    pre: &[isize],
    sz: &[usize],
    dp_sz: &mut [usize],
    dp: &mut Vec<Vec<[i64; 2]>>,
    pw: &[i64],
    tmp: &mut Vec<[i64; 2]>,
) {
    dp[u][0][0] = 1;
    dp[u][0][1] = 0;

    let mut i = lst[u];
    while i != -1 {
        let e = i as usize;
        let v = to[e];
        if v as isize != fa {
            dfs(v, u as isize, lst, to, pre, sz, dp_sz, dp, pw, tmp);

            // memset(tmp,0,sizeof(tmp));
            for t in tmp.iter_mut() {
                *t = [0, 0];
            }

            // for (int x=dp_sz[u];x>=0;x--)
            for x in (0..=dp_sz[u]).rev() {
                for y in (0..=dp_sz[v]).rev() {
                    let a0 = dp[u][x][0];
                    let a1 = dp[u][x][1];
                    let b0 = dp[v][y][0];
                    let b1 = dp[v][y][1];
                    let idx = x + y;

                    tmp[idx][0] = (tmp[idx][0] + a0 * b0) % MOD;
                    tmp[idx][0] = (tmp[idx][0] + a1 * b1) % MOD;
                    tmp[idx][1] = (tmp[idx][1] + a0 * b1) % MOD;
                    tmp[idx][1] = (tmp[idx][1] + a1 * b0) % MOD;
                }
            }

            // memcpy(dp[u],tmp,sizeof(tmp));
            for k in 0..tmp.len() {
                dp[u][k] = tmp[k];
            }

            dp_sz[u] += dp_sz[v];
        }
        i = pre[e];
    }

    if u != 0 {
        // for (int x=dp_sz[u];x>=0;x--)
        for x in (0..=dp_sz[u]).rev() {
            let c = calc(sz[u] - x, pw);
            let add0 = (dp[u][x][1] * c) % MOD;
            let add1 = (dp[u][x][0] * c) % MOD;
            let idx = sz[u];
            dp[u][idx][0] = (dp[u][idx][0] + add0) % MOD;
            dp[u][idx][1] = (dp[u][idx][1] + add1) % MOD;
        }
    }

    dp_sz[u] += 1;
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Arrays sized to match C maxima; behavior depends only on n, but keep generous.
    let maxn = 5005usize;
    let maxe = 10005usize;

    let mut lst = vec![-1isize; maxn];
    let mut to = vec![0usize; maxe];
    let mut pre = vec![-1isize; maxe];
    let mut tot: usize = 0;

    // pw[0]=1, pw[i]=pw[i-1]*(i*2-1)%MOD
    let mut pw = vec![0i64; maxn];
    pw[0] = 1;
    for i in 1..=n {
        let mul = (i as i64) * 2 - 1;
        pw[i] = (pw[i - 1] * mul) % MOD;
    }

    // add_edge(u,v): to[tot]=v; pre[tot]=lst[u]; lst[u]=tot++
    let mut add_edge = |u: usize, v: usize, lst: &mut [isize], to: &mut [usize], pre: &mut [isize], tot: &mut usize| {
        to[*tot] = v;
        pre[*tot] = lst[u];
        lst[u] = *tot as isize;
        *tot += 1;
    };

    for _ in 1..n {
        let mut u: isize = it.next().unwrap().parse().unwrap();
        let mut v: isize = it.next().unwrap().parse().unwrap();
        u -= 1;
        v -= 1;
        let uu = u as usize;
        let vv = v as usize;
        add_edge(uu, vv, &mut lst, &mut to, &mut pre, &mut tot);
        add_edge(vv, uu, &mut lst, &mut to, &mut pre, &mut tot);
    }

    let mut sz = vec![0usize; maxn];
    let mut dp_sz = vec![0usize; maxn];

    // dp[u][k][2] for u,k up to n; allocate (n+1)x(n+1)
    let mut dp: Vec<Vec<[i64; 2]>> = vec![vec![[0i64; 2]; n + 1]; n + 1];
    let mut tmp = vec![[0i64; 2]; n + 1];

    init_dfs(0, -1, &lst, &to, &pre, &mut sz);
    dfs(
        0,
        -1,
        &lst,
        &to,
        &pre,
        &sz,
        &mut dp_sz,
        &mut dp,
        &pw,
        &mut tmp,
    );

    let mut ans: i64 = 0;
    for i in 0..=n {
        let c = calc(n - i, &pw);
        ans = (ans + dp[0][i][0] * c) % MOD;
        ans = (ans - dp[0][i][1] * c) % MOD;
    }

    // printf("%lld\n",(ans+mod)%mod);
    let out = (ans + MOD) % MOD;
    print!("{out}\n");
}
