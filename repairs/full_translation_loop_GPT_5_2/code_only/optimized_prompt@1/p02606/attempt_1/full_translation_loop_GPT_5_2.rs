use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches scanf behavior for whitespace-separated ints).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let l: i32 = it.next().unwrap().parse().unwrap();
    let r: i32 = it.next().unwrap().parse().unwrap();
    let d: i32 = it.next().unwrap().parse().unwrap();

    // C uses signed int arithmetic and division truncates toward zero.
    // Rust's i32 division matches that (and will panic on division by zero,
    // whereas C is undefined; we keep the natural Rust behavior here).
    let ans: i32 = r / d - (l - 1) / d;

    // Exact output formatting: one integer followed by '\n'.
    print!("{}\n", ans);

    // fflush(stdout) equivalent: ensure output is flushed.
    use std::io::Write;
    io::stdout().flush().unwrap();
}
