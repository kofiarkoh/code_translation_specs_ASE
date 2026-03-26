use std::io::{self, BufRead};

const SMAX: usize = 4000;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let a = format!("?{}", input.next().unwrap().unwrap());
    let b = format!("?{}", input.next().unwrap().unwrap());

    let alen = a.len();
    let blen = b.len();

    let mut dp = vec![vec![-1; blen + 1]; alen + 1];
    dp[alen - 1][blen - 1] = 0;

    let mut next0 = [[0; SMAX + 2]; 2];
    let mut next1 = [[0; SMAX + 2]; 2];

    for i in (0..alen - 1).rev() {
        if a.as_bytes()[i + 1] == b'0' {
            next0[0][i] = i + 1;
            next1[0][i] = next1[0][i + 1];
        } else {
            next0[0][i] = next0[0][i + 1];
            next1[0][i] = i + 1;
        }
    }

    for i in (0..blen - 1).rev() {
        if b.as_bytes()[i + 1] == b'0' {
            next0[1][i] = i + 1;
            next1[1][i] = next1[1][i + 1];
        } else {
            next0[1][i] = next0[1][i + 1];
            next1[1][i] = i + 1;
        }
    }

    scns(0, 0, &next0, &next1, &mut dp);
    println!();
}

fn dfs(aloc: usize, bloc: usize, next0: &[[usize; SMAX + 2]; 2], next1: &[[usize; SMAX + 2]; 2], dp: &mut Vec<Vec<i32>>) -> i32 {
    if dp[aloc][bloc] != -1 {
        return dp[aloc][bloc];
    }

    let mut ans = SMAX as i32 + 2;
    let res0 = dfs(next0[0][aloc], next0[1][bloc], next0, next1, dp);
    ans = ans.min(res0);
    let res1 = dfs(next1[0][aloc], next1[1][bloc], next0, next1, dp);
    ans = ans.min(res1);

    dp[aloc][bloc] = ans + 1;
    dp[aloc][bloc]
}

fn scns(aloc: usize, bloc: usize, next0: &[[usize; SMAX + 2]; 2], next1: &[[usize; SMAX + 2]; 2], dp: &mut Vec<Vec<i32>>) {
    if dfs(aloc, bloc, next0, next1, dp) == dfs(next0[0][aloc], next0[1][bloc], next0, next1, dp) + 1 {
        print!("0");
        scns(next0[0][aloc], next0[1][bloc], next0, next1, dp);
    } else if dfs(aloc, bloc, next0, next1, dp) == dfs(next1[0][aloc], next1[1][bloc], next0, next1, dp) + 1 {
        print!("1");
        scns(next1[0][aloc], next1[1][bloc], next0, next1, dp);
    }
}
