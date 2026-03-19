use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-like whitespace-separated tokens)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Globals in C are zero-initialized; emulate with fixed-size arrays.
    let mut a = [0i32; 1000];
    let mut tmp = [0i32; 1000];

    let n: i32 = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return, // no input
    };

    for i in 0..(n as usize) {
        a[i] = it.next().unwrap().parse::<i32>().unwrap();
    }

    let mut q: i32 = it.next().unwrap().parse().unwrap();
    while q > 0 {
        q -= 1;
        let mut b: i32 = it.next().unwrap().parse().unwrap();
        let mut m: i32 = it.next().unwrap().parse().unwrap();
        let mut e: i32 = it.next().unwrap().parse().unwrap();

        // Copy a[b..e) into tmp at the same indices.
        // Note: The original C code has undefined behavior if indices are out of bounds;
        // we assume inputs keep indices within [0, 1000) and b <= e.
        for i in b..e {
            tmp[i as usize] = a[i as usize];
        }

        e -= b;
        m -= b;

        // Rotate segment: a[b+i] = tmp[b + (i+m)%e]
        // In C, (i+m)%e with negative values is well-defined (remainder keeps sign of numerator),
        // but can yield negative indices -> UB. We preserve behavior for valid inputs (non-negative).
        for i in 0..e {
            let idx = (i + m) % e;
            a[(b + i) as usize] = tmp[(b + idx) as usize];
        }
    }

    // Print with spaces and final newline exactly like: printf("%d%c", a[i], " \n"[i+1==n]);
    let n_usize = n as usize;
    let mut out = String::new();
    for i in 0..n_usize {
        out.push_str(&a[i].to_string());
        out.push(if i + 1 == n_usize { '\n' } else { ' ' });
    }
    print!("{out}");
}
