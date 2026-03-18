use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let w_opt = it.next();
        if w_opt.is_none() {
            break;
        }
        let w: i32 = w_opt.unwrap().parse().unwrap();

        // The C code does: while (scanf("%d%d", &w, &d), w) { ... }
        // i.e., it ignores scanf's return value and only checks w.
        // We still attempt to read d if present; if missing, stop (EOF).
        let d_opt = it.next();
        if d_opt.is_none() {
            break;
        }
        let d: i32 = d_opt.unwrap().parse().unwrap();

        if w == 0 {
            break;
        }

        let mut a = [0i32; 21];
        let mut b = [0i32; 21];

        for _ in 0..w {
            if let Some(tok) = it.next() {
                let t: usize = tok.parse::<i32>().unwrap() as usize;
                // C would have undefined behavior if t is out of 0..=20; we assume valid input.
                a[t] += 1;
            } else {
                break;
            }
        }

        for _ in 0..d {
            if let Some(tok) = it.next() {
                let t: usize = tok.parse::<i32>().unwrap() as usize;
                b[t] += 1;
            } else {
                break;
            }
        }

        let mut res: i32 = 0;
        for i in 1..=20 {
            let m = if a[i] > b[i] { a[i] } else { b[i] };
            res += (i as i32) * m;
        }

        println!("{res}");
    }
}
