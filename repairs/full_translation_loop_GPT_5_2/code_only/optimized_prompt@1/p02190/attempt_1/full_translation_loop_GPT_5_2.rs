use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches scanf-style whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // If input is missing, C would have undefined behavior; we just exit quietly.
    let n_opt = it.next();
    if n_opt.is_none() {
        return;
    }
    let n: usize = n_opt.unwrap().parse().unwrap();

    // C has a fixed array a[100001]; we emulate by reading n values into a Vec.
    // If fewer than n integers are present, C would read garbage/UB; we unwrap to fail fast.
    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i32 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // qsort with comparator equivalent to integer ascending order.
    a.sort_unstable();

    // C code assumes n >= 1 (uses a[0]); for n == 0 it would be UB.
    // We emulate a reasonable behavior: print 0 distinct elements.
    if n == 0 {
        print!("0\n");
        return;
    }

    let mut prev = a[0];
    let mut ret: i32 = 1;
    for &x in a.iter().skip(1) {
        if x != prev {
            ret += 1;
        }
        prev = x;
    }

    print!("{}\n", ret);
}
