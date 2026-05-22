use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin; on failure, terminate with non-zero exit status and no specified output.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }

    let mut it = input.split_whitespace();

    // Required input 1: non-negative integer count.
    let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    // Required input 2: non-empty, whitespace-delimited character sequence.
    let s = match it.next() {
        Some(v) => v,
        None => process::exit(1),
    };

    // Behavior is undefined by the specification if n > s length; we do not guard against it.
    // Evaluate first n characters: 'R' => +1, anything else => -1.
    let mut total: i32 = 0;
    for ch in s.chars().take(n) {
        if ch == 'R' {
            total += 1;
        } else {
            total -= 1;
        }
    }

    // Output affirmative if total > 0, else negative (including equality).
    if total > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
