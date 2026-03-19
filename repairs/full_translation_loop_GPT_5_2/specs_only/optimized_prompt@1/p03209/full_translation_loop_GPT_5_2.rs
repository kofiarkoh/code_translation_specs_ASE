use std::io::{self, Read};

fn main() {
    // This matches the well-known recursive definition from AtCoder ABC 115 D ("Christmas"):
    // Given n and k (0-indexed layers eaten from the top), compute how many patties are eaten.
    //
    // Recurrence (burger level n):
    // B(0) = "P"
    // B(n) = "B" + B(n-1) + "P" + B(n-1) + "B"
    //
    // Let len[n] = total layers in B(n), pat[n] = total patties in B(n).
    // Then:
    // len[0]=1, pat[0]=1
    // len[n]=2*len[n-1]+3, pat[n]=2*pat[n-1]+1
    //
    // f(n, x): patties in the first x layers of B(n) (x can be 0..len[n]).
    // f(0, x) = 0 if x<=0 else 1
    // For n>0:
    // if x<=1: 0
    // else if x<=1+len[n-1]: f(n-1, x-1)
    // else if x==1+len[n-1]+1: pat[n-1]+1
    // else if x<=1+len[n-1]+1+len[n-1]: pat[n-1]+1+f(n-1, x-(1+len[n-1]+1))
    // else: pat[n]
    //
    // Input k in the original problem is 1-indexed number of layers eaten; we use x=k.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Precompute lengths and patties up to n.
    let mut len: Vec<i64> = vec![0; n + 1];
    let mut pat: Vec<i64> = vec![0; n + 1];
    len[0] = 1;
    pat[0] = 1;
    for i in 1..=n {
        len[i] = len[i - 1] * 2 + 3;
        pat[i] = pat[i - 1] * 2 + 1;
    }

    fn f(n: usize, x: i64, len: &[i64], pat: &[i64]) -> i64 {
        if x <= 0 {
            return 0;
        }
        if n == 0 {
            return 1; // x>=1
        }
        // Structure: B + B(n-1) + P + B(n-1) + B
        if x == 1 {
            0
        } else if x <= 1 + len[n - 1] {
            f(n - 1, x - 1, len, pat)
        } else if x == 1 + len[n - 1] + 1 {
            pat[n - 1] + 1
        } else if x <= 1 + len[n - 1] + 1 + len[n - 1] {
            pat[n - 1] + 1 + f(n - 1, x - (1 + len[n - 1] + 1), len, pat)
        } else {
            pat[n]
        }
    }

    let ans = f(n, k, &len, &pat);
    println!("{}", ans);
}
