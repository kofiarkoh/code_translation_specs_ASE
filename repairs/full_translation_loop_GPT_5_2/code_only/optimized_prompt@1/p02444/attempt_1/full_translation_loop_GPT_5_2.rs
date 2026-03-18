use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style whitespace separated integers)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Global arrays in C are zero-initialized; emulate with fixed-size arrays.
    let mut a = [0i32; 1000];
    let mut tmp = [0i32; 1000];

    let n: i32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // no input
    };

    for i in 0..n {
        let v: i32 = it.next().unwrap().parse().unwrap();
        // C would write out of bounds if n > 1000; assume input respects bounds.
        a[i as usize] = v;
    }

    let mut q: i32 = it.next().unwrap().parse().unwrap();
    while q > 0 {
        q -= 1;
        let mut b: i32 = it.next().unwrap().parse().unwrap();
        let mut m: i32 = it.next().unwrap().parse().unwrap();
        let mut e: i32 = it.next().unwrap().parse().unwrap();

        // Copy a[b..e) into tmp[b..e)
        for i in b..e {
            tmp[i as usize] = a[i as usize];
        }

        // Adjust to segment-relative indices
        e -= b;
        m -= b;

        // Rotate segment: a[b+i] = tmp[b + (i+m)%e]
        // Note: In C, if e==0 this is UB (mod by zero). We preserve behavior by
        // doing nothing in that case (best-effort without invoking panic).
        if e != 0 {
            for i in 0..e {
                // C's % with negative values keeps the sign of the dividend.
                // Reproduce that, then map into [0, e) for indexing.
                let r = (i + m) % e;
                let idx = if r < 0 { r + e } else { r };
                a[(b + i) as usize] = tmp[(b + idx) as usize];
            }
        }
    }

    // Output exactly like: printf("%d%c", a[i], " \n"[i + 1 == n]);
    let mut out = String::new();
    for i in 0..n {
        let v = a[i as usize];
        let sep = if i + 1 == n { '\n' } else { ' ' };
        out.push_str(&format!("{}{}", v, sep));
    }
    print!("{}", out);
}
