use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let n_usize = n as usize;

    // Fixed internal capacity per spec/C code (undefined behavior if exceeded in C).
    // In Rust we still read into a Vec; we do not add explicit error handling.
    let mut a: Vec<i64> = Vec::with_capacity(n_usize);
    for _ in 0..n_usize {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    let mut ans: i64 = 0;

    // C loop: for (int d=1; d*2 < n-1; d++)
    // This matches spec's "considered step sizes".
    for d in 1.. {
        if d * 2 >= n - 1 {
            break;
        }

        let mut s: i32 = 0;
        let mut g: i32 = n - 1;
        let mut crr: i64 = 0;

        loop {
            let ns = s + d;
            let ng = g - d;

            // C break condition:
            // if (ng <= d || ns == ng || ns == g) break;
            // Note: This is slightly stricter than the spec's narrative wording in places;
            // we follow the C code to resolve edge cases.
            if ng <= d || ns == ng || ns == g {
                break;
            }

            s = ns;
            g = ng;

            crr += a[s as usize] + a[g as usize];
            if crr > ans {
                ans = crr;
            }
        }
    }

    // Output exactly as C: no trailing newline.
    print!("{ans}");
}
