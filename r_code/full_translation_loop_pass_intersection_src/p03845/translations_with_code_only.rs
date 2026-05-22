use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches scanf-style whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // The C code uses `int` and relies on its arithmetic behavior.
    // Use i32 and wrapping ops to emulate typical 32-bit two's-complement overflow behavior.
    let n: usize = match it.next() {
        Some(s) => s.parse::<i32>().unwrap() as usize,
        None => return,
    };

    let mut t = vec![0i32; 100];
    let mut p = vec![0i32; 100];
    let mut x = vec![0i32; 100];

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
        let idx = p[i].wrapping_sub(1) as usize;
        let ans = sum
            .wrapping_sub(t[idx])
            .wrapping_add(x[i]);
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
