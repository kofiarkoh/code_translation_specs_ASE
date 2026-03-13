// translated from the provided C program
// Preserves C `int` (assumed 32-bit) wraparound semantics for arithmetic
// by using wrapping operations where the original C could overflow.

use std::io::{self, Read};

fn time_print(mut time: i32) {
    // Follow the same sequence of operations as the C version:
    // printf("%02d:", time/3600);
    // time %= 3600;
    // printf("%02d:", time/60);
    // time %= 60;
    // printf("%02d\n", time);
    print!("{:02}:", time / 3600);
    time = time % 3600;
    print!("{:02}:", time / 60);
    time = time % 60;
    println!("{:02}", time);
}

fn main() {
    // Read entire stdin, split by whitespace, parse as i32.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read failed");
    let mut tokens = input.split_whitespace();

    loop {
        // Attempt to read three integers; if any are missing, stop.
        let t = match tokens.next() {
            Some(s) => match s.parse::<i32>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        let h = match tokens.next() {
            Some(s) => match s.parse::<i32>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        let s_val = match tokens.next() {
            Some(s) => match s.parse::<i32>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };

        // Stop on sentinel -1 -1 -1
        if t == -1 && h == -1 && s_val == -1 {
            break;
        }

        // Emulate C's int wraparound for multiplication/addition using wrapping ops.
        let time = t.wrapping_mul(3600)
                    .wrapping_add(h.wrapping_mul(60))
                    .wrapping_add(s_val);

        // Compute 7200 - time and 7200 - (time/3).
        // Use wrapping_sub for the subtraction to mimic typical C two's-complement wrap.
        let first = 7200i32.wrapping_sub(time);
        // Division by 3 cannot overflow (unlike division by -1), so plain division is fine.
        let second = 7200i32.wrapping_sub(time / 3);

        time_print(first);
        time_print(second);
    }
}
