use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style whitespace-separated tokens)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // The C code uses `int` and relies on its arithmetic behavior.
    // We use i32 and wrapping arithmetic to emulate typical 32-bit `int` overflow behavior.
    let n: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;

    let mut t = [0i32; 100];
    let mut p = [0i32; 100];
    let mut x = [0i32; 100];

    for i in 0..n {
        t[i] = it.next().unwrap().parse::<i32>().unwrap();
    }

    let m: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;

    for i in 0..m {
        p[i] = it.next().unwrap().parse::<i32>().unwrap();
        x[i] = it.next().unwrap().parse::<i32>().unwrap();
    }

    let mut sum: i32 = 0;
    for i in 0..n {
        sum = sum.wrapping_add(t[i]);
    }

    let mut out = String::new();
    for i in 0..m {
        // C indexing: t[p[i]-1]
        let idx = (p[i].wrapping_sub(1)) as usize;
        let ans = sum
            .wrapping_sub(t[idx])
            .wrapping_add(x[i]);
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
