use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse::<i32>().unwrap() as usize,
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

    // In C, signed overflow is UB; typical contest inputs avoid it.
    // We use wrapping arithmetic to be robust and to emulate common two's-complement behavior.
    let mut sum: i32 = 0;
    for i in 0..n {
        sum = sum.wrapping_add(t[i]);
    }

    let mut out = String::new();
    for i in 0..m {
        let idx = (p[i] - 1) as usize;
        let ans = sum.wrapping_sub(t[idx]).wrapping_add(x[i]);
        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
