use std::io::{self, Read};

fn main() {
    const M: i64 = 998_244_353;
    // Arrays sized exactly as in C.
    let mut d = vec![0i64; 90310];
    let mut e = vec![0i64; 90310];

    // Read all input as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C: *d = *e = scanf("%d");
    // scanf returns number of items assigned (1 on success, 0 on failure, EOF=-1).
    // With split_whitespace, we can only distinguish "has token" vs not.
    // If there is at least one integer, emulate return value 1; otherwise emulate EOF (-1).
    let first = it.next();
    let scan_ret: i64 = if first.is_some() { 1 } else { -1 };
    d[0] = scan_ret;
    e[0] = scan_ret;

    let mut s: i64 = 0;
    let mut r: i64 = 1;
    let mut t: i64;

    // C: for (...; ~scanf("%d",&a);) { ... }
    // Loop continues while scanf != -1 (EOF). Here: while there is another token.
    while let Some(tok) = it.next() {
        let a: i64 = tok.parse::<i32>().unwrap() as i64; // scanf("%d") reads int; promote to long.
        t = s;
        // C: for(t=s; ~t; --t) ...
        // Condition ~t is false only when t == -1.
        while t != -1 {
            let idx = (t + a) as usize;
            let ti = t as usize;

            // d[t+a] += d[t]*2%M;
            d[idx] = d[idx] + (d[ti] * 2).rem_euclid(M);
            // e[t+a] += e[t]%M;
            e[idx] = e[idx] + e[ti].rem_euclid(M);

            t -= 1;
        }
        s += a;
        // r = r*3%M;
        r = (r * 3).rem_euclid(M);
    }

    // C: for(t=s/2; ~t;) r=(r-d[t--]*3)%M;
    t = s / 2;
    while t != -1 {
        let ti = t as usize;
        // Use C-like % behavior (remainder keeps sign), then later final normalization matches C.
        r = (r - d[ti] * 3) % M;
        t -= 1;
    }

    // C: printf("%d",(r-~s%2*e[s/2]*3+M)%M);
    // Note: ~s%2 in C: (~s) % 2, with C remainder semantics.
    let tilde_s = !s;
    let factor = (tilde_s % 2) * e[(s / 2) as usize] * 3;
    let ans = (r - factor + M) % M;

    // C prints with "%d" (int). Value is in [-(M-1), M-1] due to %; cast to i32 for printing.
    print!("{}", ans as i32);
}
