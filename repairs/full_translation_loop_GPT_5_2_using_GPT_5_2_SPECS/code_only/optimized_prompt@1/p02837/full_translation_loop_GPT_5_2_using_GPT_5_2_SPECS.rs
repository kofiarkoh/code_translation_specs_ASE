use std::io::{self, Read};

fn counter(z: i32) -> i32 {
    // Mirrors the C recursion:
    // if (z==0) return 0; else return counter(z>>1) + (z&1);
    if z == 0 {
        0
    } else {
        counter(z >> 1) + (z & 1)
    }
}

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // no input; C would have undefined behavior, but we just exit
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
    let mut count: i32 = 0;
    let mut flag: i32 = 1;

    // In C: for (int i=0; i<(1<<n); i++)
    // Use i32 and wrapping_shl to emulate C's shift behavior as closely as possible.
    let limit: i32 = 1i32.wrapping_shl(n as u32);
    for i in 0..limit {
        flag = 1;
        for j in 0..(n as usize) {
            if (i & (1i32.wrapping_shl(j as u32))) == 0 {
                continue;
            }
            for k in 0..(a[j] as usize) {
                // C: if ((i>>(x[j][k]-1)&1)^y[j][k]) { flag=0; break; }
                // Keep i32 arithmetic and shifts; wrapping_shr handles large/negative shift counts
                // in a defined way (C would be UB), but preserves "bit trick" intent.
                let shift = (x[j][k] - 1) as u32;
                let bit = (i.wrapping_shr(shift) & 1) ^ y[j][k];
                if bit != 0 {
                    flag = 0;
                    break;
                }
            }
        }
        if flag == 1 {
            count = counter(i);
            if max_v < count {
                max_v = count;
            }
        }
    }

    // Match C: printf("%d", max);
    print!("{}", max_v);
}
