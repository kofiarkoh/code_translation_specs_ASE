use std::io::{self, Read};

fn main() {
    // Read all input tokens as integers
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a = vec![0i32; n];
    let mut b = vec![0i32; n];
    let mut c = vec![0i32; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse().unwrap();
        b[i] = it.next().unwrap().parse().unwrap();
        c[i] = it.next().unwrap().parse().unwrap();
    }

    let m = 1usize << n;

    // d initialized to 1e9, fr to -1
    let mut d = vec![vec![1e9f64; m]; n];
    let mut fr = vec![vec![-1i32; m]; n];

    // start states
    for i in 0..n {
        d[i][1usize << i] = 0.0;
    }

    // f as in C: uses global-like arrays a,b,c,n
    let f = |i: usize, j: usize, k: usize| -> f64 {
        if j == 0 {
            return 0.0;
        }
        let mut w: i32 = 0;
        // sum c[l]*20 for bits set in j
        for l in 0..n {
            if (j & (1usize << l)) != 0 {
                w += c[l] * 20;
            }
        }
        let absdiff = (b[k] - b[i]).abs() as f64;
        absdiff * ( (w + 70) as f64 ) / 2000.0
    };

    // NOTE: The original C code has a subtle precedence bug:
    //    if((1<<i)&j==0)continue;
    // which parses in C as (1<<i) & (j==0) and then tests that result for non-zero.
    // We reproduce that exact behavior here to preserve semantics.
    for mask in 0..m {
        for i in 0..n {
            // reproduce C's (1<<i)&(j==0) truthiness test:
            let c_bit = if mask == 0 { 1usize } else { 0usize };
            if ((1usize << i) & c_bit) != 0 {
                continue;
            }
            for k in 0..n {
                if (mask & (1usize << k)) != 0 {
                    continue;
                }
                let newmask = mask + (1usize << k);
                let val = d[i][mask] + f(i, mask, k);
                if d[k][newmask] > val {
                    d[k][newmask] = val;
                    fr[k][newmask] = i as i32;
                }
            }
        }
    }

    let full = m - 1;
    // find argmin over d[i][full]
    let mut best = 0usize;
    for i in 0..n {
        if d[i][full] < d[best][full] {
            best = i;
        }
    }

    // reconstruct path into r (indices)
    let mut r = vec![0usize; n];
    r[n - 1] = best;
    let mut kmask = full;
    for i in 1..n {
        let prev = r[n - i]; // index of the node placed at position n-i
        let frval = fr[prev][kmask];
        // In C this was assigned to an int array; negative values would lead to undefined behavior.
        // Here we cast the stored i32 to usize, matching the bit-pattern conversion semantics.
        r[n - i - 1] = frval as usize;
        kmask = kmask - (1usize << r[n - i]);
    }

    // output a[r[0]] ... a[r[n-1]] with spaces and final newline, matching C printf formatting
    if n > 0 {
        for i in 0..(n - 1) {
            print!("{} ", a[r[i]]);
        }
        println!("{}", a[r[n - 1]]);
    }
}
