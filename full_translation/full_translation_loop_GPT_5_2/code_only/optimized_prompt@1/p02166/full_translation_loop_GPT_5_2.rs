use std::io::{self, Read};

fn dfs(a: &mut [i32], n: usize) {
    let mut t = a[n] as usize;
    while a[t] != -1 {
        let temp = a[t] as usize;
        a[t] = -1;
        t = temp;
    }
}

fn main() {
    // Read entire stdin, parse as integers (scanf-like whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: i32 = it.next().unwrap().parse().unwrap();

    // C code uses 1-based indexing and a global array; keep 1-based with extra slot.
    let mut a = vec![0i32; n + 1];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    if n as i32 == k {
        // Emulate C's int arithmetic and % behavior (remainder keeps sign in C and Rust).
        let x: i32 = a[1] - 1;
        let mut flag: i32 = 1;
        let nn = n as i32;
        for i in 1..=n {
            let ii = i as i32;
            let cond = ((a[i] - ii + nn) % nn) == x;
            flag &= if cond { 1 } else { 0 };
        }
        if flag != 0 {
            print!("Yes\n");
        } else {
            print!("No\n");
        }
    } else {
        let mut cnt: i32 = 0;
        for i in 1..=n {
            if a[i] != -1 {
                dfs(&mut a, i);
                cnt += 1;
            }
        }
        let expr = ((n as i32 - cnt) % 2 != 0) && (k % 2 != 0);
        if expr {
            print!("No\n");
        } else {
            print!("Yes\n");
        }
    }
}
