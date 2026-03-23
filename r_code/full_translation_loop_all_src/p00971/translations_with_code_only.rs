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

    let v = ans + 1;
    dp[aloc][bloc] = v;
    v
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
    // Read entire stdin, then parse two whitespace-separated tokens like scanf("%s").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a_in = it.next().unwrap_or("");
    let b_in = it.next().unwrap_or("");

    // C stores strings starting at index 1 and sets a[0]=b[0]='?'.
    // Also, alen/blen are strlen(a) after that, i.e., length including the '?'.
    let mut a: Vec<u8> = Vec::with_capacity(a_in.len() + 2);
    a.push(b'?');
    a.extend_from_slice(a_in.as_bytes());

    let mut b: Vec<u8> = Vec::with_capacity(b_in.len() + 2);
    b.push(b'?');
    b.extend_from_slice(b_in.as_bytes());

    let alen: usize = a.len();
    let blen: usize = b.len();

    // dp initialized to -1, with dp[alen][blen]=0.
    // Use Vec<Vec<i32>> to avoid huge stack allocation.
    let mut dp: Vec<Vec<i32>> = vec![vec![-1; SMAX + 2]; SMAX + 2];
    dp[alen][blen] = 0;

    // next0/next1 arrays.
    let mut next0 = [[0usize; SMAX + 2]; 2];
    let mut next1 = [[0usize; SMAX + 2]; 2];

    // Mirror the C initialization exactly.
    // Note: This assumes alen, blen >= 1 (true because we always have the leading '?').
    next0[0][alen] = alen;
    next1[0][alen] = alen;
    next0[0][alen - 1] = alen;
    next1[0][alen - 1] = alen;

    next0[1][blen] = blen;
    next1[1][blen] = blen;
    next0[1][blen - 1] = blen;
    next1[1][blen - 1] = blen;

    // Build next arrays. The C loops run i = len-2 down to 0 inclusive.
    if alen >= 2 {
        for i in (0..=alen - 2).rev() {
            // Access a[i+1] is always in-bounds.
            if a[i + 1] == b'0' {
                next0[0][i] = i + 1;
                next1[0][i] = next1[0][i + 1];
            } else {
                next0[0][i] = next0[0][i + 1];
                next1[0][i] = i + 1;
            }
        }
    }
    if blen >= 2 {
        for i in (0..=blen - 2).rev() {
            if b[i + 1] == b'0' {
                next0[1][i] = i + 1;
                next1[1][i] = next1[1][i + 1];
            } else {
                next0[1][i] = next0[1][i + 1];
                next1[1][i] = i + 1;
            }
        }
    }

    let mut out = String::new();
    scns(0, 0, &next0, &next1, &mut dp, &mut out);
    out.push('\n');

    print!("{out}");
}
