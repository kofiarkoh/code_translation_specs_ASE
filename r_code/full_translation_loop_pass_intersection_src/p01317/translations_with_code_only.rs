use std::io::{self, Read};

const MN: usize = 205;
const INF: i32 = 100000000;

// Emulate C's `memset(a, 0x0f, sizeof(a))` for i32 arrays.
// 0x0f0f0f0f as i32 is 252_645_135, which is > INF and works as "infinity".
const INIT_I32: i32 = 0x0f0f0f0f_u32 as i32;

fn main() {
    // Read entire stdin, then parse as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();

        if n == 0 {
            break;
        }

        // Matrices and dp, initialized like the C code.
        let mut s_mat = vec![vec![INIT_I32; MN]; MN];
        let mut l_mat = vec![vec![INIT_I32; MN]; MN];
        let mut dp = vec![vec![INIT_I32; MN]; 1005];

        for i in 0..n {
            s_mat[i][i] = 0;
            l_mat[i][i] = 0;
        }

        for _ in 0..m {
            let mut x: usize = it.next().unwrap().parse::<usize>().unwrap();
            let mut y: usize = it.next().unwrap().parse::<usize>().unwrap();
            let w: i32 = it.next().unwrap().parse().unwrap();
            let t = it.next().unwrap().as_bytes()[0] as char;

            // C code decrements to 0-based.
            x -= 1;
            y -= 1;

            if t == 'L' {
                l_mat[x][y] = w;
                l_mat[y][x] = w;
            } else {
                s_mat[x][y] = w;
                s_mat[y][x] = w;
            }
        }

        let r: usize = it.next().unwrap().parse().unwrap();
        let mut de = vec![0usize; r];
        for i in 0..r {
            let v: usize = it.next().unwrap().parse::<usize>().unwrap();
            // C does `de[i]--` after reading, so store 0-based.
            de[i] = v - 1;
        }

        // Floyd-Warshall on both matrices.
        // Use wrapping_add to emulate C signed overflow behavior as closely as possible.
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    let cand_s = s_mat[i][k].wrapping_add(s_mat[k][j]);
                    if cand_s < s_mat[i][j] {
                        s_mat[i][j] = cand_s;
                    }
                    let cand_l = l_mat[i][k].wrapping_add(l_mat[k][j]);
                    if cand_l < l_mat[i][j] {
                        l_mat[i][j] = cand_l;
                    }
                }
            }
        }

        dp[0][de[0]] = 0;

        for k in 0..(r.saturating_sub(1)) {
            for i in 0..n {
                for j in 0..n {
                    let base = dp[k][j];
                    let cand = base
                        .wrapping_add(l_mat[de[k]][j])
                        .wrapping_add(s_mat[j][i])
                        .wrapping_add(l_mat[i][de[k + 1]]);
                    if cand < dp[k + 1][i] {
                        dp[k + 1][i] = cand;
                    }
                    if i == j {
                        let cand2 = base.wrapping_add(l_mat[de[k]][de[k + 1]]);
                        if cand2 < dp[k + 1][i] {
                            dp[k + 1][i] = cand2;
                        }
                    }
                }
            }
        }

        let mut ans = INF;
        if r > 0 {
            for i in 0..n {
                if dp[r - 1][i] < ans {
                    ans = dp[r - 1][i];
                }
            }
        }
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
