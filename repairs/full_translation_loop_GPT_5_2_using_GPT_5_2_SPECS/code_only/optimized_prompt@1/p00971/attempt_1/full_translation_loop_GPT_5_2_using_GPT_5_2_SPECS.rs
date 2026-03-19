use std::io::{self, Read};

const SMAX: usize = 4000;

fn dfs(
    aloc: usize,
    bloc: usize,
    next0: &[[usize; SMAX + 2]; 2],
    next1: &[[usize; SMAX + 2]; 2],
    dp: &mut Vec<Vec<i32>>,
) -> i32 {
    if dp[aloc][bloc] != -1 {
        return dp[aloc][bloc];
    }
    let mut ans: i32 = (SMAX as i32) + 2;

    let res0 = dfs(next0[0][aloc], next0[1][bloc], next0, next1, dp);
    if ans > res0 {
        ans = res0;
    }
    let res1 = dfs(next1[0][aloc], next1[1][bloc], next0, next1, dp);
    if ans > res1 {
        ans = res1;
    }

    dp[aloc][bloc] = ans + 1;
    dp[aloc][bloc]
}

fn scns(
    aloc: usize,
    bloc: usize,
    next0: &[[usize; SMAX + 2]; 2],
    next1: &[[usize; SMAX + 2]; 2],
    dp: &mut Vec<Vec<i32>>,
    out: &mut String,
) {
    let cur = dfs(aloc, bloc, next0, next1, dp);

    let n0a = next0[0][aloc];
    let n0b = next0[1][bloc];
    if cur == dfs(n0a, n0b, next0, next1, dp) + 1 {
        out.push('0');
        scns(n0a, n0b, next0, next1, dp, out);
        return;
    }

    let n1a = next1[0][aloc];
    let n1b = next1[1][bloc];
    if cur == dfs(n1a, n1b, next0, next1, dp) + 1 {
        out.push('1');
        scns(n1a, n1b, next0, next1, dp, out);
    }
}

fn main() {
    // Read all input, split by whitespace like scanf("%s").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a_in = it.next().unwrap_or("");
    let b_in = it.next().unwrap_or("");

    // Build 1-based arrays with a[0]=b[0]='?' like the C code.
    // In C, strlen(a) after writing at a+1 yields (len+1) because a[0] is non-NUL.
    // We emulate that by making length = input_len + 1.
    let alen: usize = a_in.len() + 1;
    let blen: usize = b_in.len() + 1;

    let mut a: Vec<u8> = vec![b'?'; alen + 1]; // extra slot for safety
    let mut b: Vec<u8> = vec![b'?'; blen + 1];
    for (i, &ch) in a_in.as_bytes().iter().enumerate() {
        a[i + 1] = ch;
    }
    for (i, &ch) in b_in.as_bytes().iter().enumerate() {
        b[i + 1] = ch;
    }

    // dp initialized to -1, with dp[alen][blen]=0.
    let mut dp: Vec<Vec<i32>> = vec![vec![-1; SMAX + 2]; SMAX + 2];
    dp[alen][blen] = 0;

    let mut next0 = [[0usize; SMAX + 2]; 2];
    let mut next1 = [[0usize; SMAX + 2]; 2];

    // Mirror the C initializations exactly.
    next0[0][alen] = alen;
    next1[0][alen] = alen;
    next0[0][alen - 1] = alen;
    next1[0][alen - 1] = alen;

    next0[1][blen] = blen;
    next1[1][blen] = blen;
    next0[1][blen - 1] = blen;
    next1[1][blen - 1] = blen;

    // for (i = alen - 2; i >= 0; i--)
    for i in (0..=alen - 2).rev() {
        if a[i + 1] == b'0' {
            next0[0][i] = i + 1;
            next1[0][i] = next1[0][i + 1];
        } else {
            next0[0][i] = next0[0][i + 1];
            next1[0][i] = i + 1;
        }
    }
    for i in (0..=blen - 2).rev() {
        if b[i + 1] == b'0' {
            next0[1][i] = i + 1;
            next1[1][i] = next1[1][i + 1];
        } else {
            next0[1][i] = next0[1][i + 1];
            next1[1][i] = i + 1;
        }
    }

    let mut out = String::new();
    scns(0, 0, &next0, &next1, &mut dp, &mut out);
    out.push('\n');
    print!("{out}");
}
