use std::io::{self, Write};

fn rev(mut n: i32) -> i32 {
    let mut out = 0;

    while n > 0 {
        out = out * 10 + (n % 10);
        n /= 10;
    }

    out
}

fn main() {
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure stdout is flushed before reading input
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let mut i = 0;

    loop {
        if n - i >= 0 {
            if rev(n - i) == n - i {
                println!("{}", n - i);
                break;
            }
        }
        if rev(n + i) == n + i {
            println!("{}", n + i);
            break;
        }
        i += 1;
    }
}
