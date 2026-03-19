use std::io::{self, Read};

fn main() {
    // Fixed-capacity holding area (stack) of up to 10 integers.
    // Per spec, overflow/underflow is undefined; we do not add checks.
    let mut railroad = [0i32; 10];
    let mut i: isize = 0;

    // Read all input, then parse as whitespace-separated integers.
    // Spec precondition: all tokens intended to be processed are valid integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        let car: i32 = tok.parse().unwrap();

        if car == 0 {
            // Pop and print.
            i -= 1;
            out.push_str(&format!("{}\n", railroad[i as usize]));
        } else {
            // Push.
            railroad[i as usize] = car;
            i += 1;
        }
    }

    print!("{out}");
}
