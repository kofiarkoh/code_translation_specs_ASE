use std::io;

fn solve(n: i32, c: i32, low: i32) -> i32 {
    let mut ret = 0;
    let mut i = low;

    while i * i <= n {
        if n == i * i {
            ret += 1;
        } else if c < 4 {
            ret += solve(n - i * i, c + 1, i);
        }
        i += 1;
    }
    ret
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let n: i32 = match input.trim().parse() {
            Ok(num) if num > 0 => num,
            _ => break, // Terminate on non-positive integer
        };

        let ret = solve(n, 1, 1);
        println!("{}", ret);
    }
}
