use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like C's "%d".
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse::<i32>().unwrap() as usize,
        None => return, // No input; C would have undefined behavior, but we just exit.
    };

    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(s) = it.next() {
            a.push(s.parse::<i32>().unwrap());
        } else {
            // Missing numbers; C would leave uninitialized/undefined. We stop.
            break;
        }
    }

    // Emulate qsort with integer ascending order.
    a.sort_unstable();

    // The C code assumes n >= 1 (it reads a[0] unconditionally).
    // If n == 0, C would have undefined behavior; we choose to print 0.
    if a.is_empty() {
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
