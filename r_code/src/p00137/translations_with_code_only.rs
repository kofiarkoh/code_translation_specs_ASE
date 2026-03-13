// Translated from the provided C code.
// Notes:
// - Uses i32 to represent C `int` (typical 32-bit).
// - Uses wrapping arithmetic for operations that can overflow in C (e.g., s*s and multiplications/additions),
//   to emulate two's-complement wraparound behavior that many C implementations exhibit.
// - The while(d--) loop from C is emulated with wrapping subtraction to match post-decrement semantics
//   (and to mimic wraparound behavior on two's-complement platforms).

use std::io::{self, Read};

fn pow10(mut n: i32) -> i32 {
    let mut ret: i32 = 1;
    while n > 0 {
        // use wrapping to emulate C's potential overflow behavior
        ret = ret.wrapping_mul(10);
        n -= 1;
    }
    ret
}

fn solve(s: i32) -> i32 {
    // emulate s*s with wrapping multiplication
    let mut n: i32 = s.wrapping_mul(s) / 100;
    let mut ret: i32 = 0;
    for i in 0..4 {
        let digit = n % 10; // remainder behavior matches C/Rust (sign of dividend)
        let p = pow10(i);
        // use wrapping add/mul to emulate C wraparound
        ret = ret.wrapping_add(digit.wrapping_mul(p));
        n /= 10; // integer division truncates toward zero in Rust, same as C
    }
    ret
}

fn main() {
    // Read all stdin and split on ASCII whitespace, like scanf("%d",...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut tokens = input.split_whitespace();

    // parse d as i32
    let d_i32: i32 = match tokens.next() {
        Some(t) => t.parse().unwrap_or(0),
        None => return,
    };

    // Emulate C's while(d--) loop:
    // Each iteration: the old value of d is tested; if old == 0 loop stops;
    // after the test d is decremented (post-decrement). We emulate this with
    // wrapping_sub to mimic two's-complement wraparound behavior.
    let mut d = d_i32;
    let mut case_i: i32 = 1;
    loop {
        let old = d;
        d = d.wrapping_sub(1);
        if old == 0 {
            break;
        }

        // read s
        let s: i32 = match tokens.next() {
            Some(t) => t.parse().unwrap_or(0),
            None => 0, // if input missing, default to 0 (best-effort)
        };

        println!("Case {}:", case_i);
        // replicate the C code that mutates s across 10 iterations
        let mut cur = s;
        for _ in 0..10 {
            cur = solve(cur);
            println!("{}", cur);
        }

        case_i = case_i.wrapping_add(1);
    }
}
