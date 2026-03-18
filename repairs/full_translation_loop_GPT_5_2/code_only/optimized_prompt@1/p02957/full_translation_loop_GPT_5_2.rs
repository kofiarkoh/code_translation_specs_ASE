use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-like behavior: whitespace-separated tokens).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C code does not check scanf return value; if input is missing, behavior is undefined.
    // Here we mimic "must be present" by unwrapping.
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // In C, (a+b) on long int can overflow (undefined behavior). We emulate typical
    // two's-complement wraparound using wrapping_add.
    let sum = a.wrapping_add(b);

    if sum % 2 != 0 {
        print!("IMPOSSIBLE");
        return;
    }

    // C prints with "%d" (int) even though expression is long int; this is UB.
    // We emulate common behavior by truncating to 32-bit signed int before printing.
    let half = sum / 2;
    let as_int32 = half as i32;
    print!("{}", as_int32);
}
