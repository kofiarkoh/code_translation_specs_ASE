use std::io::{self, Read};

const P: i64 = 1_000_000_007;

fn mod_norm(a: i64) -> i64 {
    let mut x = a % P;
    if x < 0 {
        x += P;
    }
    x
}

fn mod_pow(mut a: i64, mut n: i64) -> i64 {
    // Semantics match the C recursive power(): for n==0 => 1, otherwise standard mod exp.
    a = mod_norm(a);
    let mut r: i64 = 1;
    while n > 0 {
        if (n & 1) == 1 {
            r = mod_norm(r * a);
        }
        a = mod_norm(a * a);
        n >>= 1;
    }
    r
}

fn mod_inv(a: i64) -> i64 {
    // As in C: inverse(a) = a^(P-2) mod P. If a == 0 (mod P), behavior is mathematically undefined
    // but we still compute it (will yield 0 with this exponentiation).
    mod_pow(a, P - 2)
}

fn main() {
    // Read all input as whitespace-delimited tokens (matches scanf("%s") behavior for rows).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut h: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;
    let mut w: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;
    let k: i64 = it.next().unwrap().parse::<i64>().unwrap();

    let max_hw = h.max(w);

    // Square buffer max_hw x max_hw, initialized with '\0' like uninitialized C memory would be
    // (but we must initialize in Rust). This is safe because only positions that are read are
    // those written by input or by transposition from written cells.
    let mut s = vec![vec![b'\0'; max_hw]; max_hw];

    for i in 0..h {
        let row = it.next().unwrap().as_bytes();
        // Spec: each row has at least W characters.
        for j in 0..w {
            s[i][j] = row[j];
        }
    }

    if k == 0 {
        print!("1\n");
        return;
    }

    // Boundary flags from the original grid.
    let mut f_row = false;
    let mut f_col = false;

    for i in 0..h {
        if s[i][0] == b'#' && s[i][w - 1] == b'#' {
            f_row = true;
        }
    }
    for j in 0..w {
        if s[0][j] == b'#' && s[h - 1][j] == b'#' {
            f_col = true;
        }
    }

    if f_row && f_col {
        print!("1\n");
        return;
    }

    // Orientation normalization:
    // SPEC says transpose only when f_col is true and f_row is false.
    // The C code transposes whenever f_col is true (even if f_row is true),
    // but that case is unreachable here because (f_row && f_col) already returned.
    if f_col && !f_row {
        for i in 0..max_hw {
            for j in (i + 1)..max_hw {
                let tmp = s[i][j];
                s[i][j] = s[j][i];
                s[j][i] = tmp;
            }
        }
        std::mem::swap(&mut h, &mut w);
    }

    // Per-row aggregates after normalization.
    let mut x: i64 = 0;
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    let mut c: i64 = 0;

    for i in 0..h {
        let mut x_sub: i64 = 0;
        for j in 0..w {
            if s[i][j] == b'#' {
                x_sub += 1;
            }
        }

        let mut e: i64 = 0;
        for j in 1..w {
            if s[i][j - 1] == b'#' && s[i][j] == b'#' {
                e += 1;
            }
        }

        if x_sub == w as i64 {
            c += 1;
        } else {
            a += x_sub - e;
            if s[i][0] == b'#' && s[i][w - 1] == b'#' {
                a -= 2;
                b += 1;
            }
        }

        x += x_sub;
    }

    // Output formula.
    if !f_row && !f_col {
        // Note: uses x from the scan; equals total marked cells.
        let ans = mod_pow(x, k - 1);
        print!("{ans}\n");
    } else if a == 0 && b == 0 {
        let ans = mod_pow(c, k - 1);
        print!("{ans}\n");
    } else {
        let term1 = mod_norm((a + b) * mod_pow(x, k - 1));
        let term2 = mod_norm((x - (a + 2 * b + c)) * mod_pow(b + c, k - 1));
        let num = mod_norm(term1 + term2);
        let den = x - (b + c);
        let ans = mod_norm(num * mod_inv(den));
        print!("{ans}\n");
    }
}
