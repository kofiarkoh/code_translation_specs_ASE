use std::io::{self, Read};

fn clamp_i64(v: i64, lo: i64, hi: i64) -> i64 {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        // Spec says at most 101 vertices are supported; we just store what we read.
        let mut xs: Vec<i64> = Vec::with_capacity(n);
        let mut ys: Vec<i64> = Vec::with_capacity(n);

        for _ in 0..n {
            let x: i64 = it.next().unwrap().parse().unwrap();
            let y: i64 = it.next().unwrap().parse().unwrap();
            xs.push(x);
            ys.push(y);
        }

        // Read 4 points defining the rectangle via extrema.
        let mut xmin: i64 = i64::MAX;
        let mut xmax: i64 = i64::MIN;
        let mut ymin: i64 = i64::MAX;
        let mut ymax: i64 = i64::MIN;

        for _ in 0..4 {
            let a: i64 = it.next().unwrap().parse().unwrap();
            let b: i64 = it.next().unwrap().parse().unwrap();
            if a < xmin {
                xmin = a;
            }
            if a > xmax {
                xmax = a;
            }
            if b < ymin {
                ymin = b;
            }
            if b > ymax {
                ymax = b;
            }
        }

        let mut cur: i64 = 0;
        let mut win: i64 = 0;

        for i in 0..n {
            let j = if i + 1 == n { 0 } else { i + 1 };

            if xs[i] == xs[j] {
                // Use i64 arithmetic; overflow behavior is unspecified by the spec,
                // and the spec assumes intermediates fit in signed 64-bit.
                cur += xs[i] * (ys[i] - ys[j]);

                let a = clamp_i64(xs[i], xmin, xmax);
                let b = clamp_i64(ys[i], ymin, ymax);
                let c = clamp_i64(ys[j], ymin, ymax);
                win += a * (b - c);
            }
        }

        let ans = win - cur;
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
