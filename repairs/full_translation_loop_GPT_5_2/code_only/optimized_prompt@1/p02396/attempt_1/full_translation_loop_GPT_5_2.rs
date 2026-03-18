use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse integers similarly to repeated scanf("%d", &x).
    // Note: In the C code, failing scanf would leave `x` uninitialized (UB). We assume
    // well-formed input containing integers, as typical for such programs.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let mut i: i32 = 0;

    loop {
        let x: i32 = match it.next() {
            Some(tok) => tok.parse::<i32>().unwrap(),
            None => break, // no more input; C would likely block or hit EOF
        };

        // C `int` overflow is UB; use wrapping add to emulate common two's-complement behavior.
        i = i.wrapping_add(1);

        if x == 0 {
            break;
        }
        print!("Case {}: {}\n", i, x);
    }
}
