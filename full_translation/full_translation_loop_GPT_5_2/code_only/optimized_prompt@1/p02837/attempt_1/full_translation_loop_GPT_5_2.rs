use std::io::{self, Read};

fn counter(z: i32) -> i32 {
    if z == 0 {
        0
    } else {
        // In C, right shift of signed negative is implementation-defined.
        // Here z is always derived from loop index i (non-negative), so this matches.
        counter(z >> 1) + (z & 1)
    }
}

fn main() {
    // Read entire stdin and parse as integers (scanf-like whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Match C fixed-size arrays initialized to 0.
    let mut a = [0i32; 20];
    let mut x = [[0i32; 20]; 20];
    let mut y = [[0i32; 20]; 20];

    for i in 0..(n as usize) {
        a[i] = it.next().unwrap().parse().unwrap();
        for j in 0..(a[i] as usize) {
            x[i][j] = it.next().unwrap().parse().unwrap();
            y[i][j] = it.next().unwrap().parse().unwrap();
        }
    }

    let mut max_v: i32 = 0;

    // C uses (1<<n) with int; assume n is such that this is meaningful.
    // Use i32 to preserve C-like bit operations.
    let limit: i32 = 1i32 << n;

    for i in 0..limit {
        let mut flag: i32 = 1;
        for j in 0..(n as usize) {
            if (i & (1i32 << (j as i32))) == 0 {
                continue;
            }
            for k in 0..(a[j] as usize) {
                // Replicate: (i>>(x[j][k]-1)&1)^y[j][k]
                let shift = x[j][k] - 1;
                let bit = (i >> shift) & 1;
                let cond = (bit ^ y[j][k]) != 0;
                if cond {
                    flag = 0;
                    break;
                }
            }
            if flag == 0 {
                break;
            }
        }

        if flag == 1 {
            let count = counter(i);
            if max_v < count {
                max_v = count;
            }
        }
    }

    // Match C: no trailing newline.
    print!("{}", max_v);
}
