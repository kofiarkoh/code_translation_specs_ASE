use std::io::{self, Read};

fn rev(mut n: i32) -> i32 {
    let mut out: i32 = 0;
    while n > 0 {
        // In C, signed overflow is UB; typical platforms wrap. We emulate wrapping.
        out = out.wrapping_mul(10).wrapping_add(n % 10);
        n /= 10;
    }
    out
}

fn main() {
    // Read entire stdin, parse first integer like scanf("%d", &n).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = match it.next() {
        Some(s) => s.parse::<i32>().unwrap(),
        None => return, // If no input, C would leave n uninitialized (UB); we just exit.
    };

    let mut i: i32 = 0;

    loop {
        // Emulate C's int arithmetic with wrapping behavior.
        let n_minus_i = n.wrapping_sub(i);
        if n_minus_i >= 0 {
            if rev(n_minus_i) == n_minus_i {
                println!("{}", n_minus_i);
                break;
            }
        }

        let n_plus_i = n.wrapping_add(i);
        if rev(n_plus_i) == n_plus_i {
            println!("{}", n_plus_i);
            break;
        }

        i = i.wrapping_add(1);
    }
}
