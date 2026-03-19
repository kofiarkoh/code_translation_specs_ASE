use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let w_opt = it.next();
        if w_opt.is_none() {
            break; // No more input; unspecified by spec, but we stop.
        }
        let w: i32 = w_opt.unwrap().parse().unwrap();
        let d: i32 = it.next().unwrap().parse().unwrap();

        if w == 0 {
            break; // Termination condition per spec: first integer is zero.
        }

        // Counts for values 1..=20; index 0 unused.
        let mut a = [0i32; 21];
        let mut b = [0i32; 21];

        for _ in 0..w {
            let t: usize = it.next().unwrap().parse().unwrap();
            a[t] += 1;
        }
        for _ in 0..d {
            let t: usize = it.next().unwrap().parse().unwrap();
            b[t] += 1;
        }

        let mut res: i32 = 0;
        for v in 1..=20 {
            let m = if a[v] > b[v] { a[v] } else { b[v] };
            res += (v as i32) * m;
        }

        out.push_str(&format!("{res}\n"));
    }

    print!("{out}");
}
