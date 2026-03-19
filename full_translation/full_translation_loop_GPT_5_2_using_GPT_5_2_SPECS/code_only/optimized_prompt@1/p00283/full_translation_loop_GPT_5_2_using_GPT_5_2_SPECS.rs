use std::io::{self, Read};

const INF: i32 = 1000;

fn search(
    tkrk: i32,
    kkt: usize,
    n: usize,
    a: &[i32],
    e: &[i32],
    r: &[i32],
    memo: &mut [[i32; 101]; 101],
) -> i32 {
    if tkrk <= 0 {
        return 0;
    }
    // C code indexes memo[tkrk][kkt] assuming tkrk in 0..=100.
    // We preserve that assumption; inputs outside that range would be UB in C.
    let t_idx = tkrk as usize;

    if memo[t_idx][kkt] > 0 {
        return memo[t_idx][kkt] - 1;
    }

    let mut ret = INF;
    memo[t_idx][kkt] = INF + 1;

    for i in 0..n {
        if (kkt as i32) >= r[i] {
            let mut next_kkt = (kkt as i32) + e[i];
            if next_kkt > 100 {
                next_kkt = 100;
            }
            // C would have UB if next_kkt < 0; we assume inputs avoid that,
            // matching the C program's implicit precondition.
            let next_kkt_usize = next_kkt as usize;

            let candidate = search(tkrk - a[i], next_kkt_usize, n, a, e, r, memo) + 1;
            if candidate < ret {
                ret = candidate;
            }
        }
    }

    memo[t_idx][kkt] = ret + 1;
    ret
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let d_opt = it.next();
        let n_opt = it.next();
        if d_opt.is_none() || n_opt.is_none() {
            break;
        }
        let d: i32 = d_opt.unwrap().parse().unwrap();
        let n: usize = n_opt.unwrap().parse().unwrap();

        if d == 0 && n == 0 {
            break;
        }

        let mut a = vec![0i32; n];
        let mut e = vec![0i32; n];
        let mut r = vec![0i32; n];

        for i in 0..n {
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
        let ret = search(d, 0, n, &a, &e, &r, &mut memo);

        if ret == INF {
            print!("NA\n");
        } else {
            print!("{}\n", ret);
        }
    }
}
