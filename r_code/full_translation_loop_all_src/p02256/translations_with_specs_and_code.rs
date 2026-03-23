use std::io::{self, Read};
use std::process;

const INPUT_NUM_MIN: u32 = 1;
const INPUT_NUM_MAX: u32 = 1_000_000_000;

fn calc_gcd_u32(mut large: u32, mut small: u32) -> u32 {
    // Euclidean algorithm, matching the C loop structure:
    // - If divisor becomes 0, stop and return current divisor (0).
    // - Otherwise iterate until remainder becomes 0, then return divisor.
    loop {
        if small == 0 {
            break;
        }
        let rem = large % small;
        if rem == 0 {
            break;
        }
        large = small;
        small = rem;
    }
    small
}

fn main() {
    // Read entire stdin; behavior for malformed/missing input is unspecified by spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let x: u32 = it.next().unwrap().parse().unwrap();
    let y: u32 = it.next().unwrap().parse().unwrap();

    // Range validation: if out of range, produce no output and exit with status 1.
    if x < INPUT_NUM_MIN || x > INPUT_NUM_MAX || y < INPUT_NUM_MIN || y > INPUT_NUM_MAX {
        process::exit(1);
    }

    // Order so first argument is larger (or equal), per spec.
    let gcd = if x > y {
        calc_gcd_u32(x, y)
    } else {
        calc_gcd_u32(y, x)
    };

    print!("{gcd}\n");
    process::exit(0);
}
