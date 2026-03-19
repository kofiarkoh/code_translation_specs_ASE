use std::io::{self, Read};

const INF: i32 = 1000;

fn search(
    tkrk: i32,
    kkt: usize,
    n: usize,
    a: &[i32; 100],
    e: &[i32; 100],
    r: &[i32; 100],
    memo: &mut [[i32; 101]; 101],
) -> i32 {
    if tkrk <= 0 {
        return 0;
    }

    // C code indexes memo[tkrk][kkt] assuming tkrk in 1..=100 and kkt in 0..=100.
    // We preserve that assumption; inputs outside that would be UB in C.
    let t_idx = tkrk as usize;

    if memo[t_idx][kkt] > 0 {
        return memo[t_idx][kkt] - 1;
    }

    memo[t_idx][kkt] = INF + 1; // mark as visited (same trick as C)
    let mut ret = INF;

    for i in 0..n {
        if (kkt as i32) >= r[i] {
            let mut next_kkt = (kkt as i32) + e[i];
            if next_kkt > 100 {
                next_kkt = 100;
            }
            if next_kkt < 0 {
                // Not expected in typical inputs; C would index out of bounds (UB).
                // We clamp to 0 to avoid panic while keeping behavior as close as possible.
                next_kkt = 0;
            }
            let candidate = search(tkrk - a[i], next_kkt as usize, n, a, e, r, memo) + 1;
            if candidate < ret {
                ret = candidate;
            }
        }
    }

    memo[t_idx][kkt] = ret + 1;
    ret
}

fn main() {
    // Read entire stdin and parse as whitespace-separated integers (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let d_opt = it.next();
        let n_opt = it.next();
        if d_opt.is_none() || n_opt.is_none() {
            break;
        }
        let d: i32 = d_opt.unwrap().parse().unwrap();
        let n: i32 = n_opt.unwrap().parse().unwrap();

        if d == 0 && n == 0 {
            break;
        }

        let n_usize = n as usize;

        let mut a = [0i32; 100];
        let mut e = [0i32; 100];
        let mut r = [0i32; 100];

        for i in 0..n_usize {
            // In C, failing to read 3 ints returns 1 immediately.
            // Here, if missing, we exit with code 1 by returning from main after printing nothing further.
            let aa = match it.next() {
                Some(v) => v.parse::<i32>().unwrap(),
                None => std::process::exit(1),
            };
            let ee = match it.next() {
                Some(v) => v.parse::<i32>().unwrap(),
                None => std::process::exit(1),
            };
            let rr = match it.next() {
                Some(v) => v.parse::<i32>().unwrap(),
                None => std::process::exit(1),
            };
            a[i] = aa;
            e[i] = ee;
            r[i] = rr;
        }

        let mut memo = [[0i32; 101]; 101];
        let ret = search(d, 0, n_usize, &a, &e, &r, &mut memo);

        if ret == INF {
            out.push_str("NA\n");
        } else {
            out.push_str(&format!("{ret}\n"));
        }
    }

    print!("{out}");
}
