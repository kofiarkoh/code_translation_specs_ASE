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
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: i32 = match input.trim().parse() {
            Ok(num) if num > 0 => num,
            Ok(0) => break, // Termination condition
            _ => continue,   // Ignore invalid input
        };
        let ret = solve(n, 1, 1);
        println!("{}", ret);
    }
}
