use std::io::{self, Read};

const P: i64 = 1_000_000_007;

fn mod_p(a: i64) -> i64 {
    let mut x = a % P;
    if x < 0 {
        x += P;
    }
    x
}

fn power(a: i64, n: i64) -> i64 {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        power(mod_p(a.wrapping_mul(a)), n / 2)
    } else {
        mod_p(a.wrapping_mul(power(a, n - 1)))
    }
}

fn inverse(a: i64) -> i64 {
    power(a, P - 2)
}

fn max_i64(a: i64, b: i64) -> i64 {
    if a >= b { a } else { b }
}

fn main() {
    // Read all input, then parse like scanf("%lld%lld%lld") and scanf("%s")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let w: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    let max_hw = max_i64(h, w) as usize;

    // Emulate the C allocation: char** with max_hw rows, each length max_hw+1.
    // We'll store as bytes; uninitialized bytes in C are never read in this program.
    let mut s: Vec<Vec<u8>> = vec![vec![0u8; max_hw + 1]; max_hw];

    for i in 0..(h as usize) {
        let row = it.next().unwrap_or("");
        let bytes = row.as_bytes();
        // scanf("%s") writes a NUL terminator; we keep the rest as 0.
        for j in 0..bytes.len().min(max_hw) {
            s[i][j] = bytes[j];
        }
        // s[i][bytes.len()] is already 0
    }

    if k == 0 {
        print!("1\n");
        return;
    }

    let mut f_yoko = 0i64;
    let mut f_tate = 0i64;

    for i in 0..(h as usize) {
        if s[i][0] == b'#' && s[i][(w as usize) - 1] == b'#' {
            f_yoko = 1;
        }
    }
    for j in 0..(w as usize) {
        if s[0][j] == b'#' && s[(h as usize) - 1][j] == b'#' {
            f_tate = 1;
        }
    }

    if f_yoko == 1 && f_tate == 1 {
        print!("1\n");
        return;
    }

    let mut h2 = h;
    let mut w2 = w;

    if f_tate == 1 {
        // Transpose the max_hw x max_hw area, exactly like the C code.
        for i in 0..max_hw {
            for j in 0..max_hw {
                if i < j {
                    let tmp = s[i][j];
                    s[i][j] = s[j][i];
                    s[j][i] = tmp;
                }
            }
        }
        // XOR swap H and W (kept for semantic fidelity)
        h2 ^= w2;
        w2 ^= h2;
        h2 ^= w2;
    }

    let mut x: i64 = 0;
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    let mut c: i64 = 0;

    for i in 0..(h2 as usize) {
        let mut x_sub: i64 = 0;
        for j in 0..(w2 as usize) {
            if s[i][j] == b'#' {
                x_sub += 1;
            }
        }
        let mut e: i64 = 0;
        for j in 1..(w2 as usize) {
            if s[i][j - 1] == b'#' && s[i][j] == b'#' {
                e += 1;
            }
        }
        if x_sub == w2 {
            c += 1;
        } else {
            a += x_sub - e;
            if s[i][0] == b'#' && s[i][(w2 as usize) - 1] == b'#' {
                a -= 2;
                b += 1;
            }
        }
        x += x_sub;
    }

    if f_yoko == 0 && f_tate == 0 {
        print!("{}\n", power(x, k - 1));
    } else if a == 0 && b == 0 {
        print!("{}\n", power(c, k - 1));
    } else {
        // Use wrapping arithmetic to emulate C signed overflow behavior as closely as possible.
        let ab = a.wrapping_add(b);
        let term1 = ab.wrapping_mul(power(x, k - 1));
        let inner = a
            .wrapping_add(b.wrapping_mul(2))
            .wrapping_add(c);
        let term2 = x
            .wrapping_sub(inner)
            .wrapping_mul(power(b.wrapping_add(c), k - 1));
        let num = term1.wrapping_add(term2);
        let num_mod = mod_p(num);
        let denom = x.wrapping_sub(b.wrapping_add(c));
        let ans = mod_p(num_mod.wrapping_mul(inverse(denom)));
        print!("{}\n", ans);
    }
}
