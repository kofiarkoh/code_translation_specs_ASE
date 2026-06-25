use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let w_opt = it.next();
        if w_opt.is_none() {
            break; // no more input
        }
        let w: i32 = w_opt.unwrap().parse().unwrap();

        // The C code does: while (scanf("%d%d", &w, &d), w) { ... }
        // It ignores scanf's return value; we emulate by requiring d to exist if w exists.
        let d: i32 = it.next().unwrap().parse().unwrap();

        if w == 0 {
            break;
        }

        let mut a = [0i32; 21];
        let mut b = [0i32; 21];

        for _ in 0..w {
            let t: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;
            // C would have undefined behavior if t is out of 0..=20; assume input is valid.
            a[t] += 1;
        }
        for _ in 0..d {
            let t: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;
            b[t] += 1;
        }

        let mut res: i32 = 0;
        for i in 1..=20 {
            let m = if a[i] > b[i] { a[i] } else { b[i] };
            // Use wrapping arithmetic to mimic typical C int overflow behavior.
            res = res.wrapping_add((i as i32).wrapping_mul(m));
        }

        println!("{res}");
    }
}
