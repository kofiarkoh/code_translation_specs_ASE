use std::io::{self, Read};

const SMAX: usize = 4000;

fn main() {
    // Read two whitespace-delimited tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a_tok = it.next().unwrap_or("");
    let b_tok = it.next().unwrap_or("");

    // C code uses 1-based indexing for characters and a sentinel at index 0.
    // We'll build arrays a[0..=alen], b[0..=blen] where a[0]=b[0]='?'.
    let a_bytes = a_tok.as_bytes();
    let b_bytes = b_tok.as_bytes();

    // In the C code: scanf into a+1, then a[0]='?', then alen=strlen(a).
    // That makes alen == original_length + 1 (because of the sentinel at 0).
    let alen = a_bytes.len() + 1;
    let blen = b_bytes.len() + 1;

    // a[i] for i>=1 corresponds to a_tok[i-1]
    let mut a = vec![b'?'; alen + 1];
    let mut b = vec![b'?'; blen + 1];
    for (i, &ch) in a_bytes.iter().enumerate() {
        a[i + 1] = ch;
    }
    for (i, &ch) in b_bytes.iter().enumerate() {
        b[i + 1] = ch;
    }

    // next0/next1 tables: next0[which_string][pos]
    // pos ranges 0..=len, where len is the end sentinel index.
    let mut next0_a = vec![0usize; alen + 1];
    let mut next1_a = vec![0usize; alen + 1];
    let mut next0_b = vec![0usize; blen + 1];
    let mut next1_b = vec![0usize; blen + 1];

    // Match C initialization:
    // next*[len] = len and next*[len-1] = len (when len>=1).
    next0_a[alen] = alen;
    next1_a[alen] = alen;
    if alen >= 1 {
        next0_a[alen - 1] = alen;
        next1_a[alen - 1] = alen;
    }
    next0_b[blen] = blen;
    next1_b[blen] = blen;
    if blen >= 1 {
        next0_b[blen - 1] = blen;
        next1_b[blen - 1] = blen;
    }

    // Fill next tables backwards for i = len-2 down to 0.
    if alen >= 2 {
        for i in (0..=alen - 2).rev() {
            if a[i + 1] == b'0' {
                next0_a[i] = i + 1;
                next1_a[i] = next1_a[i + 1];
            } else {
                next0_a[i] = next0_a[i + 1];
                next1_a[i] = i + 1;
            }
        }
    }
    if blen >= 2 {
        for i in (0..=blen - 2).rev() {
            if b[i + 1] == b'0' {
                next0_b[i] = i + 1;
                next1_b[i] = next1_b[i + 1];
            } else {
                next0_b[i] = next0_b[i + 1];
                next1_b[i] = i + 1;
            }
        }
    }

    // DP table dp[0..=alen][0..=blen], initialized to -1, with dp[alen][blen]=0.
    // Note: This is O(alen*blen) memory, as per spec.
    let mut dp = vec![vec![-1i32; blen + 1]; alen + 1];
    dp[alen][blen] = 0;

    // Memoized DFS as in C. Recurrence:
    // L(i,j) = 1 + min(L(next0(i),next0(j)), L(next1(i),next1(j)))
    fn dfs(
        i: usize,
        j: usize,
        alen: usize,
        blen: usize,
        next0_a: &[usize],
        next1_a: &[usize],
        next0_b: &[usize],
        next1_b: &[usize],
        dp: &mut [Vec<i32>],
    ) -> i32 {
        if dp[i][j] != -1 {
            return dp[i][j];
        }
        // C uses ans initialized to SMAX+2; we can use a large i32.
        let mut ans = (SMAX as i32) + 2;

        let r0 = dfs(
            next0_a[i],
            next0_b[j],
            alen,
            blen,
            next0_a,
            next1_a,
            next0_b,
            next1_b,
            dp,
        );
        if ans > r0 {
            ans = r0;
        }
        let r1 = dfs(
            next1_a[i],
            next1_b[j],
            alen,
            blen,
            next0_a,
            next1_a,
            next0_b,
            next1_b,
            dp,
        );
        if ans > r1 {
            ans = r1;
        }

        dp[i][j] = ans + 1;
        dp[i][j]
    }

    // Reconstruct deterministically: prefer '0' when optimal, else '1'.
    let mut out = String::new();
    let mut i = 0usize;
    let mut j = 0usize;
    loop {
        if i == alen && j == blen {
            break;
        }
        let cur = dfs(
            i, j, alen, blen, &next0_a, &next1_a, &next0_b, &next1_b, &mut dp,
        );
        let n0i = next0_a[i];
        let n0j = next0_b[j];
        let v0 = dfs(
            n0i, n0j, alen, blen, &next0_a, &next1_a, &next0_b, &next1_b, &mut dp,
        );
        if cur == v0 + 1 {
            out.push('0');
            i = n0i;
            j = n0j;
            continue;
        }
        let n1i = next1_a[i];
        let n1j = next1_b[j];
        let v1 = dfs(
            n1i, n1j, alen, blen, &next0_a, &next1_a, &next0_b, &next1_b, &mut dp,
        );
        if cur == v1 + 1 {
            out.push('1');
            i = n1i;
            j = n1j;
            continue;
        }

        // By the spec/recurrence, one of the two must be optimal unless at terminal.
        break;
    }

    print!("{out}\n");
}
